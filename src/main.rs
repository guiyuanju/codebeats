//! CodeBeats - Programming Music Simulator
//!
//! Transform your coding workflow into a harmonious musical experience.
//! Every keystroke becomes a note, creating beautiful melodies while you code.
//!
//! Copyright (C) 2024 jgy
//!
//! This program is free software: you can redistribute it and/or modify
//! it under the terms of the GNU General Public License as published by
//! the Free Software Foundation, either version 3 of the License, or
//! (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU General Public License for more details.
//!
//! You should have received a copy of the GNU General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.

use clap::{Parser, Subcommand};
use codebeats::{CodeBeatsConfig, CodeBeatsEngine, Waveform, embedded_configs};

#[derive(Parser)]
#[command(
    name = "codebeats",
    about = "Programming Music Simulator - Transform your typing into music!",
    version = "0.1.0"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Language configuration to use
    #[arg(short, long, default_value = "general")]
    language: String,

    /// Waveform type to use
    #[arg(short, long)]
    waveform: Option<String>,

    /// Master volume (0.0-1.0)
    #[arg(short = 'v', long = "volume", default_value = "1.0")]
    volume: f32,

    /// Enable verbose logging
    #[arg(long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// List available language configurations
    ListConfigs,
    /// List available waveforms
    ListWaveforms,
    /// Show version information
    Version,
}

fn list_configs() {
    println!("Available language configurations:");
    println!();

    let configs = embedded_configs::get_config_names();
    for name in configs {
        if let Some(description) = embedded_configs::get_config_description(name) {
            println!("  {:<15} - {}", name, description);
        } else {
            println!("  {}", name);
        }
    }
    println!();
    println!("Usage: codebeats --language <config_name>");
}

fn list_waveforms() {
    println!("Available waveforms:");
    println!();

    let waveforms = vec![
        ("natural", "Piano-like with harmonics"),
        ("electronic", "Clean sine wave"),
        ("cyberpunk", "Blade Runner 2049 style analog synthesizer"),
        ("saw", "Bright sawtooth wave for electronic music"),
        ("square", "Retro 8-bit square wave"),
        ("triangle", "Smooth triangular wave"),
        ("fart", "Realistic fart sound synthesis"),
        ("bass", "Deep bass with rich low frequencies"),
    ];

    for (name, description) in &waveforms {
        println!("  {:<12} - {}", name, description);
    }
    println!();
    println!("Usage: codebeats --waveform <waveform_name>");
}

fn show_version() {
    println!("CodeBeats v{}", env!("CARGO_PKG_VERSION"));
    println!("Programming Music Simulator");
    println!("Transform your typing into music!");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Handle subcommands
    match &cli.command {
        Some(Commands::ListConfigs) => {
            list_configs();
            return Ok(());
        }
        Some(Commands::ListWaveforms) => {
            list_waveforms();
            return Ok(());
        }
        Some(Commands::Version) => {
            show_version();
            return Ok(());
        }
        None => {
            // Continue to main program
        }
    }

    // Validate and clamp volume
    let volume = cli.volume.clamp(0.0, 1.0);
    if cli.volume != volume && cli.verbose {
        println!("⚠️  Volume clamped to {:.1} (valid range: 0.0-1.0)", volume);
    }

    // Load keyboard configuration
    let keyboard_config = if embedded_configs::config_exists(&cli.language) {
        match embedded_configs::load_config(&cli.language) {
            Ok(config) => {
                if cli.verbose {
                    println!(
                        "✓ Loaded '{}' configuration: {}",
                        cli.language, config.description
                    );
                }
                config
            }
            Err(e) => {
                eprintln!("✗ Failed to load '{}' configuration: {}", cli.language, e);
                eprintln!("Using default configuration instead.");
                embedded_configs::get_default_config()?
            }
        }
    } else {
        eprintln!("✗ Configuration '{}' not found.", cli.language);
        eprintln!("Use 'codebeats list-configs' to see available configurations.");
        std::process::exit(1);
    };

    // Parse waveform with priority: CLI arg > config file > default
    let waveform = if let Some(cli_waveform) = &cli.waveform {
        // User explicitly specified waveform via CLI
        cli_waveform.parse().unwrap_or_else(|_| {
            eprintln!(
                "✗ Unknown waveform '{}', using 'electronic' instead.",
                cli_waveform
            );
            eprintln!("Use 'codebeats list-waveforms' to see available waveforms.");
            Waveform::Electronic
        })
    } else if let Some(config_waveform) = keyboard_config.get_waveform() {
        // Use waveform from config file
        if cli.verbose {
            println!("✓ Using waveform '{}' from configuration", config_waveform);
        }
        config_waveform
    } else {
        // Default fallback
        Waveform::Electronic
    };

    if cli.verbose {
        println!(
            "🔊 Audio settings: waveform={}, volume={:.1}",
            waveform, volume
        );
    }

    // Create configuration
    let config = CodeBeatsConfig {
        waveform,
        keyboard_config,
        volume,
        filter_cutoff: 1200.0,
        verbose: cli.verbose,
    };

    // Create and run the engine
    let mut engine = CodeBeatsEngine::new(config)?;

    if !cli.verbose {
        println!("🎵 CodeBeats started - Press Ctrl+C to exit");
    }

    engine.run_blocking()?;

    Ok(())
}
