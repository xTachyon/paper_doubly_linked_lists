use anyhow::Result;
use ascii_table::AsciiTable;
use clap::Parser;
use libloading::{Library, Symbol};
use std::{fmt::Display, mem::ManuallyDrop, time::Instant};
use tests_api::{AddFn, CreateFn, DestroyFn, LoadTestsFn, RawLoadTestsResult, SumAllFn};

struct TestData {
    pub name: &'static str,

    pub create: CreateFn,
    pub destroy: DestroyFn,
    pub add: AddFn,
    pub sum_all: SumAllFn,
}

unsafe fn wrap_raw_tests(raw_tests: RawLoadTestsResult, tests: &mut Vec<TestData>) {
    for i in 0..raw_tests.tests_count {
        let current = &*raw_tests.tests.add(i);

        let name = std::slice::from_raw_parts(current.name, current.name_size);
        let name = std::str::from_utf8(name).unwrap();

        tests.push(TestData {
            name,
            create: current.create,
            destroy: current.destroy,
            add: current.add,
            sum_all: current.sum_all,
        });
    }
}

unsafe fn load(path: &str, tests: &mut Vec<TestData>) -> Result<()> {
    println!("loading {path}");

    let lib = ManuallyDrop::new(Library::new(path)?);
    let load_tests: Symbol<LoadTestsFn> = lib.get(b"load_tests\0")?;

    let raw_tests = load_tests();
    wrap_raw_tests(raw_tests, tests);

    Ok(())
}

#[derive(Default)]
struct TestResultExtra {
    slower_total: String,
    slower_run: String,
}
struct TestResult {
    name: &'static str,
    creation_time: u128,
    run_time: u128,
    destroy_time: u128,
    total_time: u128,
    extra: TestResultExtra,
}

fn bench(test: &TestData, results: &mut Vec<TestResult>, iterations: u64) {
    println!("testing {}..", test.name);

    let time = Instant::now();
    let obj = (test.create)(iterations as usize);
    for index in 0..iterations {
        (test.add)(obj, iterations - index);
    }
    let creation_time = time.elapsed();

    let time = Instant::now();
    let sum = (test.sum_all)(obj);
    let run_time = time.elapsed();

    assert_eq!(sum, iterations * (iterations + 1) / 2);

    let time = Instant::now();
    (test.destroy)(obj);
    let destroy_time = time.elapsed();

    let total_time = creation_time + run_time + destroy_time;
    results.push(TestResult {
        name: test.name,
        creation_time: creation_time.as_millis(),
        run_time: run_time.as_millis(),
        destroy_time: destroy_time.as_millis(),
        total_time: total_time.as_millis(),
        extra: TestResultExtra::default(),
    });
}

const DEFAULT_ITERATIONS: u64 = 1_000_000;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t=DEFAULT_ITERATIONS)]
    iterations: u64
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("iterations={}", args.iterations);

    let mut tests = Vec::with_capacity(16);
    unsafe { load("rust_tests.dll", &mut tests)? };

    let mut results = Vec::new();
    for i in tests {
        bench(&i, &mut results, args.iterations);
    }

    let mut ascii_table = AsciiTable::default();
    ascii_table.set_max_width(200);
    ascii_table.column(0).set_header("name");
    ascii_table.column(1).set_header("creation");
    ascii_table.column(2).set_header("run");
    ascii_table.column(3).set_header("destroy");
    ascii_table.column(4).set_header("total");
    ascii_table.column(5).set_header("slower(total)");
    ascii_table.column(6).set_header("slower(run)");
    ascii_table.column(7).set_header("no. allocs");
    ascii_table.column(8).set_header("max memory");

    let min_total = results.iter().map(|x| x.total_time).min().unwrap() as f64;
    let min_run = results.iter().map(|x| x.run_time).min().unwrap() as f64;
    for i in results.iter_mut() {
        i.extra = TestResultExtra {
            slower_total: format!("{:.02}x", i.total_time as f64 / min_total),
            slower_run: format!("{:.02}x", i.run_time as f64 / min_run),
            // max_memory: format_size(i.max_memory, BINARY),
        };
    }

    let it = results.iter().map(|x| -> [&dyn Display; 7] {
        [
            &x.name,
            &x.creation_time,
            &x.run_time,
            &x.destroy_time,
            &x.total_time,
            &x.extra.slower_total,
            &x.extra.slower_run,
        ]
    });
    ascii_table.print(it);

    Ok(())
}
