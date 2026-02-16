#![feature(allocator_api)]

use anyhow::Result;
use ascii_table::{Align, AsciiTable};
use clap::Parser;
use humansize::{format_size, BINARY};
use indexmap::IndexMap;
use std::{
    alloc::{Allocator, Global},
    array,
    fmt::Display,
    time::{Duration, Instant},
};
use tests_api::{
    arena_alloc::ArenaAlloc,
    snalloc::SnAlloc,
    stats_alloc::{BoxedAllocator, StatsAllocator},
    FnScenarioNew, FnScenarioRun, FnScenarioRunSafe, RawLoadResult, RawScenarioInit,
    RawScenarioKind,
};

struct ScenarioData {
    name: &'static str,
    new: FnScenarioNew,
    run: FnScenarioRun,
    run_safe: FnScenarioRunSafe,
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
    is_safety: bool,
    specific_impl: Option<String>,
    specific_scenario: Option<String>,
) {
    let mut specific_impl_found = false;
    let mut specific_scenario_found = false;
    for i in 0..raw_tests.list_impl_count {
        let current = &*raw_tests.list_impl.add(i);

        let name = s(current.name, current.name_size);
        let name = format!("{}_{}", prefix, name);

        if let Some(n) = specific_impl.as_deref() {
            if n != name {
                continue;
            }
            specific_impl_found = true;
        }

        let mut scenarios = Vec::with_capacity(16);
        for i in 0..current.scenarios_count {
            let current = &*current.scenarios.add(i);

            let add = match (&current.kind, is_bench, is_validation, is_safety) {
                (RawScenarioKind::Bench, true, _, _) => true,
                (RawScenarioKind::Validation, _, true, _) => true,
                (RawScenarioKind::Safety, _, _, true) => true,
                _ => false,
            };
            if !add {
                continue;
            }

            let name = s(current.name, current.name_size);

            if let Some(n) = specific_scenario.as_deref() {
                if n != name {
                    continue;
                }
                specific_scenario_found = true;
            }

            scenarios.push(ScenarioData {
                name,
                new: current.new,
                run: current.run,
                run_safe: current.run_safe,
            });
        }

        tests.push(TestData { name, scenarios });
    }

    match specific_impl {
        Some(x) if !specific_impl_found => {
            panic!("no impl with the name `{}` was found", x);
        }
        _ => {}
    }
    match specific_scenario {
        Some(x) if !specific_scenario_found => {
            panic!("no scenario with the name `{}` was found", x);
        }
        _ => {}
    }
}

unsafe fn load(
    prefix: &str,
    _path: &str,
    tests: &mut Vec<TestData>,
    is_bench: bool,
    is_validation: bool,
    is_safety: bool,
    specific_impl: Option<String>,
    specific_scenario: Option<String>,
) -> Result<()> {
    let raw_tests = rust_tests::load_tests();
    wrap_raw_tests(
        prefix,
        raw_tests,
        tests,
        is_bench,
        is_validation,
        is_safety,
        specific_impl,
        specific_scenario,
    );

    Ok(())
}

#[derive(Default)]
struct TestResultExtra {
    run_time: String,
    alloc_time: String,
    slower_run: String,
    max_memory: String,
}

struct TestResult<'x> {
    scenario: &'x str,
    impl_name: &'x str,
    run_time: Duration,
    alloc_time: Duration,
    no_allocs: usize,
    max_memory: usize,
    extra: TestResultExtra,
}

fn bench<'x>(
    test: &'x TestData,
    results: &mut IndexMap<&str, Vec<TestResult<'x>>>,
    allocator_kind: AllocatorKind,
    percent: u32,
    is_bench: bool,
) {
    for i in test.scenarios.iter() {
        let alloc = BoxedAllocator(allocator_kind.create(is_bench));
        let alloc = StatsAllocator::new(alloc);

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
        let alloc_time = alloc.time();
        results
            .entry(i.name)
            .or_default()
            .push(TestResult {
                scenario: i.name,
                impl_name: &test.name,
                run_time: elapsed - alloc_time,
                alloc_time,
                no_allocs: alloc.no_allocs(),
                max_memory: alloc.max_allocated(),
                extra: TestResultExtra::default(),
            });
    }
}

