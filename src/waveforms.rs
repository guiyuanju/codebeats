//! Waveform types and synthesis algorithms
//!
//! This module defines different waveform types and their synthesis methods:
//! - Natural piano with harmonics
//! - Electronic pure sine wave
//! - Saw and square waves for electronic music
//! - Cyberpunk analog synthesizer emulation
//! - Bass with deep low frequencies

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
    /// Triangle wave for smooth electronic sound
    Triangle,
    /// Realistic fart sound synthesis with turbulence and frequency sweeps
    Fart,
    /// Deep bass with rich low frequencies and powerful sub-bass
    Bass,
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
            Waveform::Triangle => self.generate_triangle(phase),
            Waveform::Fart => self.generate_fart(phase, base_phase, frequency, sample_rate),
            Waveform::Bass => self.generate_bass(phase, base_phase, frequency, sample_rate),
        }
    }

    /// Get ADSR envelope parameters for this waveform
    pub fn get_adsr_params(&self) -> (f32, f32, f32, f32) {
        match self {
            // Natural/organic attack and release
            Waveform::Natural => (0.01, 0.1, 0.8, 0.3),
            Waveform::Bass => (0.02, 0.15, 0.9, 0.4), // Slightly slower for bass depth

            // Clean electronic envelopes
            Waveform::Electronic => (0.005, 0.05, 0.7, 0.2),
            Waveform::Triangle => (0.005, 0.05, 0.7, 0.2),

            // Sharp attack for electronic percussion
            Waveform::Saw => (0.001, 0.02, 0.6, 0.1),
            Waveform::Square => (0.001, 0.02, 0.6, 0.1),

            // Analog-style envelopes
            Waveform::Cyberpunk => (0.02, 0.1, 0.8, 0.25),

            // Special envelope for fart
            Waveform::Fart => (0.01, 0.05, 0.7, 0.2),
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
        let second_harmonic = (base_phase * 2.0).sin() * 0.3;
        let third_harmonic = (base_phase * 3.0).sin() * 0.2;
        let fourth_harmonic = (base_phase * 4.0).sin() * 0.1;

        // Subtle vibrato
        let vibrato_rate = 4.5;
        let vibrato_depth = 0.02;
        let vibrato = (phase * vibrato_rate * 2.0 * PI).sin() * vibrato_depth;
        let modulated_fundamental = (base_phase * (1.0 + vibrato)).sin();

        // Mix harmonics with slight vibrato
        (modulated_fundamental * 0.7 + fundamental * 0.3)
            + second_harmonic
            + third_harmonic
            + fourth_harmonic
    }

    /// Sawtooth wave generation
    fn generate_sawtooth(&self, phase: f32) -> f32 {
        2.0 * (phase - phase.floor()) - 1.0
    }

    /// Square wave generation
    fn generate_square(&self, phase: f32) -> f32 {
        if (phase % 1.0) < 0.5 { 1.0 } else { -1.0 }
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

        // Multiple analog oscillators with slight detuning
        let osc1 = base_phase.sin();
        let osc2 = (base_phase * 1.003).sin(); // Slightly detuned
        let osc3 = (base_phase * 0.997).sin(); // Slightly detuned the other way

        // Sub-oscillator (one octave down)
        let sub_osc = (base_phase * 0.5).sin() * 0.3;

        // LFO modulation
        let lfo_rate = 0.3;
        let lfo = (time * lfo_rate * 2.0 * PI).sin();
        let lfo_mod = lfo * 0.1 + 1.0;

        // Mix oscillators
        let mixed = (osc1 + osc2 * 0.8 + osc3 * 0.6) / 2.4 + sub_osc;

        // Apply LFO modulation
        let modulated = mixed * lfo_mod;

        // Soft analog-style clipping
        let clipped = modulated.tanh() * 0.8;

        clipped
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

    /// Realistic fart sound synthesis with turbulence and frequency sweeps
    ///
    /// This implementation creates a more realistic fart sound by focusing on:
    /// - Strong fundamental tones in the 40-150Hz range
    /// - Natural harmonic series for organic timbre
    /// - Body cavity resonance simulation
    /// - Gentle frequency sweeps for natural pitch variations
    /// - Tonal emphasis with minimal filtered turbulence
    fn generate_fart(&self, phase: f32, base_phase: f32, frequency: f32, sample_rate: f32) -> f32 {
        let time = phase * sample_rate / frequency;

        // Force low frequency range (40-150Hz) for realistic fart sounds
        let fart_freq = (frequency * 0.15).max(40.0).min(150.0);
        let fart_phase = base_phase * fart_freq / frequency;

        // 1. Strong fundamental tone (main component)
        let fundamental = fart_phase.sin() * 0.8;

        // 2. Harmonic series for natural timbre
        let harmonic2 = (fart_phase * 2.0).sin() * 0.4;
        let harmonic3 = (fart_phase * 3.0).sin() * 0.25;
        let harmonic4 = (fart_phase * 4.0).sin() * 0.15;

        // 3. Sub-bass for body resonance
        let sub_bass = (fart_phase * 0.5).sin() * 0.3;

        // 4. Gentle frequency sweep (much more subtle)
        let sweep_amount = (time * 0.4).sin() * 0.05; // Very small modulation
        let swept_fundamental = (fart_phase * (1.0 + sweep_amount)).sin() * 0.2;

        // 5. Body cavity formant simulation (resonant filtering effect)
        let formant_freq1 = 80.0; // First formant around 80Hz
        let formant_freq2 = 120.0; // Second formant around 120Hz
        let formant1 = (fart_phase * formant_freq1 / fart_freq).sin() * 0.3;
        let formant2 = (fart_phase * formant_freq2 / fart_freq).sin() * 0.2;

        // 6. Very gentle breath-like texture (much less harsh)
        let breath_rate = 2.0;
        let breath_mod = (time * breath_rate).sin() * 0.02 + 0.98; // Subtle amplitude variation

        // 7. Minimal filtered turbulence (not harsh noise)
        let turbulence_seed = (time * 50.0).fract();
        let gentle_turbulence = (turbulence_seed * 100.0).sin() * 0.05; // Very quiet

        // Mix tonal components (emphasis on harmonics)
        let tonal_mix =
            fundamental + harmonic2 + harmonic3 + harmonic4 + sub_bass + swept_fundamental;
        let formant_mix = tonal_mix + formant1 + formant2;

        // Apply gentle breath modulation
        let modulated = formant_mix * breath_mod;

        // Add minimal turbulence and normalize
        let final_output = (modulated + gentle_turbulence) * 0.4;

        // Apply gentle limiting to prevent harsh peaks
        final_output.tanh()
    }

    /// Deep bass with rich low frequencies and powerful sub-bass
    ///
    /// This implementation creates a deep, powerful bass sound with:
    /// - Strong fundamental with sub-bass emphasis
    /// - Rich harmonic content for thickness
    /// - Slight saturation for analog warmth
    /// - Low-frequency modulation for movement
    fn generate_bass(&self, phase: f32, base_phase: f32, frequency: f32, sample_rate: f32) -> f32 {
        let time = phase * sample_rate / frequency;

        // Emphasize lower frequencies by shifting frequency down
        let _bass_freq = frequency * 0.5; // One octave down for deeper bass
        let bass_phase = base_phase * 0.5;

        // 1. Strong fundamental (main bass tone)
        let fundamental = bass_phase.sin() * 1.0;

        // 2. Sub-bass (octave below fundamental) - very important for bass
        let sub_bass = (bass_phase * 0.5).sin() * 0.8;

        // 3. Harmonic series for thickness and richness
        let harmonic2 = (bass_phase * 2.0).sin() * 0.4; // Octave above
        let harmonic3 = (bass_phase * 3.0).sin() * 0.25; // Fifth above octave
        let harmonic4 = (bass_phase * 4.0).sin() * 0.15; // Two octaves above

        // 4. Very low sub-harmonic for rumble (two octaves below)
        let sub_harmonic = (bass_phase * 0.25).sin() * 0.6;

        // 5. Slow LFO for movement and life
        let lfo_rate = 0.1; // Very slow modulation
        let lfo = (time * lfo_rate * 2.0 * PI).sin();
        let lfo_mod = lfo * 0.05 + 1.0; // Subtle amplitude modulation

        // 6. Slight pitch modulation for analog character
        let pitch_lfo = (time * 0.08 * 2.0 * PI).sin() * 0.002; // Very subtle pitch variation
        let modulated_fundamental = (bass_phase * (1.0 + pitch_lfo)).sin() * 0.3;

        // 7. Mix all components with emphasis on low end
        let bass_mix = fundamental
            + sub_bass
            + sub_harmonic
            + harmonic2 * 0.8
            + harmonic3 * 0.6
            + harmonic4 * 0.4
            + modulated_fundamental;

        // Apply LFO modulation
        let modulated = bass_mix * lfo_mod;

        // 8. Gentle saturation for analog warmth (not too much distortion)
        let saturated = modulated.tanh() * 0.9;

        // 9. Final amplitude scaling - bass should be powerful but not overwhelming
        saturated * 0.7
    }
}

impl std::str::FromStr for Waveform {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "natural" => Ok(Waveform::Natural),
            "electronic" | "sine" => Ok(Waveform::Electronic),
            "saw" | "sawtooth" => Ok(Waveform::Saw),
            "square" => Ok(Waveform::Square),
            "cyberpunk" => Ok(Waveform::Cyberpunk),
            "triangle" => Ok(Waveform::Triangle),
            "fart" => Ok(Waveform::Fart),
            "bass" => Ok(Waveform::Bass),
            _ => Err(format!("Unknown waveform: {}", s)),
        }
    }
}

