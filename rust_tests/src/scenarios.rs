use std::marker::PhantomData;
use tests_api::TheAlloc;

use crate::solutions::double_linked_list::DoubleLinkedList;

pub trait Scenario<'x> {
    type Impl;

    fn new(alloc: &'x TheAlloc) -> Self;
    fn run(self);
}

// ----------------------------------------------------------------------------

const ITERATIONS: u64 = 1_000_000;

pub struct SumScenario<L> {
    list: L,
}
impl<'x, L: DoubleLinkedList<'x, u64>> Scenario<'x> for SumScenario<L> {
    type Impl = L;

    fn new(alloc: &'x TheAlloc) -> Self {
        let mut list = L::new(alloc, 0);
        for i in 1..=10_000_000 {
            list.push_back(i);
        }
        Self { list }
    }

    fn run(self) {
        let list = self.list;
        let mut sum = 0;

        let mut first = list.first();
        while let Some(element) = first {
            let value = list.value(element).unwrap();
            sum += value;
            first = list.next(element);
        }

        let iterations = 10_000_000;
        assert_eq!(sum, iterations * (iterations + 1) / 2);
    }
}

// ----------------------------------------------------------------------------

pub struct PushDeleteOneScenario<'x, L> {
    alloc: &'x TheAlloc,
    _p: PhantomData<L>,
}
impl<'x, L: DoubleLinkedList<'x, u64>> Scenario<'x> for PushDeleteOneScenario<'x, L> {
    type Impl = L;

    fn new(alloc: &'x TheAlloc) -> Self {
        Self {
            alloc,
            _p: PhantomData,
        }
    }

    fn run(self) {
        let mut list = L::new(self.alloc, 0);
        for i in 1..=ITERATIONS {
            let node = list.push_back(i);
            unsafe { list.delete(node) };
        }

        assert_eq!(list.first(), None);
    }
}

// ----------------------------------------------------------------------------

pub struct PushScenario<'x, L> {
    alloc: &'x TheAlloc,
    _p: PhantomData<L>,
}
impl<'x, L: DoubleLinkedList<'x, u64>> Scenario<'x> for PushScenario<'x, L> {
    type Impl = L;

    fn new(alloc: &'x TheAlloc) -> Self {
        Self { alloc, _p: PhantomData }
    }

    fn run(self) {
        let iterations = 10_000_000;
        let mut list = L::new(self.alloc, 0);
        for i in 1..=iterations {
            list.push_back(i);
        }

        assert_eq!(list.value(list.last().unwrap()), Some(&iterations));
    }
}

// ----------------------------------------------------------------------------

pub struct Fragmentation<'x, L> {
    alloc: &'x TheAlloc,
    _p: PhantomData<L>,
}
impl<'x, L: DoubleLinkedList<'x, u64>> Scenario<'x> for Fragmentation<'x, L> {
    type Impl = L;

    fn new(alloc: &'x TheAlloc) -> Self {
        Self { alloc, _p: PhantomData }
    }

    fn run(self) {
        let mut list = L::new(self.alloc, 0);
        let mut to_delete = Vec::with_capacity(1000);
        for _ in 0..=1_000 {
            to_delete.clear();

            for i in 0..10_000 {
                list.push_back(i);
            }

            for i in 0..1000 {
                let node = list.push_back(i);
                to_delete.push(node);
            }

            for i in 0..1000 {
                list.push_back(i);
            }

            for i in to_delete.iter() {
                unsafe { list.delete(*i) };
            }
        }
    }
}

// ----------------------------------------------------------------------------

pub struct First<'x, L> {
    alloc: &'x TheAlloc,
    _p: PhantomData<L>,
}
impl<'x, L: DoubleLinkedList<'x, u64>> Scenario<'x> for First<'x, L> {
    type Impl = L;

    fn new(alloc: &'x TheAlloc) -> Self {
        Self { alloc, _p: PhantomData }
    }

    fn run(self) {
        let mut list = L::new(self.alloc, 2);

        let node = list.push_front(0xDA);
        list.push_back(5);
        assert_eq!(list.first().unwrap(), node);
        assert_eq!(list.value(node), Some(&0xDA));
    }
}

// ----------------------------------------------------------------------------

pub struct Last<'x, L> {
    alloc: &'x TheAlloc,
    _p: PhantomData<L>,
}
impl<'x, L: DoubleLinkedList<'x, u64>> Scenario<'x> for Last<'x, L> {
    type Impl = L;

    fn new(alloc: &'x TheAlloc) -> Self {
        Self { alloc, _p: PhantomData }
    }

    fn run(self) {
        let mut list = L::new(self.alloc, 2);

        list.push_front(5);
        let node = list.push_back(0xDA);
        assert_eq!(list.last().unwrap(), node);
        assert_eq!(list.value(node), Some(&0xDA));
    }
}

// ----------------------------------------------------------------------------

pub struct Order<'x, L> {
    alloc: &'x TheAlloc,
    _p: PhantomData<L>,
}
impl<'x, L: DoubleLinkedList<'x, u64>> Scenario<'x> for Order<'x, L> {
    type Impl = L;

    fn new(alloc: &'x TheAlloc) -> Self {
        Self { alloc, _p: PhantomData }
    }

    fn run(self) {
        let mut list = L::new(self.alloc, 2);

        let n3 = list.push_front(3);
        let n2 = list.insert_before(n3, 2);
        let n1 = list.push_front(1);
        let n5 = list.push_back(5);
        let n4 = list.insert_before(n5, 4);

        let values = [n1, n2, n3, n4, n5];
        let mut values_index = 0;

        let mut first = list.first();
        while let Some(element) = first {
            let v = *list.value(element).unwrap();
            let w = *list.value(values[values_index]).unwrap();
            assert_eq!(v, w);
            assert_eq!(v, values_index as u64);
            values_index += 1;

            first = list.next(element);
        }
    }
}

// ----------------------------------------------------------------------------

pub struct SearchMiddle<L> {
    list: L,
}
impl<'x, L: DoubleLinkedList<'x, u64>> Scenario<'x> for SearchMiddle<L> {
    type Impl = L;

    fn new(alloc: &'x TheAlloc) -> Self {
        let mut list = L::new(alloc, 0);
        for i in 1..=10_000_000 {
            list.push_back(i);
        }
        Self { list }
    }

    fn run(self) {
        let list = self.list;

        for i in 1..10_000_000 / 100000 {
            let to_find = i * 100000;
            let f = |x: &u64| *x == to_find;
            let node = list.search(f).unwrap();

            assert_eq!(list.value(node), Some(&to_find));
            assert_eq!(list.value(list.prec(node).unwrap()), Some(&(to_find - 1)));
            assert_eq!(list.value(list.next(node).unwrap()), Some(&(to_find + 1)));
        }
    }
}
