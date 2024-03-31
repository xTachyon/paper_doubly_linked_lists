mod scenarios;
mod solutions;

use std::alloc::System;

use scenarios::{PushDeleteScenario, Scenario, SumScenario};
use stats_alloc::{StatsAlloc, INSTRUMENTED_SYSTEM};
use tests_api::{Handle, RawImpl, RawLoadResult, RawScenario};

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

const fn scenario<S: Scenario>(name: &'static str) -> RawScenario {
    unsafe extern "C" fn new<S: Scenario>() -> Handle {
        let s = Box::new(S::new());
        let ptr = Box::into_raw(s);

        ptr as Handle
    }
    unsafe extern "C" fn run<S: Scenario>(handle: Handle) {
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
        const SCENARIOS: &[RawScenario] = &[
            scenario::<SumScenario<solutions::$name::Implementation<u64>>>("sum"),
            scenario::<PushDeleteScenario<solutions::$name::Implementation<u64>>>("push_delete"),
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

extern "C" fn get_alloc() -> &'static StatsAlloc<System> {
    &GLOBAL
}

#[no_mangle]
pub unsafe extern "C" fn load_tests() -> RawLoadResult {
    const LIST_IMPLS: &[RawImpl] = &[
        list_impl!(handle_impl),
        list_impl!(slotmap_impl),
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
        get_alloc,
    }
}
