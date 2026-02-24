import re
import statistics
from collections import defaultdict

INPUT_FILE = "result.txt"
OUTPUT_DIR = "."

measurements = {
    "ms": [("s", 1000), ("us", 0.001), ("ns", 0.000001)],
    "KiB": [("MiB", 1024), ("B", 0.0009765625)]
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

scenario_name_weight = {
    "linear_search_exp": 0.5,
    "large_node_growth": 1,
    "large_node_traversal": 0.5,
    "bidir_growth": 0.75,
    "linear_lookup": 0.5,
    "full_traversal": 0.5,
    "alloc_reuse": 0.25,
    "bulk_append": 1,
    "frag_stress": 0.25,
}

speed_scenarios = [
    "linear_search_exp",
    "large_node_growth",
    "large_node_traversal",
    "bidir_growth",
    "linear_lookup",
    "full_traversal",
    "frag_stress",
]

memory_scenarios = [
    "large_node_growth",
    "bidir_growth",
    "alloc_reuse",
    "bulk_append",
    "frag_stress",
]

scenario_columns = {
    "linear_search_exp": "Linear Search, for Time Performance",
    "large_node_growth": "Large-Node Bidirectional Growth",
    "large_node_traversal": "Large-Node Traversal",
    "bidir_growth": "Bidirectional Growth",
    "linear_lookup": "Linear Lookup",
    "full_traversal": "Full Traversal",
    "alloc_reuse": "Allocation Reuse Stress",
    "bulk_append": "Bulk Append",
    "frag_stress": "Fragmentation Stress",
}

complete_data = defaultdict(lambda: defaultdict(list))


def convert_measurement(number, unit):
    for m in measurements.keys():
        for (old_meas, multiplier) in measurements[m]:
            if unit == old_meas:
                return number * multiplier
    return number


def normalize_row(row):
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


def add_median_percentage_column(criteria_index, scenario):
    values = [float(row[criteria_index]) for row in complete_data[scenario].values()]
    median = statistics.median(values)
    median_values = []
    for impl, row in complete_data[scenario].items():
        value = float(row[criteria_index])
        percentage = (value / median) if median > 0 else 0
        median_values.append([impl, f"{percentage:.2f}"])
    return median_values


def get_median_table(criteria_index, scenarios):
    median_values = {}
    for scenario_key in scenarios:
        if scenario_key in complete_data:
            median_values[scenario_key] = add_median_percentage_column(criteria_index, scenario_key)
    return median_values


def rank_impls(median_values, weights=None):
    scores = defaultdict(float)
    if weights == "uniform":
        weight_dict = {scenario: 1 for scenario in median_values.keys()}
    elif weights is None:
        weight_dict = scenario_name_weight
    else:
        weight_dict = weights

    for scenario, results in median_values.items():
        for impl, value in results:
            scores[impl] += weight_dict[scenario] * float(value)
    ranked = sorted(scores.items(), key=lambda x: x[1])
    return ranked


def generate_table(ranking, median_values, scenarios, caption, label):
    lines = []
    lines.append("\\begin{table} ")
    lines.append("\\centering")
    lines.append("\\begin{tabular}{|l|R{1cm}|R{1.7cm}|R{1.7cm}|R{1.5cm}|R{1cm}|R{2cm}|}")
    lines.append("\\hline")
    header = "\\#&Score"
    for scenario_key in scenarios:
        header += f"&{scenario_columns[scenario_key]}"
    lines.append(header + "\\\\ ")
    lines.append("\\hline")

    for impl, score in ranking:
        line = f"{impl_name_map.get(impl, impl)}&{score:.2f}"
        for scenario_key in scenarios:
            val = ""
            for impl_res in median_values.get(scenario_key, []):
                if impl == impl_res[0]:
                    val = impl_res[1]
                    break
            line += f"&{val}"
        lines.append(line + "\\\\ ")
        lines.append("\\hline")

    lines.append("\\end{tabular}")
    lines.append(f"\\caption{{{caption}}}")
    lines.append(f"\\label{{{label}}}")
    lines.append("\\end{table}")
    return "\n".join(lines)


def resolve_paths(argv):
    input_path = INPUT_FILE
    output_dir = OUTPUT_DIR
    if len(argv) >= 2:
        input_path = argv[1]
    if len(argv) >= 3:
        output_dir = argv[2]
    return input_path, output_dir


if __name__ == "__main__":
    import sys

    input_path, output_dir = resolve_paths(sys.argv)
    parse_file(input_path)
    compute_medians()
    # Speed tables (time criteria index 0)
    speed_median_values = get_median_table(0, speed_scenarios)
    rank_speed_weighted = rank_impls(speed_median_values, weights=None)
    rank_speed_uniform = rank_impls(speed_median_values, weights="uniform")

    speed_weighted_table = generate_table(
        rank_speed_weighted,
        speed_median_values,
        speed_scenarios,
        "Median Ranking - Speed (Weighted)",
        "table:median_ranking_speed_weighted",
    )
    speed_uniform_table = generate_table(
        rank_speed_uniform,
        speed_median_values,
        speed_scenarios,
        "Median Ranking - Speed (Uniform Weights)",
        "table:median_ranking_speed_uniform",
    )

    memory_median_values = get_median_table(3, memory_scenarios)
    rank_memory_weighted = rank_impls(memory_median_values, weights=None)
    rank_memory_uniform = rank_impls(memory_median_values, weights="uniform")

    memory_weighted_table = generate_table(
        rank_memory_weighted,
        memory_median_values,
        memory_scenarios,
        "Median Ranking - Memory (Weighted)",
        "table:median_ranking_memory_weighted",
    )
    memory_uniform_table = generate_table(
        rank_memory_uniform,
        memory_median_values,
        memory_scenarios,
        "Median Ranking - Memory (Uniform Weights)",
        "table:median_ranking_memory_uniform",
    )

    outputs = [
        ("median_speed_weighted.txt", speed_weighted_table),
        ("median_speed_uniform.txt", speed_uniform_table),
        ("median_memory_weighted.txt", memory_weighted_table),
        ("median_memory_uniform.txt", memory_uniform_table),
    ]

    for filename, content in outputs:
        path = f"{output_dir}\\{filename}" if output_dir != "." else filename
        with open(path, "w", encoding="utf-8") as f:
            f.write(content)
        print(f"[OK] Table saved to: {path}")