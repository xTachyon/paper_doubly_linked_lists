use std::ptr::NonNull;

pub struct Node {
    pub next: Option<NonNull<Node>>,
    pub prec: Option<NonNull<Node>>,
    pub value: u64,
}
pub struct DoubleLinkedList {
    pub head: *mut Node,
    pub tail: *mut Node,
}
impl DoubleLinkedList {
    pub fn new(_capacity: usize) -> Self {
        let start_node = Box::into_raw(Box::new(Node {
            next: None,
            prec: None,
            value: 0,
        }));
        Self {
            head: start_node,
            tail: start_node,
        }
    }
    pub fn add(&mut self, value: u64) {
        let new_node = Box::into_raw(Box::new(Node {
            next: None,
            prec: None,
            value,
        }));
        unsafe {
            (*self.tail).next = Some(NonNull::new_unchecked(new_node));
            (*new_node).prec = Some(NonNull::new_unchecked(self.tail));
        }
        self.tail = new_node;
    }
    pub fn sum_all(&self) -> u64 {
        let mut sum = 0;
        let mut current = self.head;
        loop {
            unsafe {
                sum += (*current).value;
                if let Some(next) = (*current).next {
                    current = next.as_ptr();
                } else {
                    break;
                }
            }
        }
        sum
    }
}
