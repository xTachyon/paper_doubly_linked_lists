#![feature(allocator_api)]

use anyhow::Result;
use ascii_table::{Align, AsciiTable};
use clap::{arg, Parser};
use humansize::{format_size, BINARY};
use indexmap::IndexMap;
use libloading::{Library, Symbol};
use std::{
    alloc::{Allocator, Global},
    fmt::Display,
    mem::ManuallyDrop,
    time::{Duration, Instant},
};
use tests_api::{
    arena_alloc::ArenaAlloc, snalloc::SnAlloc, stats_alloc::StatsAllocator, FnLoadTests,
    FnScenarioNew, FnScenarioRun, RawLoadResult, RawScenarioInit, RawScenarioKind,
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

unsafe fn wrap_raw_tests(
    prefix: &str,
    raw_tests: RawLoadResult,
    tests: &mut Vec<TestData>,
    is_bench: bool,
    is_validation: bool,
) {
    for i in 0..raw_tests.list_impl_count {
        let current = &*raw_tests.list_impl.add(i);

        let name = s(current.name, current.name_size);
        let name = format!("{}_{}", prefix, name);

        let mut scenarios = Vec::with_capacity(16);
        for i in 0..current.scenarios_count {
            let current = &*current.scenarios.add(i);

            let add = match (&current.kind, is_bench, is_validation) {
                (RawScenarioKind::Bench, true, _) | (RawScenarioKind::Validation, _, true) => true,
                _ => false,
            };
            if !add {
                continue;
            }

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

unsafe fn load(
    prefix: &str,
    path: &str,
    tests: &mut Vec<TestData>,
    is_bench: bool,
    is_validation: bool,
) -> Result<()> {
    println!("loading {path}");

    let lib = ManuallyDrop::new(Library::new(path)?);
    let load_tests: Symbol<FnLoadTests> = lib.get(b"load_tests\0")?;

    let raw_tests = load_tests();
    wrap_raw_tests(prefix, raw_tests, tests, is_bench, is_validation);

    Ok(())
}

#[derive(Default)]
struct TestResultExtra {
    run_time: String,
    slower_run: String,
    max_memory: String,
}

struct TestResult<'x> {
    scenario: &'x str,
    impl_name: &'x str,
    run_time: Duration,
    no_allocs: usize,
    max_memory: usize,
    extra: TestResultExtra,
}

fn bench<'x>(
    test: &'x TestData,
    results: &mut IndexMap<&str, Vec<TestResult<'x>>>,
    allocator_kind: AllocatorKind,
    percent: u32,
) {
    println!("testing {}..", test.name);

    for i in test.scenarios.iter() {
        let alloc = StatsAllocator::new(Box::leak(allocator_kind.create()) as &_);
        // TODO: leak

        let alloc_ptr: *const dyn Allocator = &alloc;
        let alloc_ptr = &alloc_ptr;
        let init = RawScenarioInit {
            alloc: alloc_ptr,
            percent,
        };
        let object = unsafe { (i.new)(init) };
        alloc.reset_time();
        let time = Instant::now();
        unsafe { (i.run)(object) };
        let elapsed = time.elapsed();

        results
            .entry(i.name)
            .or_insert(Vec::new())
            .push(TestResult {
                scenario: i.name,
                impl_name: &test.name,
                run_time: elapsed - alloc.time(),
                no_allocs: alloc.no_allocs(),
                max_memory: alloc.max_allocated(),
                extra: TestResultExtra::default(),
            });
    }
}

#[derive(Parser)]
struct Args {
    // Allocators: default, system, arena, sn
    #[arg(short, long, default_value = "default")]
    allocator: String,
    /// Percent of number of iterations of tests
    #[arg(short, long, default_value_t = 100)]
    percent: u32,

    /// Enable bench tests
    #[arg(short, long, default_value = "bench")]
    kinds: String,
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

#[derive(Clone, Copy)]
enum AllocatorKind {
    System,
    Arena,
    Sn,
}
impl AllocatorKind {
    fn create(self) -> Box<dyn Allocator> {
        match self {
            AllocatorKind::System => Box::new(Global),
            AllocatorKind::Arena => Box::new(ArenaAlloc::new(2 * 1024 * 1024 * 1024)),
            AllocatorKind::Sn => Box::new(SnAlloc::new()),
        }
    }
    fn name(self) -> &'static str {
        match self {
            AllocatorKind::System => "system",
            AllocatorKind::Arena => "arena",
            AllocatorKind::Sn => "sn",
        }
    }
    fn parse(name: &str) -> AllocatorKind {
        match name {
            "default" | "system" => AllocatorKind::System,
            "arena" => AllocatorKind::Arena,
            "sn" => AllocatorKind::Sn,
            _ => panic!("unknown allocator: {name}"),
        }
    }
}

fn parse_scenarios(s: String) -> (bool, bool) {
    let mut is_bench = false;
    let mut is_validation = false;

    for i in s.split(',') {
        match i {
            "bench" => is_bench = true,
            "validation" => is_validation = true,
            _ => panic!("unknown kind `{i}`"),
        }
    }

    (is_bench, is_validation)
}

fn create_table() -> AsciiTable {
    let mut ascii_table = AsciiTable::default();
    ascii_table.set_max_width(200);

    ascii_table
        .column(0)
        .set_header("scenario")
        .set_align(Align::Center);
    ascii_table
        .column(1)
        .set_header("name")
        .set_align(Align::Center);
    ascii_table
        .column(2)
        .set_header("run")
        .set_align(Align::Right);
    ascii_table
        .column(3)
        .set_header("slower(run)")
        .set_align(Align::Right);
    ascii_table
        .column(4)
        .set_header("no. allocs")
        .set_align(Align::Right);
    ascii_table
        .column(5)
        .set_header("max memory")
        .set_align(Align::Right);

    ascii_table
}

fn main_impl() -> Result<()> {
    let args = Args::parse();
    let allocator_kind = AllocatorKind::parse(&args.allocator);
    if !(1..=100).contains(&args.percent) {
        panic!("percent expected to between 1..=100");
    }
    let (is_bench, is_validation) = parse_scenarios(args.kinds);
    println!(
        "allocator: {}\npercent: {}\nbench: {}\nvalidation: {}",
        allocator_kind.name(),
        args.percent,
        is_bench,
        is_validation
    );

    let mut tests = Vec::with_capacity(16);
    unsafe {
        let (rust_path, _cpp_path) = DL_NAMES;
        load("rust", rust_path, &mut tests, is_bench, is_validation)?;
        // load("cpp", cpp_path, &mut tests)?;
        println!();
    };

    let mut results = IndexMap::new();
    for i in tests.iter() {
        bench(i, &mut results, allocator_kind, args.percent);
    }
    println!();

    let mut output: Vec<[&dyn Display; 6]> = Vec::with_capacity(64);
    for tests in results.values_mut() {
        let min_run = tests.iter().map(|x| x.run_time.as_millis()).min().unwrap() as f64;
        tests.sort_by_key(|x| x.run_time);
        for i in tests {
            i.extra = TestResultExtra {
                run_time: format!("{:?}", i.run_time),
                slower_run: format!("{:.02}x", i.run_time.as_millis() as f64 / min_run),
                max_memory: format_size(i.max_memory, BINARY),
            };

            output.push([
                &i.scenario,
                &i.impl_name,
                &i.extra.run_time,
                &i.extra.slower_run,
                &i.no_allocs,
                &i.extra.max_memory,
            ]);
        }
        let dashes = &"------";
        output.push([dashes, dashes, dashes, dashes, dashes, dashes]);
    }

    create_table().print(output.iter());

    Ok(())
}

fn main() -> Result<()> {
    let start = Instant::now();
    let result = main_impl();
    println!("total time: {:?}", start.elapsed());
    result
}
