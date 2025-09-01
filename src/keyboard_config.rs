//! Keyboard configuration module
//!
//! This module handles loading and managing customizable keyboard mappings
//! from configuration files, allowing users to define their own key-to-sound mappings.

use crate::waveforms::Waveform;
use device_query::Keycode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// A single key mapping configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMapping {
    /// Musical note (e.g., "C4", "F#5", "Bb3")
    pub note: String,
    /// Volume level (0.0 to 1.0)
    pub volume: f32,
    /// Optional description for the key
    pub description: Option<String>,
}

/// Complete keyboard configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardConfig {
    /// Version of the configuration format
    pub version: String,
    /// Description of this configuration
    pub description: String,
    /// Default waveform for this configuration
    #[serde(default)]
    pub waveform: Option<String>,
    /// Key mappings - map from key name to sound configuration
    pub mappings: HashMap<String, KeyMapping>,
}

impl Default for KeyboardConfig {
    fn default() -> Self {
        Self::programming_optimized()
    }
}

impl KeyboardConfig {
    /// Create the default programming-optimized keyboard configuration
    pub fn programming_optimized() -> Self {
        let mut mappings = HashMap::new();

        // Most common programming letters - pleasant pentatonic scale
        mappings.insert(
            "E".to_string(),
            KeyMapping {
                note: "E4".to_string(),
                volume: 0.3,
                description: Some("Very common letter".to_string()),
            },
        );
        mappings.insert(
            "T".to_string(),
            KeyMapping {
                note: "G4".to_string(),
                volume: 0.3,
                description: Some("Very common letter".to_string()),
            },
        );
        mappings.insert(
            "A".to_string(),
            KeyMapping {
                note: "C4".to_string(),
                volume: 0.3,
                description: Some("Most common letter".to_string()),
            },
        );
        mappings.insert(
            "O".to_string(),
            KeyMapping {
                note: "D4".to_string(),
                volume: 0.3,
                description: Some("Very common letter".to_string()),
            },
        );
        mappings.insert(
            "I".to_string(),
            KeyMapping {
                note: "A4".to_string(),
                volume: 0.3,
                description: Some("Very common letter".to_string()),
            },
        );
        mappings.insert(
            "N".to_string(),
            KeyMapping {
                note: "E5".to_string(),
                volume: 0.3,
                description: Some("Very common letter".to_string()),
            },
        );
        mappings.insert(
            "S".to_string(),
            KeyMapping {
                note: "G5".to_string(),
                volume: 0.3,
                description: Some("Very common letter".to_string()),
            },
        );
        mappings.insert(
            "H".to_string(),
            KeyMapping {
                note: "C5".to_string(),
                volume: 0.3,
                description: Some("Very common letter".to_string()),
            },
        );
        mappings.insert(
            "R".to_string(),
            KeyMapping {
                note: "D5".to_string(),
                volume: 0.3,
                description: Some("Very common letter".to_string()),
            },
        );

        // Second tier common letters
        mappings.insert(
            "L".to_string(),
            KeyMapping {
                note: "F4".to_string(),
                volume: 0.25,
                description: Some("Common letter".to_string()),
            },
        );
        mappings.insert(
            "U".to_string(),
            KeyMapping {
                note: "A3".to_string(),
                volume: 0.25,
                description: Some("Common letter".to_string()),
            },
        );
        mappings.insert(
            "D".to_string(),
            KeyMapping {
                note: "F5".to_string(),
                volume: 0.25,
                description: Some("Common letter".to_string()),
            },
        );
        mappings.insert(
            "C".to_string(),
            KeyMapping {
                note: "B4".to_string(),
                volume: 0.25,
                description: Some("Common letter".to_string()),
            },
        );
        mappings.insert(
            "M".to_string(),
            KeyMapping {
                note: "B3".to_string(),
                volume: 0.25,
                description: Some("Common letter".to_string()),
            },
        );

        // Less common letters - still harmonious
        mappings.insert(
            "F".to_string(),
            KeyMapping {
                note: "C3".to_string(),
                volume: 0.2,
                description: None,
            },
        );
        mappings.insert(
            "P".to_string(),
            KeyMapping {
                note: "D3".to_string(),
                volume: 0.2,
                description: None,
            },
        );
        mappings.insert(
            "B".to_string(),
            KeyMapping {
                note: "E3".to_string(),
                volume: 0.2,
                description: None,
            },
        );
        mappings.insert(
            "V".to_string(),
            KeyMapping {
                note: "G3".to_string(),
                volume: 0.2,
                description: None,
            },
        );
        mappings.insert(
            "K".to_string(),
            KeyMapping {
                note: "A5".to_string(),
                volume: 0.2,
                description: None,
            },
        );
        mappings.insert(
            "W".to_string(),
            KeyMapping {
                note: "F3".to_string(),
                volume: 0.2,
                description: None,
            },
        );
        mappings.insert(
            "Y".to_string(),
            KeyMapping {
                note: "B5".to_string(),
                volume: 0.2,
                description: None,
            },
        );
        mappings.insert(
            "G".to_string(),
            KeyMapping {
                note: "C6".to_string(),
                volume: 0.2,
                description: None,
            },
        );
        mappings.insert(
            "J".to_string(),
            KeyMapping {
                note: "D6".to_string(),
                volume: 0.2,
                description: None,
            },
        );
        mappings.insert(
            "Q".to_string(),
            KeyMapping {
                note: "E6".to_string(),
                volume: 0.2,
                description: None,
            },
        );
        mappings.insert(
            "X".to_string(),
            KeyMapping {
                note: "F6".to_string(),
                volume: 0.2,
                description: None,
            },
        );
        mappings.insert(
            "Z".to_string(),
            KeyMapping {
                note: "G6".to_string(),
                volume: 0.2,
                description: None,
            },
        );

        // Numbers - same scale as common letters for consistency
        mappings.insert(
            "Key0".to_string(),
            KeyMapping {
                note: "C4".to_string(),
                volume: 0.25,
                description: Some("Number key".to_string()),
            },
        );
        mappings.insert(
            "Key1".to_string(),
            KeyMapping {
                note: "E4".to_string(),
                volume: 0.25,
                description: Some("Number key".to_string()),
            },
        );
        mappings.insert(
            "Key2".to_string(),
            KeyMapping {
                note: "G4".to_string(),
                volume: 0.25,
                description: Some("Number key".to_string()),
            },
        );
        mappings.insert(
            "Key3".to_string(),
            KeyMapping {
                note: "A4".to_string(),
                volume: 0.25,
                description: Some("Number key".to_string()),
            },
        );
        mappings.insert(
            "Key4".to_string(),
            KeyMapping {
                note: "D4".to_string(),
                volume: 0.25,
                description: Some("Number key".to_string()),
            },
        );
        mappings.insert(
            "Key5".to_string(),
            KeyMapping {
                note: "F4".to_string(),
                volume: 0.25,
                description: Some("Number key".to_string()),
            },
        );
        mappings.insert(
            "Key6".to_string(),
            KeyMapping {
                note: "C5".to_string(),
                volume: 0.25,
                description: Some("Number key".to_string()),
            },
        );
        mappings.insert(
            "Key7".to_string(),
            KeyMapping {
                note: "E5".to_string(),
                volume: 0.25,
                description: Some("Number key".to_string()),
            },
        );
        mappings.insert(
            "Key8".to_string(),
            KeyMapping {
                note: "G5".to_string(),
                volume: 0.25,
                description: Some("Number key".to_string()),
            },
        );
        mappings.insert(
            "Key9".to_string(),
            KeyMapping {
                note: "A5".to_string(),
                volume: 0.25,
                description: Some("Number key".to_string()),
            },
        );

        // Programming symbols - gentle harmonics
        mappings.insert(
            "Semicolon".to_string(),
            KeyMapping {
                note: "C4".to_string(),
                volume: 0.2,
                description: Some("Programming symbol".to_string()),
            },
        );
        mappings.insert(
            "LeftBracket".to_string(),
            KeyMapping {
                note: "E4".to_string(),
                volume: 0.2,
                description: Some("Programming symbol".to_string()),
            },
        );
        mappings.insert(
            "RightBracket".to_string(),
            KeyMapping {
                note: "G4".to_string(),
                volume: 0.2,
                description: Some("Programming symbol".to_string()),
            },
        );
        mappings.insert(
            "Comma".to_string(),
            KeyMapping {
                note: "A4".to_string(),
                volume: 0.2,
                description: Some("Programming symbol".to_string()),
            },
        );
        mappings.insert(
            "Dot".to_string(),
            KeyMapping {
                note: "D4".to_string(),
                volume: 0.2,
                description: Some("Programming symbol".to_string()),
            },
        );
        mappings.insert(
            "Slash".to_string(),
            KeyMapping {
                note: "F4".to_string(),
                volume: 0.2,
                description: Some("Programming symbol".to_string()),
            },
        );
        mappings.insert(
            "BackSlash".to_string(),
            KeyMapping {
                note: "B4".to_string(),
                volume: 0.2,
                description: Some("Programming symbol".to_string()),
            },
        );
        mappings.insert(
            "Apostrophe".to_string(),
            KeyMapping {
                note: "C5".to_string(),
                volume: 0.2,
                description: Some("Programming symbol".to_string()),
            },
        );
        mappings.insert(
            "Equal".to_string(),
            KeyMapping {
                note: "D5".to_string(),
                volume: 0.2,
                description: Some("Programming symbol".to_string()),
            },
        );
        mappings.insert(
            "Minus".to_string(),
            KeyMapping {
                note: "E5".to_string(),
                volume: 0.2,
                description: Some("Programming symbol".to_string()),
            },
        );

        // Common keys - quiet to not disrupt
        mappings.insert(
            "Space".to_string(),
            KeyMapping {
                note: "C3".to_string(),
                volume: 0.1,
                description: Some("Common key - quiet".to_string()),
            },
        );
        mappings.insert(
            "Backspace".to_string(),
            KeyMapping {
                note: "G2".to_string(),
                volume: 0.1,
                description: Some("Common key - quiet".to_string()),
            },
        );
        mappings.insert(
            "Enter".to_string(),
            KeyMapping {
                note: "C3".to_string(),
                volume: 0.1,
                description: Some("Common key - quiet".to_string()),
            },
        );
        mappings.insert(
            "Tab".to_string(),
            KeyMapping {
                note: "F2".to_string(),
                volume: 0.1,
                description: Some("Common key - quiet".to_string()),
            },
        );
        mappings.insert(
            "Delete".to_string(),
            KeyMapping {
                note: "A2".to_string(),
                volume: 0.1,
                description: Some("Common key - quiet".to_string()),
            },
        );

        // Modifiers - very quiet
        mappings.insert(
            "LShift".to_string(),
            KeyMapping {
                note: "C2".to_string(),
                volume: 0.05,
                description: Some("Modifier - very quiet".to_string()),
            },
        );
        mappings.insert(
            "RShift".to_string(),
            KeyMapping {
                note: "E2".to_string(),
                volume: 0.05,
                description: Some("Modifier - very quiet".to_string()),
            },
        );
        mappings.insert(
            "LControl".to_string(),
            KeyMapping {
                note: "G2".to_string(),
                volume: 0.05,
                description: Some("Modifier - very quiet".to_string()),
            },
        );
        mappings.insert(
            "RControl".to_string(),
            KeyMapping {
                note: "A2".to_string(),
                volume: 0.05,
                description: Some("Modifier - very quiet".to_string()),
            },
        );
        mappings.insert(
            "LAlt".to_string(),
            KeyMapping {
                note: "D2".to_string(),
                volume: 0.05,
                description: Some("Modifier - very quiet".to_string()),
            },
        );
        mappings.insert(
            "RAlt".to_string(),
            KeyMapping {
                note: "F2".to_string(),
                volume: 0.05,
                description: Some("Modifier - very quiet".to_string()),
            },
        );
        mappings.insert(
            "CapsLock".to_string(),
            KeyMapping {
                note: "B1".to_string(),
                volume: 0.05,
                description: Some("Modifier - very quiet".to_string()),
            },
        );
        mappings.insert(
            "Escape".to_string(),
            KeyMapping {
                note: "C2".to_string(),
                volume: 0.05,
                description: Some("Modifier - very quiet".to_string()),
            },
        );

        // Navigation - comfortable low range
        mappings.insert(
            "Up".to_string(),
            KeyMapping {
                note: "E3".to_string(),
                volume: 0.15,
                description: Some("Navigation key".to_string()),
            },
        );
        mappings.insert(
            "Down".to_string(),
            KeyMapping {
                note: "D3".to_string(),
                volume: 0.15,
                description: Some("Navigation key".to_string()),
            },
        );
        mappings.insert(
            "Left".to_string(),
            KeyMapping {
                note: "C3".to_string(),
                volume: 0.15,
                description: Some("Navigation key".to_string()),
            },
        );
        mappings.insert(
            "Right".to_string(),
            KeyMapping {
                note: "G3".to_string(),
                volume: 0.15,
                description: Some("Navigation key".to_string()),
            },
        );
        mappings.insert(
            "Home".to_string(),
            KeyMapping {
                note: "C3".to_string(),
                volume: 0.15,
                description: Some("Navigation key".to_string()),
            },
        );
        mappings.insert(
            "End".to_string(),
            KeyMapping {
                note: "G3".to_string(),
                volume: 0.15,
                description: Some("Navigation key".to_string()),
            },
        );
        mappings.insert(
            "PageUp".to_string(),
            KeyMapping {
                note: "E3".to_string(),
                volume: 0.15,
                description: Some("Navigation key".to_string()),
            },
        );
        mappings.insert(
            "PageDown".to_string(),
            KeyMapping {
                note: "A3".to_string(),
                volume: 0.15,
                description: Some("Navigation key".to_string()),
            },
        );

        // Function keys - bright harmonics
        mappings.insert(
            "F1".to_string(),
            KeyMapping {
                note: "C6".to_string(),
                volume: 0.2,
                description: Some("Function key".to_string()),
            },
        );
        mappings.insert(
            "F2".to_string(),
            KeyMapping {
                note: "D6".to_string(),
                volume: 0.2,
                description: Some("Function key".to_string()),
            },
        );
        mappings.insert(
            "F3".to_string(),
            KeyMapping {
                note: "E6".to_string(),
                volume: 0.2,
                description: Some("Function key".to_string()),
            },
        );
        mappings.insert(
            "F4".to_string(),
            KeyMapping {
                note: "F6".to_string(),
                volume: 0.2,
                description: Some("Function key".to_string()),
            },
        );
        mappings.insert(
            "F5".to_string(),
            KeyMapping {
                note: "G6".to_string(),
                volume: 0.2,
                description: Some("Function key".to_string()),
            },
        );
        mappings.insert(
            "F6".to_string(),
            KeyMapping {
                note: "A6".to_string(),
                volume: 0.2,
                description: Some("Function key".to_string()),
            },
        );
        mappings.insert(
            "F7".to_string(),
            KeyMapping {
                note: "B6".to_string(),
                volume: 0.2,
                description: Some("Function key".to_string()),
            },
        );
        mappings.insert(
            "F8".to_string(),
            KeyMapping {
                note: "C7".to_string(),
                volume: 0.2,
                description: Some("Function key".to_string()),
            },
        );
        mappings.insert(
            "F9".to_string(),
            KeyMapping {
                note: "D7".to_string(),
                volume: 0.2,
                description: Some("Function key".to_string()),
            },
        );
        mappings.insert(
            "F10".to_string(),
            KeyMapping {
                note: "E7".to_string(),
                volume: 0.2,
                description: Some("Function key".to_string()),
            },
        );
        mappings.insert(
            "F11".to_string(),
            KeyMapping {
                note: "F7".to_string(),
                volume: 0.2,
                description: Some("Function key".to_string()),
            },
        );
        mappings.insert(
            "F12".to_string(),
            KeyMapping {
                note: "G7".to_string(),
                volume: 0.2,
                description: Some("Function key".to_string()),
            },
        );

        Self {
            version: "1.0".to_string(),
            description: "Programming-optimized keyboard mapping with frequency analysis"
                .to_string(),
            waveform: None, // Use system default
            mappings,
        }
    }

