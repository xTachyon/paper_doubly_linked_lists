mod solutions;

use tests_api::{Handle, RawLoadTestsResult, RawTestData};

macro_rules! sol {
    ($name:ident) => {{
        extern "C" fn create(nodes: usize) -> Handle {
            let sol = Box::new(solutions::$name::DoubleLinkedList::new(nodes));
            let ptr = Box::into_raw(sol);

            ptr as Handle
        }
        extern "C" fn destroy(handle: Handle) {
            let ptr = handle as *mut solutions::$name::DoubleLinkedList;
            let _ = unsafe { Box::from_raw(ptr) };
        }
        extern "C" fn add(handle: Handle, element: u64) {
            let ptr = handle as *mut solutions::$name::DoubleLinkedList;
            let obj = unsafe { &mut *ptr };
            obj.add(element);
        }
        extern "C" fn sum_all(handle: Handle) -> u64 {
            let ptr = handle as *mut solutions::$name::DoubleLinkedList;
            let obj = unsafe { &mut *ptr };
            obj.sum_all()
        }

        const NAME: &str = stringify!($name);
        RawTestData {
            name: NAME.as_ptr(),
            name_size: NAME.len(),
            create,
            destroy,
            add,
            sum_all,
        }
    }};
}

#[no_mangle]
pub unsafe extern "C" fn load_tests() -> RawLoadTestsResult {
    const TESTS: &[RawTestData] = &[
        sol!(index_impl),
        sol!(handle_impl),
        sol!(nonnull_impl),
        sol!(rc_impl),
        sol!(slotmap_impl),
        sol!(std_linked_list_impl),
        sol!(std_map_impl),
    ];

    RawLoadTestsResult {
        tests: TESTS.as_ptr(),
        tests_count: TESTS.len(),
    }
}
