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
- Bench: `linear_search_exp`, `large_node_growth`, `large_node_traversal`, `bidir_growth`, `linear_lookup`, `full_traversal`, `alloc_reuse`, `bulk_append`, `frag_stress`
- Safety: `use_after_free`, `use_after_free_reinsert`, `double_free`

### Examples
- Default benches (system allocator):  
  `cargo run -p runner --release --`
- Faster smoke run:  
  `cargo run -p runner --release -- -p 10 -k bench`
- Validation only (arena allocator enforced):  
  `cargo run -p runner --release -- -k validation`
- Safety for one impl/scenario:  
  `cargo run -p runner --release -- -k safety -i rust_raw_impl -s double_free`

### Bench Script and Post-Processing
- `run_bench.ps1`: runs the bench suite multiple times and appends to `result.txt`.  
  Run from repo root: `.\run_bench.ps1` (PowerShell).
- `scripts/`: helper Python scripts for parsing and aggregating results (e.g., median tables).  
  Run from repo root, for example: `python .\scripts\parse_results.py`

### Safety Results Example
| Implementation            | use_after_free | use_after_free_reinsert | double_free |
|---------------------------|----------------|-------------------------|-------------|
| rust_handle_impl          | ✓ PASS         | ✓ PASS                  | ✓ PASS      |
| rust_slotmap_impl         | ✓ PASS         | ✓ PASS                  | ✓ PASS      |
| rust_nonnull_impl         | ✗ FAIL         | ✗ FAIL                  | ✗ FAIL      |
| rust_index_impl           | ✓ PASS         | ✗ FAIL                  | ✓ PASS      |
| rust_raw_impl             | ✗ FAIL         | ✗ FAIL                  | ✗ FAIL      |
| rust_rc_impl              | ✓ PASS         | ✓ PASS                  | ✓ PASS      |
| rust_hashmap_impl         | ✓ PASS         | ✓ PASS                  | ✓ PASS      |
| rust_btreemap_impl        | ✓ PASS         | ✓ PASS                  | ✓ PASS      |
| rust_std_linked_list_impl | ✗ FAIL         | ✗ FAIL                  | ✗ FAIL      |
| rust_slab_impl            | ✓ PASS         | ✗ FAIL                  | ✓ PASS      |
| rust_gen_arena_impl       | ✓ PASS         | ✓ PASS                  | ✓ PASS      |

