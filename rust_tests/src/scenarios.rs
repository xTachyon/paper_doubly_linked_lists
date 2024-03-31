use crate::solutions::double_linked_list::DoubleLinkedList;

pub trait Scenario {
    type Impl;

    fn new() -> Self;
    fn run(self);
}

// ----------------------------------------------------------------------------

const ITERATIONS: u64 = 1_000_000;

pub struct SumScenario<L> {
    list: L,
}
impl<L: DoubleLinkedList<u64>> Scenario for SumScenario<L> {
    type Impl = L;

    fn new() -> Self {
        Self {
            list: L::new(ITERATIONS as usize),
        }
    }

    fn run(self) {
        let mut list = self.list;
        for i in 1..=ITERATIONS {
            list.push_back(i);
        }

        let mut sum = 0;

        let mut first = list.first();
        while let Some(element) = first {
            let value = list.value(element).unwrap();
            sum += value;
            first = list.next(element);
        }

        assert_eq!(sum, ITERATIONS * (ITERATIONS + 1) / 2);
    }
}

// ----------------------------------------------------------------------------

pub struct PushDeleteScenario<L> {
    list: L,
}
impl<L: DoubleLinkedList<u64>> Scenario for PushDeleteScenario<L> {
    type Impl = L;

    fn new() -> Self {
        Self { list: L::new(1) }
    }

    fn run(self) {
        let mut list = self.list;
        for i in 1..=ITERATIONS {
            let node = list.push_back(i);
            list.delete(node);
        }

        assert_eq!(list.first(), None);
    }
}
