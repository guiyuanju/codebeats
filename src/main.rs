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
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use device_query::{DeviceQuery, DeviceState, Keycode};
use keyboard::{get_frequency_and_volume_with_config, KeyboardConfig};
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use waveform::Waveform;

/// Application configuration and setup
struct AppConfig {
    initial_waveform: Waveform,
    sample_rate: f32,
    keyboard_config: KeyboardConfig,
}

impl AppConfig {
    /// Create configuration from command line arguments
    fn from_args() -> Result<Self, String> {
        let args: Vec<String> = env::args().collect();

        let initial_waveform = if args.len() > 1 && args[1] != "config" {
            Waveform::from_str(&args[1]).unwrap_or_else(|| {
                println!("Available waveforms: natural, electronic, saw, square, cyberpunk");
                println!("Using default: electronic");
                Waveform::Electronic
            })
        } else {
            Waveform::Electronic
        };

        // Try to load custom keyboard config, fallback to default
        let keyboard_config = Self::load_keyboard_config(&args);

        Ok(Self {
            initial_waveform,
            sample_rate: 44100.0, // Will be updated with actual device sample rate
            keyboard_config,
        })
    }

    /// Load keyboard configuration from file or create default
    fn load_keyboard_config(args: &[String]) -> KeyboardConfig {
        // Check for config file argument
        let config_path = if args.len() > 2 && args[1] == "config" {
            &args[2]
        } else {
            "keyboard_config.json"
        };

        match KeyboardConfig::load_from_file(config_path) {
            Ok(config) => {
                if config_path != "keyboard_config.json" {
                    println!(
                        "✅ Loaded custom keyboard configuration from {}",
                        config_path
                    );
                }
                config
            }
            Err(_) => {
                if config_path != "keyboard_config.json" {
                    println!("❌ Could not load config file: {}", config_path);
                    println!("📝 Using default keyboard configuration");
                } else {
                    println!("📝 Using default keyboard configuration");
                    println!("💡 To customize, run: cargo run -- generate-config");
                }
                KeyboardConfig::default()
            }
        }
    }