    /// Get the waveform for this configuration, falling back to default if none specified
    pub fn get_waveform(&self) -> Option<Waveform> {
        self.waveform.as_ref().and_then(|w| w.parse().ok())
    }

    /// Load configuration from a JSON file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: KeyboardConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to a JSON file
    #[allow(dead_code)]
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Create a simple piano layout configuration (chromatic scale)
    #[allow(dead_code)]
    pub fn piano_layout() -> Self {
        let mut mappings = HashMap::new();

        // White keys (C major scale)
        mappings.insert(
            "Q".to_string(),
            KeyMapping {
                note: "C4".to_string(),
                volume: 0.3,
                description: Some("White key - C4".to_string()),
            },
        );
        mappings.insert(
            "W".to_string(),
            KeyMapping {
                note: "D4".to_string(),
                volume: 0.3,
                description: Some("White key - D4".to_string()),
            },
        );
        mappings.insert(
            "E".to_string(),
            KeyMapping {
                note: "E4".to_string(),
                volume: 0.3,
                description: Some("White key - E4".to_string()),
            },
        );
        mappings.insert(
            "R".to_string(),
            KeyMapping {
                note: "F4".to_string(),
                volume: 0.3,
                description: Some("White key - F4".to_string()),
            },
        );
        mappings.insert(
            "T".to_string(),
            KeyMapping {
                note: "G4".to_string(),
                volume: 0.3,
                description: Some("White key - G4".to_string()),
            },
        );
        mappings.insert(
            "Y".to_string(),
            KeyMapping {
                note: "A4".to_string(),
                volume: 0.3,
                description: Some("White key - A4".to_string()),
            },
        );
        mappings.insert(
            "U".to_string(),
            KeyMapping {
                note: "B4".to_string(),
                volume: 0.3,
                description: Some("White key - B4".to_string()),
            },
        );
        mappings.insert(
            "I".to_string(),
            KeyMapping {
                note: "C5".to_string(),
                volume: 0.3,
                description: Some("White key - C5".to_string()),
            },
        );

        // Black keys (sharps/flats)
        mappings.insert(
            "Key2".to_string(),
            KeyMapping {
                note: "C#4".to_string(),
                volume: 0.25,
                description: Some("Black key - C#4".to_string()),
            },
        );
        mappings.insert(
            "Key3".to_string(),
            KeyMapping {
                note: "D#4".to_string(),
                volume: 0.25,
                description: Some("Black key - D#4".to_string()),
            },
        );
        mappings.insert(
            "Key5".to_string(),
            KeyMapping {
                note: "F#4".to_string(),
                volume: 0.25,
                description: Some("Black key - F#4".to_string()),
            },
        );
        mappings.insert(
            "Key6".to_string(),
            KeyMapping {
                note: "G#4".to_string(),
                volume: 0.25,
                description: Some("Black key - G#4".to_string()),
            },
        );
        mappings.insert(
            "Key7".to_string(),
            KeyMapping {
                note: "A#4".to_string(),
                volume: 0.25,
                description: Some("Black key - A#4".to_string()),
            },
        );

        Self {
            version: "1.0".to_string(),
            description: "Standard piano layout keyboard mapping".to_string(),
            waveform: None, // Use system default
            mappings,
        }
    }
}

/// Convert a keycode to its string representation for config lookup
#[allow(dead_code)]
pub fn keycode_to_string(keycode: Keycode) -> String {
    format!("{:?}", keycode)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = KeyboardConfig::default();
        assert_eq!(config.version, "1.0");
        assert!(!config.mappings.is_empty());
    }

    #[test]
    fn test_config_serialization() {
        let config = KeyboardConfig::default();

        // Test serialization/deserialization
        let json = serde_json::to_string_pretty(&config).unwrap();
        let deserialized: KeyboardConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.version, deserialized.version);
    }

    #[test]
    fn test_keycode_conversion() {
        let key_str = keycode_to_string(Keycode::A);
        assert_eq!(key_str, "A");
    }
}
