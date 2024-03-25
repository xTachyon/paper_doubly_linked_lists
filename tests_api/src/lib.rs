use std::ffi::c_void;

pub type Handle = *mut c_void;

pub type CreateFn = extern "C" fn(nodes: usize) -> Handle;
pub type DestroyFn = extern "C" fn(handle: Handle);
pub type AddFn = extern "C" fn(handle: Handle, element: u64);
pub type SumAllFn = extern "C" fn(handle: Handle) -> u64;

#[repr(C)]
pub struct RawTestData {
    pub name: *const u8,
    pub name_size: usize,

    pub create: CreateFn,
    pub destroy: DestroyFn,
    pub add: AddFn,
    pub sum_all: SumAllFn,
}

#[repr(C)]
pub struct RawLoadTestsResult {
    pub tests: *const RawTestData,
    pub tests_count: usize,
}

pub type LoadTestsFn = unsafe extern "C" fn() -> RawLoadTestsResult;

#[no_mangle]
pub extern "C" fn ignore_this_cbindgen_needs_to_find_stuff(_: LoadTestsFn) {}
