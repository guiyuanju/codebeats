//! CodeBeats Core Library
//!
//! This library provides the core functionality for CodeBeats, including:
//! - Audio engine and synthesis
//! - Keyboard mapping and configuration
//! - Waveform generation
//! - Real-time audio processing

pub mod audio_engine;
pub mod audio_samples;
pub mod embedded_configs;
pub mod keyboard_config;
pub mod keyboard_mapping;
pub mod sequence_detector;
pub mod waveforms;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use device_query::{DeviceQuery, DeviceState, Keycode};
use keyboard_mapping::{
    KeyboardStateTracker, VirtualKeycode, get_frequency_and_volume_with_config_virtual,
};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Configuration for the CodeBeats engine
#[derive(Debug, Clone)]
pub struct CodeBeatsConfig {
    pub waveform: waveforms::Waveform,
    pub keyboard_config: keyboard_config::KeyboardConfig,
    pub volume: f32,
    pub filter_cutoff: f32,
    pub verbose: bool,
}

impl Default for CodeBeatsConfig {
    fn default() -> Self {
        Self {
            waveform: waveforms::Waveform::Electronic,
            keyboard_config: keyboard_config::KeyboardConfig::default(),
            volume: 1.0,
            filter_cutoff: 1200.0,
            verbose: false,
        }
    }
}

/// The main CodeBeats engine
pub struct CodeBeatsEngine {
    audio_state: Arc<Mutex<audio_engine::AudioState>>,
    device_state: DeviceState,
    keyboard_tracker: KeyboardStateTracker,
    sequence_detector: sequence_detector::SequenceDetector,
    config: CodeBeatsConfig,
    is_running: bool,
    _stream: Box<dyn std::any::Any + Send>, // Keep the audio stream alive
    log_callback: Option<Arc<Mutex<dyn FnMut(&str) + Send>>>,
}

impl CodeBeatsEngine {
    /// Create a new CodeBeats engine with the given configuration
    pub fn new(config: CodeBeatsConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let audio_state = Self::setup_audio(&config)?;
        let device_state = DeviceState::new();
        let keyboard_tracker = KeyboardStateTracker::new();
        let sequence_detector = sequence_detector::SequenceDetector::new();

        Ok(Self {
            audio_state,
            device_state,
            keyboard_tracker,
            sequence_detector,
            config,
            is_running: false,
            _stream: Box::new(()), // Will be replaced with actual stream
            log_callback: None,
        })
    }

    /// Set a callback function for logging
    pub fn set_log_callback<F>(&mut self, callback: F)
    where
        F: FnMut(&str) + Send + 'static,
    {
        self.log_callback = Some(Arc::new(Mutex::new(callback)));
    }

    /// Log a message either to the callback or to stdout
    fn log(&self, message: &str) {
        if let Some(ref callback) = self.log_callback {
            if let Ok(mut cb) = callback.lock() {
                cb(message);
            }
        } else {
            println!("{}", message);
        }
    }

    /// Setup audio system and return audio state
    fn setup_audio(
        config: &CodeBeatsConfig,
    ) -> Result<Arc<Mutex<audio_engine::AudioState>>, Box<dyn std::error::Error>> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or("No output device available")?;
        let device_config = device.default_output_config()?;

        let sample_rate = device_config.sample_rate().0 as f32;
        let audio_state = audio_engine::AudioState::new(
            sample_rate,
            config.waveform.clone(),
            config.volume,
            config.filter_cutoff,
        );
        let audio_state = Arc::new(Mutex::new(audio_state));
        let audio_state_clone = audio_state.clone();

