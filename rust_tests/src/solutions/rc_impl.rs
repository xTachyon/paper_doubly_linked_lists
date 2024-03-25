use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct Node {
    pub next: Option<Rc<RefCell<Node>>>,
    pub prec: Option<Weak<RefCell<Node>>>,
    pub value: u64,
}

pub struct DoubleLinkedList {
    pub head: Rc<RefCell<Node>>,
    pub tail: Rc<RefCell<Node>>,
}

impl Drop for Node {
    fn drop(self: &mut Node) {
        loop {
            let Some(next) = self.next.take() else {
                return;
            };
            let next = next.borrow_mut().next.take();
            self.next = next;
        }
    }
}

impl DoubleLinkedList {
    pub fn new(_capacity: usize) -> Self {
        let start_node = Rc::new(RefCell::new(Node {
            next: None,
            prec: None,
            value: 0,
        }));
        Self {
            head: start_node.clone(),
            tail: start_node.clone(),
        }
    }
    pub fn add(&mut self, value: u64) {
        let new_node = Rc::new(RefCell::new(Node {
            next: None,
            prec: Some(Rc::downgrade(&self.tail)),
            value,
        }));
        (*self.tail.borrow_mut()).next = Some(new_node.clone());
        self.tail = new_node;
    }
    pub fn sum_all(&self) -> u64 {
        let mut sum = 0;
        let mut current = self.head.clone();
        loop {
            let next_node;
            {
                let tmp = current.borrow_mut();
                sum += (*tmp).value;
                next_node = if let Some(next) = (*tmp).next.as_ref() {
                    Some(next.clone())
                } else {
                    None
                };
            }
            if next_node.is_none() {
                break;
            }
            current = next_node.unwrap();
        }
        sum
    }
}
