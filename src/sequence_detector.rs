//! Input sequence detection module for Easter eggs
//!
//! This module provides functionality to detect specific input sequences
//! from keyboard input, such as the Japanese Easter egg sequence.

use device_query::Keycode;
use std::collections::VecDeque;

/// Maximum length of input history to keep for sequence detection
const MAX_HISTORY_LENGTH: usize = 50;

/// Input sequence detector for Easter eggs
pub struct SequenceDetector {
    /// History of recent key inputs (excluding spaces and modifiers)
    input_history: VecDeque<char>,
    /// Target sequence to detect (as lowercase chars)
    target_sequence: Vec<char>,
    /// Whether the sequence was recently triggered (to avoid spam)
    recently_triggered: bool,
    /// Counter to reset the recently_triggered flag after some inputs
    reset_counter: usize,
}

impl SequenceDetector {
    /// Create a new sequence detector
    pub fn new() -> Self {
        // Japanese "おっぽこ　こっぽこ　すってんてん" in romaji: "oppokokoppokosuttenten"
        let target = "oppokokoppokosuttenten";

        Self {
            input_history: VecDeque::with_capacity(MAX_HISTORY_LENGTH),
            target_sequence: target.chars().collect(),
            recently_triggered: false,
            reset_counter: 0,
        }
    }

    /// Process a key input and check for sequence match
    pub fn process_input(&mut self, keycode: Keycode) -> bool {
        // Convert keycode to character, ignoring non-letter keys and spaces
        if let Some(ch) = self.keycode_to_char(keycode) {
            // Add to history
            self.input_history.push_back(ch);

            // Trim history if too long
            if self.input_history.len() > MAX_HISTORY_LENGTH {
                self.input_history.pop_front();
            }

            // Reset recently_triggered flag after enough inputs
            self.reset_counter += 1;
            if self.reset_counter > self.target_sequence.len() {
                self.recently_triggered = false;
                self.reset_counter = 0;
            }

            // Check for sequence match (only if not recently triggered)
            if !self.recently_triggered {
                if self.check_sequence_match() {
                    self.recently_triggered = true;
                    self.reset_counter = 0;
                    return true;
                }
            }
        }

        false
    }

    /// Convert keycode to lowercase character, filtering out non-letters and spaces
    fn keycode_to_char(&self, keycode: Keycode) -> Option<char> {
        match keycode {
            // Letters
            Keycode::A => Some('a'),
            Keycode::B => Some('b'),
            Keycode::C => Some('c'),
            Keycode::D => Some('d'),
            Keycode::E => Some('e'),
            Keycode::F => Some('f'),
            Keycode::G => Some('g'),
            Keycode::H => Some('h'),
            Keycode::I => Some('i'),
            Keycode::J => Some('j'),
            Keycode::K => Some('k'),
            Keycode::L => Some('l'),
            Keycode::M => Some('m'),
            Keycode::N => Some('n'),
            Keycode::O => Some('o'),
            Keycode::P => Some('p'),
            Keycode::Q => Some('q'),
            Keycode::R => Some('r'),
            Keycode::S => Some('s'),
            Keycode::T => Some('t'),
            Keycode::U => Some('u'),
            Keycode::V => Some('v'),
            Keycode::W => Some('w'),
            Keycode::X => Some('x'),
            Keycode::Y => Some('y'),
            Keycode::Z => Some('z'),

            // Ignore all other keys (numbers, symbols, spaces, modifiers, etc.)
            _ => None,
        }
    }

    /// Check if the current input history contains the target sequence
    fn check_sequence_match(&self) -> bool {
        let history_len = self.input_history.len();
        let target_len = self.target_sequence.len();

        if history_len < target_len {
            return false;
        }

        // Check if the last N characters match the target sequence
        for i in 0..target_len {
            let history_idx = history_len - target_len + i;
            if self.input_history[history_idx] != self.target_sequence[i] {
                return false;
            }
        }

        true
    }

    /// Get the current input history as a string (for debugging)
    #[allow(dead_code)]
    pub fn get_history_string(&self) -> String {
        self.input_history.iter().collect()
    }

    /// Get the target sequence as a string (for debugging)
    #[allow(dead_code)]
    pub fn get_target_string(&self) -> String {
        self.target_sequence.iter().collect()
    }

    /// Reset the detector state
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.input_history.clear();
        self.recently_triggered = false;
        self.reset_counter = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_detector_creation() {
        let detector = SequenceDetector::new();
        assert_eq!(detector.get_target_string(), "oppokokoppokosuttenten");
        assert_eq!(detector.get_history_string(), "");
    }

