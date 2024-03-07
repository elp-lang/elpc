//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

use serde::Deserialize;
use std::env;
use std::process::Command;

/// This should match whatever is defined in mac_ddc/Package.swift
/// Anything below 10.15 would require shipping Swift libraries.
const MACOS_TARGET_VERSION: &str = "14.3";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SwiftTargetInfo {
    unversioned_triple: String,
    #[serde(rename = "librariesRequireRPath")]
    libraries_require_rpath: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SwiftPaths {
    runtime_library_paths: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SwiftTarget {
    target: SwiftTargetInfo,
    paths: SwiftPaths,
}

/// Builds mac_ddc library Swift project, sets the library search options right so we link
/// against Swift run-time correctly.
fn build_swift() {
    let profile = env::var("PROFILE").unwrap();
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target = format!("{}-apple-macosx{}", arch, MACOS_TARGET_VERSION);

    let swift_target_info_str = Command::new("swift")
        .args(&["-target", &target, "-print-target-info"])
        .output()
        .unwrap()
        .stdout;
    let swift_target_info: SwiftTarget = serde_json::from_slice(&swift_target_info_str).unwrap();
    if swift_target_info.target.libraries_require_rpath {
        panic!("Libraries require RPath! Change minimum MacOS value to fix.")
    }

    if !Command::new("swift")
        .args(&["build", "-c", &profile])
        .current_dir("./swift/rust-lib/")
        .status()
        .unwrap()
        .success()
    {
        panic!("Swift library compilation failed")
    }

    swift_target_info
        .paths
        .runtime_library_paths
        .iter()
        .for_each(|path| {
            println!("cargo:rustc-link-search=native={}", path);
        });
    println!(
        "cargo:rustc-link-search=native=packages/apple-compiler/swift/rust-lib/.build/arm64-apple-macosx/{}/",
        &profile,
    );
    println!("cargo:rustc-link-lib=dylib=rust-lib");
    println!("cargo:rerun-if-changed=swift/**/*.swift");
    println!(
        "cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET={}",
        MACOS_TARGET_VERSION
    )
}

fn main() {
    let target = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target == "macos" {
        build_swift();
    }
}
