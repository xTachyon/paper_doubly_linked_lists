use super::double_linked_list::DoubleLinkedList;
use slotmap::SlotMap;
use std::fmt::Debug;
use tests_api::TheAlloc;

type DefaultKey = slotmap::DefaultKey;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Node<T: Debug> {
    next: Option<DefaultKey>,
    prec: Option<DefaultKey>,
    value: T,
}

pub struct Implementation<'x, T: Debug> {
    map: SlotMap<DefaultKey, Node<T>, &'x TheAlloc>,
    head: Option<DefaultKey>,
    tail: Option<DefaultKey>,
}

impl<'x, T: Debug + PartialEq + Copy> DoubleLinkedList<'x, T> for Implementation<'x, T> {
    type NodeRef = DefaultKey;

    fn new(alloc: &'x TheAlloc, capacity: usize) -> Self {
        Self {
            map: SlotMap::with_capacity_and_key_in(capacity, alloc),
            head: None,
            tail: None,
        }
    }

    fn insert_after(&mut self, _node: Self::NodeRef, _value: T) -> Self::NodeRef {
        todo!()
    }

    fn insert_before(&mut self, _node: Self::NodeRef, _value: T) -> Self::NodeRef {
        todo!()
    }

    fn push_back(&mut self, value: T) -> Self::NodeRef {
        if let (Some(head), Some(tail)) = (self.head, self.tail) {
            let node = Node {
                next: None,
                prec: Some(tail),
                value,
            };
            let key = self.map.insert(node);
            self.map[tail].next = Some(key);
            self.head = Some(head);
            self.tail = Some(key);
            key
        } else {
            // first node
            let node = Node {
                next: None,
                prec: None,
                value,
            };
            let key = self.map.insert(node);
            self.head = Some(key);
            self.tail = Some(key);
            key
        }
    }

    fn push_front(&mut self, value: T) -> Self::NodeRef {
        if let (Some(head), Some(tail)) = (self.head, self.tail) {
            let node = Node {
                next: None,
                prec: Some(head),
                value,
            };
            let key = self.map.insert(node);
            self.map[head].prec = Some(key);
            self.head = Some(key);
            self.tail = Some(tail);
            key
        } else {
            // first node
            let node = Node {
                next: None,
                prec: None,
                value,
            };
            let key = self.map.insert(node);
            self.head = Some(key);
            self.tail = Some(key);
            key
        }
    }

    unsafe fn delete(&mut self, key: Self::NodeRef) {
        let node = self.map[key];
        let prec = node.prec;
        let next = node.next;

        if let Some(prec) = prec {
            self.map[prec].next = next;
        }
        if let Some(next) = next {
            self.map[next].prec = prec;
        }

        if self.head == Some(key) {
            self.head = next;
        }
        if self.tail == Some(key) {
            self.tail = prec;
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
        self.head
    }

    fn last(&self) -> Option<Self::NodeRef> {
        self.tail
    }

    fn value(&self, node: Self::NodeRef) -> Option<&T> {
        self.map.get(node).map(|x| &x.value)
    }

    fn value_mut(&mut self, _node: Self::NodeRef) -> Option<&mut T> {
        todo!()
    }
}
