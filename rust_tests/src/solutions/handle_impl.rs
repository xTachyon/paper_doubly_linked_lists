use std::marker::PhantomData;

use super::DoubleLinkedList;

static mut GLOBAL_HANDLE_UNIQUE_ID: u32 = 0;

#[derive(Eq)]
pub struct Handle<T> {
    index: u32,
    unique_id: u32,
    _type: PhantomData<T>,
}
impl<T> Copy for Handle<T> {}
impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.unique_id == other.unique_id
    }
}
impl<T> std::fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Handle")
            .field("index", &self.index)
            .field("unique_id", &self.unique_id)
            .finish()
    }
}
impl<T> Handle<T> {
    pub const INVALID: Handle<T> = Handle {
        index: 0xFFFFFFFF,
        unique_id: 0xFFFFFFFF,
        _type: PhantomData,
    };
    pub fn new(index: u32) -> Handle<T> {
        let unique_id = unsafe {
            GLOBAL_HANDLE_UNIQUE_ID = (GLOBAL_HANDLE_UNIQUE_ID + 1) % 0xFFFF_FFFE;
            GLOBAL_HANDLE_UNIQUE_ID
        };
        Self {
            index,
            unique_id,
            _type: PhantomData,
        }
    }
    #[inline(always)]
    fn is_valid(&self) -> bool {
        self.unique_id != 0xFFFF_FFFF
    }
}
struct Element<T> {
    next: Handle<T>,
    prec: Handle<T>,
    value: T,
    unique_id: u32,
}
pub struct Implementation<T> {
    data: Vec<Option<Element<T>>>,
    // TODO: rename to first, last
    head: Handle<T>,
    tail: Handle<T>,
}
impl<T> DoubleLinkedList<T> for Implementation<T> {
    type NodeRef = Handle<T>;

    fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            head: Handle::INVALID,
            tail: Handle::INVALID,
        }
    }

    fn insert_after(&mut self, node: Self::NodeRef, value: T) -> Self::NodeRef {
        if self.data.is_empty() {
            return self.add_first_element(value);
        } else {
            let new_node = self.allocate(value);
            let cnode_next = self.data[node.index as usize].as_ref().unwrap().next;
            self.link(node, new_node);
            self.link(new_node, cnode_next);
            if (node.unique_id == self.tail.unique_id) && (node.index == self.tail.index) {
                self.tail = new_node;
            }
            new_node
        }
    }

    fn insert_before(&mut self, node: Self::NodeRef, value: T) -> Self::NodeRef {
        if self.data.is_empty() {
            return self.add_first_element(value);
        } else {
            if node.is_valid() {
                let new_node = self.allocate(value);
                let cnode_prec = self.data[node.index as usize].as_ref().unwrap().prec;
                self.link(new_node, node);
                self.link(cnode_prec, new_node);
                if (node.unique_id == self.head.unique_id) && (node.index == self.head.index) {
                    self.head = new_node;
                }
                new_node
            } else {
                Handle::INVALID
            }
        }
    }

    fn push_back(&mut self, value: T) -> Self::NodeRef {
        if self.data.is_empty() {
            return self.add_first_element(value);
        } else {
            let node = self.allocate(value);
            self.link(self.tail, node);
            self.tail = node;
            node
        }
    }

    fn push_front(&mut self, value: T) -> Self::NodeRef {
        if self.data.is_empty() {
            return self.add_first_element(value);
        } else {
            let node = self.allocate(value);
            self.link(node, self.head);
            self.head = node;
            node
        }
    }

    unsafe fn delete(&mut self, node: Self::NodeRef) {
        let count = self.data.len();
        if node.index as usize >= count {
            return;
        }
        let p;
        let n;
        if let Some(elem) = self.data[node.index as usize].as_ref() {
            p = elem.prec;
            n = elem.next;
        } else {
            return;
        }
        self.link(p, n);
        if (node.index == self.head.index) && (node.unique_id == self.head.unique_id) {
            self.head = n;
        }
        if (node.index == self.tail.index) && (node.unique_id == self.tail.unique_id) {
            self.tail = p;
        }
        self.data[node.index as usize] = None;
    }

    fn next(&self, node: Self::NodeRef) -> Option<Self::NodeRef> {
        if let Some(e) = self.element(node) {
            if e.next.is_valid() {
                return Some(e.next);
            } else {
                return None;
            }
        }
        None
    }

    fn prec(&self, node: Self::NodeRef) -> Option<Self::NodeRef> {
        if let Some(e) = self.element(node) {
            if e.prec.is_valid() {
                return Some(e.prec);
            } else {
                return None;
            }
        }
        None
    }

    fn first(&self) -> Option<Self::NodeRef> {
        if self.head.is_valid() {
            return Some(self.head);
        }
        None
    }

    fn last(&self) -> Option<Self::NodeRef> {
        if self.tail.is_valid() {
            return Some(self.tail);
        }
        None
    }

    fn value(&self, node: Self::NodeRef) -> Option<&T> {
        if let Some(e) = self.element(node) {
            return Some(&e.value);
        }
        None
    }

    fn value_mut(&mut self, node: Self::NodeRef) -> Option<&mut T> {
        if let Some(e) = self.element_mut(node) {
            return Some(&mut e.value);
        }
        None
    }
}

impl<T> Implementation<T> {
    fn allocate(&mut self, value: T) -> Handle<T> {
        let idx = self.data.len();
        let h = Handle::new(idx as u32);
        self.data.push(Some(Element {
            next: Handle::INVALID,
            prec: Handle::INVALID,
            value,
            unique_id: h.unique_id,
        }));
        h
    }
    fn link(&mut self, n1: Handle<T>, n2: Handle<T>) {
        let idx1 = n1.index as usize;
        let idx2 = n2.index as usize;
        let count = self.data.len();
        if idx1 < count {
            self.data[idx1].as_mut().unwrap().next = n2;
        }
        if idx2 < count {
            self.data[idx2].as_mut().unwrap().prec = n1;
        }
    }
    fn add_first_element(&mut self, value: T) -> Handle<T> {
        // assume self.data is empty
        let h = Handle::new(0);
        self.data.push(Some(Element {
            next: Handle::INVALID,
            prec: Handle::INVALID,
            value,
            unique_id: h.unique_id,
        }));
        self.head = h;
        self.tail = h;
        h
    }
    fn element(&self, handle: Handle<T>) -> Option<&Element<T>> {
        let index = handle.index as usize;
        if index < self.data.len() {
            if let Some(obj) = &self.data[index] {
                if obj.unique_id == handle.unique_id {
                    self.data[index].as_ref()
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn element_mut(&mut self, handle: Handle<T>) -> Option<&mut Element<T>> {
        let index = handle.index as usize;
        if index < self.data.len() {
            if let Some(obj) = &mut self.data[index] {
                if obj.unique_id == handle.unique_id {
                    self.data[index].as_mut()
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}