    /// Generate and save default keyboard configuration file
    fn generate_config_file() -> Result<(), Box<dyn std::error::Error>> {
        let config = KeyboardConfig::default();
        config.save_to_file("keyboard_config.json")?;
        println!("✅ Generated keyboard_config.json with default settings");
        println!("📝 Edit this file to customize your keyboard mappings");
        println!("🎵 Each key can be mapped to any musical note (e.g., 'C4', 'F#5', 'Bb3')");
        println!("🔊 Volume ranges from 0.0 (silent) to 1.0 (full volume)");
        Ok(())
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

        let audio_state = AudioState::new(config.sample_rate, config.initial_waveform);
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
    /// Show welcome message and instructions
    fn show_welcome(waveform: Waveform, config: &KeyboardConfig) {
        println!(
            "🎵 CodeBeats - Programming Music Simulator ({:?} Mode)",
            waveform
        );
        println!(
            "📝 Keyboard Config: {} (v{})",
            config.description, config.version
        );
        println!();

        Self::show_macos_permissions_info();
        Self::show_command_key_info();
        Self::show_keyboard_layout(config);
        Self::show_usage_info();
    }

    /// Show Mac Command key detection info
    fn show_command_key_info() {
        println!("🔍 Mac Command key support: Auto-detected when pressed");
        println!("⚡ Rate limiting enabled: Rapid key presses (like vim 'jjjj') will be quieted");
        println!("📝 Volume shown in terminal reflects actual rate-limited volume");
        println!();
    }

    /// Show macOS accessibility permissions information
    fn show_macos_permissions_info() {
        println!("IMPORTANT - macOS Permission Required:");
        println!("If this fails, you need to grant accessibility permissions:");
        println!("1. Go to: System Preferences > Security & Privacy > Privacy > Accessibility");
        println!("2. Click the lock and enter your password");
        println!("3. Add your Terminal app (Terminal.app, iTerm2, etc.)");
        println!("4. Restart this program");
        println!();
    }

    /// Display keyboard layout and mapping
    fn show_keyboard_layout(config: &KeyboardConfig) {
        println!("Piano keys (works from ANY window once permissions are granted):");
        println!();
        println!("┌─────────────────────────────────────────────────────────────────┐");
        println!("│                    CURRENT KEYBOARD LAYOUT                     │");
        println!("├─────────────────────────────────────────────────────────────────┤");
        println!("│ 📝 Configuration: {}              │", config.description);
        println!("│ 🔧 Customize: Edit keyboard_config.json to change mappings     │");
        println!("│ 🎵 Generate: cargo run -- generate-config                      │");
        println!("│                                                                 │");
        println!("│ Sample mappings from current config:                           │");

        // Show some example mappings from the current config
        let examples = [
            ("A", "Most common letter"),
            ("E", "Very common letter"),
            ("Space", "Common key"),
            ("Enter", "Common key"),
            ("F1", "Function key"),
        ];

        for (key_name, description) in examples {
            if let Some(mapping) = config.mappings.get(key_name) {
                println!(
                    "│   {}={} (vol: {:.2}) - {}        │",
                    key_name.to_uppercase().chars().next().unwrap_or('?'),
                    mapping.note,
                    mapping.volume,
                    description
                );
            }
        }

        println!(
            "│   ... and {} more keys configured                         │",
            config.mappings.len() - 5
        );
        println!("│                                                                 │");
        println!("│ 🎼 Creates pleasant harmonies based on your configuration      │");
        println!("│ 🔇 Volume levels optimized to avoid disrupting concentration   │");
        println!("└─────────────────────────────────────────────────────────────────┘");
        println!();
    }

    /// Show command line usage information
    fn show_usage_info() {
        println!("💡 Command line options:");
        println!("   cargo run natural        # Start with natural piano");
        println!("   cargo run electronic     # Start with electronic");
        println!("   cargo run saw            # Start with saw wave");
        println!("   cargo run square         # Start with square wave");
        println!("   cargo run cyberpunk      # Start with Blade Runner 2049 style");
        println!("   cargo run -- generate-config  # Generate keyboard_config.json");
        println!("   cargo run config <file>  # Use specific config file");
        println!();
        println!("💡 Example configurations:");
        println!("   cargo run config example_configs/piano_layout.json");
        println!("   cargo run config example_configs/minimal.json");
        println!();
        println!("🎵 CodeBeats: Turn your coding into music!");
        println!("🎵 Waveform is fixed at startup - no runtime switching");
        println!("📝 Edit keyboard_config.json to customize key mappings");
        println!("Press Ctrl+C to exit");
        println!();
    }
}

/// Keyboard input processor
pub struct KeyboardProcessor {
    pub device_state: DeviceState,
    prev_keys: Vec<Keycode>,
}

impl KeyboardProcessor {
    /// Create new keyboard processor
    fn new() -> Self {
        Self {
            device_state: DeviceState::new(),
            prev_keys: Vec::new(),
        }
    }

