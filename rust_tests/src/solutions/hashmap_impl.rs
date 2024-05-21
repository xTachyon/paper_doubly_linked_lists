use super::DoubleLinkedList;
use std::fmt::Debug;
use hashbrown::{hash_map::DefaultHashBuilder, HashMap};
use tests_api::TheAlloc;

#[derive(Debug)]
struct Node<T> {
    value: T,
    prev: Option<usize>,
    next: Option<usize>,
}

#[derive(Debug)]
pub struct Implementation<T> {
    nodes: HashMap<usize, Node<T>, DefaultHashBuilder, &'static TheAlloc>,
    head: Option<usize>,
    tail: Option<usize>,
    key_index: usize,
}

impl<T> Implementation<T> {
    fn allocate_node(&mut self, value: T) -> usize {
        let key = self.key_index;
        self.key_index += 1;
        self.nodes.insert(
            key,
            Node {
                value,
                prev: None,
                next: None,
            },
        );
        key
    }
}

impl<'x, T> DoubleLinkedList<'x, T> for Implementation<T> {
    type NodeRef = usize;

    fn new(alloc: &'static TheAlloc, capacity: usize) -> Self {
        Implementation {
            nodes: HashMap::with_capacity_in(capacity, alloc),
            head: None,
            tail: None,
            key_index: 0,
        }
    }

    fn insert_after(&mut self, node: Self::NodeRef, value: T) -> Self::NodeRef {
        if !self.nodes.contains_key(&node) {
            return usize::MAX;
        }

        let node_next = self.nodes.get_mut(&node).unwrap().next;
        let new_node = self.allocate_node(value);
        if let Some(next) = node_next {
            self.nodes.get_mut(&next).unwrap().prev = Some(new_node);
        } else {
            self.tail = Some(new_node);
        }
        self.nodes.get_mut(&new_node).unwrap().prev = Some(node);
        self.nodes.get_mut(&new_node).unwrap().next = node_next;
        self.nodes.get_mut(&node).unwrap().next = Some(new_node);

        new_node
    }

    fn insert_before(&mut self, node: Self::NodeRef, value: T) -> Self::NodeRef {
        if !self.nodes.contains_key(&node) {
            return usize::MAX;
        }
        let node_prev = self.nodes.get_mut(&node).unwrap().prev;
        let new_node = self.allocate_node(value);
        if let Some(prev) = node_prev {
            self.nodes.get_mut(&prev).unwrap().next = Some(new_node);
        } else {
            self.head = Some(new_node);
        }
        self.nodes.get_mut(&new_node).unwrap().next = Some(node);
        self.nodes.get_mut(&new_node).unwrap().prev = node_prev;
        self.nodes.get_mut(&node).unwrap().prev = Some(new_node);

        new_node
    }

    fn push_back(&mut self, value: T) -> Self::NodeRef {
        let new_node = self.allocate_node(value);
        if let Some(tail) = self.tail {
            self.nodes.get_mut(&tail).unwrap().next = Some(new_node);
            self.nodes.get_mut(&new_node).unwrap().prev = Some(tail);
        } else {
            self.head = Some(new_node);
        }
        self.tail = Some(new_node);

        new_node
    }

    fn push_front(&mut self, value: T) -> Self::NodeRef {
        let new_node = self.allocate_node(value);
        if let Some(head) = self.head {
            self.nodes.get_mut(&head).unwrap().prev = Some(new_node);
            self.nodes.get_mut(&new_node).unwrap().next = Some(head);
        } else {
            self.tail = Some(new_node);
        }
        self.head = Some(new_node);

        new_node
    }

    unsafe fn delete(&mut self, node: Self::NodeRef) {
        if let Some(node_ref) = self.nodes.remove(&node) {
            if let Some(prev) = node_ref.prev {
                self.nodes.get_mut(&prev).unwrap().next = node_ref.next;
            } else {
                self.head = node_ref.next;
            }

            if let Some(next) = node_ref.next {
                self.nodes.get_mut(&next).unwrap().prev = node_ref.prev;
            } else {
                self.tail = node_ref.prev;
            }
        }
    }

    fn next(&self, node: Self::NodeRef) -> Option<Self::NodeRef> {
        self.nodes.get(&node).and_then(|node_ref| node_ref.next)
    }

    fn prec(&self, node: Self::NodeRef) -> Option<Self::NodeRef> {
        self.nodes.get(&node).and_then(|node_ref| node_ref.prev)
    }

    fn first(&self) -> Option<Self::NodeRef> {
        self.head
    }

    fn last(&self) -> Option<Self::NodeRef> {
        self.tail
    }

    fn value(&self, node: Self::NodeRef) -> Option<&T> {
        self.nodes.get(&node).map(|node_ref| &node_ref.value)
    }

    fn value_mut(&mut self, node: Self::NodeRef) -> Option<&mut T> {
        self.nodes
            .get_mut(&node)
            .map(|node_ref| &mut node_ref.value)
    }
}