#[derive(Clone, Copy, PartialEq)]
enum SafetyResult {
    Pass, // Panicked as expected (detected the error)
    Fail, // Did not panic (failed to detect the error)
}

impl std::fmt::Display for SafetyResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SafetyResult::Pass => write!(f, "✓ PASS"),
            SafetyResult::Fail => write!(f, "✗ FAIL"),
        }
    }
}

struct SafetyTestResult<'x> {
    impl_name: &'x str,
    results: Vec<(&'x str, SafetyResult)>,
}

fn run_safety_tests<'x>(
    test: &'x TestData,
    results: &mut Vec<SafetyTestResult<'x>>,
    allocator_kind: AllocatorKind,
    percent: u32,
) {
    let mut test_results = Vec::new();

    for scenario in test.scenarios.iter() {
        let alloc = BoxedAllocator(allocator_kind.create(false));
        let alloc = StatsAllocator::new(alloc);

        let alloc_ptr: *const dyn Allocator = &alloc;
        let alloc_ptr = &alloc_ptr;
        let init = RawScenarioInit {
            alloc: alloc_ptr,
            percent,
        };

        let object = unsafe { (scenario.new)(init) };

        let panicked = unsafe { (scenario.run_safe)(object) };

        let safety_result = if panicked {
            SafetyResult::Fail
        } else {
            SafetyResult::Pass
        };

        test_results.push((scenario.name, safety_result));
    }

    results.push(SafetyTestResult {
        impl_name: &test.name,
        results: test_results,
    });
}

fn create_safety_table(scenario_names: &[&str]) -> AsciiTable {
    let mut ascii_table = AsciiTable::default();
    ascii_table.set_max_width(200);

    ascii_table
        .column(0)
        .set_header("Implementation")
        .set_align(Align::Left);

    for (index, name) in scenario_names.iter().enumerate() {
        ascii_table
            .column(index + 1)
            .set_header(*name)
            .set_align(Align::Center);
    }

    ascii_table
}

