#![feature(allocator_api)]

pub mod arena_alloc;
pub mod snalloc;
pub mod stats_alloc;

use std::{alloc::Allocator, ffi::c_void};

pub type Handle = *mut c_void;

#[repr(C)]
pub struct RawScenarioInit {
    pub alloc: *const *const dyn Allocator,
    pub percent: u32,
}

pub type FnScenarioNew = unsafe extern "C" fn(init: RawScenarioInit) -> Handle;
pub type FnScenarioRun = unsafe extern "C" fn(handle: Handle);

#[repr(C)]
pub enum RawScenarioKind {
    Bench,
    Validation,
}

#[repr(C)]
pub struct RawScenario {
    pub name: *const u8,
    pub name_size: usize,

    pub new: FnScenarioNew,
    pub run: FnScenarioRun,

    pub kind: RawScenarioKind,
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
}

pub type FnLoadTests = unsafe extern "C" fn() -> RawLoadResult;

#[no_mangle]
pub extern "C" fn ignore_this_cbindgen_needs_to_find_stuff(_: FnLoadTests) {}

pub type TheAlloc = dyn Allocator;
