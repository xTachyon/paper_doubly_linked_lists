use std::{alloc::System, ffi::c_void};

use stats_alloc::StatsAlloc;

pub type Handle = *mut c_void;

pub type FnGetAlloc = extern "C" fn() -> &'static StatsAlloc<System>;
pub type FnScenarioNew = extern "C" fn() -> Handle;
pub type FnScenarioRun = extern "C" fn(handle: Handle);

#[repr(C)]
pub struct RawScenario {
    pub name: *const u8,
    pub name_size: usize,
    
    pub new: FnScenarioNew,
    pub run: FnScenarioRun,
}

#[repr(C)]
pub struct RawImpl {
    pub name: *const u8,
    pub name_size: usize,
    
    pub scenarios: *const RawScenario,
    pub scenarios_count: usize,
}

#[repr(C)]
pub struct RawLoadResult {
    pub list_impl: *const RawImpl,
    pub list_impl_count: usize,
    pub get_alloc: FnGetAlloc,
}

pub type FnLoadTests = unsafe extern "C" fn() -> RawLoadResult;

#[no_mangle]
pub extern "C" fn ignore_this_cbindgen_needs_to_find_stuff(_: FnLoadTests) {}
