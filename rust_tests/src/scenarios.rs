use std::{array, hint::black_box, marker::PhantomData};
use tests_api::TheAlloc;

use crate::solutions::double_linked_list::DoubleLinkedList;

pub struct ScenarioInit<'x> {
    pub alloc: &'static TheAlloc,
    pub percent: u32,
    pub _p: PhantomData<&'x ()>,
}
impl<'x> ScenarioInit<'x> {
    fn percent_usize(&self, x: usize) -> usize {
        x * self.percent as usize / 100
    }
    fn percent_u64(&self, x: u64) -> u64 {
        x * self.percent as u64 / 100
    }
}

pub trait Scenario<'x> {
    type Impl;

    fn new(init: ScenarioInit<'x>) -> Self;
    fn run(self);
}

// ----------------------------------------------------------------------------

const ITERATIONS: u64 = 1_000_000;

pub struct SumScenario<L> {
    list: L,
    iterations: usize,
}
impl<'x, L: DoubleLinkedList<'x, u64>> Scenario<'x> for SumScenario<L> {
    type Impl = L;

    fn new(init: ScenarioInit<'x>) -> Self {
        let iterations = init.percent_usize(10_000_000);
        let mut list = L::new(init.alloc, iterations);
        for i in 1..=iterations {
            list.push_back(i as u64);
        }
        Self { list, iterations }
    }

    fn run(self) {
        let list = self.list;
        let mut sum = 0;

        let mut first = list.first();
        while let Some(element) = first {
            let value = list.value(element.clone()).unwrap();
            sum += value;
            first = list.next(element);
        }

        let iterations = self.iterations as u64;
        assert_eq!(sum, iterations * (iterations + 1) / 2);
    }
}

// ----------------------------------------------------------------------------

pub struct PushDeleteOneScenario<'x, L> {
    init: ScenarioInit<'x>,
    _p: PhantomData<L>,
}
impl<'x, L: DoubleLinkedList<'x, u64>> Scenario<'x> for PushDeleteOneScenario<'x, L> {
    type Impl = L;

    fn new(init: ScenarioInit<'x>) -> Self {
        Self {
            init,
            _p: PhantomData,
        }
    }

    fn run(self) {
        let mut list = L::new(self.init.alloc, 1);
        let iterations = self.init.percent_u64(ITERATIONS);
        for i in 1..=iterations {
            let node = list.push_back(i);
            unsafe { list.delete(node) };
        }

        assert_eq!(list.first(), None);
    }
}

// ----------------------------------------------------------------------------

pub struct PushScenario<'x, L> {
    init: ScenarioInit<'x>,
    _p: PhantomData<L>,
}
impl<'x, L: DoubleLinkedList<'x, u64>> Scenario<'x> for PushScenario<'x, L> {
    type Impl = L;

    fn new(init: ScenarioInit<'x>) -> Self {
        Self {
            init,
            _p: PhantomData,
        }
    }

    fn run(self) {
        let iterations = self.init.percent_u64(10_000_000);
        let mut list = L::new(self.init.alloc, iterations as usize);
        for i in 1..=iterations {
            list.push_back(i);
        }

        assert_eq!(list.value(list.last().unwrap()), Some(&iterations));
    }
}

// ----------------------------------------------------------------------------

pub struct Fragmentation<'x, L> {
    init: ScenarioInit<'x>,
    _p: PhantomData<L>,
}
impl<'x, L: DoubleLinkedList<'x, u64>> Scenario<'x> for Fragmentation<'x, L> {
    type Impl = L;

    fn new(init: ScenarioInit<'x>) -> Self {
        Self {
            init,
            _p: PhantomData,
        }
    }

    fn run(self) {
        let mut list = L::new(self.init.alloc, 1000);
        let iterations = self.init.percent_u64(1_000);
        for _ in 0..=iterations {
            let mut to_delete = Vec::with_capacity(iterations as usize);

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

            for i in to_delete {
                unsafe { list.delete(i) };
            }
        }
    }
}

// ----------------------------------------------------------------------------

pub struct First<'x, L> {
    init: ScenarioInit<'x>,
    _p: PhantomData<L>,
}
impl<'x, L: DoubleLinkedList<'x, u64>> Scenario<'x> for First<'x, L> {
    type Impl = L;

    fn new(init: ScenarioInit<'x>) -> Self {
        Self {
            init,
            _p: PhantomData,
        }
    }

    fn run(self) {
        let mut list = L::new(self.init.alloc, 2);

        let node = list.push_front(0xDA);
        list.push_back(5);
        assert_eq!(list.first().unwrap(), node);
        assert_eq!(list.value(node), Some(&0xDA));
    }
}

// ----------------------------------------------------------------------------

pub struct Last<'x, L> {
    init: ScenarioInit<'x>,
    _p: PhantomData<L>,
}
impl<'x, L: DoubleLinkedList<'x, u64>> Scenario<'x> for Last<'x, L> {
    type Impl = L;

    fn new(init: ScenarioInit<'x>) -> Self {
        Self {
            init,
            _p: PhantomData,
        }
    }

    fn run(self) {
        let mut list = L::new(self.init.alloc, 2);

        list.push_front(5);
        let node = list.push_back(0xDA);
        assert_eq!(list.last().unwrap(), node);
        assert_eq!(list.value(node), Some(&0xDA));
    }
}

// ----------------------------------------------------------------------------

pub struct Order<'x, L> {
    init: ScenarioInit<'x>,
    _p: PhantomData<L>,
}
impl<'x, L: DoubleLinkedList<'x, u64>> Scenario<'x> for Order<'x, L> {
    type Impl = L;

