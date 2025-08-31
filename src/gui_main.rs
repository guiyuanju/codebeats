//! GUI main entry point for CodeBeats
//!
//! This binary provides a graphical user interface for configuring and launching
//! the CodeBeats command-line application.

mod gui;

fn main() -> Result<(), eframe::Error> {
    // Initialize logging
    env_logger::init();

    // Run the GUI
    gui::run_gui()
}
