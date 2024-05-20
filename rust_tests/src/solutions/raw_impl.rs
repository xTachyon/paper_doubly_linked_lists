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
    head: *mut Node<T>,
    tail: *mut Node<T>,
    alloc: &'x TheAlloc,
}

impl<'x, T> Implementation<'x, T> {
    fn allocate_node(value: T) -> *mut Node<T> {
        Box::into_raw(Box::new(Node {
            value,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }))
    }

    fn deallocate_node(node: *mut Node<T>) {
        unsafe {
            Box::from_raw(node);
        }
    }
}

impl<'x, T: Copy + PartialEq + Debug> DoubleLinkedList<'x, T> for Implementation<'x, T> {
    type NodeRef = *mut Node<T>;

    fn new(alloc: &'x TheAlloc, _capacity: usize) -> Self {
        Implementation {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            alloc,
        }
    }

    fn insert_after(&mut self, node: Self::NodeRef, value: T) -> Self::NodeRef {
        let new_node = Self::allocate_node(value);
        unsafe {
            (*new_node).prev = node;
            (*new_node).next = (*node).next;

            if !(*node).next.is_null() {
                (*(*node).next).prev = new_node;
            } else {
                self.tail = new_node;
            }

            (*node).next = new_node;
        }
        new_node
    }

    fn insert_before(&mut self, node: Self::NodeRef, value: T) -> Self::NodeRef {
        let new_node = Self::allocate_node(value);
        unsafe {
            (*new_node).next = node;
            (*new_node).prev = (*node).prev;

            if !(*node).prev.is_null() {
                (*(*node).prev).next = new_node;
            } else {
                self.head = new_node;
            }

            (*node).prev = new_node;
        }
        new_node
    }

    fn push_back(&mut self, value: T) -> Self::NodeRef {
        let new_node = Self::allocate_node(value);
        unsafe {
            (*new_node).prev = self.tail;

            if !self.tail.is_null() {
                (*self.tail).next = new_node;
            } else {
                self.head = new_node;
            }

            self.tail = new_node;
        }
        new_node
    }

    fn push_front(&mut self, value: T) -> Self::NodeRef {
        let new_node = Self::allocate_node(value);
        unsafe {
            (*new_node).next = self.head;

            if !self.head.is_null() {
                (*self.head).prev = new_node;
            } else {
                self.tail = new_node;
            }

            self.head = new_node;
        }
        new_node
    }

    unsafe fn delete(&mut self, node: Self::NodeRef) {
        unsafe {
            if !(*node).prev.is_null() {
                (*(*node).prev).next = (*node).next;
            } else {
                self.head = (*node).next;
            }

            if !(*node).next.is_null() {
                (*(*node).next).prev = (*node).prev;
            } else {
                self.tail = (*node).prev;
            }

            Self::deallocate_node(node);
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
        if self.head.is_null() {
            None
        } else {
            Some(self.head)
        }
    }

    fn last(&self) -> Option<Self::NodeRef> {
        if self.tail.is_null() {
            None
        } else {
            Some(self.tail)
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
