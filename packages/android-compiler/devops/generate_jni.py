import re
import json


callback_pattern = re.compile(r"\((\(.*\)*)\s*->(.*)\)", re.MULTILINE | re.DOTALL)


def generate_jni_bindings_and_kotlin_composables(json_file, package_name, rust_output_file, kotlin_output_file):
    with open(json_file, 'r') as f:
        composables = json.load(f)

    rust_bindings = []
    kotlin_functions = []

    for composable in composables:
        composable_name = composable['name']
        arguments = composable.get('arguments', [])

        # Generate Kotlin function signature
        kotlin_function_signature = f"fun {composable_name}("
        for arg in arguments:
            arg_name = arg['name']
            arg_type = arg['type']
            kotlin_function_signature += f"{arg_name}: {arg_type}, "
        kotlin_function_signature = kotlin_function_signature.rstrip(', ')
        kotlin_function_signature += ")"

        # Generate Kotlin function body
        kotlin_function_body = " : @Composable {\n"
        kotlin_function_body += f"\t// Your Jetpack Compose code for {composable_name} goes here\n"
        kotlin_function_body += "}\n"

        # Combine signature and body for Kotlin function
        kotlin_function = f"{kotlin_function_signature} {kotlin_function_body}"
        kotlin_functions.append(kotlin_function)

        # Generate Rust JNI binding
        rust_binding = "#[no_mangle]\n"
        rust_binding += f"pub extern \"system\" fn Java_{package_name.replace('.', '_')}_{composable_name}_nativeMethod<'local>("
        rust_binding += "env: JNIEnv<'local>, _class: JClass<'local>, "
        for arg in arguments:
            arg_name = arg['name']
            arg_type = arg['type']
            if arg_type.startswith("@Composable"):
                # If the argument is a Composable, construct a Composable struct
                rust_binding += f"{arg_name}_name: JString<'local>, "
                rust_binding += f"{arg_name}_args: JString<'local>, "
            else:
                if callback_pattern.match(arg_type) is not None:
                    out_types = []
                    return_type = "Unit"
                    for found_type in callback_pattern.findall(arg_type):
                        cb_args, return_type = found_type
                        types = re.compile(r"(\s*\w+)", re.DOTALL).findall(cb_args)
                        for type_raw in types:
                            out_types.append('"' + type_raw.strip() + '"')

                    arg_type = "KotlinCallback::new(vec![" + ", ".join(out_types) + f"], \"{return_type.strip()}\")"
                rust_binding += f"{arg_name}: {arg_type}, "

        rust_binding = rust_binding.rstrip(', ')
        rust_binding += ") -> jobject {\n"
        rust_binding += "\tlet jvm = get_jvm(env);\n"
        rust_binding += f"\tlet {composable_name}_composable = if {composable_name}_args.is_null() {{\n"
        rust_binding += f"\t\tComposable::new(env.get_string({composable_name}_name).expect(\"Couldn't get Java string\").to_str().unwrap(), vec![])\n"
        rust_binding += "\t} else {\n"
        rust_binding += f"\t\tlet args_str = env.get_string({composable_name}_args).expect(\"Couldn't get Java string\").to_str().unwrap();\n"
        rust_binding += "\t\tlet args_json: Vec<Composable> = serde_json::from_str(args_str).expect(\"Couldn't parse JSON\");\n"
        rust_binding += "\t\targs_json[0].clone()\n"
        rust_binding += "\t};\n"
        rust_binding += "\t// Call JNI function with the Composable instance\n"
        rust_binding += f"\tcall_jni_function(&jvm, &{composable_name}_composable);\n"
        rust_binding += "}\n\n"
        rust_bindings.append(rust_binding)

    # Write Kotlin functions to file
    with open(kotlin_output_file, 'w') as kotlin_file:
        kotlin_file.write("package " + package_name + "\n\n")
        kotlin_file.write("import androidx.compose.runtime.Composable\n\n")
        kotlin_file.write("\n".join(kotlin_functions))

    # Write Rust JNI bindings to file
    with open(rust_output_file, 'w') as rust_file:
        rust_file.write("use jni::*;\n")  # Add jni crate import
        rust_file.write("use serde_json;\n")  # Add serde_json crate import
        rust_file.write("use std::ptr;\n\n")  # Add std::ptr import
        rust_file.write("use crate::types::{self, get_jvm, Unit};\n\n")  # Add std::ptr import
        rust_file.write("\n".join(rust_bindings))

    print(f"Rust JNI bindings and Kotlin functions returning Jetpack Compose composables have been generated and written to '{rust_output_file}' and '{kotlin_output_file}'.")


json_file = 'composables.json'
package_name = 'com.elp'
rust_output_file = '../rust/src/composables.rs'
kotlin_output_file = 'kotlin_composables.kt'
generate_jni_bindings_and_kotlin_composables(json_file, package_name, rust_output_file, kotlin_output_file)