    #[test]
    fn test_keycode_to_char() {
        let detector = SequenceDetector::new();

        // Test letter conversion
        assert_eq!(detector.keycode_to_char(Keycode::A), Some('a'));
        assert_eq!(detector.keycode_to_char(Keycode::Z), Some('z'));

        // Test ignored keys
        assert_eq!(detector.keycode_to_char(Keycode::Space), None);
        assert_eq!(detector.keycode_to_char(Keycode::Enter), None);
        assert_eq!(detector.keycode_to_char(Keycode::Key1), None);
    }

    #[test]
    fn test_sequence_detection() {
        let mut detector = SequenceDetector::new();

        // Input the target sequence
        let sequence = "oppokokoppokosuttenten";
        let mut triggered = false;

        for ch in sequence.chars() {
            let keycode = match ch {
                'a' => Keycode::A,
                'b' => Keycode::B,
                'c' => Keycode::C,
                'd' => Keycode::D,
                'e' => Keycode::E,
                'f' => Keycode::F,
                'g' => Keycode::G,
                'h' => Keycode::H,
                'i' => Keycode::I,
                'j' => Keycode::J,
                'k' => Keycode::K,
                'l' => Keycode::L,
                'm' => Keycode::M,
                'n' => Keycode::N,
                'o' => Keycode::O,
                'p' => Keycode::P,
                'q' => Keycode::Q,
                'r' => Keycode::R,
                's' => Keycode::S,
                't' => Keycode::T,
                'u' => Keycode::U,
                'v' => Keycode::V,
                'w' => Keycode::W,
                'x' => Keycode::X,
                'y' => Keycode::Y,
                'z' => Keycode::Z,
                _ => continue,
            };

            if detector.process_input(keycode) {
                triggered = true;
                break;
            }
        }

        assert!(triggered, "Sequence should have been detected");
    }

    #[test]
    fn test_partial_sequence_no_trigger() {
        let mut detector = SequenceDetector::new();

        // Input partial sequence
        let partial = "oppoko"; // Only first part

        for ch in partial.chars() {
            let keycode = match ch {
                'o' => Keycode::O,
                'p' => Keycode::P,
                'k' => Keycode::K,
                _ => continue,
            };

            let triggered = detector.process_input(keycode);
            assert!(!triggered, "Partial sequence should not trigger");
        }
    }

    #[test]
    fn test_sequence_with_noise() {
        let mut detector = SequenceDetector::new();

        // Input some noise first
        detector.process_input(Keycode::X);
        detector.process_input(Keycode::Y);
        detector.process_input(Keycode::Z);

        // Then input the target sequence
        let sequence = "oppokokoppokosuttenten";
        let mut triggered = false;

        for ch in sequence.chars() {
            let keycode = match ch {
                'a' => Keycode::A,
                'b' => Keycode::B,
                'c' => Keycode::C,
                'd' => Keycode::D,
                'e' => Keycode::E,
                'f' => Keycode::F,
                'g' => Keycode::G,
                'h' => Keycode::H,
                'i' => Keycode::I,
                'j' => Keycode::J,
                'k' => Keycode::K,
                'l' => Keycode::L,
                'm' => Keycode::M,
                'n' => Keycode::N,
                'o' => Keycode::O,
                'p' => Keycode::P,
                'q' => Keycode::Q,
                'r' => Keycode::R,
                's' => Keycode::S,
                't' => Keycode::T,
                'u' => Keycode::U,
                'v' => Keycode::V,
                'w' => Keycode::W,
                'x' => Keycode::X,
                'y' => Keycode::Y,
                'z' => Keycode::Z,
                _ => continue,
            };

            if detector.process_input(keycode) {
                triggered = true;
                break;
            }
        }

        assert!(triggered, "Sequence should be detected even with noise");
    }

    #[test]
    fn test_no_spam_triggering() {
        let mut detector = SequenceDetector::new();

        // Input the sequence twice quickly
        let sequence = "oppokokoppokosuttenten";
        let mut trigger_count = 0;

        // First sequence
        for ch in sequence.chars() {
            let keycode = match ch {
                'o' => Keycode::O,
                'p' => Keycode::P,
                'k' => Keycode::K,
                'e' => Keycode::E,
                'n' => Keycode::N,
                's' => Keycode::S,
                'u' => Keycode::U,
                't' => Keycode::T,
                _ => continue,
            };

            if detector.process_input(keycode) {
                trigger_count += 1;
            }
        }

        // Second sequence immediately
        for ch in sequence.chars() {
            let keycode = match ch {
                'o' => Keycode::O,
                'p' => Keycode::P,
                'k' => Keycode::K,
                'e' => Keycode::E,
                'n' => Keycode::N,
                's' => Keycode::S,
                'u' => Keycode::U,
                't' => Keycode::T,
                _ => continue,
            };

            if detector.process_input(keycode) {
                trigger_count += 1;
            }
        }

        assert_eq!(trigger_count, 1, "Should only trigger once to prevent spam");
    }
}
