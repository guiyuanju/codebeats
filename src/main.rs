use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Note frequency calculation functions
fn get_frequency_from_note(note: &str) -> Option<f32> {
    let note = note.to_uppercase();

    // Parse note and octave
    let (note_name, octave) = if note.len() >= 2 {
        let octave_str = &note[note.len() - 1..];
        let octave: i32 = octave_str.parse().ok()?;
        let note_name = &note[..note.len() - 1];
        (note_name, octave)
    } else {
        return None;
    };

    // Note to semitone mapping (C = 0)
    let semitone = match note_name {
        "C" => 0,
        "C#" | "DB" => 1,
        "D" => 2,
        "D#" | "EB" => 3,
        "E" => 4,
        "F" => 5,
        "F#" | "GB" => 6,
        "G" => 7,
        "G#" | "AB" => 8,
        "A" => 9,
        "A#" | "BB" => 10,
        "B" => 11,
        _ => return None,
    };

    // Calculate frequency using A4 = 440 Hz as reference
    // Formula: f = 440 * 2^((n-69)/12) where n is MIDI note number
    let midi_note = (octave + 1) * 12 + semitone - 12; // Adjusted for our octave numbering
    let frequency = 440.0 * 2.0_f32.powf((midi_note as f32 - 69.0) / 12.0);

    Some(frequency)
}

fn get_frequency_and_volume(keycode: Keycode) -> Option<(f32, f32)> {
    let (note, volume) = match keycode {
        // Most common programming letters - pleasant pentatonic scale
        Keycode::E => ("E4", 0.3), // Very common
        Keycode::T => ("G4", 0.3), // Very common
        Keycode::A => ("C4", 0.3), // Most common
        Keycode::O => ("D4", 0.3), // Very common
        Keycode::I => ("A4", 0.3), // Very common
        Keycode::N => ("E5", 0.3), // Very common
        Keycode::S => ("G5", 0.3), // Very common
        Keycode::H => ("C5", 0.3), // Very common
        Keycode::R => ("D5", 0.3), // Very common

        // Second tier common letters
        Keycode::L => ("F4", 0.25), // Common
        Keycode::U => ("A3", 0.25), // Common
        Keycode::D => ("F5", 0.25), // Common
        Keycode::C => ("B4", 0.25), // Common
        Keycode::M => ("B3", 0.25), // Common

        // Less common letters - still harmonious
        Keycode::F => ("C3", 0.2),
        Keycode::P => ("D3", 0.2),
        Keycode::B => ("E3", 0.2),
        Keycode::V => ("G3", 0.2),
        Keycode::K => ("A5", 0.2),
        Keycode::W => ("F3", 0.2),
        Keycode::Y => ("B5", 0.2),
        Keycode::G => ("C6", 0.2),
        Keycode::J => ("D6", 0.2),
        Keycode::Q => ("E6", 0.2),
        Keycode::X => ("F6", 0.2),
        Keycode::Z => ("G6", 0.2),

        // Numbers - same scale as common letters for consistency
        Keycode::Key0 => ("C4", 0.25),
        Keycode::Key1 => ("E4", 0.25),
        Keycode::Key2 => ("G4", 0.25),
        Keycode::Key3 => ("A4", 0.25),
        Keycode::Key4 => ("D4", 0.25),
        Keycode::Key5 => ("F4", 0.25),
        Keycode::Key6 => ("C5", 0.25),
        Keycode::Key7 => ("E5", 0.25),
        Keycode::Key8 => ("G5", 0.25),
        Keycode::Key9 => ("A5", 0.25),

        // Programming symbols - gentle harmonics
        Keycode::Semicolon => ("C4", 0.2),
        Keycode::LeftBracket => ("E4", 0.2),
        Keycode::RightBracket => ("G4", 0.2),
        Keycode::Comma => ("A4", 0.2),
        Keycode::Dot => ("D4", 0.2),
        Keycode::Slash => ("F4", 0.2),
        Keycode::BackSlash => ("B4", 0.2),
        Keycode::Apostrophe => ("C5", 0.2),
        Keycode::Equal => ("D5", 0.2),
        Keycode::Minus => ("E5", 0.2),

        // Common keys - quiet to not disrupt
        Keycode::Space => ("C3", 0.1),
        Keycode::Backspace => ("G2", 0.1),
        Keycode::Enter => ("C3", 0.1),
        Keycode::Tab => ("F2", 0.1),
        Keycode::Delete => ("A2", 0.1),

        // Modifiers - very quiet
        Keycode::LShift => ("C2", 0.05),
        Keycode::RShift => ("E2", 0.05),
        Keycode::LControl => ("G2", 0.05),
        Keycode::RControl => ("A2", 0.05),
        Keycode::LAlt => ("D2", 0.05),
        Keycode::RAlt => ("F2", 0.05),
        Keycode::CapsLock => ("B1", 0.05),
        Keycode::Escape => ("C2", 0.05),

        // Navigation - comfortable low range
        Keycode::Up => ("E3", 0.15),
        Keycode::Down => ("D3", 0.15),
        Keycode::Left => ("C3", 0.15),
        Keycode::Right => ("G3", 0.15),
        Keycode::Home => ("C3", 0.15),
        Keycode::End => ("G3", 0.15),
        Keycode::PageUp => ("E3", 0.15),
        Keycode::PageDown => ("A3", 0.15),

        // Function keys - bright harmonics
        Keycode::F1 => ("C6", 0.2),
        Keycode::F2 => ("D6", 0.2),
        Keycode::F3 => ("E6", 0.2),
        Keycode::F4 => ("F6", 0.2),
        Keycode::F5 => ("G6", 0.2),
        Keycode::F6 => ("A6", 0.2),
        Keycode::F7 => ("B6", 0.2),
        Keycode::F8 => ("C7", 0.2),
        Keycode::F9 => ("D7", 0.2),
        Keycode::F10 => ("E7", 0.2),
        Keycode::F11 => ("F7", 0.2),
        Keycode::F12 => ("G7", 0.2),

        _ => return None,
    };

    get_frequency_from_note(note).map(|freq| (freq, volume))
}

