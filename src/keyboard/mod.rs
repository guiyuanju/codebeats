//! Keyboard input handling and note mapping module
//!
//! This module provides keyboard input processing and musical note mapping
//! functionality, including programming-optimized key assignments and
//! waveform switching controls.

pub mod config;
pub mod mapping;

pub use config::KeyboardConfig;
pub use mapping::get_frequency_and_volume_with_config;
