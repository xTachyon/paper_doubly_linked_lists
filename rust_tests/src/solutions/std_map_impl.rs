use std::collections::HashMap;

#[allow(dead_code)]
struct Node {
    next: Option<u32>,
    prec: Option<u32>,
    value: u64,
}

pub struct DoubleLinkedList {
    map: HashMap<u32, Node>,
    head: u32,
    tail: u32,
}

impl DoubleLinkedList {
    pub fn new(capacity: usize) -> DoubleLinkedList {
        let mut map = HashMap::with_capacity(capacity);
        let node = Node {
            next: None,
            prec: None,
            value: 0,
        };
        map.insert(0, node);
        DoubleLinkedList {
            map,
            head: 0,
            tail: 0,
        }
    }
    pub fn add(&mut self, value: u64) {
        let node = Node {
            next: None,
            prec: Some(self.tail),
            value,
        };
        let new_tail = self.tail + 1;
        self.map.insert(new_tail, node);
        self.map.get_mut(&self.tail).unwrap().next = Some(new_tail);
        self.tail = new_tail;
    }
    pub fn sum_all(&self) -> u64 {
        let mut sum = 0;

        let mut node = Some(self.head);
        while let Some(current) = node {
            let current = &self.map[&current];
            sum += current.value;
            node = current.next;
        }

        sum
    }
}
