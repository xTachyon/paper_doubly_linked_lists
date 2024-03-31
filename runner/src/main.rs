use anyhow::Result;
use ascii_table::AsciiTable;
use clap::Parser;
use libloading::{Library, Symbol};
use std::{fmt::Display, mem::ManuallyDrop, time::Instant};
use tests_api::{FnLoadTests, FnScenarioNew, FnScenarioRun, RawLoadResult};

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
    slower_run: String,
}
struct TestResult<'x> {
    scenario_name: &'x str,
    impl_name: &'x str,
    run_time: u128,
    extra: TestResultExtra,
}

fn bench<'x>(test: &'x TestData, results: &mut Vec<TestResult<'x>>) {
    println!("testing {}..", test.name);

    for i in test.scenarios.iter() {
        let object = (i.new)();
        let time = Instant::now();
        (i.run)(object);
        let elapsed = time.elapsed();

        results.push(TestResult {
            scenario_name: i.name,
            impl_name: &test.name,
            run_time: elapsed.as_millis(),
            extra: TestResultExtra::default(),
        });
    }
}

// fn bench<'x>(test: &'x TestData, results: &mut Vec<TestResult<'x>>, iterations: u64) {
//     println!("testing {}..", test.name);

//     let time = Instant::now();
//     let obj = (test.create)(iterations as usize + 1);
//     for index in 0..iterations {
//         (test.add)(obj, iterations - index);
//     }
//     let creation_time = time.elapsed();

//     let time = Instant::now();
//     let sum = (test.sum_all)(obj);
//     let run_time = time.elapsed();

//     assert_eq!(sum, iterations * (iterations + 1) / 2);

//     let time = Instant::now();
//     (test.destroy)(obj);
//     let destroy_time = time.elapsed();

//     let total_time = creation_time + run_time + destroy_time;
//     results.push(TestResult {
//         name: &test.name,
//         creation_time: creation_time.as_millis(),
//         run_time: run_time.as_millis(),
//         destroy_time: destroy_time.as_millis(),
//         total_time: total_time.as_millis(),
//         extra: TestResultExtra::default(),
//     });
// }

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

    let mut results = Vec::new();
    for i in tests.iter() {
        bench(i, &mut results);
    }
    println!();

    results.sort_by_key(|x| x.scenario_name);

    let mut ascii_table = AsciiTable::default();
    ascii_table.set_max_width(200);
    ascii_table.column(0).set_header("scenario");
    ascii_table.column(1).set_header("impl");
    ascii_table.column(2).set_header("run");
    ascii_table.column(3).set_header("slower(run)");
    ascii_table.column(4).set_header("no. allocs");
    ascii_table.column(5).set_header("max memory");

    let min_run = results.iter().map(|x| x.run_time).min().unwrap() as f64;
    for i in results.iter_mut() {
        i.extra = TestResultExtra {
            slower_run: format!("{:.02}x", i.run_time as f64 / min_run),
            // max_memory: format_size(i.max_memory, BINARY),
        };
    }

    let it = results.iter().map(|x| -> [&dyn Display; 4] {
        [
            &x.scenario_name,
            &x.impl_name,
            &x.run_time,
            &x.extra.slower_run,
        ]
    });
    ascii_table.print(it);

    Ok(())
}
