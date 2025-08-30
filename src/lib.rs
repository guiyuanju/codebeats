//! Piano Keyboard Sound Simulator Library
//!
//! This library provides a complete real-time audio synthesis system for mapping
//! keyboard keys to musical notes with multiple waveform types and ADSR envelopes.
//!
//! # Features
//!
//! - **Real-time Audio Synthesis**: Low-latency polyphonic audio generation
//! - **Multiple Waveforms**: Natural piano, electronic, saw, square, and cyberpunk synth
//! - **ADSR Envelope System**: Advanced attack/decay/sustain/release for natural sound
//! - **Programming-Optimized Mapping**: Keyboard layout designed for coding workflows
//! - **Cross-Platform**: Works on macOS, Windows, and Linux
//!
//! # Quick Start
//!
//! ```rust
//! use sound::{AudioState, Waveform};
//! use device_query::Keycode;
//!
//! // Create audio state with electronic waveform
//! let mut audio_state = AudioState::new(44100.0, Waveform::Electronic);
//!
//! // Waveform is set at initialization and cannot be changed at runtime
//!
//! // Start a note
//! audio_state.start_note(Keycode::A, 440.0, 0.5);
//!
//! // Generate audio samples
//! let sample = audio_state.generate_sample();
//! ```
//!
//! # Architecture
//!
//! The library is organized into several modules:
//!
//! - [`audio`] - Core audio engine with ADSR envelopes
//! - [`waveform`] - Waveform types and synthesis algorithms
//! - [`keyboard`] - Keyboard mapping and note calculation
//!
//! # Waveform Types
//!
//! - **Natural**: Piano-like tone with complex harmonics
//! - **Electronic**: Pure sine wave for clean electronic sound
//! - **Saw**: Bright sawtooth wave for electronic music
//! - **Square**: Retro 8-bit square wave
//! - **Cyberpunk**: Blade Runner 2049 style analog synthesizer

pub mod audio;
pub mod keyboard;
pub mod waveform;

// Re-export main types for convenience
pub use audio::AudioState;
pub use keyboard::get_frequency_and_volume;
pub use waveform::Waveform;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library name
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Get library information
pub fn version_info() -> String {
    format!("{} v{}", NAME, VERSION)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keyboard::mapping::get_frequency_from_note;

    #[test]
    fn test_version_info() {
        let info = version_info();
        assert!(info.contains("sound"));
        assert!(info.contains("v"));
    }

    #[test]
    fn test_audio_state_creation() {
        let _state = AudioState::new(44100.0, Waveform::Electronic);
        // AudioState should be created successfully
        assert!(true);
    }

    #[test]
    fn test_waveform_types() {
        // Test waveform parsing
        assert!(matches!(
            Waveform::from_str("natural"),
            Some(Waveform::Natural)
        ));
        assert!(matches!(
            Waveform::from_str("electronic"),
            Some(Waveform::Electronic)
        ));
        assert!(matches!(
            Waveform::from_str("cyberpunk"),
            Some(Waveform::Cyberpunk)
        ));
        assert!(matches!(Waveform::from_str("invalid"), None));
    }

    #[test]
    fn test_frequency_calculation() {
        let freq = get_frequency_from_note("A4").unwrap();
        assert!((freq - 440.0).abs() < 0.001);
    }
}
