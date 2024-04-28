use std::collections::LinkedList;

use tests_api::TheAlloc;

use super::double_linked_list::DoubleLinkedList;

pub struct Implementation<T> {
    _list: LinkedList<T>,
}

impl<'x, T> DoubleLinkedList<'x, T> for Implementation<T> {
    type NodeRef = *const u8;

    fn new(_alloc: &TheAlloc, _capacity: usize) -> Self {
        todo!()
    }

    fn insert_after(&mut self, _node: Self::NodeRef, _value: T) -> Self::NodeRef {
        todo!()
    }

    fn insert_before(&mut self, _node: Self::NodeRef, _value: T) -> Self::NodeRef {
        todo!()
    }

    fn push_back(&mut self, _value: T) -> Self::NodeRef {
        todo!()
    }

    fn push_front(&mut self, _value: T) -> Self::NodeRef {
        todo!()
    }

    unsafe fn delete(&mut self, _node: Self::NodeRef) {
        todo!()
    }

    fn next(&self, _node: Self::NodeRef) -> Option<Self::NodeRef> {
        todo!()
    }

    fn prec(&self, _node: Self::NodeRef) -> Option<Self::NodeRef> {
        todo!()
    }

    fn first(&self) -> Option<Self::NodeRef> {
        todo!()
    }

    fn last(&self) -> Option<Self::NodeRef> {
        todo!()
    }

    fn value(&self, _node: Self::NodeRef) -> Option<&T> {
        todo!()
    }

    fn value_mut(&mut self, _node: Self::NodeRef) -> Option<&mut T> {
        todo!()
    }
}