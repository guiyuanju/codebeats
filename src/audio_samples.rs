//! Audio samples module for loading and playing WAV files
//!
//! This module handles loading audio samples from WAV files and playing them back
//! in the audio synthesis pipeline. Used for realistic sound effects like fart sounds.

use hound;
use std::path::Path;

/// Audio sample data loaded from a WAV file
#[derive(Clone, Debug)]
pub struct AudioSample {
    /// Raw PCM sample data (f32 normalized to -1.0 to 1.0)
    pub samples: Vec<f32>,
    /// Sample rate of the loaded audio
    pub sample_rate: u32,
    /// Number of channels (1 for mono, 2 for stereo)
    pub channels: u16,
}

impl AudioSample {
    /// Load an audio sample from a WAV file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let mut reader = hound::WavReader::open(path)?;
        let spec = reader.spec();

        // Read all samples and convert to f32
        let samples: Result<Vec<f32>, _> = match spec.sample_format {
            hound::SampleFormat::Float => reader.samples::<f32>().collect(),
            hound::SampleFormat::Int => {
                // Convert integer samples to float
                let bit_depth = spec.bits_per_sample;
                let max_val = (1 << (bit_depth - 1)) as f32;

                reader
                    .samples::<i32>()
                    .map(|sample| sample.map(|s| s as f32 / max_val))
                    .collect()
            }
        };

        let samples = samples?;

        Ok(AudioSample {
            samples,
            sample_rate: spec.sample_rate,
            channels: spec.channels,
        })
    }

    /// Get the duration of the sample in seconds
    pub fn duration(&self) -> f32 {
        self.samples.len() as f32 / (self.sample_rate as f32 * self.channels as f32)
    }

    /// Get the duration when played at a different sample rate (for proper timing)
    pub fn duration_at_sample_rate(&self, target_sample_rate: f32) -> f32 {
        let original_duration = self.duration();
        // If target sample rate is higher, duration should be longer to maintain original pitch
        original_duration * (target_sample_rate / self.sample_rate as f32)
    }

    /// Get a sample at a specific time position with proper sample rate conversion
    pub fn get_sample_at_time(&self, time_seconds: f32, target_sample_rate: f32) -> f32 {
        if self.samples.is_empty() {
            return 0.0;
        }

        // Convert time to sample position accounting for sample rate difference
        // If target rate is higher, we need to slow down the playback
        let time_scaling = self.sample_rate as f32 / target_sample_rate;
        let adjusted_time = time_seconds * time_scaling;
        let sample_pos = adjusted_time * self.sample_rate as f32;
        let frame_index = sample_pos as usize;

        // For stereo files, each frame has multiple samples
        let sample_index = frame_index * self.channels as usize;

        // Return 0 if we're past the end of the sample
        if sample_index >= self.samples.len() {
            return 0.0;
        }

        // For stereo, average the channels
        let current_sample = if self.channels == 2 && sample_index + 1 < self.samples.len() {
            (self.samples[sample_index] + self.samples[sample_index + 1]) * 0.5
        } else {
            self.samples[sample_index]
        };

        // Linear interpolation between frames
        let next_frame_index = frame_index + 1;
        let next_sample_index = next_frame_index * self.channels as usize;

        if next_sample_index < self.samples.len() {
            let next_sample = if self.channels == 2 && next_sample_index + 1 < self.samples.len() {
                (self.samples[next_sample_index] + self.samples[next_sample_index + 1]) * 0.5
            } else {
                self.samples[next_sample_index]
            };

            let frac = sample_pos.fract();
            current_sample + (next_sample - current_sample) * frac
        } else {
            current_sample
        }
    }

    /// Check if the sample playback is finished at a given time
    pub fn is_finished(&self, time_seconds: f32) -> bool {
        time_seconds >= self.duration()
    }

    /// Check if the sample playback is finished at a given time with sample rate conversion
    pub fn is_finished_at_sample_rate(&self, time_seconds: f32, target_sample_rate: f32) -> bool {
        time_seconds >= self.duration_at_sample_rate(target_sample_rate)
    }

    /// Get sample rate for proper resampling
    pub fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }
}