// Audio state management
struct AudioState {
    active_notes: HashMap<Keycode, (f32, f32)>, // frequency, volume
    sample_rate: f32,
    phase: f32,
}

impl AudioState {
    fn new(sample_rate: f32) -> Self {
        Self {
            active_notes: HashMap::new(),
            sample_rate,
            phase: 0.0,
        }
    }

    fn generate_sample(&mut self) -> f32 {
        let mut sample = 0.0;

        for (frequency, volume) in self.active_notes.values() {
            sample += (self.phase * frequency * 2.0 * std::f32::consts::PI / self.sample_rate)
                .sin()
                * volume;
        }

        self.phase += 1.0;
        if self.phase >= self.sample_rate {
            self.phase = 0.0;
        }

        sample * 0.3 // Global volume adjustment
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize audio
    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let config = device.default_output_config()?;
    let sample_rate = config.sample_rate().0 as f32;

    let audio_state = Arc::new(Mutex::new(AudioState::new(sample_rate)));
    let audio_state_clone = audio_state.clone();

    // Create audio stream
    let stream = device.build_output_stream(
        &config.into(),
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

    // Interactive keyboard mode
    println!("🎹 Piano Keyboard Sound Simulator");
    println!();
    println!("IMPORTANT - macOS Permission Required:");
    println!("If this fails, you need to grant accessibility permissions:");
    println!("1. Go to: System Preferences > Security & Privacy > Privacy > Accessibility");
    println!("2. Click the lock and enter your password");
    println!("3. Add your Terminal app (Terminal.app, iTerm2, etc.)");
    println!("4. Restart this program");
    println!();
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
    println!("Press Ctrl+C to exit");
    println!();

    // Initialize device state for global keyboard input
    let device_state = DeviceState::new();
    let mut prev_keys: Vec<Keycode> = Vec::new();

    // Main keyboard polling loop
    loop {
        let keys: Vec<Keycode> = device_state.get_keys();

        // Check for newly pressed keys
        for key in &keys {
            if !prev_keys.contains(key) {
                if let Some((frequency, volume)) = get_frequency_and_volume(*key) {
                    let mut audio_state = audio_state.lock().unwrap();
                    audio_state.active_notes.insert(*key, (frequency, volume));
                    println!(
                        "Playing: {:?} ({:.2} Hz, vol: {:.2})",
                        key, frequency, volume
                    );
                }
            }
        }

        // Check for released keys
        for key in &prev_keys {
            if !keys.contains(key) {
                if get_frequency_and_volume(*key).is_some() {
                    let mut audio_state = audio_state.lock().unwrap();
                    audio_state.active_notes.remove(key);
                    println!("Stopped: {:?}", key);
                }
            }
        }

        prev_keys = keys;
        thread::sleep(Duration::from_millis(10)); // Small delay to prevent excessive CPU usage
    }
}
