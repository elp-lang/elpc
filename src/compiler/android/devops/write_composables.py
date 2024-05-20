import os
import re
import json


def parse_kotlin_arguments(arguments):
    print(f"FUCK YOU {arguments}")
    arg_pattern = r'(?P<arg_name>\w+)\s*:\s*(?P<arg_type>[^,]+)(?:\s*=\s*(?P<default_value>[^,]+))?(?:,|$)'
    matches = re.finditer(arg_pattern, arguments)
    parsed_arguments = []
    for match in matches:
        arg_name = match.group('arg_name').strip()
        arg_type = match.group('arg_type').strip()
        default_value = match.group('default_value')
        optional = False

        if "=" in arg_type:
            arg_type = arg_type.split("=")[0].strip()
            optional = True

        # Check if the argument type is optional
        if arg_type.endswith("?"):
            arg_type = arg_type[:-1]  # Remove the '?' from the type
            optional = True

        if "." in arg_type:
            arg_type = arg_type.split(".")[0]

        parsed_arguments.append({
            "name": arg_name,
            "type": arg_type,
            "optional": optional,
            "default_value": default_value.strip() if default_value else None
        })
    return parsed_arguments


def convert_to_generic_composable(arg_type):
    if "@Composable" in arg_type:
        return "Composable"
    else:
        return arg_type


def convert_function_callback(callback_signature):
    arg_pattern = r'\((?P<args>.*)\)\s*->\s*(?P<return_type>.+)'
    match = re.match(arg_pattern, callback_signature)
    if match:
        args = match.group('args')
        return_type = match.group('return_type')
        parsed_arguments = parse_kotlin_arguments(args)
        transformed_args = ', '.join(
            [f"{convert_to_generic_composable(arg['type'])} {arg['name']}" for arg in parsed_arguments])
        return f"fn({transformed_args}) -> {return_type.rstrip(')')}"
    else:
        return callback_signature


def parse_kotlin_file(filename):
    with open(filename, 'r') as file:
        content = file.read()

    composable_functions = []
    function_declaration_pattern = r'@Composable\s+fun\s+(?P<function_name>\w+)\((?P<arguments>.*?)\)\s*[{=]'
    matches = re.finditer(function_declaration_pattern, content, re.DOTALL)
    for match in matches:
        function_name = match.group('function_name')
        arguments = match.group('arguments')
        # Fix the issue with commas nested within generic type arguments
        arguments = re.sub(r'<[^<>]*>', lambda x: x.group(0).replace(',', '__comma__'), arguments)
        parsed_arguments = parse_kotlin_arguments(arguments)
        for i, arg in enumerate(parsed_arguments):
            if re.match(r'^\(\s*[^)]*\)?\s*->', arg['type']):
                arg['type'] = convert_function_callback(arg['type'])
        composable_functions.append({
            "file": filename,
            "function_name": function_name,
            "arguments": [{
                "name": arg['name'],
                "type": convert_to_generic_composable(arg['type']).replace("__comma__", ","),
                "optional": arg['optional']
            } for arg in parsed_arguments]
        })

    return composable_functions


def process_folders(folders):
    all_composable_functions = []
    for folder in folders:
        for root, _, files in os.walk(folder):
            for file in files:
                if file.endswith(".kt"):
                    file_path = os.path.join(root, file)
                    print(f"reading: '{file_path}'")
                    composable_functions = parse_kotlin_file(file_path)
                    all_composable_functions.extend(composable_functions)
    return all_composable_functions


def main():
    folders_to_walk = [
        "/Users/dave/www/junk/androidx/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3"]
    output_file = "composable_functions.json"  # Output JSON file

    all_composable_functions = process_folders(folders_to_walk)

    with open(output_file, 'w') as f:
        json.dump(all_composable_functions, f, indent=4)

    print(f"Successfully parsed @Composable functions and saved to {output_file}")


if __name__ == "__main__":
    main()
