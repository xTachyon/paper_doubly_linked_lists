## Running the tests

Use the workspace runner binary; it exposes all test parameters.

- Prereq: nightly Rust (`#![feature(allocator_api)]` is used).
- From the repo root: `cargo run -p runner --release -- [flags]`
- Quick help: `cargo run -p runner -- --help`

### Flags
- `-a, --allocator <default|system|arena|sn>`: allocator choice. `default` picks `arena` for validation/safety, otherwise `system`.
- `-p, --percent <1-100>`: scales iteration count (e.g. `10` for a quick pass).
- `-k, --kinds <bench,validation,safety>`: comma-separated; default `bench`.
- `-i, --impl-name <name>`: run one implementation (names are prefixed with `rust_`, e.g. `rust_handle_impl`, `rust_slotmap_impl`, `rust_raw_impl`, `rust_gen_arena_impl`, …).
- `-s, --scenario <name>`: run one scenario.

### Scenarios
- Validation: `first`, `last`, `order`, `mutate_in_place`
- Bench: `find_string`, `push_pages`, `iterate_pages`, `add_front_back`, `search_middle`, `sum`, `push_delete_one`, `push`, `fragmentation`
- Safety: `use_after_delete`, `use_after_delete_reinsert`, `double_free`

### Examples
- Default benches (system allocator):  
  `cargo run -p runner --release --`
- Faster smoke run:  
  `cargo run -p runner --release -- -p 10 -k bench`
- Validation only (arena allocator enforced):  
  `cargo run -p runner --release -- -k validation`
- Safety for one impl/scenario:  
  `cargo run -p runner --release -- -k safety -i rust_raw_impl -s double_free`
