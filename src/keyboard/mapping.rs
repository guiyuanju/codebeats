//! Keyboard mapping and note calculation module
//!
//! This module handles:
//! - Mapping keyboard keys to musical notes using configurable mappings
//! - Note frequency calculation using standard tuning
//! - Programming-optimized key assignments for pleasant coding experience
//! - Rate limiting to prevent high-pitched sounds from rapid key presses

use crate::keyboard::config::KeyboardConfig;
use device_query::Keycode;
use std::collections::HashMap;
use std::collections::HashSet;
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

/// Keyboard state tracker for handling shifted characters
pub struct KeyboardStateTracker {
    shift_pressed: bool,
    pressed_keys: HashSet<Keycode>,
}

impl KeyboardStateTracker {
    pub fn new() -> Self {
        Self {
            shift_pressed: false,
            pressed_keys: HashSet::new(),
        }
    }

    /// Update keyboard state based on pressed and released keys
    pub fn update(&mut self, pressed_keys: &[Keycode], released_keys: &[Keycode]) {
        // First, add all newly pressed keys
        for key in pressed_keys {
            self.pressed_keys.insert(*key);
        }

        // Then, remove all released keys
        for key in released_keys {
            self.pressed_keys.remove(key);
        }

        // Update shift state based on current pressed keys
        self.shift_pressed = self.pressed_keys.contains(&Keycode::LShift)
            || self.pressed_keys.contains(&Keycode::RShift);
    }

    /// Get the virtual keycode for shifted characters
    pub fn get_virtual_keycode(&self, physical_key: Keycode) -> Option<VirtualKeycode> {
        // Check if shift is currently pressed (including keys pressed in this frame)
        let shift_currently_pressed = self.shift_pressed;

        if !shift_currently_pressed {
            return Some(VirtualKeycode::Physical(physical_key));
        }

        // Map shifted characters
        let shifted_key = match physical_key {
            Keycode::Key1 => VirtualKeycode::Shifted("Exclamation"),
            Keycode::Key2 => VirtualKeycode::Shifted("At"),
            Keycode::Key3 => VirtualKeycode::Shifted("Hash"),
            Keycode::Key4 => VirtualKeycode::Shifted("Dollar"),
            Keycode::Key5 => VirtualKeycode::Shifted("Percent"),
            Keycode::Key6 => VirtualKeycode::Shifted("Caret"),
            Keycode::Key7 => VirtualKeycode::Shifted("Ampersand"),
            Keycode::Key8 => VirtualKeycode::Shifted("Asterisk"),
            Keycode::Key9 => VirtualKeycode::Shifted("LeftParen"),
            Keycode::Key0 => VirtualKeycode::Shifted("RightParen"),
            Keycode::Minus => VirtualKeycode::Shifted("Underscore"),
            Keycode::Equal => VirtualKeycode::Shifted("Plus"),
            Keycode::LeftBracket => VirtualKeycode::Shifted("LeftBrace"),
            Keycode::RightBracket => VirtualKeycode::Shifted("RightBrace"),
            Keycode::BackSlash => VirtualKeycode::Shifted("Pipe"),
            Keycode::Semicolon => VirtualKeycode::Shifted("Colon"),
            Keycode::Apostrophe => VirtualKeycode::Shifted("DoubleQuote"),
            Keycode::Comma => VirtualKeycode::Shifted("LessThan"),
            Keycode::Dot => VirtualKeycode::Shifted("GreaterThan"),
            Keycode::Slash => VirtualKeycode::Shifted("Question"),
            Keycode::Grave => VirtualKeycode::Shifted("Tilde"),
            // Don't shift modifier keys themselves
            Keycode::LShift | Keycode::RShift => VirtualKeycode::Physical(physical_key),
            // Regular letters get capitalized when shift is pressed, but we treat them the same
            _ => VirtualKeycode::Physical(physical_key),
        };

        Some(shifted_key)
    }
}

/// Virtual keycode that can represent both physical keys and shifted characters
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VirtualKeycode {
    Physical(Keycode),
    Shifted(&'static str),
}

impl VirtualKeycode {
    pub fn to_string(&self) -> String {
        match self {
            VirtualKeycode::Physical(keycode) => format!("{:?}", keycode),
            VirtualKeycode::Shifted(name) => name.to_string(),
        }
    }
}

/// Get frequency and volume for a virtual keycode using the provided keyboard configuration
/// Returns (frequency, volume, note_name) for a given virtual keycode
pub fn get_frequency_and_volume_with_config_virtual(
    virtual_keycode: &VirtualKeycode,
    config: &KeyboardConfig,
) -> Option<(f32, f32, String)> {
    let key_name = virtual_keycode.to_string();
    let mapping = config.mappings.get(&key_name)?;
    let frequency = get_frequency_from_note(&mapping.note)?;
    Some((frequency, mapping.volume, mapping.note.clone()))
}

/// Get frequency and volume for a keycode using the provided keyboard configuration
/// Returns (frequency, volume, note_name) for a given keycode
pub fn get_frequency_and_volume_with_config(
    keycode: Keycode,
    config: &KeyboardConfig,
) -> Option<(f32, f32, String)> {
    let virtual_key = VirtualKeycode::Physical(keycode);
    get_frequency_and_volume_with_config_virtual(&virtual_key, config)
}

/// Programming-optimized keyboard mapping (using default config)
/// Returns (frequency, volume, note_name) for a given keycode
///
/// This function maintains backward compatibility by using the default configuration.
/// For customizable mappings, use get_frequency_and_volume_with_config instead.
pub fn get_frequency_and_volume(keycode: Keycode) -> Option<(f32, f32, &'static str)> {
    // Use a static default config for backward compatibility
    use std::sync::OnceLock;
    static DEFAULT_CONFIG: OnceLock<KeyboardConfig> = OnceLock::new();
    let config = DEFAULT_CONFIG.get_or_init(|| KeyboardConfig::default());

    if let Some((freq, vol, note)) = get_frequency_and_volume_with_config(keycode, config) {
        // We need to return a &'static str, so we'll need to handle the string conversion
        // For now, let's use a simple approach - we know our default config uses standard notes
        let note_static = match note.as_str() {
            "C2" => "C2",
            "D2" => "D2",
            "E2" => "E2",
            "F2" => "F2",
            "G2" => "G2",
            "A2" => "A2",
            "B2" => "B2",
            "C3" => "C3",
            "D3" => "D3",
            "E3" => "E3",
            "F3" => "F3",
            "G3" => "G3",
            "A3" => "A3",
            "B3" => "B3",
            "C4" => "C4",
            "D4" => "D4",
            "E4" => "E4",
            "F4" => "F4",
            "G4" => "G4",
            "A4" => "A4",
            "B4" => "B4",
            "C5" => "C5",
            "D5" => "D5",
            "E5" => "E5",
            "F5" => "F5",
            "G5" => "G5",
            "A5" => "A5",
            "B5" => "B5",
            "C6" => "C6",
            "D6" => "D6",
            "E6" => "E6",
            "F6" => "F6",
            "G6" => "G6",
            "A6" => "A6",
            "B6" => "B6",
            "C7" => "C7",
            "D7" => "D7",
            "E7" => "E7",
            "F7" => "F7",
            "G7" => "G7",
            "A7" => "A7",
            "B7" => "B7",
            "B1" => "B1",
            _ => "C4", // Fallback to middle C
        };
        Some((freq, vol, note_static))
    } else {
        None
    }
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
