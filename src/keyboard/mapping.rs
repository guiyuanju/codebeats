//! Keyboard mapping and note calculation module
//!
//! This module handles:
//! - Mapping keyboard keys to musical notes
//! - Note frequency calculation using standard tuning
//! - Programming-optimized key assignments for pleasant coding experience
//! - Rate limiting to prevent high-pitched sounds from rapid key presses

use device_query::Keycode;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Rate limiter for preventing annoying high-pitched sounds from rapid key presses
pub struct KeyRateLimiter {
    last_press_times: HashMap<Keycode, Instant>,
    press_counts: HashMap<Keycode, u32>,
}

impl KeyRateLimiter {
    pub fn new() -> Self {
        Self {
            last_press_times: HashMap::new(),
            press_counts: HashMap::new(),
        }
    }

    /// Check if a key press should be allowed (returns adjusted volume multiplier)
    pub fn check_key_press(&mut self, keycode: Keycode) -> f32 {
        let now = Instant::now();
        let min_interval = Duration::from_millis(150); // Minimum 150ms between same key presses
        let reset_interval = Duration::from_millis(1000); // Reset counter after 1 second of no presses

        // Get current press count
        let press_count = *self.press_counts.get(&keycode).unwrap_or(&0);

        // Check if we should throttle based on recent presses
        if let Some(last_press) = self.last_press_times.get(&keycode) {
            let time_since_last = now.duration_since(*last_press);

            if time_since_last < min_interval {
                // Too frequent - reduce volume based on press count
                let new_count = press_count + 1;
                let volume_reduction = match new_count {
                    1 => 1.0,     // First press normal
                    2 => 0.7,     // Second press slightly reduced
                    3 => 0.4,     // Third press half volume
                    4..=5 => 0.2, // Very quiet
                    _ => 0.0,     // Silent after too many rapid presses
                };

                self.press_counts.insert(keycode, new_count);
                self.last_press_times.insert(keycode, now);

                return volume_reduction;
            } else if time_since_last > reset_interval {
                // Reset counter after long pause
                self.press_counts.insert(keycode, 1);
                self.last_press_times.insert(keycode, now);
                return 1.0;
            }
        }

        // First press or normal interval
        self.press_counts.insert(keycode, 1);
        self.last_press_times.insert(keycode, now);

        1.0 // Normal volume
    }
}

/// Calculate frequency from musical note string (e.g., "C4", "F#5", "Bb3")
pub fn get_frequency_from_note(note: &str) -> Option<f32> {
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

    // Calculate frequency using A4=440Hz tuning
    // Formula: f = 440 * 2^((n-69)/12) where n is MIDI note number
    let midi_note = octave * 12 + semitone + 12; // MIDI note number (C4 = 60)
    let frequency = 440.0 * 2.0_f32.powf((midi_note as f32 - 69.0) / 12.0);

    Some(frequency)
}

/// Programming-optimized keyboard mapping
/// Returns (frequency, volume, note_name) for a given keycode
///
/// The mapping is designed to:
/// - Map frequent programming keys to pleasant pentatonic scales
/// - Use lower volumes for common keys to avoid disrupting concentration
/// - Create harmonic relationships between related keys
/// - Include Mac Command keys (Meta keys) if available
pub fn get_frequency_and_volume(keycode: Keycode) -> Option<(f32, f32, &'static str)> {
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

        // Try to detect Mac Command keys (these may or may not be available in device_query)
        // Using a broader pattern match to catch potential Meta/Command key variants
        keycode
            if format!("{:?}", keycode).contains("Meta")
                || format!("{:?}", keycode).contains("Cmd")
                || format!("{:?}", keycode).contains("Command")
                || format!("{:?}", keycode).contains("LWin")
                || format!("{:?}", keycode).contains("RWin") =>
        {
            // Map Command keys to gentle bass notes like other modifiers
            ("D2", 0.05)
        }

        _ => return None,
    };

    get_frequency_from_note(note).map(|freq| (freq, volume, note))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_calculation() {
        // Test A4 = 440Hz
        let a4_freq = get_frequency_from_note("A4").unwrap();
        assert!((a4_freq - 440.0).abs() < 0.01);

        // Test middle C
        let c4_freq = get_frequency_from_note("C4").unwrap();
        assert!((c4_freq - 261.63).abs() < 1.0);

        // Test octave relationships
        let c3_freq = get_frequency_from_note("C3").unwrap();
        let c5_freq = get_frequency_from_note("C5").unwrap();
        assert!((c5_freq / c4_freq - 2.0).abs() < 0.01);
        assert!((c4_freq / c3_freq - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_sharp_and_flat_notes() {
        let cs4_freq = get_frequency_from_note("C#4").unwrap();
        let db4_freq = get_frequency_from_note("DB4").unwrap();
        assert!((cs4_freq - db4_freq).abs() < 0.01);
    }

    #[test]
    fn test_rate_limiter() {
        let mut limiter = KeyRateLimiter::new();

        // First press should be full volume
        let first_volume = limiter.check_key_press(Keycode::J);
        println!("First press volume: {}", first_volume);
        assert_eq!(first_volume, 1.0);

        // Immediate second press should reduce volume (simulating rapid press)
        let second_volume = limiter.check_key_press(Keycode::J);
        println!("Second press volume: {}", second_volume);
        assert!(
            second_volume < 1.0,
            "Expected reduced volume but got {}",
            second_volume
        );

        // Third rapid press should reduce even more
        let third_volume = limiter.check_key_press(Keycode::J);
        println!("Third press volume: {}", third_volume);
        assert!(
            third_volume <= second_volume,
            "Expected further reduction but got {}",
            third_volume
        );

        // Different keys shouldn't interfere
        let different_key_volume = limiter.check_key_press(Keycode::K);
        println!("Different key volume: {}", different_key_volume);
        assert_eq!(different_key_volume, 1.0);
    }

    #[test]
    fn test_keyboard_mapping() {
        // Test common programming keys
        let (freq, vol, note) = get_frequency_and_volume(Keycode::A).unwrap();
        assert_eq!(note, "C4");
        assert_eq!(vol, 0.3);
        assert!((freq - 261.63).abs() < 1.0);

        // Test that we can get frequency and volume for valid keys
        assert!(get_frequency_and_volume(Keycode::F9).is_some());
    }

    #[test]
    fn test_volume_levels() {
        // Common keys should be louder
        let (_, common_vol, _) = get_frequency_and_volume(Keycode::E).unwrap();
        let (_, modifier_vol, _) = get_frequency_and_volume(Keycode::LShift).unwrap();

        assert!(common_vol > modifier_vol);
    }

    #[test]
    fn test_invalid_notes() {
        assert_eq!(get_frequency_from_note("H4"), None);
        assert_eq!(get_frequency_from_note("C"), None);
        assert_eq!(get_frequency_from_note(""), None);
    }
}
