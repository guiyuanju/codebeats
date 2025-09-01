//! CodeBeats CLI - Programming Music Simulator
//!
//! Transform your coding workflow into a harmonious musical experience.
//! Every keystroke becomes a note, creating beautiful melodies while you code.

use clap::{Parser, Subcommand};
use codebeats::{CodeBeatsConfig, CodeBeatsEngine, KeyboardConfig, Waveform};
use serde_json;

#[derive(Parser)]
#[command(
    name = "codebeats",
    about = "Programming Music Simulator",
    version = "0.1.0"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    // Global options for backward compatibility
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

    #[arg(
        long = "filter-cutoff",
        value_name = "FREQUENCY",
        help = "Low-pass filter cutoff frequency in Hz (200-8000, default: 1200)"
    )]
    filter_cutoff: Option<f32>,

    #[arg(long = "verbose", help = "Enable verbose terminal logging")]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Run CodeBeats interactively
    Run {
        #[arg(short, long)]
        waveform: Option<String>,

        #[arg(short = 'l', long = "language")]
        language_config: Option<String>,

        #[arg(short, long)]
        config: Option<String>,

        #[arg(short = 'v', long = "volume", value_name = "LEVEL")]
        volume: Option<f32>,

        #[arg(long = "filter-cutoff", value_name = "FREQUENCY")]
        filter_cutoff: Option<f32>,

        #[arg(long = "verbose")]
        verbose: bool,
    },
    /// List available waveforms
    ListWaveforms,
    /// List available language configurations
    ListConfigs,
    /// Validate a configuration file
    ValidateConfig {
        #[arg(value_name = "FILE")]
        config_path: String,
    },
    /// Get version information
    Version,
    /// Test audio system
    TestAudio,
}

fn load_keyboard_config(config_path: Option<&str>, verbose: bool) -> KeyboardConfig {
    if let Some(path) = config_path {
        match CodeBeatsEngine::load_keyboard_config(path) {
            Ok(config) => {
                if verbose {
                    println!("✓ Loaded keyboard config from: {}", path);
                }
                return config;
            }
            Err(e) => {
                if verbose {
                    println!("✗ Failed to load config from {}: {}", path, e);
                }
            }
        }
    } else if let Ok(config) =
        CodeBeatsEngine::load_keyboard_config("language_configs/general_programming_language.json")
    {
        if verbose {
            println!("✓ Loaded default programming language config");
        }
        return config;
    }

    if verbose {
        println!("✓ Using built-in default keyboard config");
    }
    KeyboardConfig::default()
}

fn list_waveforms() {
    println!("Available waveforms:");
    let waveforms = [
        ("natural", "Piano-like with harmonics"),
        ("electronic", "Clean sine wave"),
        ("cyberpunk", "Analog synthesizer atmosphere"),
        ("harmonic", "Mathematical overtone series"),
        ("triangle", "Triangle wave"),
        ("saw", "Sawtooth wave"),
        ("square", "Square wave"),
        ("fart", "Real fart audio sample"),
    ];

    for (name, description) in &waveforms {
        println!("  {:<12} - {}", name, description);
    }
}

fn list_configs() {
    println!("Available language configurations:");

    // Try to list config files from language_configs directory
    if let Ok(entries) = std::fs::read_dir("language_configs") {
        for entry in entries.flatten() {
            if let Some(filename) = entry.file_name().to_str() {
                if filename.ends_with(".json") {
                    let display_name = filename
                        .strip_suffix(".json")
                        .unwrap_or(filename)
                        .replace("_", " ")
                        .replace("-", " ");
                    println!("  {:<30} - {}", display_name, filename);
                }
            }
        }
    } else {
        println!("  No language_configs directory found.");
        println!("  Using built-in default configuration.");
    }
}

fn validate_config(config_path: &str) {
    match CodeBeatsEngine::load_keyboard_config(config_path) {
        Ok(config) => {
            println!("✓ Configuration is valid");
            println!("  Description: {}", config.description);
            println!("  Note mappings: {} keys configured", config.mappings.len());

            // Output config details as JSON for GUI consumption
            if let Ok(json) = serde_json::to_string_pretty(&config) {
                println!("\nConfiguration details:");
                println!("{}", json);
            }
        }
        Err(e) => {
            eprintln!("✗ Configuration is invalid: {}", e);
            std::process::exit(1);
        }
    }
}

