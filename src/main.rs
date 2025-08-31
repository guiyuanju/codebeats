//! CodeBeats - Programming Music Simulator
//!
//! Transform your coding workflow into a harmonious musical experience.
//! Every keystroke becomes a note, creating beautiful melodies while you code.
//!
//! Features:
//! - Real-time polyphonic audio synthesis
//! - Multiple waveforms (Electronic default, Natural piano, Saw, Square, Cyberpunk)
//! - Programming-optimized keyboard mapping based on key frequency analysis
//! - ADSR envelope system for natural sound transitions
//! - Command-line waveform selection (no runtime switching)
//! - Real-time waveform switching with function keys

mod audio;
mod keyboard;
mod scale_comparison;
mod waveform;

use audio::AudioState;
use clap::{Parser, Subcommand};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use device_query::{DeviceQuery, DeviceState, Keycode};
use keyboard::{
    get_frequency_and_volume_with_config_virtual, KeyboardConfig, KeyboardStateTracker,
    VirtualKeycode,
};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use waveform::Waveform;

#[derive(Parser)]
#[command(
    name = "codebeats",
    about = "Programming Music Simulator",
    version = "0.1.0"
)]
struct Cli {
    #[arg(short, long)]
    waveform: Option<String>,

    #[arg(short = 'l', long = "language")]
    language_config: Option<String>,

    #[arg(short, long)]
    config: Option<String>,

    #[arg(
        short = 'v',
        long = "volume",
        value_name = "LEVEL",
        help = "Master volume (0.0-1.0)"
    )]
    volume: Option<f32>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    CompareScales,
}

struct AppConfig {
    initial_waveform: Waveform,
    sample_rate: f32,
    keyboard_config: KeyboardConfig,
    master_volume: f32,
}

impl AppConfig {
    fn from_cli(cli: &Cli) -> Result<Self, String> {
        let master_volume = cli.volume.unwrap_or(1.0).clamp(0.0, 1.0);

        let keyboard_config = Self::load_keyboard_config(cli);

        // Determine waveform: CLI arg > language config > default
        let initial_waveform = if let Some(waveform_str) = &cli.waveform {
            // CLI argument takes highest priority
            Waveform::from_str(waveform_str).unwrap_or_else(|| {
                println!(
                    "Invalid waveform '{}', using default: electronic",
                    waveform_str
                );
                Waveform::Electronic
            })
        } else if let Some(config_waveform) = keyboard_config.get_waveform() {
            // Use waveform from language config
            println!("Using waveform from config: {:?}", config_waveform);
            config_waveform
        } else {
            // Default fallback
            Waveform::Electronic
        };

        Ok(Self {
            initial_waveform,
            sample_rate: 44100.0,
            keyboard_config,
            master_volume,
        })
    }

    fn load_keyboard_config(cli: &Cli) -> KeyboardConfig {
        let config_path = cli.config.as_deref().or(cli.language_config.as_deref());

        if let Some(path) = config_path {
            match KeyboardConfig::load_from_file(path) {
                Ok(config) => {
                    println!("✅ Loaded: {}", path);
                    return config;
                }
                Err(_) => {
                    println!("❌ Could not load: {}", path);
                }
            }
        } else if let Ok(config) =
            KeyboardConfig::load_from_file("language_configs/general_programming_language.json")
        {
            return config;
        }

        println!("📝 Using default configuration");
        KeyboardConfig::default()
    }
}

/// Audio system manager
struct AudioSystem {
    #[allow(dead_code)]
    stream: cpal::Stream,
    state: Arc<Mutex<AudioState>>,
}

impl AudioSystem {
    /// Initialize audio system with given configuration
    fn new(config: &mut AppConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or("No output device available")?;
        let device_config = device.default_output_config()?;

        // Update config with actual sample rate
        config.sample_rate = device_config.sample_rate().0 as f32;

        let audio_state = AudioState::new(
            config.sample_rate,
            config.initial_waveform,
            config.master_volume,
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

        Ok(Self {
            stream,
            state: audio_state,
        })
    }

    /// Get reference to audio state for external control
    fn state(&self) -> &Arc<Mutex<AudioState>> {
        &self.state
    }
}

/// User interface and display manager
struct UIManager;

impl UIManager {
    fn show_welcome(waveform: Waveform, config: &KeyboardConfig) {
        println!("🎵 CodeBeats ({:?}) - {}", waveform, config.description);
        println!("Press Ctrl+C to exit");
    }
}

/// Keyboard input processor with shift detection
pub struct KeyboardProcessor {
    pub device_state: DeviceState,
    prev_keys: Vec<Keycode>,
    keyboard_tracker: KeyboardStateTracker,
}

impl KeyboardProcessor {
    /// Create new keyboard processor
    fn new() -> Self {
        Self {
            device_state: DeviceState::new(),
            prev_keys: Vec::new(),
            keyboard_tracker: KeyboardStateTracker::new(),
        }
    }