    /// Process keyboard input for one frame
    /// Returns lists of newly pressed and released keys
    fn process_input(&mut self) -> (Vec<Keycode>, Vec<Keycode>) {
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

        self.prev_keys = current_keys;

        (pressed_keys, released_keys)
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

    /// Handle a pressed key using the configured keyboard mapping
    fn handle_key_press(&self, key: Keycode, config: &KeyboardConfig) {
        // Handle musical keys using custom configuration
        if let Some((frequency, volume, note)) = get_frequency_and_volume_with_config(key, config) {
            let mut audio_state = self.audio_system.state().lock().unwrap();
            let actual_volume = audio_state.start_note(key, frequency, volume);

            if actual_volume > 0.0 {
                if actual_volume < volume {
                    println!(
                        "Playing: {:?} -> {} ({:.2} Hz, vol: {:.2}) [Rate Limited]",
                        key, note, frequency, actual_volume
                    );
                } else {
                    println!(
                        "Playing: {:?} -> {} ({:.2} Hz, vol: {:.2})",
                        key, note, frequency, actual_volume
                    );
                }
            } else {
                println!("Silenced: {:?} -> {} (too rapid)", key, note);
            }
        } else {
            // Check if this might be a Command key we want to detect
            let key_debug = format!("{:?}", key);
            if key_debug.contains("Meta")
                || key_debug.contains("Cmd")
                || key_debug.contains("Command")
                || key_debug.contains("LWin")
                || key_debug.contains("RWin")
            {
                println!("🔍 Detected potential Command key: {:?}", key);
                println!("   (This key is not currently mapped to a musical note)");
                println!("💡 Add it to keyboard_config.json to assign a sound");
            }
        }
    }

    /// Handle a released key using the configured keyboard mapping
    fn handle_key_release(&self, key: Keycode, config: &KeyboardConfig) {
        if let Some((_, _, note)) = get_frequency_and_volume_with_config(key, config) {
            let mut audio_state = self.audio_system.state().lock().unwrap();
            audio_state.stop_note(key);
            println!("Stopped: {:?} -> {}", key, note);
        }
    }

    /// Main application loop
    fn run(&mut self, config: &KeyboardConfig) -> Result<(), Box<dyn std::error::Error>> {
        let mut last_hold_check = Instant::now();

        loop {
            let (pressed_keys, released_keys) = self.keyboard_processor.process_input();

            // Handle pressed keys
            for key in pressed_keys {
                self.handle_key_press(key, config);
            }

            // Handle released keys
            for key in released_keys {
                self.handle_key_release(key, config);
            }

            // Check for long-held keys every second
            if last_hold_check.elapsed() > Duration::from_secs(1) {
                self.check_long_held_keys(config);
                last_hold_check = Instant::now();
            }

            // Small delay to prevent excessive CPU usage
            thread::sleep(Duration::from_millis(10));
        }
    }

    /// Check and report on keys that have been held for a long time
    fn check_long_held_keys(&self, config: &KeyboardConfig) {
        let current_keys: Vec<Keycode> = self.keyboard_processor.device_state.get_keys();
        let audio_state = self.audio_system.state().lock().unwrap();

        for key in current_keys {
            if let Some(hold_duration) = audio_state.get_hold_duration(key) {
                if hold_duration > 2.0 && hold_duration % 2.0 < 1.0 {
                    // Report every 2 seconds after the initial 2 seconds
                    if let Some((_, _, note)) = get_frequency_and_volume_with_config(key, config) {
                        // Get the actual smooth volume from the note state
                        if let Some(note_state) = audio_state.active_notes.get(&key) {
                            println!(
                                "Held: {:?} -> {} ({:.1}s, vol: {:.2})",
                                key, note, hold_duration, note_state.current_hold_volume
                            );
                        }
                    }
                }
            }
        }
    }
}

/// Application entry point
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // Handle special commands
    if args.len() > 1 && args[1] == "generate-config" {
        return AppConfig::generate_config_file();
    }

    // Handle scale comparison command
    if args.len() > 1 && args[1] == "compare-scales" {
        return scale_comparison::demonstrate_scale_system();
    }

    // Show help for config command
    if args.len() > 1 && args[1] == "config" && args.len() < 3 {
        println!("Usage: cargo run config <config_file>");
        println!("Example: cargo run config example_configs/piano_layout.json");
        println!("Special commands:");
        println!("  cargo run generate-config   - Generate default config");
        println!("  cargo run compare-scales     - Show language scale comparison");
        return Ok(());
    }

    let config = AppConfig::from_args()?;
    let keyboard_config = config.keyboard_config.clone();

    let mut app = PianoApp::new(config)?;
    app.run(&keyboard_config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_config_default() {
        let config = AppConfig::from_args().unwrap();
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
