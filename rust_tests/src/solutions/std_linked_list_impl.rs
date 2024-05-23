use std::ptr::NonNull;

use std_stuff::linked_list::{LinkedList, Node};
use tests_api::TheAlloc;

use super::double_linked_list::DoubleLinkedList;

pub struct Implementation<T> {
    nodes: LinkedList<T, &'static TheAlloc>,
}

impl<'x, T> DoubleLinkedList<'x, T> for Implementation<T> {
    type NodeRef = NonNull<Node<T>>;

    fn new(alloc: &'static TheAlloc, _capacity: usize) -> Self {
        Self {
            nodes: LinkedList::new_in(alloc),
        }
    }

    fn push_back(&mut self, value: T) -> Self::NodeRef {
        self.nodes.push_back(value)
    }

    fn push_front(&mut self, value: T) -> Self::NodeRef {
        self.nodes.push_front(value)
    }

    unsafe fn delete(&mut self, node: Self::NodeRef) {
        let mut index = 0;
        let mut current = self.nodes.front_raw();
        while let Some(x) = current {
            if x == node {
                break;
            }
            index += 1;
            current = x.as_ref().next;
        }
        let current = current.unwrap();
        assert_eq!(current, node);
        self.nodes.remove(index);
    }

    fn next(&self, node: Self::NodeRef) -> Option<Self::NodeRef> {
        unsafe { node.as_ref().next }
    }

    fn prec(&self, node: Self::NodeRef) -> Option<Self::NodeRef> {
        unsafe { node.as_ref().prev }
    }

    fn first(&self) -> Option<Self::NodeRef> {
        self.nodes.front_raw()
    }

    fn last(&self) -> Option<Self::NodeRef> {
        self.nodes.back_raw()
    }

    fn insert_after(&mut self, _node: Self::NodeRef, _value: T) -> Self::NodeRef {
        todo!()
    }

    fn insert_before(&mut self, _node: Self::NodeRef, _value: T) -> Self::NodeRef {
        todo!()
    }

    fn value(&self, node: Self::NodeRef) -> Option<&T> {
        unsafe { Some(&node.as_ref().element) }
    }

    fn value_mut(&mut self, mut node: Self::NodeRef) -> Option<&mut T> {
        unsafe { Some(&mut node.as_mut().element) }
    }
}
