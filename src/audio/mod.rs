//! Audio processing and synthesis module
//!
//! This module provides the core audio engine functionality including:
//! - Real-time audio synthesis with ADSR envelopes
//! - Polyphonic note management
//! - Waveform-specific audio state handling
//! - Low-latency audio generation for real-time keyboard input

pub mod engine;

pub use engine::AudioState;
