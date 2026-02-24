import os
import re
import statistics
import sys
from collections import defaultdict

INPUT_FILE = os.path.join("..", "results_parsed.txt")
OUTPUT_FILE = "overall_scores_table.tex"

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

impl_name_reverse_map = {v: k for k, v in impl_name_map.items()}

scenario_name_map = {
    "find_string": "Find String",
    "push_pages": "Push Pages",
    "iterate_pages": "Iterate Pages",
    "add_front_back": "Add Front Back",
    "search_middle": "Search Inner",
    "sum": "Sum",
    "push_delete_one": "Push Delete One",
    "push": "Push",
    "fragmentation": "Fragmentation",
}

scenario_name_reverse_map = {v: k for k, v in scenario_name_map.items()}

criteria_map = {
    "time": 1,
    "alloc_time": 2,
    "no_allocs": 3,
    "max_mem": 4,
}

scenario_name_weight = {
    "find_string": 0.5,
    "push_pages": 1,
    "iterate_pages": 0.5,
    "add_front_back": 0.75,
    "search_middle": 0.5,
    "sum": 0.5,
    "push_delete_one": 0.25,
    "push": 1,
    "fragmentation": 0.25,
}

speed_scenarios = [
    "find_string",
    "push_pages",
    "iterate_pages",
    "add_front_back",
    "search_middle",
    "sum",
    "fragmentation",
]

memory_scenarios = [
    "push_pages",
    "add_front_back",
    "push_delete_one",
    "push",
    "fragmentation",
]

alpha_values = [0.1, 0.25, 0.5, 0.75, 0.9]

output_order = [
    "rust_index_impl",
    "rust_nonnull_impl",
    "rust_raw_impl",
    "rust_slab_impl",
    "rust_slotmap_impl",
    "rust_handle_impl",
    "rust_gen_arena_impl",
    "rust_std_linked_list_impl",
    "rust_rc_impl",
    "rust_btreemap_impl",
    "rust_hashmap_impl",
]


def parse_latex_table(filepath):
    data = {}
    with open(filepath, encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("\\begin") or line.startswith("\\end") or \
               line.startswith("\\hline") or line.startswith("\\centering") or \
               line.startswith("\\Scenario") or "&Method&" in line or \
               line.startswith("\\caption"):
                continue
            if "&" in line:
                line = line.replace("\\\\", "").strip()
                parts = [p.strip() for p in line.split("&")]
                if len(parts) >= 5:
                    scenario_display = parts[0]
                    method_display = parts[1]
                    values = parts[2:]

                    if scenario_display in scenario_name_reverse_map:
                        scenario_key = scenario_name_reverse_map[scenario_display]
                    else:
                        continue
                    if method_display in impl_name_reverse_map:
                        method_key = impl_name_reverse_map[method_display]
                    else:
                        continue

                    row = [method_key] + values
                    if scenario_key in data:
                        data[scenario_key].append(row)
                    else:
                        data[scenario_key] = [row]
    return data


def add_median_percentage_column(criteria, scenario, data):
    index = criteria_map.get(criteria)
    values = [float(x[index]) for x in data[scenario]]
    median = statistics.median(values)
    median_table = []
    for row in data[scenario]:
        value = float(row[index])
        percentage = (value / median)
        new_row = row + [f"{percentage:.2f}"]
        median_table.append(new_row)
    data[scenario] = median_table


def get_median_tables(data, scenarios, criteria):
    for s in scenarios:
        if s in data:
            add_median_percentage_column(criteria, s, data)

    median_values = {}
    for s in scenarios:
        if s not in data:
            continue
        median_values[s] = []
        for row in data[s]:
            median_values[s].append([row[0], row[len(row) - 1]])
    return median_values


def rank_impls(data, weights=None):
    scores = defaultdict(float)
    if weights == "uniform":
        weight_dict = {scenario: 1 for scenario in data.keys()}
    elif weights is None:
        weight_dict = scenario_name_weight
    else:
        weight_dict = weights

    for scenario, results in data.items():
        for impl, value in results:
            scores[impl] += weight_dict[scenario] * float(value)
    ranked = sorted(scores.items(), key=lambda x: x[1])
    return ranked


def calculate_overall_scores(time_scores, memory_scores):
    results = {}
    all_impls = set(time_scores.keys()) | set(memory_scores.keys())
    for impl in all_impls:
        t_score = time_scores.get(impl, 0)
        m_score = memory_scores.get(impl, 0)
        results[impl] = {}
        for alpha in alpha_values:
            results[impl][alpha] = alpha * t_score + (1 - alpha) * m_score
    return results


def generate_table(overall_scores):
    lines = []
    lines.append("\\begin{table}[htbp]")
    lines.append("\\centering")
    lines.append("\\caption{Overall Scores for Different $\\alpha$ Values}")
    lines.append("\\label{tab:overall_scores}")
    lines.append("\\begin{tabular}{lccccc}")
    lines.append("\\toprule")
    lines.append("\\textbf{Implementation} & $\\alpha=0.10$ & $\\alpha=0.25$ & $\\alpha=0.50$ & $\\alpha=0.75$ & $\\alpha=0.90$ \\\\")
    lines.append("\\midrule")

    for impl in output_order:
        if impl not in overall_scores:
            continue
        display = impl_name_map.get(impl, impl)
        values = [overall_scores[impl][alpha] for alpha in alpha_values]
        line = f"{display}   & {values[0]:.2f} & {values[1]:.2f} & {values[2]:.2f} & {values[3]:.2f} & {values[4]:.2f} \\\\"
        lines.append(line)

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
    data = parse_latex_table(input_path)

    time_table = get_median_tables(data, speed_scenarios, "time")
    rank_time = rank_impls(time_table, weights="uniform")
    time_scores = {impl: score for impl, score in rank_time}

    memory_table = get_median_tables(data, memory_scenarios, "max_mem")
    rank_memory = rank_impls(memory_table, weights="uniform")
    memory_scores = {impl: score for impl, score in rank_memory}

    overall_scores = calculate_overall_scores(time_scores, memory_scores)
    latex_table = generate_table(overall_scores)

    with open(output_path, "w", encoding="utf-8") as f:
        f.write(latex_table)
    print(f"[OK] Table saved to: {output_path}")