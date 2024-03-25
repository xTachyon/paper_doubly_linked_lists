use slotmap::{new_key_type, SlotMap};

new_key_type! { struct Key; }

#[allow(dead_code)]
struct Node {
    next: Option<Key>,
    prec: Option<Key>,
    value: u64,
}

pub struct DoubleLinkedList {
    map: SlotMap<Key, Node>,
    head: Key,
    tail: Key,
}

impl DoubleLinkedList {
    pub fn new(capacity: usize) -> DoubleLinkedList {
        let mut map = SlotMap::with_capacity_and_key(capacity);
        let node = map.insert(Node {
            next: None,
            prec: None,
            value: 0,
        });
        DoubleLinkedList {
            map,
            head: node,
            tail: node,
        }
    }
    pub fn add(&mut self, value: u64) {
        let new_node = Node {
            next: None,
            prec: Some(self.tail),
            value,
        };
        let new_node_key = self.map.insert(new_node);
        self.map[self.tail].next = Some(new_node_key);
        self.tail = new_node_key;
    }
    pub fn sum_all(&self) -> u64 {
        let mut sum = 0;

        let mut node = Some(self.head);
        while let Some(current) = node {
            let current = &self.map[current];
            sum += current.value;
            node = current.next;
        }

        sum
    }
}
