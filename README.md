# CodeBeats 🎵

Transform your typing into music. Every keystroke becomes a note, creating melodies while you code or write.

## Features

**GUI Interface:**
- Dynamic language selection (automatically detects all available configurations)
- Dynamic waveform selection (automatically detects all available waveforms)  
- Volume and filter controls
- Real-time log display
- Start/stop functionality
- Refresh button to reload available options

**Waveforms:**
- `natural` - Piano with harmonics and vibrato
- `electronic` - Clean sine wave
- `cyberpunk` - Blade Runner 2049 style analog synthesizer
- `saw` - Bright sawtooth wave for electronic music
- `square` - Retro 8-bit square wave
- `triangle` - Smooth triangular wave
- `fart` - Realistic fart sound synthesis 💨
- `bass` - Deep bass with rich low frequencies

**Programming Language Configs:**
- C, C++, C#, Go, Rust, Python, JavaScript, TypeScript
- Java, Kotlin, Swift, PHP, Ruby, Clojure, Haskell
- Scheme, Emacs Lisp, General Programming

**Human Language Configs:**
- English, Chinese, Japanese, Spanish, French, German

## Easter Egg 🥚

Type `oppokokoppokosuttenten` for a special fart sound effect!

## Complete Feature List 📋

**8 Distinctive Waveforms:**
Each waveform has unique sonic characteristics and ADSR envelope settings:
- `natural` - Piano with harmonics and subtle vibrato
- `electronic` - Clean sine wave for precise tones
- `cyberpunk` - Multi-oscillator analog synth with LFO modulation
- `saw` - Bright electronic sawtooth with rich harmonics
- `square` - Classic 8-bit retro square wave
- `triangle` - Smooth triangular wave for mellow tones
- `fart` - Realistic body resonance with formant filtering 💨
- `bass` - Deep bass with powerful sub-frequencies and analog warmth

**24 Language Configurations:**
- **Programming Languages** (16): C, C++, C#, TypeScript, Swift, PHP, Ruby, Kotlin, plus Python, Rust, JavaScript, Java, Go, Clojure, Haskell, Scheme, Emacs Lisp, General Programming
- **Human Languages** (6): English, Chinese, Japanese, Spanish, French, German
- Each configuration maps keyboard keys to musically meaningful notes based on letter frequency and language-specific characteristics

## Build Requirements

- Rust 1.70+

## Architecture 🏗️

CodeBeats uses a clean three-layer architecture:

**Layer 1: Core Library** (`src/lib.rs`)
- Audio engine and synthesis
- Keyboard mapping and configuration
- Waveform generation
- Real-time audio processing

**Layer 2: CLI Interface** (`src/main.rs`)
- Command-line tool using the core library
- Subcommands: `run`, `list-waveforms`, `list-configs`, `validate-config`, etc.
- Provides clean interface for automation and GUI integration

**Layer 3: GUI Interface** (`src/gui.rs`)
- Graphical interface that uses the CLI layer
- Spawns CLI processes for all operations
- Provides user-friendly configuration and control
- Maintains separation of concerns

This architecture ensures clean separation: GUI → CLI → Library, with each layer having clear responsibilities.

## Building macOS App 🍎

To create a native macOS application bundle:

```bash
./build_macos_app.sh
```

### Build Options

```bash
# Quick build (current architecture)
./build_macos_app.sh

# Debug build (faster compilation)
./build_macos_app.sh --debug

# Universal binary (Intel + Apple Silicon)
./build_macos_app.sh --universal
```

The script creates a complete `CodeBeats.app` bundle with:
- GUI and CLI executables
- All language configurations and sound effects  
- Proper code signing and permissions
- Optional DMG installer

For detailed instructions, see [BUILD_MACOS.md](BUILD_MACOS.md)

### Installation
```bash
# Install to Applications
cp -r CodeBeats.app /Applications/

# Or double-click to run directly
open CodeBeats.app
```

## Usage

### GUI Mode
1. Launch `codebeats-gui` 
2. Select language config and waveform (automatically loaded from CLI)
3. Use the 🔄 Refresh button to reload if you add new configurations
4. Adjust volume and filter settings
5. Click "Start CodeBeats"
6. Start typing - each key produces a musical note!

### CLI Mode
```bash
# Run interactively with default settings
cargo run --bin codebeats

# Run with specific configuration
cargo run --bin codebeats run --waveform cyberpunk --language rust.json --volume 0.7

# List available options
cargo run --bin codebeats list-waveforms
cargo run --bin codebeats list-configs

# Validate a configuration
cargo run --bin codebeats validate-config language_configs/python.json

# Test audio system
cargo run --bin codebeats test-audio
```

The three-layer design ensures the GUI uses the CLI, which uses the core library, maintaining clean separation and making each component testable and reusable.
