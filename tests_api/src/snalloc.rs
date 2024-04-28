use snmalloc_sys::{
    sn_rust_inst_alloc, sn_rust_inst_alloc_zeroed, sn_rust_inst_create, sn_rust_inst_dealloc,
    sn_rust_inst_destroy, AllocCtx,
};
use std::{
    alloc::{AllocError, Allocator, Layout},
    ptr::NonNull,
};

pub struct SnAlloc {
    ctx: *mut AllocCtx,
}
impl SnAlloc {
    pub fn new() -> SnAlloc {
        let ctx = unsafe { sn_rust_inst_create() };
        assert!(!ctx.is_null());
        SnAlloc { ctx: ctx }
    }

    fn alloc(&self, layout: Layout, zeroed: bool) -> Result<NonNull<[u8]>, AllocError> {
        let f = if zeroed {
            sn_rust_inst_alloc_zeroed
        } else {
            sn_rust_inst_alloc
        };
        let ptr = unsafe { f(self.ctx, layout.align(), layout.size()) };
        if ptr.is_null() {
            return Err(AllocError);
        }

        let nonnull = unsafe { NonNull::new_unchecked(ptr) };
        Ok(NonNull::slice_from_raw_parts(nonnull, layout.size()))
    }
}
impl Drop for SnAlloc {
    fn drop(&mut self) {
        unsafe {
            sn_rust_inst_destroy(self.ctx);
        }
    }
}

unsafe impl Allocator for &SnAlloc {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        self.alloc(layout, false)
    }
    fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        self.alloc(layout, true)
    }
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        sn_rust_inst_dealloc(self.ctx, ptr.as_ptr(), layout.align(), layout.size());
    }
}
