//! Audio engine with ADSR envelope system and state management
//!
//! This module provides the core audio generation capabilities including:
//! - ADSR envelope processing
//! - Polyphonic note management
//! - Real-time audio synthesis

use crate::waveform::types::Waveform;
use device_query::Keycode;
use std::collections::HashMap;

/// ADSR envelope states
#[derive(Clone, Debug)]
pub enum EnvelopeState {
    Attack,
    Decay,
    Sustain,
    Release,
}

/// ADSR parameters configuration
#[derive(Clone, Debug)]
pub struct ADSRParams {
    pub attack_time: f32,   // seconds
    pub decay_time: f32,    // seconds
    pub sustain_level: f32, // 0.0 to 1.0
    pub release_time: f32,  // seconds
}

impl ADSRParams {
    /// Create natural piano ADSR
    pub fn natural() -> Self {
        Self {
            attack_time: 0.02,
            decay_time: 0.1,
            sustain_level: 0.7,
            release_time: 0.15,
        }
    }

    /// Create electronic synth ADSR
    pub fn electronic() -> Self {
        Self {
            attack_time: 0.01,
            decay_time: 0.05,
            sustain_level: 0.8,
            release_time: 0.1,
        }
    }

    /// Create punchy electronic ADSR (for saw/square)
    pub fn punchy() -> Self {
        Self {
            attack_time: 0.005,
            decay_time: 0.08,
            sustain_level: 0.75,
            release_time: 0.12,
        }
    }

    /// Create cyberpunk atmospheric ADSR
    pub fn cyberpunk() -> Self {
        Self {
            attack_time: 0.08,
            decay_time: 0.3,
            sustain_level: 0.6,
            release_time: 0.4,
        }
    }
}

/// Individual note state with envelope and synthesis parameters
pub struct NoteState {
    pub frequency: f32,
    pub base_volume: f32,
    pub phase: f32,
    pub envelope_state: EnvelopeState,
    pub envelope_time: f32,
    pub adsr: ADSRParams,
    pub waveform: Waveform,
}

impl NoteState {
    /// Create a new note state
    pub fn new(frequency: f32, volume: f32, adsr: ADSRParams, waveform: Waveform) -> Self {
        Self {
            frequency,
            base_volume: volume,
            phase: 0.0,
            envelope_state: EnvelopeState::Attack,
            envelope_time: 0.0,
            adsr,
            waveform,
        }
    }

    /// Update envelope and return current amplitude multiplier
    pub fn update_envelope(&mut self, dt: f32) -> f32 {
        self.envelope_time += dt;

        match self.envelope_state {
            EnvelopeState::Attack => {
                if self.envelope_time >= self.adsr.attack_time {
                    self.envelope_state = EnvelopeState::Decay;
                    self.envelope_time = 0.0;
                    1.0
                } else {
                    // Exponential attack curve for more natural sound
                    let progress = self.envelope_time / self.adsr.attack_time;
                    progress * progress
                }
            }
            EnvelopeState::Decay => {
                if self.envelope_time >= self.adsr.decay_time {
                    self.envelope_state = EnvelopeState::Sustain;
                    self.envelope_time = 0.0;
                    self.adsr.sustain_level
                } else {
                    let progress = self.envelope_time / self.adsr.decay_time;
                    // Exponential decay from 1.0 to sustain_level
                    1.0 - (1.0 - self.adsr.sustain_level) * progress * progress
                }
            }
            EnvelopeState::Sustain => self.adsr.sustain_level,
            EnvelopeState::Release => {
                let progress = self.envelope_time / self.adsr.release_time;
                if progress >= 1.0 {
                    0.0 // Signal for removal
                } else {
                    // Exponential release curve
                    self.adsr.sustain_level * (1.0 - progress * progress)
                }
            }
        }
    }