    fn new(init: ScenarioInit<'x>) -> Self {
        Self {
            init,
            _p: PhantomData,
        }
    }

    fn run(self) {
        let mut list = L::new(self.init.alloc, 2);

        let n3 = list.push_front(3);
        let n2 = list.insert_before(n3.clone(), 2);
        let n1 = list.push_front(1);
        let n5 = list.push_back(5);
        let n4 = list.insert_before(n5.clone(), 4);

        let values = [n1, n2, n3, n4, n5];
        let mut values_index = 0;

        let mut first = list.first();
        while let Some(element) = first {
            let v = *list.value(element.clone()).unwrap();
            let w = *list.value(values[values_index].clone()).unwrap();
            assert_eq!(v, w);
            assert_eq!(v, values_index as u64);
            values_index += 1;

            first = list.next(element);
        }
    }
}

// ----------------------------------------------------------------------------

pub struct UseAfterDelete<'x, L> {
    init: ScenarioInit<'x>,
    _p: PhantomData<L>,
}
impl<'x, L: DoubleLinkedList<'x, u64>> Scenario<'x> for UseAfterDelete<'x, L> {
    type Impl = L;

    fn new(init: ScenarioInit<'x>) -> Self {
        Self {
            init,
            _p: PhantomData,
        }
    }

    fn run(self) {
        let mut list = L::new(self.init.alloc, 2);

        let node = list.push_front(0xDA);
        unsafe { list.delete(node.clone()) };
        // UB incoming
        black_box(list.value(node));
    }
}

// ----------------------------------------------------------------------------

pub struct SearchMiddle<L> {
    list: L,
    iterations: u64,
    batch: u64,
}
impl<'x, L: DoubleLinkedList<'x, u64>> Scenario<'x> for SearchMiddle<L> {
    type Impl = L;

    fn new(init: ScenarioInit<'x>) -> Self {
        let iterations = init.percent_u64(10_000_000);
        let batch = init.percent_u64(100_000);

        let mut list = L::new(init.alloc, iterations as usize);
        for i in 1..=10_000_000 {
            list.push_back(i);
        }
        Self {
            list,
            iterations,
            batch,
        }
    }

    fn run(self) {
        let list = self.list;

        for i in 1..self.iterations / self.batch {
            let to_find = i * self.batch;
            let f = |x: &u64| *x == to_find;
            let node = list.search(f).unwrap();

            assert_eq!(list.value(node.clone()), Some(&to_find));
            assert_eq!(
                list.value(list.prec(node.clone()).unwrap()),
                Some(&(to_find - 1))
            );
            assert_eq!(list.value(list.next(node).unwrap()), Some(&(to_find + 1)));
        }
    }
}

// ----------------------------------------------------------------------------

pub struct AddFrontBack<'x, L> {
    init: ScenarioInit<'x>,
    _p: PhantomData<L>,
}
impl<'x, L: DoubleLinkedList<'x, u64>> Scenario<'x> for AddFrontBack<'x, L> {
    type Impl = L;

    fn new(init: ScenarioInit<'x>) -> Self {
        Self {
            init,
            _p: PhantomData,
        }
    }

    fn run(self) {
        let iterations = self.init.percent_usize(1_000_000);
        let mut list = L::new(self.init.alloc, iterations);

        for i in 0..iterations {
            list.push_back(i as u64);
            list.push_front(i as u64);
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Clone, Debug)]
#[repr(align(4096))]
pub struct Page {
    data: [u8; 4096],
}

pub struct IteratePages<L> {
    list: L,
    sum_one: u64,
    _p: PhantomData<L>,
}
impl<'x, L: DoubleLinkedList<'x, Page>> Scenario<'x> for IteratePages<L> {
    type Impl = L;

    fn new(init: ScenarioInit<'x>) -> Self {
        let iterations = init.percent_u64(1_000);
        let mut list = L::new(init.alloc, iterations as usize);
        let page = Page {
            data: array::from_fn(|x| x as u8),
        };
        let sum_one = page.data.iter().map(|x| *x as u64).sum();
        for _ in 0..iterations {
            list.push_back(page.clone());
            list.push_front(page.clone());
        }

        Self {
            list,
            sum_one,
            _p: PhantomData,
        }
    }

    fn run(self) {
        let list = self.list;

        let mut first = list.first();
        while let Some(element) = first {
            let value = list.value(element.clone()).unwrap();
            let current_sum = value.data.iter().map(|x| *x as u64).sum::<u64>();
            assert_eq!(current_sum, self.sum_one);
            first = list.next(element);
        }
    }
}

// ----------------------------------------------------------------------------

pub struct FindString<L> {
    list: L,
    iterations: u64,
    _p: PhantomData<L>,
}
impl<'x, L: DoubleLinkedList<'x, String>> Scenario<'x> for FindString<L> {
    type Impl = L;

    fn new(init: ScenarioInit<'x>) -> Self {
        let iterations = init.percent_u64(1_000);
        let mut list = L::new(init.alloc, iterations as usize);
        let mut s = String::with_capacity(4096);
        for _ in 0..iterations {
            s.push_str("abc");
            list.push_back(s.clone());
        }

        Self {
            list,
            iterations,
            _p: PhantomData,
        }
    }

    fn run(self) {
        let list = self.list;

        let mut s = String::with_capacity(4096);
        for _ in 0..self.iterations {
            s.push_str("abc");

            list.search(|x| *x == s).expect("String should be found");
        }
    }
}
