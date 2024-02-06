# A naive but pretty much functional way to extract composables from the androidx repo.
# You must have the androidx repo cloned somewhere on your computer.
# git clone git@github.com/androidx/androidx.git
import re
import os
import json

paths = [
    "/Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3"
]

pattern = re.compile(r"@Composable\s*fun\s+(\w+)\s*\(\s*([^{]*)", re.MULTILINE)


def extract_arguments(args):
    arguments = []
    for arg in args.split(","):
        # Split the argument into name and type
        arg_parts = arg.strip().split(":")
        arg_name = arg_parts[0].strip()
        if len(arg_parts) > 1:
            # Join the parts after the first one to capture the full type
            arg_type = ":".join(arg_parts[1:]).strip()
        else:
            arg_type = None

        if arg_name is not None:
            arguments.append({"name": arg_name, "type": arg_type})
    return arguments


def extract_composables_from_file(file_path):
    result = {}
    with open(file_path, "r") as file:
        content = file.read()
        matches = pattern.findall(content)
        for match in matches:
            name, args = match
            args = extract_arguments(args)
            result[name] = {"arguments": args}
    return result


def extract_composables_from_directories(paths):
    combined_result = {}
    for path in paths:
        for root, _, files in os.walk(path):
            for file_name in files:
                if file_name.endswith(".kt"):
                    file_path = os.path.join(root, file_name)
                    result = extract_composables_from_file(file_path)
                    for name, props in result.items():
                        combined_result.setdefault(name, []).append(props)
    return combined_result


# Extract composable functions from the directories
composables = extract_composables_from_directories(paths)

# Output the result as JSON
print(json.dumps(composables, indent=4))
