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

mod audio_engine;
mod keyboard_config;
mod keyboard_mapping;
mod waveforms;

use audio_engine::AudioState;
use clap::Parser;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use device_query::{DeviceQuery, DeviceState, Keycode};
use keyboard_config::KeyboardConfig;
use keyboard_mapping::{
    get_frequency_and_volume_with_config_virtual, KeyboardStateTracker, VirtualKeycode,
};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use waveforms::Waveform;

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

fn setup_audio(
    initial_waveform: Waveform,
    master_volume: f32,
) -> Result<Arc<Mutex<AudioState>>, Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or("No output device available")?;
    let device_config = device.default_output_config()?;

    let sample_rate = device_config.sample_rate().0 as f32;
    let audio_state = AudioState::new(sample_rate, initial_waveform, master_volume);
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

fn handle_key_press(
    virtual_key: &VirtualKeycode,
    config: &KeyboardConfig,
    audio_state: &Arc<Mutex<AudioState>>,
) {
    let key_id = virtual_key.to_string();

    if let Some((frequency, volume, note)) =
        get_frequency_and_volume_with_config_virtual(virtual_key, config)
    {
        let mut state = audio_state.lock().unwrap();
        let actual_volume = state.start_note_with_id(&key_id, frequency, volume);

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
        println!("🔍 Detected unmapped key: {}", virtual_key.to_string());
        println!("💡 Add it to keyboard_config.json to assign a sound");
    }
}

fn handle_key_release(
    virtual_key: &VirtualKeycode,
    config: &KeyboardConfig,
    audio_state: &Arc<Mutex<AudioState>>,
) {
    if let Some((_, _, note)) = get_frequency_and_volume_with_config_virtual(virtual_key, config) {
        let mut state = audio_state.lock().unwrap();
        let key_id = virtual_key.to_string();
        state.stop_note_with_id(&key_id);
        println!("Stopped: {} -> {}", key_id, note);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let master_volume = cli.volume.unwrap_or(1.0).clamp(0.0, 1.0);
    let keyboard_config = load_keyboard_config(&cli);

    // Determine waveform: CLI arg > language config > default
    let initial_waveform = if let Some(waveform_str) = &cli.waveform {
        Waveform::from_str(waveform_str).unwrap_or_else(|| {
            println!(
                "Invalid waveform '{}', using default: electronic",
                waveform_str
            );
            Waveform::Electronic
        })
    } else if let Some(config_waveform) = keyboard_config.get_waveform() {
        println!("Using waveform from config: {:?}", config_waveform);
        config_waveform
    } else {
        Waveform::Electronic
    };

    // Setup audio
    let audio_state = setup_audio(initial_waveform, master_volume)?;

    // Show welcome
    println!(
        "🎵 CodeBeats ({:?}) - {}",
        initial_waveform, keyboard_config.description
    );
    println!("Press Ctrl+C to exit");

    // Setup keyboard input
    let device_state = DeviceState::new();
    let mut prev_keys: Vec<Keycode> = Vec::new();
    let mut keyboard_tracker = KeyboardStateTracker::new();

    // Main loop
    loop {
        let current_keys: Vec<Keycode> = device_state.get_keys();

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
        keyboard_tracker.update(&pressed_keys, &released_keys);

        // Handle pressed keys
        for key in pressed_keys {
            if let Some(virtual_key) = keyboard_tracker.get_virtual_keycode_for_press(key) {
                handle_key_press(&virtual_key, &keyboard_config, &audio_state);
            }
        }

        // Handle released keys
        for key in released_keys {
            if let Some(virtual_key) = keyboard_tracker.get_virtual_keycode_for_release(key) {
                handle_key_release(&virtual_key, &keyboard_config, &audio_state);
            }
        }

        prev_keys = current_keys;
        thread::sleep(Duration::from_millis(10));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_waveform_parsing() {
        // Test valid waveforms
        assert_eq!(Waveform::from_str("natural"), Some(Waveform::Natural));
        assert_eq!(Waveform::from_str("electronic"), Some(Waveform::Electronic));
        assert_eq!(Waveform::from_str("cyberpunk"), Some(Waveform::Cyberpunk));

        // Test invalid waveform
        assert_eq!(Waveform::from_str("invalid"), None);
    }

    #[test]
    fn test_keyboard_config_loading() {
        let keyboard_config = KeyboardConfig::default();
        assert!(!keyboard_config.description.is_empty());
    }
}
