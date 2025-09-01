//! Embedded Configuration Files and Audio Effects
//!
//! This module contains all language configurations and audio effects embedded at compile time.
//! This ensures the binary is self-contained and doesn't require external files.

use crate::keyboard_config::KeyboardConfig;
use once_cell::sync::Lazy;
use std::collections::HashMap;

// Embed all configuration files at compile time
static CONFIGS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut configs = HashMap::new();

    // Programming languages
    configs.insert("c", include_str!("../language_configs/c.json"));
    configs.insert("cpp", include_str!("../language_configs/cpp.json"));
    configs.insert("csharp", include_str!("../language_configs/csharp.json"));
    configs.insert("clojure", include_str!("../language_configs/clojure.json"));
    configs.insert(
        "emacs-lisp",
        include_str!("../language_configs/emacs-lisp.json"),
    );
    configs.insert("go", include_str!("../language_configs/go.json"));
    configs.insert("haskell", include_str!("../language_configs/haskell.json"));
    configs.insert("java", include_str!("../language_configs/java.json"));
    configs.insert(
        "javascript",
        include_str!("../language_configs/javascript.json"),
    );
    configs.insert("kotlin", include_str!("../language_configs/kotlin.json"));
    configs.insert("php", include_str!("../language_configs/php.json"));
    configs.insert("python", include_str!("../language_configs/python.json"));
    configs.insert("ruby", include_str!("../language_configs/ruby.json"));
    configs.insert("rust", include_str!("../language_configs/rust.json"));
    configs.insert("scheme", include_str!("../language_configs/scheme.json"));
    configs.insert("swift", include_str!("../language_configs/swift.json"));
    configs.insert(
        "typescript",
        include_str!("../language_configs/typescript.json"),
    );

    // Natural languages
    configs.insert("chinese", include_str!("../language_configs/chinese.json"));
    configs.insert("english", include_str!("../language_configs/english.json"));
    configs.insert("french", include_str!("../language_configs/french.json"));
    configs.insert("german", include_str!("../language_configs/german.json"));
    configs.insert(
        "japanese",
        include_str!("../language_configs/japanese.json"),
    );
    configs.insert("spanish", include_str!("../language_configs/spanish.json"));

    // General
    configs.insert(
        "general",
        include_str!("../language_configs/general_programming_language.json"),
    );

    configs
});

// Embed audio effects at compile time
static FART_AUDIO_DATA: &[u8] = include_bytes!("../effects/fart-quick-short.wav");

/// Get embedded fart audio data
pub fn get_fart_audio_data() -> &'static [u8] {
    FART_AUDIO_DATA
}

/// Get all available configuration names
pub fn get_config_names() -> Vec<&'static str> {
    let mut names: Vec<_> = CONFIGS.keys().copied().collect();
    names.sort();
    names
}

/// Load a keyboard configuration by name
pub fn load_config(name: &str) -> Result<KeyboardConfig, Box<dyn std::error::Error>> {
    let json_content = CONFIGS
        .get(name)
        .ok_or_else(|| format!("Configuration '{}' not found", name))?;

    let config: KeyboardConfig = serde_json::from_str(json_content)?;
    Ok(config)
}

/// Get default configuration (general programming language)
pub fn get_default_config() -> Result<KeyboardConfig, Box<dyn std::error::Error>> {
    load_config("general")
}

/// Check if a configuration exists
pub fn config_exists(name: &str) -> bool {
    CONFIGS.contains_key(name)
}

/// Get configuration description
pub fn get_config_description(name: &str) -> Option<String> {
    if let Ok(config) = load_config(name) {
        Some(config.description)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio_samples::AudioSample;

    #[test]
    fn test_embedded_fart_audio_data() {
        let fart_data = get_fart_audio_data();

        // Check that we have some data
        assert!(!fart_data.is_empty(), "Fart audio data should not be empty");

        // Check that it's a valid WAV file by trying to load it
        let audio_sample = AudioSample::load_from_bytes(fart_data);
        assert!(
            audio_sample.is_ok(),
            "Should be able to load fart audio from embedded data"
        );

        if let Ok(sample) = audio_sample {
            assert!(sample.samples.len() > 0, "Audio sample should have data");
            assert!(sample.sample_rate > 0, "Sample rate should be positive");
            assert!(sample.channels > 0, "Should have at least one channel");

            println!(
                "✓ Embedded fart audio: {} samples, {}Hz, {} channels",
                sample.samples.len(),
                sample.sample_rate,
                sample.channels
            );
        }
    }
}