fn print_safety_results(results: &[SafetyTestResult]) {
    if results.is_empty() {
        println!("No safety tests to run.");
        return;
    }

    let scenario_names: Vec<&str> = results
        .first()
        .map(|r| r.results.iter().map(|(name, _)| *name).collect())
        .unwrap_or_default();

    if scenario_names.is_empty() {
        println!("No safety scenarios found.");
        return;
    }

    let table = create_safety_table(&scenario_names);

    let mut output: Vec<Vec<String>> = Vec::new();
    for result in results {
        let mut row = vec![result.impl_name.to_string()];
        for (_, safety_result) in &result.results {
            row.push(safety_result.to_string());
        }
        output.push(row);
    }

    let output_refs: Vec<Vec<&dyn Display>> = output
        .iter()
        .map(|row| row.iter().map(|s| s as &dyn Display).collect())
        .collect();

    table.print(output_refs.iter().map(|r| r.as_slice()));

    println!();
    let total_tests = results.len() * scenario_names.len();
    let passed = results
        .iter()
        .flat_map(|r| &r.results)
        .filter(|(_, r)| *r == SafetyResult::Pass)
        .count();
    println!(
        "Safety Summary: {}/{} implementations correctly detect errors",
        passed, total_tests
    );
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

    /// Run only a specific impl
    #[arg(short, long)]
    impl_name: Option<String>,

    /// Run only a specific scenario
    #[arg(short, long)]
    scenario: Option<String>,
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum AllocatorKind {
    System,
    Arena,
    Sn,
}
impl AllocatorKind {
    fn create(self, is_bench: bool) -> Box<dyn Allocator> {
        let size = if is_bench {
            2 * 1024 * 1024 * 1024
        } else {
            4096
        };
        match self {
            AllocatorKind::System => Box::new(Global),
            AllocatorKind::Arena => Box::new(ArenaAlloc::new(size)),
            AllocatorKind::Sn => Box::new(SnAlloc::new()),
        }
    }

    fn parse(name: &str, default: AllocatorKind) -> AllocatorKind {
        match name {
            "default" => default,
            "system" => AllocatorKind::System,
            "arena" => AllocatorKind::Arena,
            "sn" => AllocatorKind::Sn,
            _ => panic!("unknown allocator: {name}"),
        }
    }
}

fn parse_scenarios(s: String) -> (bool, bool, bool) {
    let mut is_bench = false;
    let mut is_validation = false;
    let mut is_safety = false;

    for i in s.split(',') {
        match i {
            "bench" => is_bench = true,
            "validation" => is_validation = true,
            "safety" => is_safety = true,
            _ => panic!("unknown kind `{i}`"),
        }
    }

    (is_bench, is_validation, is_safety)
}

fn create_table() -> AsciiTable {
    let mut ascii_table = AsciiTable::default();
    ascii_table.set_max_width(200);

    let columns = [
        ("scenario", Align::Center),
        ("name", Align::Center),
        ("time", Align::Right),
        ("alloc_time", Align::Right),
        ("slower(run)", Align::Right),
        ("no. allocs", Align::Right),
        ("max memory", Align::Right),
    ];

    for (index, (name, alignment)) in columns.iter().enumerate() {
        ascii_table
            .column(index)
            .set_header(*name)
            .set_align(*alignment);
    }

    ascii_table
}

fn main_impl() -> Result<()> {
    let args = Args::parse();
    if !(1..=100).contains(&args.percent) {
        panic!("percent expected to between 1..=100");
    }
    let (is_bench, is_validation, is_safety) = parse_scenarios(args.kinds);
    let default_allocator = if is_validation || is_safety {
        AllocatorKind::Arena
    } else {
        AllocatorKind::System
    };
    let allocator_kind = AllocatorKind::parse(&args.allocator, default_allocator);
    if is_validation && allocator_kind != AllocatorKind::Arena {
        panic!("validation must be run with arena allocator");
    }

    let mut tests = Vec::with_capacity(16);
    unsafe {
        let (rust_path, _cpp_path) = DL_NAMES;
        load(
            "rust",
            rust_path,
            &mut tests,
            is_bench,
            is_validation,
            is_safety,
            args.impl_name,
            args.scenario,
        )?;
    };

    if is_safety {
        let mut safety_results = Vec::new();
        for test in tests.iter() {
            run_safety_tests(test, &mut safety_results, allocator_kind, args.percent);
        }
        print_safety_results(&safety_results);
        return Ok(());
    }

    let mut results = IndexMap::new();
    for i in tests.iter() {
        bench(i, &mut results, allocator_kind, args.percent, is_bench);
    }
 

    let mut output: Vec<[&dyn Display; 7]> = Vec::with_capacity(64);
    for tests in results.values_mut() {
        let min_run = tests.iter().map(|x| x.run_time.as_millis()).min().unwrap() as f64;
        tests.sort_by_key(|x| x.run_time);
        for i in tests {
            i.extra = TestResultExtra {
                run_time: format!("{:?}", i.run_time),
                alloc_time: format!("{:?}", i.alloc_time),
                slower_run: format!("{:.02}x", i.run_time.as_millis() as f64 / min_run),
                max_memory: format_size(i.max_memory, BINARY),
            };

            output.push([
                &i.scenario,
                &i.impl_name,
                &i.extra.run_time,
                &i.extra.alloc_time,
                &i.extra.slower_run,
                &i.no_allocs,
                &i.extra.max_memory,
            ]);
        }
        let dashes = &"------";
        let arr = array::from_fn(|_| dashes as &dyn Display);
        output.push(arr);
    }

    if is_bench {
        create_table().print(output.iter());
    }

    Ok(())
}

// fn main() -> Result<()> {
//     let f = || {
//         loop{
//             let start = Instant::now();
//             let _ = main_impl();
//             //println!("total time: {:?}", start.elapsed());
//         }
//     };
//     stacker::grow(64 * 1024 * 1024, f)
// }

fn main() -> Result<()> {
    let f = || {
        //let start = Instant::now();
        let result = main_impl();
        // println!("total time: {:?}", start.elapsed());
        result
    };
    stacker::grow(64 * 1024 * 1024, f)
}
