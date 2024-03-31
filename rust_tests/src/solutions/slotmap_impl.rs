use super::double_linked_list::DoubleLinkedList;
use slotmap::SlotMap;
use std::fmt::Debug;

type DefaultKey = slotmap::DefaultKey;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Node<T: Debug> {
    next: Option<DefaultKey>,
    prec: Option<DefaultKey>,
    value: T,
}

pub struct Implementation<T: Debug> {
    map: SlotMap<DefaultKey, Node<T>>,
    head: Option<DefaultKey>,
    tail: Option<DefaultKey>,
}

impl<T: Debug + PartialEq + Copy> DoubleLinkedList<T> for Implementation<T> {
    type Node = DefaultKey;

    fn new(capacity: usize) -> Self {
        Self {
            map: SlotMap::with_capacity_and_key(capacity),
            head: None,
            tail: None,
        }
    }

    fn insert_after(&mut self, _node: Self::Node, _value: T) -> Self::Node {
        todo!()
    }

    fn insert_before(&mut self, _node: Self::Node, _value: T) -> Self::Node {
        todo!()
    }

    fn push_back(&mut self, value: T) -> Self::Node {
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

    fn push_top(&mut self, _value: T) -> Self::Node {
        todo!()
    }

    fn delete(&mut self, key: Self::Node) {
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

    fn next(&self, node: Self::Node) -> Option<Self::Node> {
        let node = self.map.get(node)?;
        node.next
    }

    fn prec(&self, _node: Self::Node) -> Option<Self::Node> {
        todo!()
    }

    fn first(&self) -> Option<Self::Node> {
        self.head
    }

    fn last(&self) -> Option<Self::Node> {
        self.tail
    }

    fn value(&self, node: Self::Node) -> Option<&T> {
        self.map.get(node).map(|x| &x.value)
    }

    fn value_mut(&mut self, _node: Self::Node) -> Option<&mut T> {
        todo!()
    }
}
