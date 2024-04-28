use tests_api::alloc::ArenaAlloc;

use super::DoubleLinkedList;
use std::{fmt::Debug, ptr::NonNull};

struct InternalNode<T> {
    next: Option<NonNull<InternalNode<T>>>,
    prec: Option<NonNull<InternalNode<T>>>,
    value: T,
}
pub struct Implementation<'x, T> {
    head: Option<NonNull<InternalNode<T>>>,
    tail: Option<NonNull<InternalNode<T>>>,
    alloc: &'x ArenaAlloc,
}

pub struct Node<T>
where
    T: Copy + PartialEq + std::fmt::Debug,
{
    ptr: *mut InternalNode<T>,
}
impl<T: Copy + PartialEq + Debug> Node<T> {
    fn from_internal(internal: &InternalNode<T>) -> Self {
        Self {
            ptr: (internal as *const InternalNode<T>) as *mut InternalNode<T>,
        }
    }
}

impl<T: Copy + PartialEq + Debug> Copy for Node<T> {}
impl<T: Copy + PartialEq + Debug> Clone for Node<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: Copy + PartialEq + Debug> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}
impl<T: Copy + PartialEq + Debug> std::fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node").field("ptr", &self.ptr).finish()
    }
}

type Box<'x, T> = std::boxed::Box<T, &'x ArenaAlloc>;

impl<'x, T> Implementation<'x, T> {
    fn allocate(&mut self, value: T) -> NonNull<InternalNode<T>> {
        let b = Box::new_in(
            InternalNode {
                next: None,
                prec: None,
                value,
            },
            self.alloc,
        );
        unsafe { NonNull::new_unchecked(Box::into_raw(b)) }
    }
}

impl<'x, T: Copy + PartialEq + Debug> DoubleLinkedList<'x, T> for Implementation<'x, T> {
    type NodeRef = Node<T>;

    fn new(alloc: &'x ArenaAlloc, _capacity: usize) -> Self {
        Self {
            head: None,
            tail: None,
            alloc,
        }
    }
    fn insert_after(&mut self, _node: Self::NodeRef, _value: T) -> Self::NodeRef {
        todo!()
    }

    fn insert_before(&mut self, _node: Self::NodeRef, _value: T) -> Self::NodeRef {
        todo!()
    }

    fn push_back(&mut self, value: T) -> Self::NodeRef {
        if let Some(mut node) = self.tail {
            let mut new_node = self.allocate(value);
            unsafe {
                new_node.as_mut().prec = self.tail;
                node.as_mut().next = Some(new_node);
                self.tail = Some(new_node);
                Self::NodeRef::from_internal(new_node.as_ref())
            }
        } else {
            // first node
            let n = self.allocate(value);
            self.head = Some(n);
            self.tail = Some(n);
            unsafe { Self::NodeRef::from_internal(n.as_ref()) }
        }
    }

    fn push_front(&mut self, value: T) -> Self::NodeRef {
        if let Some(mut node) = self.head {
            let mut new_node = self.allocate(value);
            unsafe {
                new_node.as_mut().prec = self.head;
                node.as_mut().next = Some(new_node);
                self.head = Some(new_node);
                Self::NodeRef::from_internal(new_node.as_ref())
            }
        } else {
            // first node
            let n = self.allocate(value);
            self.head = Some(n);
            self.tail = Some(n);
            unsafe { Self::NodeRef::from_internal(n.as_ref()) }
        }
    }

    unsafe fn delete(&mut self, node: Self::NodeRef) {
        unsafe {
            let n = *(&node.ptr);
            let prec = (*n).prec;
            let next = (*n).next;
            if let Some(mut p) = prec {
                p.as_mut().next = next;
            }
            if let Some(mut n) = next {
                n.as_mut().prec = prec;
            }
            // temp solution - in reality we need to check if n is head or tail and then update them
            self.head = None;
            self.tail = None;
            let _ = Box::from_raw_in(n, self.alloc);
        }
    }

    fn next(&self, node: Self::NodeRef) -> Option<Self::NodeRef> {
        if let Some(next) = unsafe { (*node.ptr).next } {
            unsafe { Some(Self::NodeRef::from_internal(next.as_ref())) }
        } else {
            None
        }
    }

    fn prec(&self, node: Self::NodeRef) -> Option<Self::NodeRef> {
        if let Some(prec) = unsafe { (*node.ptr).prec } {
            unsafe { Some(Self::NodeRef::from_internal(prec.as_ref())) }
        } else {
            None
        }
    }

    fn first(&self) -> Option<Self::NodeRef> {
        if let Some(first) = self.head {
            unsafe { Some(Self::NodeRef::from_internal(first.as_ref())) }
        } else {
            None
        }
    }

    fn last(&self) -> Option<Self::NodeRef> {
        if let Some(last) = self.tail {
            unsafe { Some(Self::NodeRef::from_internal(last.as_ref())) }
        } else {
            None
        }
    }

    fn value(&self, node: Self::NodeRef) -> Option<&T> {
        Some(unsafe { &(*node.ptr).value })
    }

    fn value_mut(&mut self, node: Self::NodeRef) -> Option<&mut T> {
        Some(unsafe { &mut (*node.ptr).value })
    }
}
