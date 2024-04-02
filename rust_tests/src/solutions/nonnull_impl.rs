use super::DoubleLinkedList;
use std::{fmt::Debug, ptr::NonNull};

struct InternalNode<T> {
    next: Option<NonNull<InternalNode<T>>>,
    prec: Option<NonNull<InternalNode<T>>>,
    value: T,
}
pub struct Implementation<T> {
    head: Option<NonNull<InternalNode<T>>>,
    tail: Option<NonNull<InternalNode<T>>>,
}


pub struct Node<T> where T: Copy + PartialEq + std::fmt::Debug {
    ptr: *mut InternalNode<T>
}
impl<T: Copy+PartialEq+Debug> Node<T> {
    fn from_internal(internal: &InternalNode<T>)->Self {
        Self {
            ptr: (internal as *const InternalNode<T>) as *mut InternalNode<T> 
        }
    }
}

impl<T: Copy+PartialEq+Debug> Copy for Node<T> {}
impl<T: Copy+PartialEq+Debug> Clone for Node<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T:Copy+PartialEq+Debug> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}
impl<T:Copy+PartialEq+Debug> std::fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("ptr", &self.ptr)
            .finish()
    }
}

// impl Implementation {
//     pub fn new(_capacity: usize) -> Self {
//         let start_node = Box::into_raw(Box::new(InternalNode {
//             next: None,
//             prec: None,
//             value: 0,
//         }));
//         Self {
//             head: start_node,
//             tail: start_node,
//         }
//     }
//     pub fn add(&mut self, value: u64) {
//         let new_node = Box::into_raw(Box::new(InternalNode {
//             next: None,
//             prec: None,
//             value,
//         }));
//         unsafe {
//             (*self.tail).next = Some(NonNull::new_unchecked(new_node));
//             (*new_node).prec = Some(NonNull::new_unchecked(self.tail));
//         }
//         self.tail = new_node;
//     }
//     pub fn sum_all(&self) -> u64 {
//         let mut sum = 0;
//         let mut current = self.head;
//         loop {
//             unsafe {
//                 sum += (*current).value;
//                 if let Some(next) = (*current).next {
//                     current = next.as_ptr();
//                 } else {
//                     break;
//                 }
//             }
//         }
//         sum
//     }
// }

impl<T:Copy+PartialEq+Debug> DoubleLinkedList<T> for Implementation<T>  {
    type Node = Node<T>;

    fn new(capacity: usize) -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
    fn insert_after(&mut self, node: Self::Node, value: T) -> Self::Node {
        todo!()
    }

    fn insert_before(&mut self, node: Self::Node, value: T) -> Self::Node {
        todo!()
    }

    fn push_back(&mut self, value: T) -> Self::Node {
        todo!()
    }

    fn push_top(&mut self, value: T) -> Self::Node {
        todo!()
    }

    fn delete(&mut self, node: Self::Node) {
        todo!()
    }

    fn next(&self, node: Self::Node) -> Option<Self::Node> {
        todo!()
    }

    fn prec(&self, node: Self::Node) -> Option<Self::Node> {
        todo!()
    }

    fn first(&self) -> Option<Self::Node> {
        if let Some(first) = self.head {
            unsafe { Some(Self::Node::from_internal(first.as_ref())) }
        } else {
            None
        }
    }

    fn last(&self) -> Option<Self::Node> {
        if let Some(last) = self.tail {
            unsafe { Some(Self::Node::from_internal(last.as_ref())) }
        } else {
            None
        }
    }

    fn value(&self, node: Self::Node) -> Option<&T> {
        Some(unsafe { &(*node.ptr).value })
    }

    fn value_mut(&mut self, node: Self::Node) -> Option<&mut T> {
        Some(unsafe { &mut (*node.ptr).value })
    }
}
