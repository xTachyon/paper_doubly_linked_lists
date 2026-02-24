import re
import statistics
import sys
from collections import defaultdict

INPUT_FILE = "result.txt"
OUTPUT_FILE = "results_parsed.txt"

measurements = {
    "ms": [("s", 1000), ("us", 0.001), ("ns", 0.000001)],
    "KiB": [("MiB", 1024), ("B", 0.0009765625)]
}

scenario_order = [
    "linear_search_exp",
    "large_node_growth",
    "large_node_traversal",
    "bidir_growth",
    "linear_lookup",
    "full_traversal",
    "alloc_reuse",
    "bulk_append",
    "frag_stress",
]

scenario_description_map = {
    "linear_search_exp": "Linear Search, for Time Performance",
    "large_node_growth": "Large-Node Bidirectional Growth, for Time Performance and Memory Footprint",
    "large_node_traversal": "Large-Node Traversal, for Time Performance",
    "bidir_growth": "Bidirectional Growth, for Time Performance and Memory Footprint",
    "linear_lookup": "Linear Lookup, for Time Performance",
    "full_traversal": "Full Traversal, for Time Performance",
    "alloc_reuse": "Allocation Reuse, for Memory Footprint",
    "bulk_append": "Bulk Append, for Time Performance and Memory Footprint",
    "frag_stress": "Fragmentation Stress, for Time Performance and Memory Footprint",
}

impl_name_map = {
    "rust_std_linked_list_impl": "Std",
    "rust_raw_impl": "Raw",
    "rust_nonnull_impl": "NonNull",
    "rust_index_impl": "Index",
    "rust_slab_impl": "Slab",
    "rust_handle_impl": "Handle",
    "rust_gen_arena_impl": "Arena",
    "rust_rc_impl": "Rc",
    "rust_btreemap_impl": "BTree",
    "rust_hashmap_impl": "Hash",
    "rust_slotmap_impl": "Slot",
}

complete_data = defaultdict(lambda: defaultdict(list))


def convert_measurement(number, unit):
    for m in measurements.keys():
        for (old_meas, multiplier) in measurements[m]:
            if unit == old_meas:
                return number * multiplier
    return number


def normalize_row(row):
    """Normalize a row: time/alloc to ms, max mem to KiB."""
    normalized = list(row)
    for i, v in enumerate(row):
        value = v.replace("µ", "u")
        match = re.match(r"([0-9.]+)\s*([a-zA-Z]+)", value)
        if not match:
            continue
        number = float(match.group(1))
        unit = match.group(2)
        if unit not in ["ms", "KiB", "x"]:
            new_value = convert_measurement(number, unit)
            normalized[i] = str(round(new_value, 2))
        else:
            if unit != "x":
                normalized[i] = str(round(number, 2))
            else:
                normalized[i] = str(number)
    return normalized


def parse_file(path):
    data = {}
    with open(path, "r", encoding="utf-8") as f:
        for line in f:
            if not line:
                continue
            if line[0] == "┌":
                data.clear()
            elif line[0] == "└":
                process_one_table(data)
            elif line.strip() == "":
                continue
            else:
                words = [w.strip() for w in line.split("│") if w.strip() != ""]
                if len(words) <= 1 or words[1] == "------" or words[0] == "scenario":
                    continue
                if words[0] in data.keys():
                    data[words[0]].append(words[1:])
                else:
                    data[words[0]] = [words[1:]]


def process_one_table(data):
    for scenario in data.keys():
        scenario_data = data.get(scenario)
        for line in scenario_data:
            rust_impl = line[0]
            values = normalize_row(line[1:])
            complete_data[scenario][rust_impl].append(values)


def compute_medians():
    for scenario in complete_data.keys():
        for impl in complete_data[scenario]:
            time_vals = [float(x[0]) for x in complete_data[scenario][impl]]
            alloc_vals = [float(x[1]) for x in complete_data[scenario][impl]]
            no_allocs = [float(x[3]) for x in complete_data[scenario][impl]]
            mem_vals = [float(x[4]) for x in complete_data[scenario][impl]]
            complete_data[scenario][impl] = [
                statistics.median(time_vals),
                statistics.median(alloc_vals),
                statistics.median(no_allocs),
                statistics.median(mem_vals),
            ]


def format_no_allocs(value):
    if abs(value - round(value)) < 0.01:
        return str(int(round(value)))
    return f"{value:.2f}"


def generate_latex_table(data):
    lines = []
    lines.append("\\begin{table}[htbp]")
    lines.append("\\centering")
    lines.append("\\caption{Complete Results (Part 1)}")
    lines.append("\\label{tab:all_results1}")
    lines.append("\\setlength{\\tabcolsep}{5pt}")
    lines.append("\\renewcommand{\\arraystretch}{1.05}")
    lines.append("\\begin{tabular}{p{1.8cm} l")
    lines.append("                S[table-format=4.2]")
    lines.append("                S[table-format=3.2]")
    lines.append("                S[table-format=8.0]")
    lines.append("                S[table-format=6.0]}")
    lines.append("\\toprule")
    lines.append("\\textbf{Scenario} & \\textbf{Method} &")
    lines.append("\\textbf{ExecT (ms)} & \\textbf{AllocT (ms)} &")
    lines.append("\\textbf{NoAlloc} & \\textbf{MaxMem (KiB)} \\\\")
    lines.append("\\midrule")

    for idx, scenario in enumerate(scenario_order):
        if scenario not in data:
            continue
        impls = list(data[scenario].keys())
        scenario_label = scenario_description_map.get(scenario, scenario)
        row_count = len(impls)
        first_row = True
        for impl in impls:
            time_val, alloc_val, no_allocs, mem_val = data[scenario][impl]
            impl_display = impl_name_map.get(impl, impl)
            if first_row:
                lines.append(
                    f"\\multirow{{{row_count}}}{{=}}{{{scenario_label}}}\n"
                    f" & {impl_display} & {time_val:.2f} & {alloc_val:.2f} & "
                    f"{format_no_allocs(no_allocs)} & {mem_val:.2f} \\\\"
                )
                first_row = False
            else:
                lines.append(
                    f" & {impl_display} & {time_val:.2f} & {alloc_val:.2f} & "
                    f"{format_no_allocs(no_allocs)} & {mem_val:.2f} \\\\"
                )
        if idx < len(scenario_order) - 1:
            lines.append("\\midrule")
    lines.append("\\bottomrule")
    lines.append("\\end{tabular}")
    lines.append("\\end{table}")
    return "\n".join(lines)


def resolve_paths(argv):
    input_path = INPUT_FILE
    output_path = OUTPUT_FILE
    if len(argv) >= 2:
        input_path = argv[1]
    if len(argv) >= 3:
        output_path = argv[2]
    return input_path, output_path


if __name__ == "__main__":
    input_path, output_path = resolve_paths(sys.argv)
    parse_file(input_path)
    compute_medians()
    latex_table = generate_latex_table(complete_data)
    with open(output_path, "w", encoding="utf-8") as f:
        f.write(latex_table)
    print(f"[OK] Table saved to: {output_path}")