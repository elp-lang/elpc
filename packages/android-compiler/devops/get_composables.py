# A naive but pretty much functional way to extract composables from the androidx repo.
# You must have the androidx repo cloned somewhere on your computer.
# git clone git@github.com/androidx/androidx.git
import re
import os
import json

paths = [
    "/Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3"
]

# Capture the function header.
pattern = re.compile(r"@Composable\s*fun\s*(\w+)\s*\(((\s*(\w+):(.*))*)\s*\)")

# Capture each argument.
arg_pattern = re.compile(r"^\s*(\w+):\s*(.*)$", re.MULTILINE)


def extract_arguments(args: str):
    arguments = []
    params = arg_pattern.findall(args)
    for arg in params:
        arg_name, arg_type = arg
        arg_default = arg_type.split("=")
        arg_type = arg_default[0].strip()

        if arg_name is not None:
            arguments.append({
                "name": arg_name,
                "type": arg_type.rstrip("?").rstrip(","),
                "optional": arg_type.endswith("?") or len(arg_default) > 1,
            })
    return arguments


def extract_composables_from_file(file_path):
    print(f"reading '{file_path}'")
    results = []
    with open(file_path, "r") as file:
        content = file.read()
        matches = pattern.finditer(content)
        for match in matches:
            name, args, *_ = match.groups()
            args = extract_arguments(args)
            results.append({"name": name, "arguments": args})
    return results


def extract_composables_from_directories(paths):
    combined_result = []
    for path in paths:
        for root, _, files in os.walk(path):
            for file_name in files:
                if file_name.endswith(".kt"):
                    file_path = os.path.join(root, file_name)
                    results = extract_composables_from_file(file_path)
                    combined_result.extend(results)
    return combined_result


# Extract composable functions from the directories
composables = extract_composables_from_directories(paths)

# Output the result as JSON
as_json = json.dumps(composables, indent=4)

with open('composables.json', 'w') as f:
    f.write(as_json)
