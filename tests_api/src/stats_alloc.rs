use std::{
    alloc::{AllocError, Allocator, Layout},
    cell::Cell,
    ptr::NonNull,
    time::{Duration, Instant},
};

pub struct StatsAllocator<T: Allocator> {
    inner: T,

    time: Cell<Duration>,
    no_allocs: Cell<usize>,
    current_allocated: Cell<usize>,
    max_allocated: Cell<usize>,
}
impl<T: Allocator> StatsAllocator<T> {
    pub fn new(alloc: T) -> Self {
        Self {
            inner: alloc,
            time: Cell::new(Duration::ZERO),
            no_allocs: Cell::new(0),
            current_allocated: Cell::new(0),
            max_allocated: Cell::new(0),
        }
    }

    pub fn max_allocated(&self) -> usize {
        self.max_allocated.get()
    }
    pub fn no_allocs(&self) -> usize {
        self.no_allocs.get()
    }
    pub fn time(&self) -> Duration {
        self.time.get()
    }
    pub fn reset_time(&self) {
        self.time.set(Duration::ZERO);
    }
}

fn calc_time<R, F: Fn() -> R>(time: &Cell<Duration>, f: F) -> R {
    let start = Instant::now();
    let result = f();
    time.set(time.get() + start.elapsed());
    result
}

unsafe impl<T: Allocator> Allocator for StatsAllocator<T> {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        self.no_allocs.set(self.no_allocs.get() + 1);
        self.current_allocated
            .set(self.current_allocated.get() + layout.size());
        self.max_allocated
            .set(self.max_allocated.get().max(self.current_allocated.get()));

        calc_time(&self.time, || self.inner.allocate(layout))
    }

    fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        self.no_allocs.set(self.no_allocs.get() + 1);
        self.current_allocated
            .set(self.current_allocated.get() + layout.size());
        self.max_allocated
            .set(self.max_allocated.get().max(self.current_allocated.get()));

        calc_time(&self.time, || self.inner.allocate_zeroed(layout))
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        self.current_allocated
            .set(self.current_allocated.get() - layout.size());

        calc_time(&self.time, || self.inner.deallocate(ptr, layout))
    }
}
