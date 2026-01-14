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
        self.nodes.remove_extremely_unsafe(node);
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

    fn insert_after(&mut self, node: Self::NodeRef, value: T) -> Self::NodeRef {
        let target_elem_ptr = unsafe { &(*node.as_ptr()).element as *const T };
        let mut cursor = self.nodes.cursor_front_mut();
        loop {
            if let Some(curr) = cursor.current() {
                if (curr as *const T) == target_elem_ptr {
                    cursor.insert_after(value);
                    return unsafe { (*node.as_ptr()).next.unwrap() };
                }
                cursor.move_next();
            } else {
                break;
            }
        }
        panic!("insert_after called with a node that is not in this list")
    }

    fn insert_before(&mut self, node: Self::NodeRef, value: T) -> Self::NodeRef {
        let target_elem_ptr = unsafe { &(*node.as_ptr()).element as *const T };
        let mut cursor = self.nodes.cursor_front_mut();
        loop {
            if let Some(curr) = cursor.current() {
                if (curr as *const T) == target_elem_ptr {
                    cursor.insert_before(value);
                    return unsafe { (*node.as_ptr()).prev.unwrap() };
                }
                cursor.move_next();
            } else {
                break;
            }
        }
        panic!("insert_before called with a node that is not in this list")
    }

    fn value(&self, node: Self::NodeRef) -> Option<&T> {
        unsafe { Some(&node.as_ref().element) }
    }

    fn value_mut(&mut self, mut node: Self::NodeRef) -> Option<&mut T> {
        unsafe { Some(&mut node.as_mut().element) }
    }
}
