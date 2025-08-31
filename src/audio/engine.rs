//! Audio engine with ADSR envelope system and state management

use crate::keyboard::mapping::KeyRateLimiter;
use crate::waveform::types::Waveform;
use device_query::Keycode;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum EnvelopeState {
    Attack,
    Decay,
    Sustain,
    Release,
}

#[derive(Clone, Debug)]
pub struct ADSRParams {
    pub attack_time: f32,
    pub decay_time: f32,
    pub sustain_level: f32,
    pub release_time: f32,
}

impl ADSRParams {
    pub fn natural() -> Self {
        Self {
            attack_time: 0.02,
            decay_time: 0.1,
            sustain_level: 0.7,
            release_time: 0.15,
        }
    }

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
            decay_time: 0.02,
            sustain_level: 0.6,
            release_time: 0.08,
        }
    }

    pub fn cyberpunk() -> Self {
        Self {
            attack_time: 0.03,
            decay_time: 0.15,
            sustain_level: 0.65,
            release_time: 0.25,
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
    pub start_time: std::time::Instant,
    pub current_hold_volume: f32,
    pub target_hold_volume: f32,
}

impl NoteState {
    pub fn new(frequency: f32, volume: f32, adsr_params: ADSRParams, waveform: Waveform) -> Self {
        Self {
            frequency,
            base_volume: volume,
            phase: 0.0,
            envelope_state: EnvelopeState::Attack,
            envelope_time: 0.0,
            adsr: adsr_params,
            waveform,
            start_time: std::time::Instant::now(),
            current_hold_volume: 1.0,
            target_hold_volume: 1.0,
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

        // Update smooth hold duration volume
        self.update_smooth_hold_volume(sample_rate);

        // Apply envelope, base volume, and smooth hold duration reduction
        let final_sample =
            wave_sample * self.base_volume * envelope_multiplier * self.current_hold_volume;

        // Update phase
        self.phase += self.frequency / sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        final_sample
    }

    /// Update smooth hold duration volume to prevent audio crackling
    fn update_smooth_hold_volume(&mut self, sample_rate: f32) {
        let hold_duration = self.start_time.elapsed().as_secs_f32();

        // Calculate target volume based on hold duration
        self.target_hold_volume = match hold_duration {
            t if t < 0.5 => 1.0, // Normal volume for first 0.5 seconds
            t if t < 1.0 => 0.8, // Slight reduction after 0.5s
            t if t < 2.0 => 0.6, // More reduction after 1s
            t if t < 3.0 => 0.4, // Significant reduction after 2s
            t if t < 5.0 => 0.2, // Very quiet after 3s
            _ => 0.1,            // Almost silent after 5s
        };

        // Smoothly interpolate to target volume to prevent crackling
        let volume_change_rate = 0.5; // Volume units per second (slower for smoother transitions)
        let max_change_per_sample = volume_change_rate / sample_rate;

        if self.current_hold_volume < self.target_hold_volume {
            self.current_hold_volume =
                (self.current_hold_volume + max_change_per_sample).min(self.target_hold_volume);
        } else if self.current_hold_volume > self.target_hold_volume {
            self.current_hold_volume =
                (self.current_hold_volume - max_change_per_sample).max(self.target_hold_volume);
        }
    }

    /// Calculate volume reduction based on how long key has been held (for testing)
    #[cfg(test)]
    fn calculate_hold_volume_reduction(&self, hold_duration: f32) -> f32 {
        match hold_duration {
            t if t < 0.5 => 1.0, // Normal volume for first 0.5 seconds
            t if t < 1.0 => 0.8, // Slight reduction after 0.5s
            t if t < 2.0 => 0.6, // More reduction after 1s
            t if t < 3.0 => 0.4, // Significant reduction after 2s
            t if t < 5.0 => 0.2, // Very quiet after 3s
            _ => 0.1,            // Almost silent after 5s
        }
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

pub struct AudioState {
    pub active_notes: HashMap<Keycode, NoteState>,
    sample_rate: f32,
    current_waveform: Waveform,
    default_adsr: ADSRParams,
    rate_limiter: KeyRateLimiter,
    master_volume: f32,
}

impl AudioState {
    pub fn new(sample_rate: f32, waveform: Waveform, master_volume: f32) -> Self {
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
            rate_limiter: KeyRateLimiter::new(),
            master_volume: master_volume.clamp(0.0, 1.0),
        }
    }

    pub fn start_note(&mut self, keycode: Keycode, frequency: f32, volume: f32) -> f32 {
        let volume_multiplier = self.rate_limiter.check_key_press(keycode);

        if volume_multiplier <= 0.01 {
            return 0.0;
        }

        let adjusted_volume = volume * volume_multiplier * self.master_volume;

        let note_state = NoteState::new(
            frequency,
            adjusted_volume,
            self.default_adsr.clone(),
            self.current_waveform,
        );
        self.active_notes.insert(keycode, note_state);

        adjusted_volume
    }

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
        let state = AudioState::new(44100.0, Waveform::Electronic, 1.0);
        assert_eq!(state.sample_rate, 44100.0);
        assert_eq!(state.active_notes.len(), 0);
    }

    #[test]
    fn test_note_lifecycle() {
        use crate::waveform::Waveform;
        let mut state = AudioState::new(44100.0, Waveform::Electronic, 1.0);

        // Start note
        state.start_note(Keycode::A, 440.0, 0.5);
        assert_eq!(state.active_notes.len(), 1);

        // Stop note
        state.stop_note(Keycode::A);
        // Note should still be active (in release phase)
        assert_eq!(state.active_notes.len(), 1);
    }

    #[test]
    fn test_hold_duration_volume_reduction() {
        use crate::waveform::Waveform;

        let mut state = AudioState::new(44100.0, Waveform::Electronic, 1.0);

        // Start a note
        state.start_note(Keycode::J, 440.0, 0.5);

        // Get the note to test hold duration calculation
        let note = state.active_notes.get(&Keycode::J).unwrap();

        // Test initial volume (should be 1.0 for first 0.5s)
        let initial_reduction = note.calculate_hold_volume_reduction(0.1);
        assert_eq!(initial_reduction, 1.0);

        // Test volume reduction after 1 second
        let one_sec_reduction = note.calculate_hold_volume_reduction(1.0);
        assert_eq!(one_sec_reduction, 0.6);

        // Test volume reduction after 3 seconds
        let three_sec_reduction = note.calculate_hold_volume_reduction(3.0);
        assert_eq!(three_sec_reduction, 0.2);

        // Test volume reduction after 6 seconds
        let six_sec_reduction = note.calculate_hold_volume_reduction(6.0);
        assert_eq!(six_sec_reduction, 0.1);
    }
}