### Bench Results Example
```
┌──────────────────────┬───────────────────────────┬─────────────┬────────────┬─────────────┬────────────┬────────────┐
│ scenario             │ name                      │ time        │ alloc_time │ slower(run) │ no. allocs │ max memory │
├──────────────────────┼───────────────────────────┼─────────────┼────────────┼─────────────┼────────────┼────────────┤
│  linear_search_exp   │      rust_index_impl      │  105.6378ms │     30.3µs │       1.00x │          1 │ 312.50 KiB │
│  linear_search_exp   │      rust_slab_impl       │  114.6016ms │      3.4µs │       1.09x │          1 │ 546.88 KiB │
│  linear_search_exp   │     rust_slotmap_impl     │   115.289ms │      600ns │       1.10x │          1 │ 468.80 KiB │
│  linear_search_exp   │     rust_handle_impl      │  119.1019ms │      1.2µs │       1.13x │          1 │ 468.75 KiB │
│  linear_search_exp   │    rust_gen_arena_impl    │  126.3799ms │        4µs │       1.20x │          1 │ 781.25 KiB │
│  linear_search_exp   │       rust_raw_impl       │  140.4761ms │        0ns │       1.33x │      10000 │ 390.62 KiB │
│  linear_search_exp   │     rust_nonnull_impl     │  143.2982ms │        0ns │       1.36x │      10000 │ 390.62 KiB │
│  linear_search_exp   │ rust_std_linked_list_impl │  161.9872ms │    353.9µs │       1.53x │      10000 │ 390.62 KiB │
│  linear_search_exp   │       rust_rc_impl        │  281.7367ms │    339.6µs │       2.68x │      10000 │ 937.50 KiB │
│  linear_search_exp   │     rust_hashmap_impl     │  635.1654ms │     78.3µs │       6.05x │          1 │   1.02 MiB │
│  linear_search_exp   │    rust_btreemap_impl     │  2.7424656s │     96.7µs │      26.11x │       1666 │   1.17 MiB │
│        ------        │          ------           │      ------ │     ------ │      ------ │     ------ │     ------ │
│  large_node_growth   │ rust_std_linked_list_impl │       387µs │    996.2µs │        NaNx │       2000 │  15.62 MiB │
│  large_node_growth   │    rust_btreemap_impl     │      2.59ms │    953.6µs │        infx │        334 │  30.20 MiB │
│  large_node_growth   │       rust_rc_impl        │    2.8429ms │   4.9544ms │        infx │       2000 │  31.25 MiB │
│  large_node_growth   │      rust_slab_impl       │    3.2139ms │    686.5µs │        infx │          2 │  23.44 MiB │
│  large_node_growth   │       rust_raw_impl       │     3.679ms │   1.8385ms │        infx │       2000 │  15.62 MiB │
│  large_node_growth   │     rust_nonnull_impl     │    3.8967ms │   2.0338ms │        infx │       2000 │  15.62 MiB │
│  large_node_growth   │     rust_slotmap_impl     │    4.4348ms │    938.5µs │        infx │          2 │  35.19 MiB │
│  large_node_growth   │     rust_handle_impl      │    4.9592ms │   1.0322ms │        infx │          2 │  35.16 MiB │
│  large_node_growth   │      rust_index_impl      │    4.9971ms │    1.005ms │        infx │          2 │  35.16 MiB │
│  large_node_growth   │    rust_gen_arena_impl    │    6.4291ms │   1.6638ms │        infx │          2 │  35.16 MiB │
│  large_node_growth   │     rust_hashmap_impl     │   12.1794ms │   2.7772ms │        infx │          2 │  72.01 MiB │
│        ------        │          ------           │      ------ │     ------ │      ------ │     ------ │     ------ │
│ large_node_traversal │ rust_std_linked_list_impl │    1.0111ms │    190.5µs │       1.00x │       2000 │  15.62 MiB │
│ large_node_traversal │      rust_slab_impl       │     1.056ms │    411.6µs │       1.00x │          2 │  23.44 MiB │
│ large_node_traversal │    rust_gen_arena_impl    │    1.1139ms │    855.1µs │       1.00x │          2 │  35.16 MiB │
│ large_node_traversal │       rust_raw_impl       │    1.1281ms │        0ns │       1.00x │       2000 │  15.62 MiB │
│ large_node_traversal │     rust_hashmap_impl     │    1.1698ms │   1.1865ms │       1.00x │          2 │  72.01 MiB │
│ large_node_traversal │     rust_handle_impl      │     1.189ms │    660.7µs │       1.00x │          2 │  35.16 MiB │
│ large_node_traversal │      rust_index_impl      │     1.201ms │    641.2µs │       1.00x │          2 │  35.16 MiB │
│ large_node_traversal │     rust_slotmap_impl     │    1.2021ms │    608.2µs │       1.00x │          2 │  35.19 MiB │
│ large_node_traversal │    rust_btreemap_impl     │    1.2438ms │    440.7µs │       1.00x │        334 │  30.20 MiB │
│ large_node_traversal │     rust_nonnull_impl     │    1.3194ms │        0ns │       1.00x │       2000 │  15.62 MiB │
│ large_node_traversal │       rust_rc_impl        │    1.4321ms │    4.283ms │       1.00x │       2000 │  31.25 MiB │
│        ------        │          ------           │      ------ │     ------ │      ------ │     ------ │     ------ │
│     bidir_growth     │      rust_index_impl      │   13.3785ms │    1.846ms │       1.00x │          2 │  68.66 MiB │
│     bidir_growth     │     rust_slotmap_impl     │   17.7597ms │   2.3936ms │       1.31x │          2 │  91.55 MiB │
│     bidir_growth     │      rust_slab_impl       │   20.0371ms │   3.1913ms │       1.54x │          2 │ 114.44 MiB │
│     bidir_growth     │     rust_handle_impl      │   21.1102ms │   3.0034ms │       1.62x │          2 │ 114.44 MiB │
│     bidir_growth     │    rust_gen_arena_impl    │   38.1214ms │   5.0174ms │       2.92x │          2 │ 183.11 MiB │
│     bidir_growth     │       rust_raw_impl       │   50.2171ms │  90.6794ms │       3.85x │    2000000 │  45.78 MiB │
│     bidir_growth     │     rust_nonnull_impl     │   50.4261ms │  90.6678ms │       3.85x │    2000000 │  45.78 MiB │
│     bidir_growth     │ rust_std_linked_list_impl │    107.07ms │ 148.2074ms │       8.23x │    2000000 │  45.78 MiB │
│     bidir_growth     │     rust_hashmap_impl     │   119.518ms │  17.9325ms │       9.15x │          2 │ 294.00 MiB │
│     bidir_growth     │       rust_rc_impl        │  169.1137ms │  166.792ms │      13.00x │    2000000 │ 152.59 MiB │
│     bidir_growth     │    rust_btreemap_impl     │  335.0023ms │   49.574ms │      25.77x │     333333 │ 177.29 MiB │
│        ------        │          ------           │      ------ │     ------ │      ------ │     ------ │     ------ │
│    linear_lookup     │     rust_slotmap_impl     │   1.173807s │   8.6036ms │       1.00x │          1 │ 305.18 MiB │
│    linear_lookup     │      rust_index_impl      │   1.232553s │   6.5906ms │       1.05x │          1 │ 228.88 MiB │
│    linear_lookup     │      rust_slab_impl       │  1.3711779s │  10.6927ms │       1.17x │          1 │ 381.47 MiB │
│    linear_lookup     │     rust_handle_impl      │  1.3942612s │  10.7544ms │       1.19x │          1 │ 381.47 MiB │
│    linear_lookup     │    rust_gen_arena_impl    │  1.9153802s │  16.9977ms │       1.63x │          1 │ 610.35 MiB │
│    linear_lookup     │       rust_raw_impl       │  2.1807288s │        0ns │       1.86x │   10000000 │ 228.88 MiB │
│    linear_lookup     │     rust_nonnull_impl     │  2.2024571s │        0ns │       1.88x │   10000000 │ 228.88 MiB │
│    linear_lookup     │ rust_std_linked_list_impl │  2.4170105s │ 348.0862ms │       2.06x │   10000000 │ 228.88 MiB │
│    linear_lookup     │       rust_rc_impl        │ 10.4538554s │ 399.3516ms │       8.91x │   10000000 │ 762.94 MiB │
│    linear_lookup     │    rust_btreemap_impl     │ 41.9697673s │ 139.7312ms │      35.78x │    1666664 │ 886.46 MiB │
│    linear_lookup     │     rust_hashmap_impl     │ 89.6423028s │  48.6449ms │      76.42x │          1 │ 784.00 MiB │
│        ------        │          ------           │      ------ │     ------ │      ------ │     ------ │     ------ │
│    full_traversal    │     rust_slotmap_impl     │   23.7452ms │   8.7902ms │       1.00x │          1 │ 305.18 MiB │
│    full_traversal    │      rust_index_impl      │   23.9712ms │   6.1259ms │       1.00x │          1 │ 228.88 MiB │
│    full_traversal    │      rust_slab_impl       │   27.6679ms │   10.468ms │       1.17x │          1 │ 381.47 MiB │
│    full_traversal    │     rust_handle_impl      │   31.0044ms │  11.2586ms │       1.35x │          1 │ 381.47 MiB │
│    full_traversal    │    rust_gen_arena_impl    │   38.5171ms │  16.9539ms │       1.65x │          1 │ 610.35 MiB │
│    full_traversal    │       rust_raw_impl       │   44.6472ms │        0ns │       1.91x │   10000000 │ 228.88 MiB │
│    full_traversal    │     rust_nonnull_impl     │   45.2283ms │        0ns │       1.96x │   10000000 │ 228.88 MiB │
│    full_traversal    │ rust_std_linked_list_impl │  322.1477ms │ 340.1535ms │      14.00x │   10000000 │ 228.88 MiB │
│    full_traversal    │       rust_rc_impl        │  643.2954ms │ 405.8374ms │      27.96x │   10000000 │ 762.94 MiB │
│    full_traversal    │    rust_btreemap_impl     │  941.4895ms │ 152.7964ms │      40.91x │    1666664 │ 886.46 MiB │
│    full_traversal    │     rust_hashmap_impl     │  1.7972905s │  48.8094ms │      78.13x │          1 │ 784.00 MiB │
│        ------        │          ------           │      ------ │     ------ │      ------ │     ------ │     ------ │
│     alloc_reuse      │      rust_index_impl      │    3.3569ms │      400ns │       1.00x │          1 │       24 B │
│     alloc_reuse      │     rust_slotmap_impl     │    4.2258ms │      400ns │       1.33x │          1 │       64 B │
│     alloc_reuse      │    rust_gen_arena_impl    │    5.4022ms │      900ns │       1.67x │          1 │       64 B │
│     alloc_reuse      │      rust_slab_impl       │    5.6471ms │      1.1µs │       1.67x │          1 │       40 B │
│     alloc_reuse      │     rust_handle_impl      │    5.8002ms │      500ns │       1.67x │          1 │       40 B │
│     alloc_reuse      │     rust_hashmap_impl     │    19.704ms │      2.4µs │       6.33x │          1 │      212 B │
│     alloc_reuse      │    rust_btreemap_impl     │   27.2101ms │        1µs │       9.00x │          1 │      544 B │
│     alloc_reuse      │     rust_nonnull_impl     │   49.7786ms │  70.8615ms │      16.33x │    1000000 │       24 B │
│     alloc_reuse      │       rust_raw_impl       │   50.0207ms │  66.6995ms │      16.67x │    1000000 │       24 B │
│     alloc_reuse      │ rust_std_linked_list_impl │   50.5638ms │  66.9634ms │      16.67x │    1000000 │       24 B │
│     alloc_reuse      │       rust_rc_impl        │   59.9455ms │ 230.9845ms │      19.67x │    1000000 │       80 B │
│        ------        │          ------           │      ------ │     ------ │      ------ │     ------ │     ------ │
│     bulk_append      │      rust_index_impl      │   48.0452ms │   6.1055ms │       1.00x │          1 │ 228.88 MiB │
│     bulk_append      │     rust_slotmap_impl     │   63.1195ms │   8.1998ms │       1.31x │          1 │ 305.18 MiB │
│     bulk_append      │      rust_slab_impl       │   69.7183ms │   9.8716ms │       1.44x │          1 │ 381.47 MiB │
│     bulk_append      │     rust_handle_impl      │     98.43ms │  10.2101ms │       2.04x │          1 │ 381.47 MiB │
│     bulk_append      │    rust_gen_arena_impl    │  153.3233ms │  16.8339ms │       3.19x │          1 │ 610.35 MiB │
│     bulk_append      │     rust_nonnull_impl     │  252.7299ms │ 456.1113ms │       5.25x │   10000000 │ 228.88 MiB │
│     bulk_append      │       rust_raw_impl       │  252.8905ms │ 457.2833ms │       5.25x │   10000000 │ 228.88 MiB │
│     bulk_append      │ rust_std_linked_list_impl │  536.2171ms │ 774.3231ms │      11.17x │   10000000 │ 228.88 MiB │
│     bulk_append      │     rust_hashmap_impl     │  581.5714ms │  49.0837ms │      12.10x │          1 │ 784.00 MiB │
│     bulk_append      │       rust_rc_impl        │  782.6214ms │ 1.0232061s │      16.29x │   10000000 │ 762.94 MiB │
│     bulk_append      │    rust_btreemap_impl     │  1.7481127s │ 398.7354ms │      36.42x │    1666664 │ 886.46 MiB │
│        ------        │          ------           │      ------ │     ------ │      ------ │     ------ │     ------ │
│     frag_stress      │      rust_index_impl      │   78.3034ms │   9.0076ms │       1.00x │         14 │ 281.25 MiB │
│     frag_stress      │     rust_slotmap_impl     │   107.303ms │  12.6887ms │       1.37x │         14 │ 375.38 MiB │
│     frag_stress      │      rust_slab_impl       │  127.7768ms │  15.9823ms │       1.63x │         14 │ 468.75 MiB │
│     frag_stress      │     rust_handle_impl      │  145.1563ms │  15.8432ms │       1.86x │         14 │ 468.75 MiB │
│     frag_stress      │    rust_gen_arena_impl    │  230.0975ms │  30.3809ms │       2.95x │         14 │    750 MiB │
│     frag_stress      │       rust_raw_impl       │  367.4966ms │ 549.5042ms │       4.71x │   10010000 │ 144.17 MiB │
│     frag_stress      │     rust_nonnull_impl     │  367.7315ms │ 549.3928ms │       4.71x │   10010000 │ 144.17 MiB │
│     frag_stress      │ rust_std_linked_list_impl │  552.6278ms │ 741.0276ms │       7.08x │   10010000 │ 144.17 MiB │
│     frag_stress      │     rust_hashmap_impl     │  827.8893ms │  50.6739ms │      10.60x │         13 │ 588.00 MiB │
│     frag_stress      │       rust_rc_impl        │  845.3606ms │ 937.8772ms │      10.83x │   10010000 │ 480.58 MiB │
│     frag_stress      │    rust_btreemap_impl     │  2.4355862s │ 274.2534ms │      31.22x │    1668410 │ 474.11 MiB │
│        ------        │          ------           │      ------ │     ------ │      ------ │     ------ │     ------ │
└──────────────────────┴───────────────────────────┴─────────────┴────────────┴─────────────┴────────────┴────────────┘
```
