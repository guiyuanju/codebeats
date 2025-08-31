use serde_json::Value;
use std::collections::HashMap;
use std::fs;

/// Musical scale analysis for different programming languages
#[derive(Debug, Clone)]
pub struct Scale {
    pub name: String,
    pub notes: Vec<String>,
    pub description: String,
}

/// Language configuration with its musical characteristics
#[derive(Debug)]
pub struct LanguageScale {
    pub language: String,
    pub scale: Scale,
    pub key_mappings: Vec<(String, String)>, // key -> note
}

pub struct ScaleComparison;

impl ScaleComparison {
    /// Load and analyze all language configurations
    pub fn analyze_all_languages() -> Result<Vec<LanguageScale>, Box<dyn std::error::Error>> {
        let config_dir = "language_configs";
        let mut language_scales = Vec::new();

        // Define the scales used by each language
        let scales = Self::get_language_scales();

        // Read each language config file
        for entry in fs::read_dir(config_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map_or(false, |ext| ext == "json") {
                let language_name = path.file_stem().unwrap().to_str().unwrap().to_string();

                let content = fs::read_to_string(&path)?;
                let config: Value = serde_json::from_str(&content)?;

                let scale = scales
                    .get(&language_name)
                    .cloned()
                    .unwrap_or_else(|| Scale {
                        name: "Unknown".to_string(),
                        notes: vec!["Unknown".to_string()],
                        description: "Scale not defined".to_string(),
                    });

                let key_mappings = Self::extract_key_mappings(&config);

                language_scales.push(LanguageScale {
                    language: language_name,
                    scale,
                    key_mappings,
                });
            }
        }

        Ok(language_scales)
    }

    /// Get the musical scales defined for each language
    fn get_language_scales() -> HashMap<String, Scale> {
        let mut scales = HashMap::new();

        // Rust - C Minor Pentatonic (power and directness)
        scales.insert(
            "rust".to_string(),
            Scale {
                name: "C Minor Pentatonic".to_string(),
                notes: vec![
                    "C".to_string(),
                    "Eb".to_string(),
                    "F".to_string(),
                    "G".to_string(),
                    "Bb".to_string(),
                ],
                description: "Systems programming power - emphasizes strength and directness"
                    .to_string(),
            },
        );

        // JavaScript - D Mixolydian (modern and flexible)
        scales.insert(
            "javascript".to_string(),
            Scale {
                name: "D Mixolydian".to_string(),
                notes: vec![
                    "D".to_string(),
                    "E".to_string(),
                    "F#".to_string(),
                    "G".to_string(),
                    "A".to_string(),
                    "B".to_string(),
                    "C".to_string(),
                ],
                description:
                    "Dynamic web development - flexible and modern with characteristic flat seventh"
                        .to_string(),
            },
        );

        // C - A Natural Minor (serious and precise)
        scales.insert(
            "c".to_string(),
            Scale {
                name: "A Natural Minor".to_string(),
                notes: vec![
                    "A".to_string(),
                    "B".to_string(),
                    "C".to_string(),
                    "D".to_string(),
                    "E".to_string(),
                    "F".to_string(),
                    "G".to_string(),
                ],
                description:
                    "Systems programming foundation - serious and precise for low-level control"
                        .to_string(),
            },
        );

        // Go - G Major Pentatonic (simple and efficient)
        scales.insert(
            "go".to_string(),
            Scale {
                name: "G Major Pentatonic".to_string(),
                notes: vec![
                    "G".to_string(),
                    "A".to_string(),
                    "B".to_string(),
                    "D".to_string(),
                    "E".to_string(),
                ],
                description: "Simple and efficient - clean and clear like Go's philosophy"
                    .to_string(),
            },
        );

        // Python - F Major (friendly and readable)
        scales.insert(
            "python".to_string(),
            Scale {
                name: "F Major".to_string(),
                notes: vec![
                    "F".to_string(),
                    "G".to_string(),
                    "A".to_string(),
                    "Bb".to_string(),
                    "C".to_string(),
                    "D".to_string(),
                    "E".to_string(),
                ],
                description:
                    "Friendly and readable - warm major scale reflecting Python's accessibility"
                        .to_string(),
            },
        );

        scales
    }