/// Sample playback state for tracking individual sample instances
#[derive(Clone, Debug)]
pub struct SamplePlayback {
    /// Reference to the audio sample data
    pub sample: AudioSample,
    /// Start time of playback
    pub start_time: f32,
    /// Volume multiplier for this playback instance
    pub volume: f32,
    /// Whether this playback instance is still active
    pub active: bool,
}

impl SamplePlayback {
    /// Create a new sample playback instance
    pub fn new(sample: AudioSample, start_time: f32, volume: f32) -> Self {
        Self {
            sample,
            start_time,
            volume,
            active: true,
        }
    }

    /// Get the current sample value for this playback instance
    pub fn get_current_sample(&self, current_time: f32, target_sample_rate: f32) -> f32 {
        if !self.active {
            return 0.0;
        }

        let elapsed = current_time - self.start_time;
        if elapsed < 0.0 {
            return 0.0;
        }

        if self
            .sample
            .is_finished_at_sample_rate(elapsed, target_sample_rate)
        {
            return 0.0;
        }

        self.sample.get_sample_at_time(elapsed, target_sample_rate) * self.volume
    }

    /// Check if this playback instance is finished
    pub fn is_finished(&self, current_time: f32, target_sample_rate: f32) -> bool {
        let elapsed = current_time - self.start_time;
        elapsed >= 0.0
            && self
                .sample
                .is_finished_at_sample_rate(elapsed, target_sample_rate)
    }

    /// Stop this playback instance
    pub fn stop(&mut self) {
        self.active = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_sample_creation() {
        let sample = AudioSample {
            samples: vec![0.0, 0.5, 1.0, 0.5, 0.0, -0.5, -1.0, -0.5],
            sample_rate: 44100,
            channels: 1,
        };

        assert_eq!(sample.channels, 1);
        assert_eq!(sample.sample_rate, 44100);
        assert!(sample.duration() > 0.0);
    }

    #[test]
    fn test_sample_playback() {
        let sample = AudioSample {
            samples: vec![1.0, 0.5, 0.0, -0.5, -1.0],
            sample_rate: 44100,
            channels: 1,
        };

        let mut playback = SamplePlayback::new(sample, 0.0, 0.8);

        // Should be active initially
        assert!(playback.active);

        // Should return sample value multiplied by volume
        let sample_val = playback.get_current_sample(0.0, 44100.0);
        assert!((sample_val - 0.8).abs() < 0.001); // 1.0 * 0.8

        // Should be able to stop
        playback.stop();
        assert!(!playback.active);
        assert_eq!(playback.get_current_sample(0.0, 44100.0), 0.0);
    }

    #[test]
    fn test_sample_interpolation() {
        let sample = AudioSample {
            samples: vec![0.0, 1.0],
            sample_rate: 2, // 2 samples per second for easy testing
            channels: 1,
        };

        // At 0.5 seconds, should be halfway between samples
        let mid_sample = sample.get_sample_at_time(0.25, 2.0);
        assert!((mid_sample - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_inspect_fart_wav() {
        if let Err(e) = inspect_wav_file("effects/fart-quick-short.wav") {
            println!("Could not inspect WAV file: {}", e);
        }
    }

    /// Debug utility to inspect WAV file parameters
    pub fn inspect_wav_file<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
        let reader = hound::WavReader::open(path)?;
        let spec = reader.spec();

        println!("WAV File Info:");
        println!("  Sample Rate: {} Hz", spec.sample_rate);
        println!("  Channels: {}", spec.channels);
        println!("  Bits per Sample: {}", spec.bits_per_sample);
        println!("  Sample Format: {:?}", spec.sample_format);

        let sample_count = reader.len();
        let duration = sample_count as f32 / (spec.sample_rate as f32 * spec.channels as f32);
        println!("  Sample Count: {}", sample_count);
        println!("  Duration: {:.3} seconds", duration);

        Ok(())
    }
}
