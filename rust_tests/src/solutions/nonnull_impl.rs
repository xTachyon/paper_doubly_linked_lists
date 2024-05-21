use tests_api::TheAlloc;

use super::DoubleLinkedList;
use std::{fmt::Debug, ptr::NonNull};


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Node<T> {
    value: T,
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
}

pub struct Implementation<'x, T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    alloc: &'x TheAlloc,
}

impl<'x, T> Implementation<'x, T> {
    fn allocate_node(&mut self, value: T) -> NonNull<Node<T>> {
        NonNull::new(Box::into_raw(Box::new_in(
            Node {
                value,
                prev: None,
                next: None,
            },
            self.alloc,
        )))
        .expect("Failed to allocate node")
    }

    fn deallocate_node(&mut self, node: NonNull<Node<T>>) {
        unsafe {
            Box::from_raw_in(node.as_ptr(), self.alloc);
        }
    }
}

impl<'x, T: Copy + PartialEq + Debug> DoubleLinkedList<'x, T> for Implementation<'x, T> {
    type NodeRef = NonNull<Node<T>>;

    fn new(alloc: &'x TheAlloc, _capacity: usize) -> Self {
        Implementation {
            head: None,
            tail: None,
            alloc,
        }
    }

    fn insert_after(&mut self, mut node: Self::NodeRef, value: T) -> Self::NodeRef {
        let mut new_node = self.allocate_node(value);
        unsafe {
            new_node.as_mut().prev = Some(node);
            new_node.as_mut().next = node.as_ref().next;

            if let Some(mut next_node) = node.as_ref().next {
                next_node.as_mut().prev = Some(new_node);
            } else {
                self.tail = Some(new_node);
            }

            node.as_mut().next = Some(new_node);
        }
        new_node
    }

    fn insert_before(&mut self, mut node: Self::NodeRef, value: T) -> Self::NodeRef {
        let mut new_node = self.allocate_node(value);
        unsafe {
            new_node.as_mut().next = Some(node);
            new_node.as_mut().prev = node.as_ref().prev;

            if let Some(mut prev_node) = node.as_ref().prev {
                prev_node.as_mut().next = Some(new_node);
            } else {
                self.head = Some(new_node);
            }

            node.as_mut().prev = Some(new_node);
        }
        new_node
    }

    fn push_back(&mut self, value: T) -> Self::NodeRef {
        let mut new_node = self.allocate_node(value);
        unsafe {
            new_node.as_mut().prev = self.tail;

            if let Some(mut tail_node) = self.tail {
                tail_node.as_mut().next = Some(new_node);
            } else {
                self.head = Some(new_node);
            }

            self.tail = Some(new_node);
        }
        new_node
    }

    fn push_front(&mut self, value: T) -> Self::NodeRef {
        let mut new_node = self.allocate_node(value);
        unsafe {
            new_node.as_mut().next = self.head;

            if let Some(mut head_node) = self.head {
                head_node.as_mut().prev = Some(new_node);
            } else {
                self.tail = Some(new_node);
            }

            self.head = Some(new_node);
        }
        new_node
    }

    unsafe fn delete(&mut self, node: Self::NodeRef) {
        unsafe {
            if let Some(mut prev_node) = node.as_ref().prev {
                prev_node.as_mut().next = node.as_ref().next;
            } else {
                self.head = node.as_ref().next;
            }

            if let Some(mut next_node) = node.as_ref().next {
                next_node.as_mut().prev = node.as_ref().prev;
            } else {
                self.tail = node.as_ref().prev;
            }

            self.deallocate_node(node);
        }
    }

    fn next(&self, node: Self::NodeRef) -> Option<Self::NodeRef> {
        unsafe { node.as_ref().next }
    }

    fn prec(&self, node: Self::NodeRef) -> Option<Self::NodeRef> {
        unsafe { node.as_ref().prev }
    }

    fn first(&self) -> Option<Self::NodeRef> {
        self.head
    }

    fn last(&self) -> Option<Self::NodeRef> {
        self.tail
    }

    fn value(&self, node: Self::NodeRef) -> Option<&T> {
        unsafe { Some(&node.as_ref().value) }
    }

    fn value_mut(&mut self, mut node: Self::NodeRef) -> Option<&mut T> {
        unsafe { Some(&mut node.as_mut().value) }
    }
}
