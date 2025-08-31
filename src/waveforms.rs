//! Waveform types and synthesis algorithms
//!
//! This module defines different waveform types and their synthesis methods:
//! - Natural piano with harmonics
//! - Electronic pure sine wave
//! - Saw and square waves for electronic music
//! - Cyberpunk analog synthesizer emulation

use std::f32::consts::PI;

/// Available waveform types
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Waveform {
    /// Natural piano with complex harmonics and subtle vibrato
    Natural,
    /// Pure sine wave for clean electronic sound
    Electronic,
    /// Sawtooth wave for bright electronic music
    Saw,
    /// Square wave for retro 8-bit sound
    Square,
    /// Blade Runner 2049 style analog synthesizer
    Cyberpunk,
    /// Mathematical harmonic series with precise overtones
    Harmonic,
    /// Pure sine wave (alias for Electronic)
    Sine,
    /// Sawtooth wave (alias for Saw)
    Sawtooth,
    /// Triangle wave for smooth electronic sound
    Triangle,
}

impl Waveform {
    /// Generate a single audio sample for this waveform
    pub fn generate_sample(&self, phase: f32, frequency: f32, sample_rate: f32) -> f32 {
        let base_phase = phase * 2.0 * PI;

        match self {
            Waveform::Electronic => self.generate_sine(base_phase),
            Waveform::Natural => self.generate_natural_piano(phase, base_phase, sample_rate),
            Waveform::Saw => self.generate_sawtooth(phase),
            Waveform::Square => self.generate_square(phase),
            Waveform::Cyberpunk => {
                self.generate_cyberpunk(phase, base_phase, frequency, sample_rate)
            }
            Waveform::Harmonic => self.generate_harmonic(base_phase),
            Waveform::Sine => self.generate_sine(base_phase),
            Waveform::Sawtooth => self.generate_sawtooth(phase),
            Waveform::Triangle => self.generate_triangle(phase),
        }
    }

    /// Pure sine wave generation
    fn generate_sine(&self, base_phase: f32) -> f32 {
        base_phase.sin()
    }

    /// Natural piano with harmonics and vibrato
    fn generate_natural_piano(&self, phase: f32, base_phase: f32, _sample_rate: f32) -> f32 {
        // Harmonic series for piano-like tone
        let fundamental = base_phase.sin();
        let harmonic2 = (base_phase * 2.0).sin() * 0.3;
        let harmonic3 = (base_phase * 3.0).sin() * 0.15;
        let harmonic4 = (base_phase * 4.0).sin() * 0.08;
        let harmonic5 = (base_phase * 5.0).sin() * 0.05;

        // Add subtle vibrato for natural feel
        let vibrato_rate = 6.0;
        let vibrato_depth = 0.002;
        let vibrato = (phase * vibrato_rate * 2.0 * PI).sin() * vibrato_depth;
        let modulated_phase = base_phase * (1.0 + vibrato);

        // Combine all harmonics
        fundamental + harmonic2 + harmonic3 + harmonic4 + harmonic5 + modulated_phase.sin() * 0.02
    }

    /// Sawtooth wave generation
    fn generate_sawtooth(&self, phase: f32) -> f32 {
        2.0 * (phase - phase.floor()) - 1.0
    }

    /// Square wave generation
    fn generate_square(&self, phase: f32) -> f32 {
        if (phase % 1.0) < 0.5 {
            1.0
        } else {
            -1.0
        }
    }

    /// Cyberpunk 2049 style analog synthesizer
    fn generate_cyberpunk(
        &self,
        phase: f32,
        base_phase: f32,
        frequency: f32,
        sample_rate: f32,
    ) -> f32 {
        let time = phase * sample_rate / frequency;

        // Multi-oscillator setup
        let saw = self.generate_sawtooth(phase);
        let sub_bass = (base_phase * 0.5).sin() * 0.4; // Sub-octave bass

        // LFO modulation for analog character
        let lfo1_rate = 0.3;
        let lfo2_rate = 0.7;
        let lfo1 = (time * lfo1_rate).sin();
        let lfo2 = (time * lfo2_rate).sin();

        // PWM (Pulse Width Modulation) for analog warmth
        let pulse_width = 0.5 + lfo1 * 0.1;
        let pulse = if (phase % 1.0) < pulse_width {
            1.0
        } else {
            -1.0
        };

        // Mix oscillators
        let mixed = saw * 0.6 + pulse * 0.3 + sub_bass;

        // Analog-style low-pass filter simulation
        let cutoff_modulation = 0.7 + lfo2 * 0.2;
        let filtered = mixed * cutoff_modulation;

        // Soft saturation for analog warmth
        let drive_amount = 1.2;
        let driven = filtered * drive_amount;
        let saturated = self.soft_clip(driven, 0.8);

        // Chorus effect with slight detuning
        let detune1 = (base_phase * 1.003).sin() * 0.15;
        let detune2 = (base_phase * 0.997).sin() * 0.15;

        saturated + detune1 + detune2
    }

