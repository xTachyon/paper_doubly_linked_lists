use std::marker::PhantomData;

use super::DoubleLinkedList;

static mut GLOBAL_HANDLE_UNIQUE_ID: u32 = 0;

#[derive(Copy, Clone)]
pub struct Handle<T> {
    index: u32,
    unique_id: u32,
    _type: PhantomData<T>,
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
    head: Handle<T>,
    tail: Handle<T>,
}
impl<T> DoubleLinkedList<T> for Implementation<T> {
    type Node = Handle<T>;

    fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            head: Handle::INVALID,
            tail: Handle::INVALID,
        }
    }

    fn insert_after(&mut self, node: Self::Node, value: T) -> Self::Node {
        if self.data.is_empty() {
            return self.add_first_element(value);
        } else {
            todo!()
        }
    }

    fn insert_before(&mut self, node: Self::Node, value: T) -> Self::Node {
        if self.data.is_empty() {
            return self.add_first_element(value);
        } else {
            todo!()
        }
    }

    fn push_back(&mut self, value: T) -> Self::Node {
        if self.data.is_empty() {
            return self.add_first_element(value);
        } else {
            todo!()
        }
    }

    fn push_top(&mut self, value: T) -> Self::Node {
        if self.data.is_empty() {
            return self.add_first_element(value);
        } else {
            todo!()
        }
    }

    fn delete(&mut self, node: Self::Node) {
        todo!()
    }

    fn next(&self, node: Self::Node) -> Option<Self::Node> {
        if let Some(e) = self.element(node) {
            if e.next.is_valid() {
                return Some(e.next);
            } else {
                return None;
            }
        }
        None
    }

    fn prec(&self, node: Self::Node) -> Option<Self::Node> {
        if let Some(e) = self.element(node) {
            if e.prec.is_valid() {
                return Some(e.prec);
            } else {
                return None;
            }
        }
        None
    }

    fn first(&self) -> Option<Self::Node> {
        if self.head.is_valid() {
            return Some(self.head);
        }
        None
    }

    fn last(&self) -> Option<Self::Node> {
        if self.tail.is_valid() {
            return Some(self.tail);
        }
        None
    }

    fn value(&self, node: Self::Node) -> Option<&T> {
        if let Some(e) = self.element(node) {
            return Some(&e.value);
        }
        None
    }

    fn value_mut(&mut self, node: Self::Node) -> Option<&mut T> {
        if let Some(e) = self.element_mut(node) {
            return Some(&mut e.value);
        }
        None
    }
}

impl<T> Implementation<T> {
    fn add_first_element(&mut self, value: T) -> Handle<T> {
        // assume self.data is empty
        let h = Handle::new(0);
        self.data.push(Some(Element {
            next: Handle::INVALID,
            prec: Handle::INVALID,
            value,
            unique_id: h.unique_id,
        }));
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
