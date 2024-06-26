#![feature(allocator_api)]

use anyhow::Result;
use ascii_table::{Align, AsciiTable};
use clap::{arg, Parser};
use humansize::{format_size, BINARY};
use indexmap::IndexMap;
use libloading::{Library, Symbol};
use std::{
    alloc::{Allocator, Global},
    array,
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

            let add = match (&current.kind, is_bench, is_validation) {
                (RawScenarioKind::Bench, true, _) | (RawScenarioKind::Validation, _, true) => true,
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
    path: &str,
    tests: &mut Vec<TestData>,
    is_bench: bool,
    is_validation: bool,
    specific_impl: Option<String>,
    specific_scenario: Option<String>,
) -> Result<()> {
    println!("loading {path}");

    let load_tests = if false {
        let lib = ManuallyDrop::new(Library::new(path)?);
        let _load_tests: Symbol<FnLoadTests> = lib.get(b"load_tests\0")?;
        todo!()
    } else {
        rust_tests::load_tests
    };

    let raw_tests = load_tests();
    wrap_raw_tests(
        prefix,
        raw_tests,
        tests,
        is_bench,
        is_validation,
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
    println!("testing {}", test.name);

    for i in test.scenarios.iter() {
        println!("    scenario {}", i.name);
        let alloc = allocator_kind.create(is_bench);
        let alloc: &'static dyn Allocator = unsafe {
            // TODO: this is here to transmute the lifetime to static.
            // This is not great and should fixed at some point.
            std::mem::transmute(&*alloc)
        };
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
            .or_insert(Vec::new())
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
    fn name(self) -> &'static str {
        match self {
            AllocatorKind::System => "system",
            AllocatorKind::Arena => "arena",
            AllocatorKind::Sn => "sn",
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
    let (is_bench, is_validation) = parse_scenarios(args.kinds);
    let default_allocator = if is_validation {
        AllocatorKind::Arena
    } else {
        AllocatorKind::System
    };
    let allocator_kind = AllocatorKind::parse(&args.allocator, default_allocator);
    if is_validation && allocator_kind != AllocatorKind::Arena {
        panic!("validation must be run with arena allocator");
    }
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
        load(
            "rust",
            rust_path,
            &mut tests,
            is_bench,
            is_validation,
            args.impl_name,
            args.scenario,
        )?;
        // load("cpp", cpp_path, &mut tests)?;
        println!();
    };

    println!(
        "no of impls: {}\nno of scenarios: {}\n",
        tests.len(),
        tests.first().unwrap().scenarios.len()
    );

    let mut results = IndexMap::new();
    for i in tests.iter() {
        bench(i, &mut results, allocator_kind, args.percent, is_bench);
    }
    println!();

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

fn main() -> Result<()> {
    let f = || {
        let start = Instant::now();
        let result = main_impl();
        println!("total time: {:?}", start.elapsed());
        result
    };
    stacker::grow(64 * 1024 * 1024, f)
}