    /// Process keyboard input for one frame
    /// Returns lists of newly pressed and released virtual keycodes
    fn process_input(&mut self) -> (Vec<VirtualKeycode>, Vec<VirtualKeycode>) {
        let current_keys: Vec<Keycode> = self.device_state.get_keys();

        // Find newly pressed keys
        let pressed_keys: Vec<Keycode> = current_keys
            .iter()
            .filter(|key| !self.prev_keys.contains(key))
            .cloned()
            .collect();

        // Find newly released keys
        let released_keys: Vec<Keycode> = self
            .prev_keys
            .iter()
            .filter(|key| !current_keys.contains(key))
            .cloned()
            .collect();

        // Update keyboard state tracker
        self.keyboard_tracker.update(&pressed_keys, &released_keys);

        // Convert physical keys to virtual keys (handles shift detection)
        let virtual_pressed: Vec<VirtualKeycode> = pressed_keys
            .iter()
            .filter_map(|key| self.keyboard_tracker.get_virtual_keycode(*key))
            .collect();

        let virtual_released: Vec<VirtualKeycode> = released_keys
            .iter()
            .filter_map(|key| self.keyboard_tracker.get_virtual_keycode(*key))
            .collect();

        self.prev_keys = current_keys;

        (virtual_pressed, virtual_released)
    }
}

/// Main application controller
struct PianoApp {
    audio_system: AudioSystem,
    keyboard_processor: KeyboardProcessor,
}

impl PianoApp {
    /// Create and initialize the piano application
    fn new(config: AppConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let mut app_config = config;
        let audio_system = AudioSystem::new(&mut app_config)?;
        let keyboard_processor = KeyboardProcessor::new();

        // Show welcome screen
        UIManager::show_welcome(app_config.initial_waveform, &app_config.keyboard_config);

        Ok(Self {
            audio_system,
            keyboard_processor,
        })
    }

    /// Handle a pressed virtual key using the configured keyboard mapping
    fn handle_key_press(&self, virtual_key: &VirtualKeycode, config: &KeyboardConfig) {
        let key_id = virtual_key.to_string();

        // Handle musical keys using custom configuration
        if let Some((frequency, volume, note)) =
            get_frequency_and_volume_with_config_virtual(virtual_key, config)
        {
            let mut audio_state = self.audio_system.state().lock().unwrap();
            let actual_volume = audio_state.start_note_with_id(&key_id, frequency, volume);

            if actual_volume > 0.0 {
                if actual_volume < volume {
                    println!(
                        "Playing: {} -> {} ({:.2} Hz, vol: {:.2}) [Rate Limited]",
                        key_id, note, frequency, actual_volume
                    );
                } else {
                    println!(
                        "Playing: {} -> {} ({:.2} Hz, vol: {:.2})",
                        key_id, note, frequency, actual_volume
                    );
                }
            } else {
                println!("Silenced: {} -> {} (too rapid)", key_id, note);
            }
        } else {
            // Check if this might be an unmapped key
            println!("🔍 Detected unmapped key: {}", virtual_key.to_string());
            println!("💡 Add it to keyboard_config.json to assign a sound");
        }
    }

    /// Handle a released virtual key using the configured keyboard mapping
    fn handle_key_release(&self, virtual_key: &VirtualKeycode, config: &KeyboardConfig) {
        if let Some((_, _, note)) =
            get_frequency_and_volume_with_config_virtual(virtual_key, config)
        {
            let mut audio_state = self.audio_system.state().lock().unwrap();
            let key_id = virtual_key.to_string();
            audio_state.stop_note_with_id(&key_id);
            println!("Stopped: {} -> {}", key_id, note);
        }
    }

    /// Main application loop
    fn run(&mut self, config: &KeyboardConfig) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let (pressed_keys, released_keys) = self.keyboard_processor.process_input();

            for virtual_key in pressed_keys {
                self.handle_key_press(&virtual_key, config);
            }

            for virtual_key in released_keys {
                self.handle_key_release(&virtual_key, config);
            }

            thread::sleep(Duration::from_millis(10));
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Some(command) = &cli.command {
        match command {
            Commands::CompareScales => {
                return scale_comparison::demonstrate_scale_system();
            }
        }
    }

    let config = AppConfig::from_cli(&cli)?;
    let keyboard_config = config.keyboard_config.clone();

    let mut app = PianoApp::new(config)?;
    app.run(&keyboard_config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_config_default() {
        let cli = Cli::parse_from(&["codebeats"]);
        let config = AppConfig::from_cli(&cli).unwrap();
        assert_eq!(config.initial_waveform, Waveform::Electronic);
    }

    #[test]
    fn test_keyboard_processor() {
        let mut processor = KeyboardProcessor::new();

        // First call should return empty vectors (no previous state)
        let (pressed, released) = processor.process_input();
        assert!(pressed.is_empty());
        assert!(released.is_empty());
    }

    #[test]
    fn test_waveform_parsing() {
        // Test valid waveforms
        assert_eq!(Waveform::from_str("natural"), Some(Waveform::Natural));
        assert_eq!(Waveform::from_str("electronic"), Some(Waveform::Electronic));
        assert_eq!(Waveform::from_str("cyberpunk"), Some(Waveform::Cyberpunk));

        // Test invalid waveform
        assert_eq!(Waveform::from_str("invalid"), None);
    }
}
