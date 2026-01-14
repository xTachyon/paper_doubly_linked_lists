
use slab::Slab;
use tests_api::TheAlloc;

use super::double_linked_list::DoubleLinkedList;

type Key = usize;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Node<T> {
    next: Option<Key>,
    prec: Option<Key>,
    value: T,
}

pub struct Implementation<'x, T> {
    map: Slab<Node<T>, &'x TheAlloc>,
    first: Option<Key>,
    last: Option<Key>,
}

impl<'x, T> Implementation<'x, T> {
    fn allocate(&mut self, value: T) -> Key {
        self.map.insert(Node {
            next: None,
            prec: None,
            value,
        })
    }
}
impl<'x, T> DoubleLinkedList<'x, T> for Implementation<'x, T> {
    type NodeRef = Key;

    fn new(alloc: &'static TheAlloc, capacity: usize) -> Self {
        Self {
            map: Slab::with_capacity_in(capacity, alloc),
            first: None,
            last: None,
        }
    }

    fn insert_after(&mut self, node: Self::NodeRef, value: T) -> Self::NodeRef {
        let next = self.map.get(node).unwrap().next;
        let new_key = self.allocate(value);

        {
            let new_node = self.map.get_mut(new_key).unwrap();
            new_node.prec = Some(node);
            new_node.next = next;
        }

        if let Some(next_key) = next {
            self.map.get_mut(next_key).unwrap().prec = Some(new_key);
        } else {
            self.last = Some(new_key);
        }

        self.map.get_mut(node).unwrap().next = Some(new_key);
        new_key
    }

    fn insert_before(&mut self, node: Self::NodeRef, value: T) -> Self::NodeRef {
        let prec = self.map.get(node).unwrap().prec;
        let new_key = self.allocate(value);

        {
            let new_node = self.map.get_mut(new_key).unwrap();
            new_node.next = Some(node);
            new_node.prec = prec;
        }

        if let Some(prec_key) = prec {
            self.map.get_mut(prec_key).unwrap().next = Some(new_key);
        } else {
            self.first = Some(new_key);
        }

        self.map.get_mut(node).unwrap().prec = Some(new_key);
        new_key
    }

    fn push_back(&mut self, value: T) -> Self::NodeRef {
        if let (Some(first_node), Some(last_node)) = (self.first, self.last) {
            let node = Node {
                next: None,
                prec: Some(last_node),
                value,
            };
            let key = self.map.insert(node);
            self.map[last_node].next = Some(key);
            self.first = Some(first_node);
            self.last = Some(key);
            key
        } else {
            // first node
            let node = Node {
                next: None,
                prec: None,
                value,
            };
            let key = self.map.insert(node);
            self.first = Some(key);
            self.last = Some(key);
            key
        }
    }

    fn push_front(&mut self, value: T) -> Self::NodeRef {
        if let (Some(first_node), Some(last_node)) = (self.first, self.last) {
            let node = Node {
                next: Some(first_node),
                prec: None,
                value,
            };
            let key = self.map.insert(node);
            self.map[first_node].prec = Some(key);
            self.first = Some(key);
            self.last = Some(last_node);
            key
        } else {
            // first node
            let node = Node {
                next: None,
                prec: None,
                value,
            };
            let key = self.map.insert(node);
            self.first = Some(key);
            self.last = Some(key);
            key
        }
    }

    unsafe fn delete(&mut self, key: Self::NodeRef) {
        let node = &self.map[key];
        let prec = node.prec;
        let next = node.next;

        if let Some(prec) = prec {
            self.map[prec].next = next;
        }
        if let Some(next) = next {
            self.map[next].prec = prec;
        }

        if self.first == Some(key) {
            self.first = next;
        }
        if self.last == Some(key) {
            self.last = prec;
        }

        self.map.remove(key);
    }

    fn next(&self, node: Self::NodeRef) -> Option<Self::NodeRef> {
        let node = self.map.get(node)?;
        node.next
    }

    fn prec(&self, node: Self::NodeRef) -> Option<Self::NodeRef> {
        let node = self.map.get(node)?;
        node.prec
    }

    fn first(&self) -> Option<Self::NodeRef> {
        self.first
    }

    fn last(&self) -> Option<Self::NodeRef> {
        self.last
    }

    fn value(&self, node: Self::NodeRef) -> Option<&T> {
        self.map.get(node).map(|x| &x.value)
    }

    fn value_mut(&mut self, node: Self::NodeRef) -> Option<&mut T> {
        self.map.get_mut(node).map(|x| &mut x.value)
    }
}
