#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#[cfg(target_os = "ios")]
pub mod ui {
    include!("uikit.rs");
}

#[cfg(target_os = "macos")]
pub mod ui {
    include!("appkit.rs");

    pub fn initialize() {
        unsafe {
            // Initialize the NSApplication
            let _ = NSApplication::initialize();

            // Create an NSWindow and set it as the main window
            let window = NSWindow::initialize();
        }
    }
}