    /// Extract key to note mappings from a language config
    fn extract_key_mappings(config: &Value) -> Vec<(String, String)> {
        let mut mappings = Vec::new();

        if let Some(mappings_obj) = config.get("mappings").and_then(|m| m.as_object()) {
            for (key, value) in mappings_obj {
                if let Some(note) = value.get("note").and_then(|n| n.as_str()) {
                    mappings.push((key.clone(), note.to_string()));
                }
            }
        }

        // Sort by note for better visualization
        mappings.sort_by(|a, b| Self::note_to_number(&a.1).cmp(&Self::note_to_number(&b.1)));
        mappings
    }

    /// Convert note name to number for sorting (simplified)
    fn note_to_number(note: &str) -> i32 {
        let note_values = [
            ("C", 0),
            ("C#", 1),
            ("Db", 1),
            ("D", 2),
            ("D#", 3),
            ("Eb", 3),
            ("E", 4),
            ("F", 5),
            ("F#", 6),
            ("Gb", 6),
            ("G", 7),
            ("G#", 8),
            ("Ab", 8),
            ("A", 9),
            ("A#", 10),
            ("Bb", 10),
            ("B", 11),
        ];

        // Extract octave and note name
        let octave = note
            .chars()
            .last()
            .and_then(|c| c.to_digit(10))
            .unwrap_or(4) as i32;
        let note_name = &note[..note.len() - 1];

        let base_value = note_values
            .iter()
            .find(|(name, _)| *name == note_name)
            .map(|(_, value)| *value)
            .unwrap_or(0);

        octave * 12 + base_value
    }

    /// Print a comprehensive comparison of all language scales
    pub fn print_comparison(language_scales: &[LanguageScale]) {
        println!("🎵 Programming Language Musical Scale Comparison 🎵\n");
        println!("{}", "=".repeat(80));

        for lang_scale in language_scales {
            println!("\n📝 {} Configuration", lang_scale.language.to_uppercase());
            println!("{}", "-".repeat(50));
            println!("Scale: {}", lang_scale.scale.name);
            println!("Notes: {}", lang_scale.scale.notes.join(" - "));
            println!("Philosophy: {}", lang_scale.scale.description);

            println!("\nKey Mapping Examples (first 10):");
            for (i, (key, note)) in lang_scale.key_mappings.iter().take(10).enumerate() {
                println!("  {}: {} → {}", i + 1, key, note);
            }

            if lang_scale.key_mappings.len() > 10 {
                println!(
                    "  ... and {} more mappings",
                    lang_scale.key_mappings.len() - 10
                );
            }
        }

        println!("\n\n🎼 Scale Characteristics Summary 🎼");
        println!("{}", "=".repeat(80));

        for lang_scale in language_scales {
            println!(
                "• {}: {} - {}",
                lang_scale.language.to_uppercase(),
                lang_scale.scale.name,
                Self::get_scale_mood(&lang_scale.scale.name)
            );
        }

        println!("\n💡 Usage Tips:");
        println!(
            "- Each language uses a different musical scale to match its programming philosophy"
        );
        println!("- Rust (C Minor Pentatonic): Power and strength for systems programming");
        println!("- JavaScript (D Mixolydian): Modern and flexible for web development");
        println!("- C (A Natural Minor): Serious and precise for low-level programming");
        println!("- Go (G Major Pentatonic): Simple and clean like Go's design");
        println!("- Python (F Major): Warm and friendly for accessible programming");
    }

