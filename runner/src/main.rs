#![feature(allocator_api)]

use anyhow::Result;
use ascii_table::{Align, AsciiTable};
use clap::Parser;
use humansize::{format_size, BINARY};
use libloading::{Library, Symbol};
use std::{
    alloc::{Allocator, Global},
    collections::HashMap,
    fmt::Display,
    mem::ManuallyDrop,
    time::{Duration, Instant},
};
use tests_api::{
    stats_alloc::StatsAllocator, FnLoadTests, FnScenarioNew, FnScenarioRun, RawLoadResult,
};

struct ScenarioData {
    name: &'static str,
    new: FnScenarioNew,
    run: FnScenarioRun,
}

struct TestData {
    name: String,
    scenarios: Vec<ScenarioData>,
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

        tests.push(TestData { name, scenarios });
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
    run_time: String,
    slower_run: String,
    max_memory: String,
}

struct TestResult<'x> {
    impl_name: &'x str,
    run_time: Duration,
    no_allocs: usize,
    max_memory: usize,
    extra: TestResultExtra,
}

fn bench<'x>(test: &'x TestData, results: &mut HashMap<&str, Vec<TestResult<'x>>>) {
    println!("testing {}..", test.name);

    for i in test.scenarios.iter() {
        let alloc = StatsAllocator::new(Box::leak(Box::new(Global)) as &_);

        let alloc_ptr: *const dyn Allocator = &alloc;
        let alloc_ptr = &alloc_ptr;
        let object = unsafe { (i.new)(alloc_ptr) };
        alloc.reset_time();
        let time = Instant::now();
        unsafe { (i.run)(object) };
        let elapsed = time.elapsed();

        results
            .entry(i.name)
            .or_insert(Vec::new())
            .push(TestResult {
                impl_name: &test.name,
                run_time: elapsed - alloc.time(),
                no_allocs: alloc.no_allocs(),
                max_memory: alloc.max_allocated(),
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

    for (scenario, mut tests) in results {
        let mut ascii_table = AsciiTable::default();
        ascii_table.set_max_width(200);
        ascii_table
            .column(0)
            .set_header(format!("scenario: {scenario}"))
            .set_align(Align::Center);
        ascii_table
            .column(1)
            .set_header("run")
            .set_align(Align::Right);
        ascii_table
            .column(2)
            .set_header("slower(run)")
            .set_align(Align::Right);
        ascii_table
            .column(3)
            .set_header("no. allocs")
            .set_align(Align::Right);
        ascii_table
            .column(4)
            .set_header("max memory")
            .set_align(Align::Right);

        let min_run = tests.iter().map(|x| x.run_time.as_millis()).min().unwrap() as f64;
        for i in tests.iter_mut() {
            i.extra = TestResultExtra {
                run_time: format!("{:?}", i.run_time),
                slower_run: format!("{:.02}x", i.run_time.as_millis() as f64 / min_run),
                max_memory: format_size(i.max_memory, BINARY),
            };
        }

        let it = tests.iter().map(|x| -> [&dyn Display; 5] {
            [
                &x.impl_name,
                &x.extra.run_time,
                &x.extra.slower_run,
                &x.no_allocs,
                &x.extra.max_memory,
            ]
        });
        ascii_table.print(it);
    }

    Ok(())
}
