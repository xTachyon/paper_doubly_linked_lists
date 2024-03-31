use anyhow::Result;
use ascii_table::AsciiTable;
use clap::Parser;
use humansize::{format_size, BINARY};
use libloading::{Library, Symbol};
use stats_alloc::Region;
use std::{collections::HashMap, fmt::Display, mem::ManuallyDrop, time::Instant};
use tests_api::{FnGetAlloc, FnLoadTests, FnScenarioNew, FnScenarioRun, RawLoadResult};

struct ScenarioData {
    name: &'static str,
    new: FnScenarioNew,
    run: FnScenarioRun,
}

struct TestData {
    name: String,
    scenarios: Vec<ScenarioData>,
    get_alloc: FnGetAlloc,
}

unsafe fn s(ptr: *const u8, size: usize) -> &'static str {
    let name = std::slice::from_raw_parts(ptr, size);
    let name = std::str::from_utf8(name).unwrap();
    name
}

unsafe fn wrap_raw_tests(prefix: &str, raw_tests: RawLoadResult, tests: &mut Vec<TestData>) {
    for i in 0..raw_tests.list_impl_count {
        let current = &*raw_tests.list_impl.add(i);

        let name = s(current.name, current.name_size);
        let name = format!("{}_{}", prefix, name);

        let mut scenarios = Vec::with_capacity(16);
        for i in 0..current.scenarios_count {
            let current = &*current.scenarios.add(i);

            let name = s(current.name, current.name_size);

            scenarios.push(ScenarioData {
                name,
                new: current.new,
                run: current.run,
            });
        }

        tests.push(TestData {
            name,
            scenarios,
            get_alloc: raw_tests.get_alloc,
        });
    }
}

unsafe fn load(prefix: &str, path: &str, tests: &mut Vec<TestData>) -> Result<()> {
    println!("loading {path}");

    let lib = ManuallyDrop::new(Library::new(path)?);
    let load_tests: Symbol<FnLoadTests> = lib.get(b"load_tests\0")?;

    let raw_tests = load_tests();
    wrap_raw_tests(prefix, raw_tests, tests);

    Ok(())
}

#[derive(Default)]
struct TestResultExtra {
    slower_run: String,
    max_memory: String,
}

struct TestResult<'x> {
    scenario_name: &'x str,
    impl_name: &'x str,
    run_time: u128,
    no_allocs: usize,
    max_memory: usize,
    extra: TestResultExtra,
}

fn bench<'x>(test: &'x TestData, results: &mut HashMap<&str, Vec<TestResult<'x>>>) {
    println!("testing {}..", test.name);

    for i in test.scenarios.iter() {
        let alloc = (test.get_alloc)();
        let region = Region::new(alloc);
        let object = (i.new)();
        let time = Instant::now();
        (i.run)(object);
        let elapsed = time.elapsed();
        let stats = region.change();

        results
            .entry(i.name)
            .or_insert(Vec::new())
            .push(TestResult {
                scenario_name: i.name,
                impl_name: &test.name,
                run_time: elapsed.as_millis(),
                no_allocs: stats.allocations + stats.reallocations,
                max_memory: stats.bytes_allocated,
                extra: TestResultExtra::default(),
            });
    }
}

const DEFAULT_ITERATIONS: u64 = 10_000_000;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t=DEFAULT_ITERATIONS)]
    iterations: u64,
}

const DL_NAMES: (&str, &str) = if cfg!(target_os = "windows") {
    ("rust_tests.dll", "cpp_tests.dll")
} else if cfg!(target_os = "linux") || cfg!(target_os = "android") {
    ("librust_tests.so", "libcpp_tests.so")
} else if cfg!(target_os = "macos") {
    ("librust_tests.dylib", "libcpp_tests.dylib")
} else {
    panic!("what are you running on? 🤔");
};

fn main() -> Result<()> {
    // let args = Args::parse();
    // println!("iterations={}", args.iterations);

    let mut tests = Vec::with_capacity(16);
    unsafe {
        let (rust_path, _cpp_path) = DL_NAMES;
        load("rust", rust_path, &mut tests)?;
        // load("cpp", cpp_path, &mut tests)?;
        println!();
    };

    let mut results = HashMap::new();
    for i in tests.iter() {
        bench(i, &mut results);
    }
    println!();

    for tests in results.values_mut() {
        let mut ascii_table = AsciiTable::default();
        ascii_table.set_max_width(200);
        ascii_table.column(0).set_header("scenario");
        ascii_table.column(1).set_header("impl");
        ascii_table.column(2).set_header("run");
        ascii_table.column(3).set_header("slower(run)");
        ascii_table.column(4).set_header("no. allocs");
        ascii_table.column(5).set_header("max memory");

        let min_run = tests.iter().map(|x| x.run_time).min().unwrap() as f64;
        for i in tests.iter_mut() {
            i.extra = TestResultExtra {
                slower_run: format!("{:.02}x", i.run_time as f64 / min_run),
                max_memory: format_size(i.max_memory, BINARY),
            };
        }

        let it = tests.iter().map(|x| -> [&dyn Display; 6] {
            [
                &x.scenario_name,
                &x.impl_name,
                &x.run_time,
                &x.extra.slower_run,
                &x.no_allocs,
                &x.extra.max_memory,
            ]
        });
        ascii_table.print(it);
    }

    Ok(())
}
