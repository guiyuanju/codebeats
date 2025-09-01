//! GUI main entry point for CodeBeats
//!
//! This binary provides a graphical user interface for configuring and launching
//! the CodeBeats command-line application.

// Hide console window on Windows
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod gui;

fn main() -> Result<(), eframe::Error> {
    // Initialize logging
    env_logger::init();

    // Run the GUI
    gui::run_gui()
}
