# CodeBeats 🎵

Transform your typing into music. Every keystroke becomes a note, creating melodies while you code or write.

## Features

**GUI Interface:**
- Language selection (Python, Rust, JavaScript, etc.)
- Waveform selection (natural, electronic, cyberpunk, fart, etc.)
- Volume and filter controls
- Real-time log display
- Start/stop functionality

**Waveforms:**
- `natural` - Piano with harmonics
- `electronic` - Clean sine wave
- `cyberpunk` - Analog synth atmosphere
- `fart` - Real fart audio samples 💨

**Language Configs:**
- Programming languages: Python, Rust, JavaScript, Java, Go, etc.
- Human languages: English, Chinese, Japanese

## Easter Egg 🥚

Type `oppokokoppokosuttenten` for a special fart sound effect!

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
2. Select language config and waveform
3. Adjust volume and filter settings
4. Click "Start CodeBeats"
5. Start typing - each key produces a musical note!

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