    /// Mathematical harmonic series with precise overtones
    fn generate_harmonic(&self, base_phase: f32) -> f32 {
        // Pure harmonic series based on mathematical ratios
        let fundamental = base_phase.sin();
        let harmonic2 = (base_phase * 2.0).sin() * 0.5; // Octave (2:1)
        let harmonic3 = (base_phase * 3.0).sin() * 0.333; // Perfect fifth (3:2)
        let harmonic4 = (base_phase * 4.0).sin() * 0.25; // Double octave (4:1)
        let harmonic5 = (base_phase * 5.0).sin() * 0.2; // Major third (5:4)
        let harmonic6 = (base_phase * 6.0).sin() * 0.167; // Perfect fifth + octave (3:1)
        let harmonic7 = (base_phase * 7.0).sin() * 0.143; // Harmonic seventh (7:4)
        let harmonic8 = (base_phase * 8.0).sin() * 0.125; // Triple octave (8:1)

        // Mathematical precision with golden ratio influence
        let phi = 1.618034; // Golden ratio
        let phi_harmonic = (base_phase * phi).sin() * 0.1;

        fundamental
            + harmonic2
            + harmonic3
            + harmonic4
            + harmonic5
            + harmonic6
            + harmonic7
            + harmonic8
            + phi_harmonic
    }

    /// Triangle wave generation
    fn generate_triangle(&self, phase: f32) -> f32 {
        let t = phase % 1.0;
        if t < 0.5 {
            4.0 * t - 1.0
        } else {
            3.0 - 4.0 * t
        }
    }

    /// Soft clipping for warm analog distortion
    fn soft_clip(&self, input: f32, threshold: f32) -> f32 {
        if input > threshold {
            threshold + (input - threshold) * 0.3
        } else if input < -threshold {
            -threshold + (input + threshold) * 0.3
        } else {
            input
        }
    }

    /// Get a human-readable description of the waveform
    /// Parse waveform from string (case insensitive)
    pub fn from_str(s: &str) -> Option<Waveform> {
        match s.to_lowercase().as_str() {
            "natural" => Some(Waveform::Natural),
            "electronic" => Some(Waveform::Electronic),
            "saw" => Some(Waveform::Saw),
            "square" => Some(Waveform::Square),
            "cyberpunk" => Some(Waveform::Cyberpunk),
            "harmonic" => Some(Waveform::Harmonic),
            "sine" => Some(Waveform::Sine),
            "sawtooth" => Some(Waveform::Sawtooth),
            "triangle" => Some(Waveform::Triangle),
            _ => None,
        }
    }
}

impl Default for Waveform {
    fn default() -> Self {
        Waveform::Natural
    }
}

impl std::fmt::Display for Waveform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_waveform_generation() {
        let waveforms = vec![
            Waveform::Natural,
            Waveform::Electronic,
            Waveform::Saw,
            Waveform::Square,
            Waveform::Cyberpunk,
            Waveform::Harmonic,
            Waveform::Sine,
            Waveform::Sawtooth,
            Waveform::Triangle,
        ];

        for waveform in waveforms {
            let sample = waveform.generate_sample(0.25, 440.0, 44100.0);
            assert!(
                sample.is_finite(),
                "Waveform {:?} produced invalid sample",
                waveform
            );
            assert!(
                sample >= -2.0 && sample <= 2.0,
                "Waveform {:?} sample out of reasonable range: {}",
                waveform,
                sample
            );
        }
    }

    #[test]
    fn test_waveform_parsing() {
        assert_eq!(Waveform::from_str("natural"), Some(Waveform::Natural));
        assert_eq!(Waveform::from_str("ELECTRONIC"), Some(Waveform::Electronic));
        assert_eq!(Waveform::from_str("invalid"), None);
    }

    #[test]
    fn test_sine_wave() {
        let wave = Waveform::Electronic;

        // Test known values
        let sample_0 = wave.generate_sample(0.0, 440.0, 44100.0);
        let sample_quarter = wave.generate_sample(0.25, 440.0, 44100.0);

        assert!((sample_0 - 0.0).abs() < 0.001);
        assert!((sample_quarter - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_square_wave() {
        let wave = Waveform::Square;

        let sample_low = wave.generate_sample(0.25, 440.0, 44100.0);
        let sample_high = wave.generate_sample(0.75, 440.0, 44100.0);

        assert_eq!(sample_low, 1.0);
        assert_eq!(sample_high, -1.0);
    }
}
