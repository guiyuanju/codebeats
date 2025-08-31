//! Audio engine with ADSR envelope system and state management

use crate::waveforms::Waveform;
use device_query::Keycode;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Rate limiter to reduce volume for rapid successive key presses
pub struct RateLimiter {
    press_history: HashMap<String, Vec<Instant>>,
    window_duration: Duration,
    volume_reduction_factor: f32,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            press_history: HashMap::new(),
            window_duration: Duration::from_millis(500), // 500ms window
            volume_reduction_factor: 0.7,                // Each rapid press reduces volume by 30%
        }
    }

    /// Record a key press and return volume multiplier based on recent press frequency
    pub fn record_press_and_get_volume_multiplier(&mut self, key_id: &str) -> f32 {
        let now = Instant::now();

        // Get or create press history for this key
        let history = self
            .press_history
            .entry(key_id.to_string())
            .or_insert_with(Vec::new);

        // Remove old presses outside the window
        history.retain(|&press_time| now.duration_since(press_time) <= self.window_duration);

        // Calculate volume multiplier based on recent presses
        let rapid_press_count = history.len() as f32;
        let volume_multiplier = self.volume_reduction_factor.powf(rapid_press_count);

        // Record this press
        history.push(now);

        volume_multiplier
    }
}

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
    pub active_notes_by_id: HashMap<String, NoteState>,
    sample_rate: f32,
    current_waveform: Waveform,
    default_adsr: ADSRParams,
    master_volume: f32,
    filter_cutoff: f32,
    rate_limiter: RateLimiter,
}

impl AudioState {
    pub fn new(
        sample_rate: f32,
        waveform: Waveform,
        master_volume: f32,
        filter_cutoff: f32,
    ) -> Self {
        let default_adsr = match waveform {
            Waveform::Natural => ADSRParams::natural(),
            Waveform::Electronic => ADSRParams::electronic(),
            Waveform::Saw | Waveform::Square => ADSRParams::punchy(),
            Waveform::Cyberpunk => ADSRParams::cyberpunk(),
        };

        Self {
            active_notes: HashMap::new(),
            active_notes_by_id: HashMap::new(),
            sample_rate,
            current_waveform: waveform,
            default_adsr,
            master_volume: master_volume.clamp(0.0, 1.0),
            filter_cutoff,
            rate_limiter: RateLimiter::new(),
        }
    }

    #[allow(dead_code)]
    pub fn start_note(&mut self, keycode: Keycode, frequency: f32, volume: f32) -> f32 {
        let adjusted_volume = volume * self.master_volume;

        let note_state = NoteState::new(
            frequency,
            adjusted_volume,
            self.default_adsr.clone(),
            self.current_waveform,
        );
        self.active_notes.insert(keycode, note_state);

        adjusted_volume
    }

    #[allow(dead_code)]
    pub fn stop_note(&mut self, keycode: Keycode) {
        if let Some(note_state) = self.active_notes.get_mut(&keycode) {
            note_state.release();
        }
    }

    pub fn stop_note_with_id(&mut self, key_id: &str) {
        if let Some(note_state) = self.active_notes_by_id.get_mut(key_id) {
            note_state.release();
        }
    }

    /// Start a note with string-based identifier (for virtual keys)
    pub fn start_note_with_id(&mut self, key_id: &str, frequency: f32, volume: f32) -> f32 {
        // Apply rate limiting - get volume multiplier based on recent press frequency
        let rate_limit_multiplier = self
            .rate_limiter
            .record_press_and_get_volume_multiplier(key_id);
        let adjusted_volume = volume * self.master_volume * rate_limit_multiplier;

        let note_state = NoteState::new(
            frequency,
            adjusted_volume,
            self.default_adsr.clone(),
            self.current_waveform,
        );
        self.active_notes_by_id
            .insert(key_id.to_string(), note_state);

        adjusted_volume
    }

    /// Generate a single audio sample (main synthesis loop)
    pub fn generate_sample(&mut self) -> f32 {
        let mut sample = 0.0;
        let dt = 1.0 / self.sample_rate;
        let mut to_remove = Vec::new();
        let mut to_remove_by_id = Vec::new();

        // Process each active note (keycode-based)
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

        // Process each active note (string ID-based)
        for (key_id, note_state) in self.active_notes_by_id.iter_mut() {
            // Update envelope
            let envelope_multiplier = note_state.update_envelope(dt);

            // Check if note should be removed
            if note_state.is_finished(envelope_multiplier) {
                to_remove_by_id.push(key_id.clone());
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
        for key_id in to_remove_by_id {
            self.active_notes_by_id.remove(&key_id);
        }

        // Global volume adjustment - normalized for comfortable listening
        sample
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
        let state = AudioState::new(44100.0, Waveform::Electronic, 1.0, 1200.0);
        assert_eq!(state.sample_rate, 44100.0);
    }

    #[test]
    fn test_note_lifecycle() {
        use device_query::Keycode;
        let mut state = AudioState::new(44100.0, Waveform::Electronic, 1.0, 1200.0);

        // Start note
        state.start_note(Keycode::A, 440.0, 0.5);
        assert_eq!(state.active_notes.len(), 1);

        // Stop note
        state.stop_note(Keycode::A);
        // Note should still exist but be in release phase
        assert_eq!(state.active_notes.len(), 1);
    }

    #[test]
    fn test_rate_limiter() {
        let mut limiter = RateLimiter::new();

        // First press should have full volume
        let vol1 = limiter.record_press_and_get_volume_multiplier("test_key");
        assert_eq!(vol1, 1.0);

        // Rapid second press should have reduced volume (0.7)
        let vol2 = limiter.record_press_and_get_volume_multiplier("test_key");
        assert!((vol2 - 0.7).abs() < 0.01);

        // Different key should start fresh
        let vol_other = limiter.record_press_and_get_volume_multiplier("other_key");
        assert_eq!(vol_other, 1.0);
    }
}