fn show_version() {
    println!("CodeBeats v{}", env!("CARGO_PKG_VERSION"));
    println!("Programming Music Simulator");
    println!("Transform your typing into music!");
}

fn test_audio() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing audio system...");

    let config = CodeBeatsConfig {
        waveform: Waveform::Electronic,
        keyboard_config: KeyboardConfig::default(),
        volume: 0.5,
        filter_cutoff: 1200.0,
        verbose: true,
    };

    let _engine = CodeBeatsEngine::new(config)?;
    println!("✓ Audio system initialized successfully");
    println!("✓ Engine created without errors");

    // Test that we can access audio devices
    println!("✓ Audio test completed");
    Ok(())
}

fn run_codebeats(
    waveform: Option<String>,
    language_config: Option<String>,
    config: Option<String>,
    volume: Option<f32>,
    filter_cutoff: Option<f32>,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let master_volume = volume.unwrap_or(1.0).clamp(0.0, 1.0);
    let filter_cutoff = filter_cutoff.unwrap_or(1200.0).clamp(200.0, 8000.0);
    let keyboard_config =
        load_keyboard_config(config.as_deref().or(language_config.as_deref()), verbose);

    if verbose {
        println!(
            "🔊 Audio settings: volume={:.1}, filter={:.0}Hz",
            master_volume, filter_cutoff
        );
    }

    // Determine waveform: CLI arg > language config > default
    let waveform = if let Some(waveform_str) = &waveform {
        Waveform::from_str(waveform_str).unwrap_or(Waveform::Electronic)
    } else if let Some(config_waveform) = keyboard_config.get_waveform() {
        config_waveform
    } else {
        Waveform::Electronic
    };

    // Create configuration
    let config = CodeBeatsConfig {
        waveform,
        keyboard_config,
        volume: master_volume,
        filter_cutoff,
        verbose,
    };

    // Create and run the engine
    let mut engine = CodeBeatsEngine::new(config)?;
    engine.run_blocking()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Run {
            waveform,
            language_config,
            config,
            volume,
            filter_cutoff,
            verbose,
        }) => {
            run_codebeats(
                waveform.clone(),
                language_config.clone(),
                config.clone(),
                *volume,
                *filter_cutoff,
                *verbose,
            )?;
        }
        Some(Commands::ListWaveforms) => {
            list_waveforms();
        }
        Some(Commands::ListConfigs) => {
            list_configs();
        }
        Some(Commands::ValidateConfig { config_path }) => {
            validate_config(config_path);
        }
        Some(Commands::Version) => {
            show_version();
        }
        Some(Commands::TestAudio) => {
            test_audio()?;
        }
        None => {
            // Backward compatibility: run with global options if no subcommand
            run_codebeats(
                cli.waveform,
                cli.language_config,
                cli.config,
                cli.volume,
                cli.filter_cutoff,
                cli.verbose,
            )?;
        }
    }

    Ok(())
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
        assert_eq!(Waveform::from_str("fart"), Some(Waveform::Fart));

        // Test invalid waveform
        assert_eq!(Waveform::from_str("invalid"), None);
    }

    #[test]
    fn test_keyboard_config_loading() {
        let keyboard_config = KeyboardConfig::default();
        assert!(!keyboard_config.description.is_empty());
    }

    #[test]
    fn test_cli_parsing() {
        use clap::Parser;

        // Test run subcommand
        let cli =
            Cli::try_parse_from(&["codebeats", "run", "--verbose", "--volume", "0.5"]).unwrap();
        match cli.command {
            Some(Commands::Run {
                verbose, volume, ..
            }) => {
                assert!(verbose);
                assert_eq!(volume, Some(0.5));
            }
            _ => panic!("Expected Run command"),
        }

        // Test list-waveforms subcommand
        let cli = Cli::try_parse_from(&["codebeats", "list-waveforms"]).unwrap();
        matches!(cli.command, Some(Commands::ListWaveforms));
    }
}