    /// Generate audio sample for this note
    pub fn generate_sample(&mut self, sample_rate: f32, envelope_multiplier: f32) -> f32 {
        // Generate waveform sample
        let wave_sample = self
            .waveform
            .generate_sample(self.phase, self.frequency, sample_rate);

        // Apply envelope and base volume
        let final_sample = wave_sample * self.base_volume * envelope_multiplier;

        // Update phase
        self.phase += self.frequency / sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        final_sample
    }

    /// Start release phase
    pub fn release(&mut self) {
        if !matches!(self.envelope_state, EnvelopeState::Release) {
            self.envelope_state = EnvelopeState::Release;
            self.envelope_time = 0.0;
        }
    }

    /// Check if note should be removed (fully released)
    pub fn is_finished(&self, envelope_multiplier: f32) -> bool {
        matches!(self.envelope_state, EnvelopeState::Release) && envelope_multiplier <= 0.0
    }
}

/// Main audio state manager
pub struct AudioState {
    active_notes: HashMap<Keycode, NoteState>,
    sample_rate: f32,
    current_waveform: Waveform,
    default_adsr: ADSRParams,
}

impl AudioState {
    /// Create new audio state
    pub fn new(sample_rate: f32, waveform: Waveform) -> Self {
        let default_adsr = match waveform {
            Waveform::Natural => ADSRParams::natural(),
            Waveform::Electronic => ADSRParams::electronic(),
            Waveform::Saw | Waveform::Square => ADSRParams::punchy(),
            Waveform::Cyberpunk => ADSRParams::cyberpunk(),
        };

        Self {
            active_notes: HashMap::new(),
            sample_rate,
            current_waveform: waveform,
            default_adsr,
        }
    }

    /// Start a new note
    pub fn start_note(&mut self, keycode: Keycode, frequency: f32, volume: f32) {
        let note_state = NoteState::new(
            frequency,
            volume,
            self.default_adsr.clone(),
            self.current_waveform,
        );
        self.active_notes.insert(keycode, note_state);
    }

    /// Stop a note (begin release phase)
    pub fn stop_note(&mut self, keycode: Keycode) {
        if let Some(note_state) = self.active_notes.get_mut(&keycode) {
            note_state.release();
        }
    }

    /// Generate a single audio sample (main synthesis loop)
    pub fn generate_sample(&mut self) -> f32 {
        let mut sample = 0.0;
        let dt = 1.0 / self.sample_rate;
        let mut to_remove = Vec::new();

        // Process each active note
        for (keycode, note_state) in self.active_notes.iter_mut() {
            // Update envelope
            let envelope_multiplier = note_state.update_envelope(dt);

            // Check if note should be removed
            if note_state.is_finished(envelope_multiplier) {
                to_remove.push(*keycode);
                continue;
            }

            // Generate sample for this note
            let note_sample = note_state.generate_sample(self.sample_rate, envelope_multiplier);
            sample += note_sample;
        }

        // Remove finished notes
        for keycode in to_remove {
            self.active_notes.remove(&keycode);
        }

        // Global volume adjustment
        sample * 0.3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adsr_creation() {
        let adsr = ADSRParams::natural();
        assert_eq!(adsr.attack_time, 0.02);
        assert_eq!(adsr.sustain_level, 0.7);
    }

    #[test]
    fn test_audio_state_creation() {
        use crate::waveform::Waveform;
        let state = AudioState::new(44100.0, Waveform::Electronic);
        assert_eq!(state.sample_rate, 44100.0);
        assert_eq!(state.active_notes.len(), 0);
    }

    #[test]
    fn test_note_lifecycle() {
        use crate::waveform::Waveform;
        let mut state = AudioState::new(44100.0, Waveform::Electronic);

        // Start note
        state.start_note(Keycode::A, 440.0, 0.5);
        assert_eq!(state.active_notes.len(), 1);

        // Stop note
        state.stop_note(Keycode::A);
        // Note should still be active (in release phase)
        assert_eq!(state.active_notes.len(), 1);
    }
}
