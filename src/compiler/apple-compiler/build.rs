use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use toml::Value;

struct Replacement {
    pub from: String,
    pub to: String,
}

fn get_replacements() -> Vec<Replacement> {
    let mut out: Vec<Replacement> = vec![];

    // Read the contents of the TOML file
    let mut file = File::open("bindgen.toml").expect("Failed to open file");
    let mut toml_str = String::new();
    file.read_to_string(&mut toml_str)
        .expect("Failed to read file");

    // Parse the TOML contents into a TOML Value
    let toml_value = toml_str.parse::<Value>().expect("Failed to parse TOML");

    // Access the sections and values in the TOML data
    if let Some(default_section) = toml_value.get("default") {
        if let Some(replacements_array) = default_section.get("replacements") {
            if let Some(replacements_array) = replacements_array.as_array() {
                for replacement in replacements_array {
                    if let Some(replacement_str) = replacement.as_str() {
                        let repl = replacement_str.to_string();
                        let (from, to) = repl
                            .split_once(" #=># ")
                            .expect("Bindgen.toml is malformed");

                        let replacement = Replacement {
                            from: from.into(),
                            to: to.into(),
                        };
                        out.push(replacement);
                    }
                }
            }
        }
    }

    out
}

fn sdk_path(target: &str) -> Result<String, std::io::Error> {
    let sdk = if vec![
        "x86_64-apple-ios",
        "i386-apple-ios",
        "aarch64-apple-ios-sim",
    ]
    .contains(&target)
    {
        "iphonesimulator"
    } else if target == "aarch64-apple-ios"
        || target == "armv7-apple-ios"
        || target == "armv7s-apple-ios"
    {
        "iphoneos"
    } else {
        unreachable!();
    };

    let output = Command::new("xcrun")
        .args(&["--sdk", sdk, "--show-sdk-path"])
        .output()?
        .stdout;
    let prefix_str = std::str::from_utf8(&output).expect("invalid output from `xcrun`");
    Ok(prefix_str.trim_end().to_string())
}

#[derive(Debug)]
struct EncodesCallback {}
impl bindgen::callbacks::ParseCallbacks for EncodesCallback {}

fn build_appkit() {
    let output = Command::new("xcrun")
        .args(&["--show-sdk-path"])
        .output()
        .unwrap()
        .stdout;

    let sdk_path = std::str::from_utf8(&output)
        .expect("invalid output from `xcrun`")
        .trim();

    println!("cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS");
    println!("cargo:rustc-link-lib=framework=AppKit");

    let mut builder = bindgen::Builder::default();

    let clang_args = vec!["-x", "objective-c", "-fblocks", "-isysroot", sdk_path];

    builder = builder
        .clang_args(&clang_args)
        .parse_callbacks(Box::new(EncodesCallback {}))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .layout_tests(false)
        .generate_block(true)
        .generate_cstr(true)
        .header_contents("AppKit.h", "#include<AppKit/AppKit.h>");

    // Generate the bindings.
    let bindings = builder.generate().expect("unable to generate bindings");

    let replacements = get_replacements();

    let mut out = bindings.to_string();

    // remove redundant and malformed definitions of `id`
    out = out.replace("pub type id = *mut objc::runtime::Object", "PUB-TYPE-ID");
    let re = regex::Regex::new("pub type id = .*;").unwrap();
    out = re.replace_all(&mut out, "").into_owned();
    out = out.replace("PUB-TYPE-ID", "pub type id = *mut objc::runtime::Object");

    // Bindgen.toml `replacements`
    for replacement in &replacements {
        out = out.replace(&replacement.from, &replacement.to);
    }

    let file = "./src/bindings/appkit.rs";
    if Path::new(file).exists() {
        fs::remove_file(file).unwrap();
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(file)
        .unwrap();

    file.write_all(out.as_bytes())
        .expect("failed to write ./src/bindings/appkit.rs");
}

fn build(sdk_path: Option<&str>, target: &str) {
    println!("cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS");
    println!("cargo:rustc-link-lib=framework=UIKit");

    let target = if target == "aarch64-apple-ios" {
        "arm64-apple-ios"
    } else {
        target
    };
    // Begin building the bindgen params.
    let mut builder = bindgen::Builder::default();

    let target_arg = format!("--target={}", target);
    let mut clang_args = vec!["-x", "objective-c", "-fblocks", &target_arg];
    if let Some(sdk_path) = sdk_path {
        clang_args.extend(&["-isysroot", sdk_path]);
    }

    builder = builder
        .clang_args(&clang_args)
        .layout_tests(false)
        .blocklist_item("timezone")
        .blocklist_item("IUIStepper")
        .blocklist_function("dividerImageForLeftSegmentState_rightSegmentState_")
        .blocklist_item("objc_object")
        .header_contents("UIKit.h", "#include<UIKit/UIKit.h>");

    // Generate the bindings.
    let bindings = builder.generate().expect("unable to generate bindings");

    let replacements = get_replacements();

    let mut out = bindings.to_string();

    // remove redundant and malformed definitions of `id`
    out = out.replace("pub type id = *mut objc::runtime::Object", "PUB-TYPE-ID");
    let re = regex::Regex::new("pub type id = .*;").unwrap();
    out = re.replace_all(&mut out, "").into_owned();
    out = out.replace("PUB-TYPE-ID", "pub type id = *mut objc::runtime::Object");

    // Bindgen.toml `replacements`
    for replacement in &replacements {
        out = out.replace(&replacement.from, &replacement.to);
    }

    let file = "./src/bindings/uikit.rs";
    if Path::new(file).exists() {
        fs::remove_file(file).unwrap();
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(file)
        .unwrap();

    file.write_all(out.as_bytes())
        .expect("failed to write ./src/bindings/appkit.rs");
}

fn main() {
    let target = std::env::var("TARGET").unwrap();
    //if !target.contains("apple-ios") {
    //    build_appkit();
    //} else {
    let directory = sdk_path(&target).ok();
    build(directory.as_ref().map(String::as_ref), &target);
    //}
}
