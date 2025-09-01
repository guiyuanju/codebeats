//! GUI main entry point for CodeBeats
//!
//! This binary provides a graphical user interface for configuring and launching
//! the CodeBeats command-line application.

// Hide console window on Windows
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod gui;

use std::panic;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    // Set panic hook for better error reporting
    panic::set_hook(Box::new(|panic_info| {
        eprintln!("GUI panic: {}", panic_info);
        if let Some(location) = panic_info.location() {
            eprintln!("Location: {}:{}", location.file(), location.line());
        }
    }));

    println!("Starting CodeBeats GUI...");

    // Run the GUI with error handling
    match gui::run_gui() {
        Ok(_) => {
            println!("GUI closed normally");
            Ok(())
        }
        Err(e) => {
            eprintln!("GUI error: {}", e);
            Err(e)
        }
    }
}