        let stream = device.build_output_stream(
            &device_config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let mut state = audio_state_clone.lock().unwrap();
                for sample in data.iter_mut() {
                    *sample = state.generate_sample();
                }
            },
            |err| eprintln!("Audio stream error: {}", err),
            None,
        )?;

        stream.play()?;

        // Keep the stream alive by leaking it (simpler than managing lifetime)
        std::mem::forget(stream);

        Ok(audio_state)
    }

    /// Start the CodeBeats engine
    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_running {
            return Ok(());
        }

        self.is_running = true;
        if self.config.verbose {
            self.log(&format!(
                "🎵 CodeBeats started - {} ({})",
                self.config.keyboard_config.description, self.config.waveform
            ));
            self.log(&format!(
                "🔊 Audio settings: volume={:.1}, filter={:.0}Hz",
                self.config.volume, self.config.filter_cutoff
            ));
            self.log("💡 Easter egg hint: Try typing 'oppokokoppokosuttenten' for a surprise! 🎉");
        }

        Ok(())
    }

    /// Stop the CodeBeats engine
    pub fn stop(&mut self) {
        self.is_running = false;
        if self.config.verbose {
            self.log("🔇 CodeBeats stopped");
        }
    }

    /// Check if the engine is running
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    /// Process one iteration of keyboard input
    /// Returns true if the engine should continue running
    pub fn process_input(&mut self) -> bool {
        if !self.is_running {
            return false;
        }

        let current_keys: Vec<Keycode> = self.device_state.get_keys();

        // Get the previous keys for comparison
        let prev_keys = self.keyboard_tracker.get_current_keys();

        // Find newly pressed keys
        let pressed_keys: Vec<Keycode> = current_keys
            .iter()
            .filter(|key| !prev_keys.contains(key))
            .copied()
            .collect();

        // Find newly released keys
        let released_keys: Vec<Keycode> = prev_keys
            .iter()
            .filter(|key| !current_keys.contains(key))
            .copied()
            .collect();

        // Update keyboard state tracker
        self.keyboard_tracker.update(&pressed_keys, &released_keys);

        // Handle pressed keys
        for key in pressed_keys {
            // Check for Easter egg sequence
            if self.sequence_detector.process_input(key) {
                self.trigger_easter_egg();
            }

            if let Some(virtual_key) = self.keyboard_tracker.get_virtual_keycode_for_press(key) {
                self.handle_key_press(&virtual_key);
            }
        }

        // Handle released keys
        for key in released_keys {
            if let Some(virtual_key) = self.keyboard_tracker.get_virtual_keycode_for_release(key) {
                self.handle_key_release(&virtual_key);
            }
        }

        true
    }

    /// Handle a key press event
    fn handle_key_press(&self, virtual_key: &VirtualKeycode) {
        let key_id = virtual_key.to_string();

        if let Some((frequency, volume, note)) =
            get_frequency_and_volume_with_config_virtual(virtual_key, &self.config.keyboard_config)
        {
            let mut state = self.audio_state.lock().unwrap();
            let actual_volume = state.start_note_with_id(&key_id, frequency, volume);

            if self.config.verbose {
                self.log(&format!(
                    "🎵 Key: {} → {} ({:.1}Hz, vol: {:.2})",
                    key_id, note, frequency, actual_volume
                ));
            }
        } else if self.config.verbose {
            self.log(&format!("⚪ Key: {} (unmapped)", key_id));
        }
    }

    /// Handle a key release event
    fn handle_key_release(&self, virtual_key: &VirtualKeycode) {
        if let Some((_, _, note)) =
            get_frequency_and_volume_with_config_virtual(virtual_key, &self.config.keyboard_config)
        {
            let mut state = self.audio_state.lock().unwrap();
            let key_id = virtual_key.to_string();
            state.stop_note_with_id(&key_id);

            if self.config.verbose {
                self.log(&format!("🔇 Key: {} → {} (released)", key_id, note));
            }
        }
    }

    /// Trigger the Easter egg fart sound
    fn trigger_easter_egg(&self) {
        if self.config.verbose {
            self.log("🎉 Easter egg triggered: おっぽこ　こっぽこ　すってんてん! 💨");
        }

        let mut state = self.audio_state.lock().unwrap();
        if let Some(fart_sample) = state.get_fart_sample() {
            let playback = audio_samples::SamplePlayback::new(
                fart_sample.clone(),
                state.get_global_time(),
                0.7, // Easter egg volume
            );
            state.add_sample_playback(playback);
        } else if self.config.verbose {
            self.log("⚠️ Fart sample not available for Easter egg");
        }
    }

    /// Run the engine in a blocking loop (for CLI usage)
    pub fn run_blocking(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.start()?;

        self.log("Press Ctrl+C to exit");

        // Main loop
        loop {
            if !self.process_input() {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }

        self.stop();
        Ok(())
    }

    /// Update the engine configuration
    pub fn update_config(
        &mut self,
        new_config: CodeBeatsConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Update audio state with new settings
        if let Ok(mut state) = self.audio_state.lock() {
            state.set_waveform(new_config.waveform.clone());
            state.set_volume(new_config.volume);
        }

        self.config = new_config;
        Ok(())
    }

    /// Get current configuration
    pub fn get_config(&self) -> &CodeBeatsConfig {
        &self.config
    }

    /// Load keyboard configuration from file
    pub fn load_keyboard_config(
        path: &str,
    ) -> Result<keyboard_config::KeyboardConfig, Box<dyn std::error::Error>> {
        keyboard_config::KeyboardConfig::load_from_file(path).map_err(|e| e.into())
    }

    /// Get available language configurations
    pub fn discover_language_configs() -> Vec<String> {
        let mut configs = Vec::new();

        let mut possible_dirs = vec!["language_configs".to_string()];

        // For macOS app bundles, add executable-relative paths
        if cfg!(target_os = "macos") {
            if let Ok(exe_path) = std::env::current_exe() {
                if let Some(exe_dir) = exe_path.parent() {
                    // Contents/MacOS/ -> Contents/Resources/language_configs
                    if let Some(contents_dir) = exe_dir.parent() {
                        let resources_config =
                            contents_dir.join("Resources").join("language_configs");
                        if let Some(path_str) = resources_config.to_str() {
                            possible_dirs.insert(0, path_str.to_string());
                        }
                    }
                }
            }
        }

        // Add more fallback paths
        possible_dirs.push("Contents/Resources/language_configs".to_string());
        possible_dirs.push("../Resources/language_configs".to_string());

        for dir in possible_dirs {
            if let Ok(entries) = std::fs::read_dir(&dir) {
                for entry in entries.flatten() {
                    if let Some(path) = entry.path().to_str() {
                        if path.ends_with(".json") {
                            if let Some(name) = entry.path().file_stem() {
                                if let Some(name_str) = name.to_str() {
                                    if !configs.contains(&name_str.to_string()) {
                                        configs.push(name_str.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
                break; // Use first directory that exists
            }
        }

        configs.sort();
        configs
    }
}

// Re-export commonly used types for external use
pub use audio_engine::AudioState;
pub use keyboard_config::KeyboardConfig;
pub use waveforms::Waveform;
