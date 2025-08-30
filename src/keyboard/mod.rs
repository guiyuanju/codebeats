//! Keyboard input handling and note mapping module
//!
//! This module provides keyboard input processing and musical note mapping
//! functionality, including programming-optimized key assignments and
//! waveform switching controls.

pub mod mapping;

pub use mapping::{
    get_frequency_and_volume, get_frequency_from_note, get_waveform_for_key, is_waveform_key,
    theory,
};
