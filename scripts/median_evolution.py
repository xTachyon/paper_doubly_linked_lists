import os
import re
import sys
import statistics
import matplotlib.pyplot as plt

INPUT_FILE = "result.txt"
OUTPUT_DIR = "output"

measurements = {
    "ms": [("s", 1000), ("us", 0.001), ("ns", 0.000001)],
    "KiB": [("MiB", 1024), ("B", 0.0009765625)],
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

memory_scenarios = []


def convert_measurement(number, unit):
    for m in measurements.keys():
        for (old_meas, multiplier) in measurements[m]:
            if unit == old_meas:
                return number * multiplier
    return number


def normalize_value(value):
    value = value.replace("µ", "u")
    match = re.match(r"([0-9.]+)\s*([a-zA-Z]+)", value)
    if not match:
        return value
    number = float(match.group(1))
    unit = match.group(2)
    if unit not in ["ms", "KiB", "x"]:
        new_value = convert_measurement(number, unit)
        return str(round(new_value, 2))
    if unit != "x":
        return str(round(number, 2))
    return str(number)


def _read_text(path):
    with open(path, "rb") as f:
        raw = f.read()
    for enc in ("utf-8", "utf-16", "utf-16-le"):
        try:
            text = raw.decode(enc)
            return text.lstrip("\ufeff")
        except UnicodeError:
            continue
    # Fallback: decode with utf-8 and ignore errors
    return raw.decode("utf-8", errors="ignore").lstrip("\ufeff")


def parse_file(path):
    """Parse result.txt and return all data organized by scenario and implementation."""
    all_data = {}
    data = {}
    index = 0

    text = _read_text(path)
    for line in text.splitlines():
            if not line:
                continue
            if line[0] == "┌":
                data.clear()
                index += 1
            elif line[0] == "└":
                for scenario in data.keys():
                    if scenario not in all_data:
                        all_data[scenario] = {}
                    for row in data[scenario]:
                        impl_name = row[0]
                        if impl_name == "name" or impl_name == "------":
                            continue
                        if impl_name not in all_data[scenario]:
                            all_data[scenario][impl_name] = []

                        if len(row) > 1 and scenario in speed_scenarios:
                            time_value = normalize_value(row[1])
                            try:
                                all_data[scenario][impl_name].append((index, float(time_value)))
                            except ValueError:
                                continue
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

    return all_data


def create_medians_data(index_value_tuples):
    median_values = []
    for i, item in enumerate(index_value_tuples):
        if i < 2:
            median_values.append(item)
        else:
            value = statistics.median([x[1] for x in index_value_tuples[: i + 1]])
            median_values.append((item[0], value))
    return median_values


def create_graphic(scenario, implementation, index_value_tuples, output_dir):
    if not index_value_tuples:
        return

    median_values = create_medians_data(index_value_tuples)
    xpoints = [x[0] for x in median_values]
    ypoints = [x[1] for x in median_values]

    plt.figure(figsize=(10, 6))
    plt.ylabel("Execution Time (ms)")
    plt.xlabel("Iteration")
    plt.title(f"{scenario} - {implementation}")
    plt.plot(xpoints, ypoints, marker="o", markersize=3)
    plt.grid(True, alpha=0.3)

    filename = f"{scenario}_{implementation}.png"
    filepath = os.path.join(output_dir, filename)
    plt.savefig(filepath, dpi=200, bbox_inches="tight")
    plt.close()


def generate_all_graphics(all_data, output_dir):
    os.makedirs(output_dir, exist_ok=True)
    for scenario in all_data:
        for impl_name, data_points in all_data[scenario].items():
            create_graphic(scenario, impl_name, data_points, output_dir)


def resolve_paths(argv):
    input_path = INPUT_FILE
    output_dir = OUTPUT_DIR
    if len(argv) >= 2:
        input_path = argv[1]
    if len(argv) >= 3:
        output_dir = argv[2]
    return input_path, output_dir


if __name__ == "__main__":
    input_path, output_dir = resolve_paths(sys.argv)
    print(f"Parsing file: {input_path}")
    all_data = parse_file(input_path)
    print(f"Found scenarios: {list(all_data.keys())}")
    print(f"Generating graphics in '{output_dir}/' directory...")
    generate_all_graphics(all_data, output_dir)
    print("Done.")