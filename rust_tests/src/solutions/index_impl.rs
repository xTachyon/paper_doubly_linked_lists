pub struct Node {
    pub next: usize,
    pub prec: usize,
    pub value: u64,
}
pub struct DoubleLinkedList {
    data: Vec<Option<Node>>,
    pub head: usize,
    pub tail: usize,
}
impl DoubleLinkedList {
    pub const INVALID_INDEX: usize = usize::MAX;
    pub fn new(capacity: usize) -> Self {
        let mut me = Self {
            data: Vec::with_capacity(capacity),
            head: 0,
            tail: 0,
        };
        me.data.push(Some(Node {
            next: DoubleLinkedList::INVALID_INDEX,
            prec: DoubleLinkedList::INVALID_INDEX,
            value: 0,
        }));
        me
    }
    pub fn add(&mut self, value: u64) {
        let new_node = Node {
            next: DoubleLinkedList::INVALID_INDEX,
            prec: self.tail,
            value,
        };
        self.data.push(Some(new_node));
        let last_index = self.data.len() - 1;
        if let Some(previous_tail) = self.get_node_mut(self.tail) {
            previous_tail.next = last_index;
        }
        self.tail = last_index;
    }
    #[inline(always)]
    fn get_node(&self, index: usize) -> Option<&Node> {
        if index < self.data.len() {
            self.data[index].as_ref()
        } else {
            None
        }
    }
    #[inline(always)]
    fn get_node_mut(&mut self, index: usize) -> Option<&mut Node> {
        if index < self.data.len() {
            self.data[index].as_mut()
        } else {
            None
        }
    }
    pub fn sum_all(&self) -> u64 {
        let mut sum = 0;
        let mut current = self.head;
        while let Some(node) = self.get_node(current) {
            sum += node.value;
            current = node.next;
        }
        sum
    }
}
