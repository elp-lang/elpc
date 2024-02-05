use bindgen::Builder;
use std::{fmt, process::Command};

const MACOS_SDK_VERSION: &str = "14.0";

#[derive(Debug)]
enum TargetArch {
    X86_64,
    ARMV8a,
    ARMV7,
    ARMV7s,
}

impl fmt::Display for TargetArch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

enum TargetPlatform {
    IphoneOS,
    IphoneOSSim,
    MacOS,
}

struct BuildInformation {
    pub arch: TargetArch,
    pub platform: TargetPlatform,
}

impl BuildInformation {
    fn get_sdk_name(&self, include_version: bool) -> String {
        match &self.platform {
            TargetPlatform::IphoneOS => "iphoneos".into(),
            TargetPlatform::IphoneOSSim => "iphoneossim".into(),
            TargetPlatform::MacOS => {
                if include_version {
                    format!("macosx{}", MACOS_SDK_VERSION.clone())
                } else {
                    "macosx".into()
                }
            }
        }
    }

    pub fn get_triplet(&self) -> String {
        match self.arch {
            TargetArch::X86_64 => match self.platform {
                TargetPlatform::IphoneOS => "x86_64-apple-ios".into(),
                TargetPlatform::IphoneOSSim => "x86_64-apple-ios-sim".into(),
                TargetPlatform::MacOS => "x86_64-apple-darwin".into(),
            },
            TargetArch::ARMV7 => match self.platform {
                TargetPlatform::IphoneOS => "armv7-apple-ios".into(),
                TargetPlatform::IphoneOSSim => "armv7-apple-ios-sim".into(),
                TargetPlatform::MacOS => {
                    panic!("MacOS target doesn't support the ARM7 architecture")
                }
            },
            TargetArch::ARMV7s => match self.platform {
                TargetPlatform::IphoneOS => "armv7s-apple-ios".into(),
                TargetPlatform::IphoneOSSim => "armv7s-apple-ios-sim".into(),
                TargetPlatform::MacOS => {
                    panic!("MacOS target doesn't support the ARM7s architecture")
                }
            },
            TargetArch::ARMV8a => match self.platform {
                TargetPlatform::IphoneOS => "armv8-apple-ios".into(),
                TargetPlatform::IphoneOSSim => "armv8-apple-ios-sim".into(),
                TargetPlatform::MacOS => "arm64-apple-darwin".into(),
            },
        }
    }

    fn clang_args(&self) -> Vec<String> {
        let triplet = self.get_triplet().to_owned();
        println!("sdk: {}", self.get_sdk_name(false));
        println!("triplet: {}", triplet);
        let mut clang_args = vec![
            "-x".to_string(),
            "objective-c".into(),
            "-fblocks".into(),
            "--target".into(),
            triplet,
        ];

        let output = Command::new("xcrun")
            .args(&["--sdk", &self.get_sdk_name(false), "--show-sdk-path"])
            .output()
            .unwrap()
            .stdout;

        let prefix_str = std::str::from_utf8(&output).unwrap().trim_end().to_owned();
        clang_args.push("-isysroot".into());
        clang_args.push(prefix_str);

        clang_args
    }
}

fn generate_uikit(build_info: &BuildInformation) {
    let clang_args = build_info.clang_args();
    println!("{:?}", clang_args);

    let mut uikit_builder = Builder::default();
    uikit_builder = uikit_builder
        .clang_args(clang_args)
        .objc_extern_crate(true)
        .layout_tests(false)
        //.block_extern_crate(true)
        //.generate_block(true)
        // time.h as has a variable called timezone that conflicts with some of the objective-c
        // calls from NSCalendar.h in the Foundation framework. This removes that one variable.
        .blocklist_item("timezone")
        // https://github.com/rust-lang/rust-bindgen/issues/1705
        .blocklist_item("IUIStepper")
        .blocklist_function("dividerImageForLeftSegmentState_rightSegmentState_")
        .blocklist_item("objc_object")
        .header_contents("UIKit.h", "#include<UIKit/UIKit.h>");

    println!("{:#?}", uikit_builder);
    match uikit_builder.generate() {
        Ok(uikit_bindings) => {
            uikit_bindings
                .write_to_file(format!("src/bindings/uikit_{}.rs", build_info.arch))
                .expect("could not write bindings");
        }
        Err(err) => panic!("{}", err),
    }
}

fn generate_appkit(build_info: &BuildInformation) {
    let clang_args = build_info.clang_args();
    println!("{:?}", clang_args);

    let mut appkit_builder = Builder::default();
    appkit_builder = appkit_builder
        .clang_args(&clang_args)
        .objc_extern_crate(true)
        .layout_tests(false)
        //.block_extern_crate(true)
        //.generate_block(true)
        // time.h as has a variable called timezone that conflicts with some of the objective-c
        // calls from NSCalendar.h in the Foundation framework. This removes that one variable.
        .blocklist_item("timezone")
        // https://github.com/rust-lang/rust-bindgen/issues/1705
        .blocklist_item("IUIStepper")
        .blocklist_function("dividerImageForLeftSegmentState_rightSegmentState_")
        .blocklist_item("objc_object")
        .header_contents("AppKit.h", "#import <AppKit/AppKit.h>");

    println!("{:#?}", appkit_builder);
    match appkit_builder.generate() {
        Ok(appkit_bindings) => {
            appkit_bindings
                .write_to_file(format!("src/bindings/appkit_{}.rs", build_info.arch))
                .expect("could not write bindings");
        }
        Err(err) => panic!("{}", err),
    }
}

fn main() {
    println!("cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS");
    println!("cargo:rustc-link-lib=framework=UIKit");

    let targets = vec![
        BuildInformation {
            platform: TargetPlatform::MacOS,
            arch: TargetArch::ARMV8a,
        },
        //        BuildInformation {
        //            platform: TargetPlatform::MacOS,
        //            arch: TargetArch::ARMV8a,
        //        },
        //        BuildInformation {
        //            platform: TargetPlatform::IphoneOS,
        //            arch: TargetArch::ARMV8a,
        //        },
        //        BuildInformation {
        //            platform: TargetPlatform::IphoneOS,
        //            arch: TargetArch::ARMV7,
        //        },
        //        BuildInformation {
        //            platform: TargetPlatform::IphoneOS,
        //            arch: TargetArch::ARMV7s,
        //        },
    ];

    for target in targets {
        match &target.platform {
            TargetPlatform::MacOS => generate_appkit(&target),
            TargetPlatform::IphoneOS => generate_uikit(&target),
            TargetPlatform::IphoneOSSim => generate_uikit(&target),
        }
    }
}
