use super::DoubleLinkedList;
use std::ptr::NonNull;

struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    prec: Option<NonNull<Node<T>>>,
    value: T,
}
pub struct Implementation<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
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

impl<T> DoubleLinkedList<T> for Implementation<T> {
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
        todo!()
    }

    fn last(&self) -> Option<Self::Node> {
        todo!()
    }

    fn value(&self, node: Self::Node) -> Option<&T> {
        todo!()
    }

    fn value_mut(&mut self, node: Self::Node) -> Option<&mut T> {
        todo!()
    }
}
