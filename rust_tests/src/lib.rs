#![feature(allocator_api)]

mod scenarios;
mod solutions;

use std::alloc::System;

use scenarios::Scenario;
use stats_alloc::{StatsAlloc, INSTRUMENTED_SYSTEM};
use tests_api::{alloc::ArenaAlloc, Handle, RawImpl, RawLoadResult, RawScenario};

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

const fn s<'x, S: Scenario<'x>>(name: &'static str) -> RawScenario {
    unsafe extern "C" fn new<'x, S: Scenario<'x>>(alloc: *const ArenaAlloc) -> Handle {
        let s = Box::new(S::new(&*alloc));
        let ptr = Box::into_raw(s);

        ptr as Handle
    }
    unsafe extern "C" fn run<'x, S: Scenario<'x>>(handle: Handle) {
        let ptr = handle as *mut S;
        let obj = Box::from_raw(ptr);
        obj.run();
    }

    RawScenario {
        name: name.as_ptr(),
        name_size: name.len(),
        new: new::<S>,
        run: run::<S>,
    }
}

macro_rules! list_impl {
    ($name:ident) => {{
        use scenarios::*;

        const SCENARIOS: &[RawScenario] = &[
            s::<First<solutions::$name::Implementation<u64>>>("first"),
            s::<Last<solutions::$name::Implementation<u64>>>("last"),
            s::<Last<solutions::$name::Implementation<u64>>>("order"),
            s::<SearchMiddle<solutions::$name::Implementation<u64>>>("search_middle"),
            s::<SumScenario<solutions::$name::Implementation<u64>>>("sum"),
            s::<PushDeleteOneScenario<solutions::$name::Implementation<u64>>>("push_delete_one"),
            s::<PushScenario<solutions::$name::Implementation<u64>>>("push"),
            s::<Fragmentation<solutions::$name::Implementation<u64>>>("fragmentation"),
        ];

        const NAME: &str = stringify!($name);
        RawImpl {
            name: NAME.as_ptr(),
            name_size: NAME.len(),
            scenarios: SCENARIOS.as_ptr(),
            scenarios_count: SCENARIOS.len(),
        }
    }};
}

#[no_mangle]
pub unsafe extern "C" fn load_tests() -> RawLoadResult {
    const LIST_IMPLS: &[RawImpl] = &[
        list_impl!(handle_impl),
        list_impl!(slotmap_impl),
        list_impl!(nonnull_impl),
        list_impl!(index_impl),
        // sol!(index_impl),
        // sol!(nonnull_impl),
        // sol!(rc_impl),
        // sol!(slotmap_impl),
        // sol!(std_linked_list_impl),
        // sol!(std_map_impl),
    ];

    RawLoadResult {
        list_impl: LIST_IMPLS.as_ptr(),
        list_impl_count: LIST_IMPLS.len(),
    }
}