impl std::fmt::Display for Waveform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Waveform::Natural => "natural",
            Waveform::Electronic => "electronic",
            Waveform::Saw => "saw",
            Waveform::Square => "square",
            Waveform::Cyberpunk => "cyberpunk",
            Waveform::Triangle => "triangle",
            Waveform::Fart => "fart",
            Waveform::Bass => "bass",
        };
        write!(f, "{}", name)
    }
}

/// Get all available waveforms with descriptions
pub fn get_all_waveforms() -> Vec<(&'static str, &'static str)> {
    vec![
        ("natural", "Piano-like with harmonics"),
        ("electronic", "Clean sine wave"),
        ("saw", "Bright sawtooth wave for electronic music"),
        ("square", "Retro 8-bit square wave"),
        ("cyberpunk", "Blade Runner 2049 style analog synthesizer"),
        ("triangle", "Smooth triangular wave"),
        ("fart", "Realistic fart sound synthesis"),
        ("bass", "Deep bass with rich low frequencies"),
    ]
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
            Waveform::Triangle,
            Waveform::Fart,
            Waveform::Bass,
        ];

        for waveform in waveforms {
            let sample = waveform.generate_sample(0.25, 440.0, 44100.0);
            assert!(
                sample.is_finite(),
                "Waveform {:?} produced invalid sample: {}",
                waveform,
                sample
            );
            assert!(
                sample >= -5.0 && sample <= 5.0,
                "Waveform {:?} sample out of reasonable range: {}",
                waveform,
                sample
            );
        }
    }

    #[test]
    fn test_waveform_parsing() {
        // Test valid waveforms
        assert_eq!("natural".parse::<Waveform>().unwrap(), Waveform::Natural);
        assert_eq!(
            "electronic".parse::<Waveform>().unwrap(),
            Waveform::Electronic
        );
        assert_eq!("sine".parse::<Waveform>().unwrap(), Waveform::Electronic); // Alias
        assert_eq!(
            "cyberpunk".parse::<Waveform>().unwrap(),
            Waveform::Cyberpunk
        );
        assert_eq!("saw".parse::<Waveform>().unwrap(), Waveform::Saw);
        assert_eq!("sawtooth".parse::<Waveform>().unwrap(), Waveform::Saw); // Alias
        assert_eq!("square".parse::<Waveform>().unwrap(), Waveform::Square);
        assert_eq!("triangle".parse::<Waveform>().unwrap(), Waveform::Triangle);
        assert_eq!("fart".parse::<Waveform>().unwrap(), Waveform::Fart);
        assert_eq!("bass".parse::<Waveform>().unwrap(), Waveform::Bass);

        // Test case insensitive
        assert_eq!("NATURAL".parse::<Waveform>().unwrap(), Waveform::Natural);
        assert_eq!(
            "Electronic".parse::<Waveform>().unwrap(),
            Waveform::Electronic
        );

        // Test invalid waveforms
        assert!("invalid".parse::<Waveform>().is_err());
        assert!("".parse::<Waveform>().is_err());
        assert!("organ".parse::<Waveform>().is_err()); // Removed waveform
    }

    #[test]
    fn test_adsr_params() {
        let waveforms = vec![
            Waveform::Natural,
            Waveform::Electronic,
            Waveform::Saw,
            Waveform::Square,
            Waveform::Cyberpunk,
            Waveform::Triangle,
            Waveform::Fart,
            Waveform::Bass,
        ];

        for waveform in waveforms {
            let (attack, decay, sustain, release) = waveform.get_adsr_params();

            // All ADSR parameters should be non-negative
            assert!(
                attack >= 0.0,
                "Attack should be non-negative for {:?}: {}",
                waveform,
                attack
            );
            assert!(
                decay >= 0.0,
                "Decay should be non-negative for {:?}: {}",
                waveform,
                decay
            );
            assert!(
                release >= 0.0,
                "Release should be non-negative for {:?}: {}",
                waveform,
                release
            );

            // Sustain should be between 0 and 1
            assert!(
                sustain >= 0.0 && sustain <= 1.0,
                "Sustain should be 0-1 for {:?}: {}",
                waveform,
                sustain
            );

            // Reasonable limits for timing parameters
            assert!(
                attack <= 2.0,
                "Attack too long for {:?}: {}",
                waveform,
                attack
            );
            assert!(decay <= 2.0, "Decay too long for {:?}: {}", waveform, decay);
            assert!(
                release <= 3.0,
                "Release too long for {:?}: {}",
                waveform,
                release
            );
        }
    }

    #[test]
    fn test_bass_waveform_characteristics() {
        let waveform = Waveform::Bass;

        // Test that bass produces low-frequency content
        let sample_low = waveform.generate_sample(0.25, 55.0, 44100.0); // A1
        let sample_mid = waveform.generate_sample(0.25, 440.0, 44100.0); // A4

        // Both should be finite and reasonable
        assert!(sample_low.is_finite());
        assert!(sample_mid.is_finite());
        assert!(sample_low.abs() <= 3.0);
        assert!(sample_mid.abs() <= 3.0);

        // Bass should have good ADSR for sustained notes
        let (attack, _decay, sustain, _release) = waveform.get_adsr_params();
        assert!(sustain >= 0.8, "Bass should have high sustain: {}", sustain);
        assert!(
            attack >= 0.01,
            "Bass should have some attack time: {}",
            attack
        );
    }

    #[test]
    fn test_fart_waveform_frequency_limiting() {
        let waveform = Waveform::Fart;

        // Fart should work with various input frequencies but focus on low frequencies
        let high_freq_sample = waveform.generate_sample(0.25, 2000.0, 44100.0);
        let low_freq_sample = waveform.generate_sample(0.25, 80.0, 44100.0);

        assert!(high_freq_sample.is_finite());
        assert!(low_freq_sample.is_finite());

        // Should produce reasonable output even with high input frequency
        assert!(high_freq_sample.abs() <= 2.0);
        assert!(low_freq_sample.abs() <= 2.0);
    }

    #[test]
    fn test_cyberpunk_complexity() {
        let waveform = Waveform::Cyberpunk;

        // Test multiple phases to ensure variation
        let samples: Vec<f32> = (0..8)
            .map(|i| waveform.generate_sample(i as f32 * 0.125, 440.0, 44100.0))
            .collect();

        // All samples should be finite
        for (i, sample) in samples.iter().enumerate() {
            assert!(sample.is_finite(), "Sample {} is not finite: {}", i, sample);
        }

        // Cyberpunk should have variation (not all samples identical)
        let first_sample = samples[0];
        let has_variation = samples.iter().any(|&s| (s - first_sample).abs() > 0.001);
        assert!(
            has_variation,
            "Cyberpunk waveform should have variation between phases"
        );
    }
}
