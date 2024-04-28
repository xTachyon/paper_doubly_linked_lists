use std::{
    alloc::{AllocError, Allocator, Layout},
    cell::Cell,
    ptr::NonNull,
    slice,
};

const INIT_BYTE: u8 = 0xCD;
const PAGE_SIZE: usize = 4096;
const ALIGN: usize = 16;

pub struct Stats {
    pub no_allocs: usize,
    pub max_allocated: usize,
}

#[repr(C, align(4096))]
#[derive(Clone, Copy)]
struct Page([u8; PAGE_SIZE]);

pub struct ArenaAlloc {
    buffer: Vec<Page>,
    offset: Cell<usize>,
    capacity: usize,
    no_allocs: Cell<usize>,
    current_allocated: Cell<usize>,
    max_allocated: Cell<usize>,
}

impl ArenaAlloc {
    pub fn new(cap: usize) -> ArenaAlloc {
        if cap % PAGE_SIZE != 0 {
            panic!("capacity is not aligned to 4096");
        }

        ArenaAlloc {
            buffer: vec![Page([INIT_BYTE; PAGE_SIZE]); cap / PAGE_SIZE],
            offset: Cell::new(0),
            capacity: cap,
            no_allocs: Cell::new(0),
            current_allocated: Cell::new(0),
            max_allocated: Cell::new(0),
        }
    }

    fn buffer(&self) -> *mut u8 {
        self.buffer.as_ptr() as *mut u8
    }

    pub fn reset(&mut self) {
        let buffer = self.buffer();
        let slice = unsafe { slice::from_raw_parts_mut(buffer, self.capacity) };
        slice.fill(INIT_BYTE);

        *self = ArenaAlloc {
            buffer: std::mem::take(&mut self.buffer),
            offset: Cell::new(0),
            capacity: self.capacity,
            no_allocs: Cell::new(0),
            current_allocated: Cell::new(0),
            max_allocated: Cell::new(0),
        };
    }

    pub fn stats(&self) -> Stats {
        Stats {
            no_allocs: self.no_allocs.get(),
            max_allocated: self.max_allocated.get(),
        }
    }
}
unsafe impl Allocator for &ArenaAlloc {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        // TODO: use handle_alloc_error instead of panic

        self.no_allocs.set(self.no_allocs.get() + 1);
        self.current_allocated
            .set(self.current_allocated.get() + layout.size());
        self.max_allocated
            .set(self.max_allocated.get().max(self.current_allocated.get()));

        if layout.align() > ALIGN {
            panic!("align not supported");
        }
        let layout = layout
            .align_to(ALIGN)
            .expect("align_to failed")
            .pad_to_align();
        let size = layout.size();
        let offset = self.offset.get();
        if offset + size > self.capacity {
            panic!("space exhausted");
        }
        let ptr = unsafe { self.buffer().add(offset) };
        self.offset.set(offset + size);

        let nonnull = unsafe { NonNull::new_unchecked(ptr) };
        Ok(NonNull::slice_from_raw_parts(nonnull, size))
    }

    unsafe fn deallocate(&self, _ptr: NonNull<u8>, layout: Layout) {
        self.current_allocated
            .set(self.current_allocated.get() - layout.size());
    }
}
