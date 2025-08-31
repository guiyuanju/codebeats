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
mod waveform;

use audio::AudioState;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use device_query::{DeviceQuery, DeviceState, Keycode};
use keyboard::get_frequency_and_volume;
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use waveform::Waveform;

/// Application configuration and setup
struct AppConfig {
    initial_waveform: Waveform,
    sample_rate: f32,
}

impl AppConfig {
    /// Create configuration from command line arguments
    fn from_args() -> Result<Self, String> {
        let args: Vec<String> = env::args().collect();

        let initial_waveform = if args.len() > 1 {
            Waveform::from_str(&args[1]).unwrap_or_else(|| {
                println!("Available waveforms: natural, electronic, saw, square, cyberpunk");
                println!("Using default: electronic");
                Waveform::Electronic
            })
        } else {
            Waveform::Electronic
        };

        Ok(Self {
            initial_waveform,
            sample_rate: 44100.0, // Will be updated with actual device sample rate
        })
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
    fn show_welcome(waveform: Waveform) {
        println!(
            "🎵 CodeBeats - Programming Music Simulator ({:?} Mode)",
            waveform
        );
        println!();

        Self::show_macos_permissions_info();
        Self::show_command_key_info();
        Self::show_keyboard_layout();
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
    fn show_keyboard_layout() {
        println!("Piano keys (works from ANY window once permissions are granted):");
        println!();
        println!("┌─────────────────────────────────────────────────────────────────┐");
        println!("│              PROGRAMMING-OPTIMIZED PIANO LAYOUT                │");
        println!("├─────────────────────────────────────────────────────────────────┤");
        println!("│ ⌨️  OPTIMIZED FOR CODING - Harmonic mapping based on key usage   │");
        println!("│                                                                 │");
        println!("│ 🎵 Letters (C Major Pentatonic - Pleasant & Harmonious):       │");
        println!("│   E=E4  T=G4  A=C4  O=D4  I=A4  N=E5  S=G5  H=C5  R=D5        │");
        println!("│   L=F4  U=A3  D=F5  C=B4  M=B3  F=C3  P=D3  B=E3  V=G3        │");
        println!("│   K=A5  W=F3  Y=B5  G=C6  J=D6  Q=E6  X=F6  Z=G6              │");
        println!("│                                                                 │");
        println!("│ 🔢 Numbers (Gentle Harmony - Same as common letters):          │");
        println!("│   0=C4  1=E4  2=G4  3=A4  4=D4  5=F4  6=C5  7=E5  8=G5  9=A5  │");
        println!("│                                                                 │");
        println!("│ ⚡ Symbols (Programming-friendly harmonics):                    │");
        println!("│   ;=C4  [=E4  ]=G4  ,=A4  .=D4  /=F4  \\=B4  '=C5  ==D5  -=E5   │");
        println!("│                                                                 │");
        println!("│ 🔇 Common Keys (Quiet bass - won't disrupt flow):              │");
        println!("│   SPACE=C3  BACKSPACE=G2  ENTER=C3  TAB=F2  DELETE=A2         │");
        println!("│                                                                 │");
        println!("│ 🎛️  Modifiers (Very quiet - barely audible):                   │");
        println!("│   SHIFT=C2/E2  CTRL=G2/A2  ALT=D2/F2  CAPS=B1  ESC=C2         │");
        println!("│                                                                 │");
        println!("│ 🧭 Navigation (Comfortable low range):                         │");
        println!("│   ↑=E3  ↓=D3  ←=C3  →=G3  HOME/END=C3/G3  PG_UP/DN=E3/A3     │");
        println!("│                                                                 │");
        println!("│ 🔧 Function Keys (Bright harmonics for special actions):       │");
        println!("│   F1-F6=C6-A6  F7-F12=B6-G7 (Higher for advanced functions)   │");
        println!("└─────────────────────────────────────────────────────────────────┘");
        println!();
        println!("🎹 Programming-Optimized Music Mapping!");
        println!("🎵 Based on key frequency analysis during coding");
        println!("🎼 Creates pleasant harmonies using C major pentatonic scale");
        println!("🔇 Common keys are quieter to avoid disrupting concentration");
        println!();
    }

    /// Show command line usage information
    fn show_usage_info() {
        println!("💡 Command line options:");
        println!("   cargo run natural    # Start with natural piano");
        println!("   cargo run electronic # Start with electronic");
        println!("   cargo run saw        # Start with saw wave");
        println!("   cargo run square     # Start with square wave");
        println!("   cargo run cyberpunk  # Start with Blade Runner 2049 style");
        println!();
        println!("🎵 CodeBeats: Turn your coding into music!");
        println!("🎵 Waveform is fixed at startup - no runtime switching");
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
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut config = AppConfig::from_args()?;
        let audio_system = AudioSystem::new(&mut config)?;
        let keyboard_processor = KeyboardProcessor::new();

        // Show welcome screen
        UIManager::show_welcome(config.initial_waveform);

        Ok(Self {
            audio_system,
            keyboard_processor,
        })
    }

    /// Handle a pressed key
    fn handle_key_press(&self, key: Keycode) {
        // Handle musical keys
        if let Some((frequency, volume, note)) = get_frequency_and_volume(key) {
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
                || key_debug.contains("Win")
                || key_debug.contains("Super")
            {
                println!("🔍 Detected potential Command key: {:?}", key);
                println!("   (This key is not currently mapped to a musical note)");
            }
        }
    }

    /// Handle a released key
    fn handle_key_release(&self, key: Keycode) {
        if let Some((_, _, note)) = get_frequency_and_volume(key) {
            let mut audio_state = self.audio_system.state().lock().unwrap();
            audio_state.stop_note(key);
            println!("Stopped: {:?} -> {}", key, note);
        }
    }

    /// Main application loop
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut last_hold_check = Instant::now();

        loop {
            let (pressed_keys, released_keys) = self.keyboard_processor.process_input();

            // Handle pressed keys
            for key in pressed_keys {
                self.handle_key_press(key);
            }

            // Handle released keys
            for key in released_keys {
                self.handle_key_release(key);
            }

            // Check for long-held keys every second
            if last_hold_check.elapsed() > Duration::from_secs(1) {
                self.check_long_held_keys();
                last_hold_check = Instant::now();
            }

            // Small delay to prevent excessive CPU usage
            thread::sleep(Duration::from_millis(10));
        }
    }

    /// Check and report on keys that have been held for a long time
    fn check_long_held_keys(&self) {
        let current_keys: Vec<Keycode> = self.keyboard_processor.device_state.get_keys();
        let audio_state = self.audio_system.state().lock().unwrap();

        for key in current_keys {
            if let Some(hold_duration) = audio_state.get_hold_duration(key) {
                if hold_duration > 2.0 && hold_duration % 2.0 < 1.0 {
                    // Report every 2 seconds after the initial 2 seconds
                    if let Some((_, _, note)) = get_frequency_and_volume(key) {
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
    let mut app = PianoApp::new()?;
    app.run()
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
