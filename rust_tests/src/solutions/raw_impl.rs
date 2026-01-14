use super::DoubleLinkedList;
use core::fmt::Debug;
use std::ptr;
use tests_api::TheAlloc;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Node<T> {
    value: T,
    prev: *mut Node<T>,
    next: *mut Node<T>,
}

pub struct Implementation<'x, T> {
    first: *mut Node<T>,
    last: *mut Node<T>,
    alloc: &'x TheAlloc,
}

impl<'x, T> Implementation<'x, T> {
    fn allocate_node(&mut self, value: T) -> *mut Node<T> {
        Box::into_raw(Box::new_in(
            Node {
                value,
                prev: ptr::null_mut(),
                next: ptr::null_mut(),
            },
            self.alloc,
        ))
    }

    fn deallocate_node(&mut self, node: *mut Node<T>) {
        unsafe {
            Box::from_raw_in(node, self.alloc);
        }
    }
}

impl<'x, T> DoubleLinkedList<'x, T> for Implementation<'x, T> {
    type NodeRef = *mut Node<T>;

    fn new(alloc: &'x TheAlloc, _capacity: usize) -> Self {
        Implementation {
            first: ptr::null_mut(),
            last: ptr::null_mut(),
            alloc,
        }
    }

    fn insert_after(&mut self, node: Self::NodeRef, value: T) -> Self::NodeRef {
        if node.is_null() {
            panic!("insert_after called with null node");
        }
        let new_node = self.allocate_node(value);
        unsafe {
            (*new_node).prev = node;
            (*new_node).next = (*node).next;

            if !(*node).next.is_null() {
                (*(*node).next).prev = new_node;
            } else {
                self.last = new_node;
            }

            (*node).next = new_node;
        }
        new_node
    }

    fn insert_before(&mut self, node: Self::NodeRef, value: T) -> Self::NodeRef {
        if node.is_null() {
            panic!("insert_before called with null node");
        }
        let new_node = self.allocate_node(value);
        unsafe {
            (*new_node).next = node;
            (*new_node).prev = (*node).prev;

            if !(*node).prev.is_null() {
                (*(*node).prev).next = new_node;
            } else {
                self.first = new_node;
            }

            (*node).prev = new_node;
        }
        new_node
    }

    fn push_back(&mut self, value: T) -> Self::NodeRef {
        let new_node = self.allocate_node(value);
        unsafe {
            (*new_node).prev = self.last;

            if !self.last.is_null() {
                (*self.last).next = new_node;
            } else {
                self.first = new_node;
            }

            self.last = new_node;
        }
        new_node
    }

    fn push_front(&mut self, value: T) -> Self::NodeRef {
        let new_node = self.allocate_node(value);
        unsafe {
            (*new_node).next = self.first;

            if !self.first.is_null() {
                (*self.first).prev = new_node;
            } else {
                self.last = new_node;
            }

            self.first = new_node;
        }
        new_node
    }

    unsafe fn delete(&mut self, node: Self::NodeRef) {
        unsafe {
            if !(*node).prev.is_null() {
                (*(*node).prev).next = (*node).next;
            } else {
                self.first = (*node).next;
            }

            if !(*node).next.is_null() {
                (*(*node).next).prev = (*node).prev;
            } else {
                self.last = (*node).prev;
            }

            self.deallocate_node(node);
        }
    }

    fn next(&self, node: Self::NodeRef) -> Option<Self::NodeRef> {
        unsafe {
            if (*node).next.is_null() {
                None
            } else {
                Some((*node).next)
            }
        }
    }

    fn prec(&self, node: Self::NodeRef) -> Option<Self::NodeRef> {
        unsafe {
            if (*node).prev.is_null() {
                None
            } else {
                Some((*node).prev)
            }
        }
    }

    fn first(&self) -> Option<Self::NodeRef> {
        if self.first.is_null() {
            None
        } else {
            Some(self.first)
        }
    }

    fn last(&self) -> Option<Self::NodeRef> {
        if self.last.is_null() {
            None
        } else {
            Some(self.last)
        }
    }

    fn value(&self, node: Self::NodeRef) -> Option<&T> {
        unsafe {
            if node.is_null() {
                None
            } else {
                Some(&(*node).value)
            }
        }
    }

    fn value_mut(&mut self, node: Self::NodeRef) -> Option<&mut T> {
        unsafe {
            if node.is_null() {
                None
            } else {
                Some(&mut (*node).value)
            }
        }
    }
}