    /// Get the mood/character of a musical scale
    fn get_scale_mood(scale_name: &str) -> &'static str {
        match scale_name {
            "C Minor Pentatonic" => "Powerful, direct, bluesy",
            "D Mixolydian" => "Modern, jazzy, slightly unresolved",
            "A Natural Minor" => "Serious, melancholic, precise",
            "G Major Pentatonic" => "Simple, bright, folk-like",
            "F Major" => "Warm, friendly, pastoral",
            _ => "Neutral",
        }
    }

    /// Analyze note distribution for a language
    pub fn analyze_note_distribution(lang_scale: &LanguageScale) {
        println!(
            "\n🔍 Note Distribution Analysis for {}",
            lang_scale.language.to_uppercase()
        );
        println!("{}", "-".repeat(60));

        let mut note_count: HashMap<String, usize> = HashMap::new();

        for (_, note) in &lang_scale.key_mappings {
            // Extract base note (without octave)
            let base_note = note
                .chars()
                .take_while(|c| !c.is_ascii_digit())
                .collect::<String>();
            *note_count.entry(base_note).or_insert(0) += 1;
        }

        // Sort by frequency
        let mut sorted_notes: Vec<_> = note_count.into_iter().collect();
        sorted_notes.sort_by(|a, b| b.1.cmp(&a.1));

        println!("Most frequently mapped notes:");
        for (i, (note, count)) in sorted_notes.iter().take(8).enumerate() {
            let percentage = (*count as f32 / lang_scale.key_mappings.len() as f32) * 100.0;
            println!(
                "  {}: {} (used {} times, {:.1}%)",
                i + 1,
                note,
                count,
                percentage
            );
        }
    }

    /// Compare scales between two languages
    pub fn compare_languages(lang1: &LanguageScale, lang2: &LanguageScale) {
        println!(
            "\n🎭 Scale Comparison: {} vs {}",
            lang1.language.to_uppercase(),
            lang2.language.to_uppercase()
        );
        println!("{}", "=".repeat(70));

        println!("{}: {}", lang1.language, lang1.scale.name);
        println!("  Notes: {}", lang1.scale.notes.join(" - "));
        println!("  Character: {}", Self::get_scale_mood(&lang1.scale.name));

        println!("{}: {}", lang2.language, lang2.scale.name);
        println!("  Notes: {}", lang2.scale.notes.join(" - "));
        println!("  Character: {}", Self::get_scale_mood(&lang2.scale.name));

        // Find common notes
        let common_notes: Vec<_> = lang1
            .scale
            .notes
            .iter()
            .filter(|note| lang2.scale.notes.contains(note))
            .collect();

        if !common_notes.is_empty() {
            println!(
                "\nCommon notes: {}",
                common_notes
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        } else {
            println!("\nNo common notes - completely different tonal centers!");
        }

        // Analyze complexity
        println!("\nScale complexity:");
        println!(
            "  {}: {} notes ({})",
            lang1.language,
            lang1.scale.notes.len(),
            if lang1.scale.notes.len() == 5 {
                "pentatonic"
            } else {
                "heptatonic"
            }
        );
        println!(
            "  {}: {} notes ({})",
            lang2.language,
            lang2.scale.notes.len(),
            if lang2.scale.notes.len() == 5 {
                "pentatonic"
            } else {
                "heptatonic"
            }
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_to_number_conversion() {
        assert_eq!(ScaleComparison::note_to_number("C4"), 48);
        assert_eq!(ScaleComparison::note_to_number("A4"), 57);
        assert_eq!(ScaleComparison::note_to_number("Bb4"), 58);
    }

    #[test]
    fn test_scale_definitions() {
        let scales = ScaleComparison::get_language_scales();

        // Verify all expected languages have scales
        assert!(scales.contains_key("rust"));
        assert!(scales.contains_key("javascript"));
        assert!(scales.contains_key("c"));
        assert!(scales.contains_key("go"));
        assert!(scales.contains_key("python"));

        // Verify pentatonic scales have 5 notes
        assert_eq!(scales["rust"].notes.len(), 5);
        assert_eq!(scales["go"].notes.len(), 5);

        // Verify heptatonic scales have 7 notes
        assert_eq!(scales["javascript"].notes.len(), 7);
        assert_eq!(scales["c"].notes.len(), 7);
        assert_eq!(scales["python"].notes.len(), 7);
    }
}

/// Example usage function
pub fn demonstrate_scale_system() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎼 Loading language scale configurations...\n");

    let language_scales = ScaleComparison::analyze_all_languages()?;

    // Print comprehensive comparison
    ScaleComparison::print_comparison(&language_scales);

    // Analyze individual languages
    for lang_scale in &language_scales {
        ScaleComparison::analyze_note_distribution(lang_scale);
    }

    // Compare specific language pairs
    if language_scales.len() >= 2 {
        let rust_config = language_scales.iter().find(|l| l.language == "rust");
        let js_config = language_scales.iter().find(|l| l.language == "javascript");

        if let (Some(rust), Some(js)) = (rust_config, js_config) {
            ScaleComparison::compare_languages(rust, js);
        }

        let c_config = language_scales.iter().find(|l| l.language == "c");
        let python_config = language_scales.iter().find(|l| l.language == "python");

        if let (Some(c), Some(python)) = (c_config, python_config) {
            ScaleComparison::compare_languages(c, python);
        }
    }

    println!("\n🎯 Summary:");
    println!(
        "Successfully implemented different musical scales for {} programming languages!",
        language_scales.len()
    );
    println!("Each language now has its own unique tonal character that matches its programming philosophy.");

    Ok(())
}
