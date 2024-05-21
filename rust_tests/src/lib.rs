#![feature(allocator_api)]

mod scenarios;
mod solutions;

use scenarios::Scenario;
use tests_api::{Handle, RawImpl, RawLoadResult, RawScenario, RawScenarioInit, RawScenarioKind};

use crate::scenarios::ScenarioInit;

const fn sc<'x, S: Scenario<'x>>(name: &'static str, kind: RawScenarioKind) -> RawScenario {
    // TODO: + 'static?
    unsafe extern "C" fn new<'x, S: Scenario<'x>>(init: RawScenarioInit) -> Handle {
        let alloc = &**init.alloc;
        let init = ScenarioInit {
            alloc,
            percent: init.percent,
        };
        let s = Box::new(S::new(init));
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
        kind,
    }
}

const fn sb<'x, S: Scenario<'x>>(name: &'static str) -> RawScenario {
    sc::<S>(name, RawScenarioKind::Bench)
}
const fn sv<'x, S: Scenario<'x>>(name: &'static str) -> RawScenario {
    sc::<S>(name, RawScenarioKind::Validation)
}

macro_rules! list_impl {
    ($name:ident) => {{
        use scenarios::*;

        const SCENARIOS: &[RawScenario] = &[
            // validation
            // sv::<UseAfterDelete<solutions::$name::Implementation<u64>>>("use_after_delete"),
            sv::<First<solutions::$name::Implementation<u64>>>("first"),
            sv::<Last<solutions::$name::Implementation<u64>>>("last"),
            sv::<Last<solutions::$name::Implementation<u64>>>("order"),
            // bench
            sb::<SearchMiddle<solutions::$name::Implementation<u64>>>("search_middle"),
            sb::<SumScenario<solutions::$name::Implementation<u64>>>("sum"),
            sb::<PushDeleteOneScenario<solutions::$name::Implementation<u64>>>("push_delete_one"),
            sb::<PushScenario<solutions::$name::Implementation<u64>>>("push"),
            sb::<Fragmentation<solutions::$name::Implementation<u64>>>("fragmentation"),
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
