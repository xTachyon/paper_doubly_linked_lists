use tests_api::TheAlloc;
use super::DoubleLinkedList;

struct Element<T> {
    next: u32,
    prec: u32,
    value: T,
}
pub struct Implementation<'x, T> {
    data: Vec<Option<Element<T>>, &'x TheAlloc>,
    free_list: Vec<u32>,
    // TODO: rename to first, last
    head: u32,
    tail: u32,
}
impl<'x, T> DoubleLinkedList<'x, T> for Implementation<'x, T> {
    type NodeRef = u32;

    fn new(alloc: &'x TheAlloc, capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity_in(capacity, alloc),
            free_list: Vec::with_capacity(32),
            head: u32::MAX,
            tail: u32::MAX,
        }
    }

    fn insert_after(&mut self, node: Self::NodeRef, value: T) -> Self::NodeRef {
        if self.data.is_empty() {
            return self.add_first_element(value);
        } else {
            let new_node = self.allocate(value);
            let cnode_next = self.data[node as usize].as_ref().unwrap().next;
            self.link(node, new_node);
            self.link(new_node, cnode_next);
            if node == self.tail {
                self.tail = new_node;
            }
            new_node
        }
    }

    fn insert_before(&mut self, node: Self::NodeRef, value: T) -> Self::NodeRef {
        if self.data.is_empty() {
            return self.add_first_element(value);
        } else if node != u32::MAX {
            let new_node = self.allocate(value);
            let cnode_prec = self.data[node as usize].as_ref().unwrap().prec;
            self.link(new_node, node);
            self.link(cnode_prec, new_node);
            if node == self.head {
                self.head = new_node;
            }
            new_node
        } else {
            u32::MAX
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
        if node as usize >= count {
            return;
        }
        let p;
        let n;
        if let Some(elem) = self.data[node as usize].as_ref() {
            p = elem.prec;
            n = elem.next;
        } else {
            return;
        }
        self.link(p, n);
        if node == self.head {
            self.head = n;
        }
        if node == self.tail {
            self.tail = p;
        }
        self.data[node as usize] = None;
        self.free_list.push(node);
    }

    fn next(&self, node: Self::NodeRef) -> Option<Self::NodeRef> {
        if let Some(e) = self.element(node) {
            if e.next != u32::MAX {
                return Some(e.next);
            } else {
                return None;
            }
        }
        None
    }

    fn prec(&self, node: Self::NodeRef) -> Option<Self::NodeRef> {
        if let Some(e) = self.element(node) {
            if e.prec != u32::MAX {
                return Some(e.prec);
            } else {
                return None;
            }
        }
        None
    }

    fn first(&self) -> Option<Self::NodeRef> {
        if self.head != u32::MAX {
            return Some(self.head);
        }
        None
    }

    fn last(&self) -> Option<Self::NodeRef> {
        if self.tail != u32::MAX {
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

impl<'x, T> Implementation<'x, T> {
    fn allocate(&mut self, value: T) -> u32 {
        let allocated = self.data.len();
        let idx = self
            .free_list
            .pop()
            .map(|f| f as usize)
            .unwrap_or(allocated);
        if idx < allocated {
            self.data[idx] = Some(Element {
                next: u32::MAX,
                prec: u32::MAX,
                value,
            });
        } else {
            self.data.push(Some(Element {
                next: u32::MAX,
                prec: u32::MAX,
                value,
            }));
        }
        idx as u32
    }
    fn link(&mut self, n1: u32, n2: u32) {
        let idx1 = n1 as usize;
        let idx2 = n2 as usize;
        let count = self.data.len();
        if idx1 < count {
            self.data[idx1].as_mut().unwrap().next = n2;
        }
        if idx2 < count {
            self.data[idx2].as_mut().unwrap().prec = n1;
        }
    }
    fn add_first_element(&mut self, value: T) -> u32 {
        // assume self.data is empty
        self.data.push(Some(Element {
            next: u32::MAX,
            prec: u32::MAX,
            value,
        }));
        self.head = 0;
        self.tail = 0;
        0
    }
    fn element(&self, handle: u32) -> Option<&Element<T>> {
        let index = handle as usize;
        if index < self.data.len() {
            self.data[index].as_ref()
        } else {
            None
        }
    }

    fn element_mut(&mut self, handle: u32) -> Option<&mut Element<T>> {
        let index = handle as usize;
        if index < self.data.len() {
            self.data[index].as_mut()
        } else {
            None
        }
    }
}
