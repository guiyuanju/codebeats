# CodeBeats 🎵

Transform your typing into music. Every keystroke becomes a note, creating melodies while you code or write.

## Quick Start

### Using the Unified Script (Easiest)
```bash
# Launch GUI (default)
./codebeats

# Launch CLI with default settings
./codebeats cli

# Launch CLI with specific options
./codebeats cli -w cyberpunk -v 0.5 -l language_configs/python.json

# Build both versions
./codebeats build
```

### Direct Cargo Commands
```bash
# GUI version (recommended for beginners)
cargo run --bin codebeats-gui

# CLI version
cargo run --bin codebeats

# CLI with options
cargo run --bin codebeats -- -l language_configs/python.json -w cyberpunk -v 0.5
```

## Language Configurations

### Programming Languages
- `general_programming_language.json` - Balanced programming
- `python.json` - F Major, warm and friendly
- `rust.json` - C Minor Pentatonic, powerful
- `javascript.json` - D Mixolydian, modern
- `java.json` - Structured enterprise
- `haskell.json` - Mathematical precision
- `clojure.json` - Jazz-influenced Lisp
- `c.json` - Serious and precise
- `go.json` - Clean and simple
- `scheme.json` - Minimalist elegance
- `emacs-lisp.json` - Editor-optimized

### Human Languages
- `english.json` - English letter frequency optimized
- `chinese.json` - Pinyin input patterns
- `japanese.json` - Romaji input patterns

## Waveforms

- `natural` - Piano-like with harmonics
- `electronic` - Clean sine wave
- `cyberpunk` - Analog synth atmosphere
- `harmonic` - Mathematical overtones
- `triangle` - Smooth electronic
- `saw` / `square` - Classic electronic
- `fart` - Real fart audio sample playback (effects/fart-quick-short.wav)

## Options

```bash
-l, --language <CONFIG>     Language configuration file
-w, --waveform <WAVEFORM>   Audio waveform type
-v, --volume <LEVEL>        Master volume (0.0-1.0)
--filter-cutoff <HZ>        Low-pass filter (200-8000Hz)
--verbose                   Enable terminal logging for key presses
```

## Examples

```bash
# Python coding with gentle piano sounds
cargo run -- -l language_configs/python.json -w natural -v 0.4

# Atmospheric Rust development
cargo run -- -l language_configs/rust.json -w cyberpunk -v 0.6

# English writing
cargo run -- -l language_configs/english.json -v 0.3

# Chinese input with filtering
cargo run -- -l language_configs/chinese.json --filter-cutoff 800

# Debug mode with verbose logging
cargo run -- --verbose -w electronic

# For the adventurous... 💨 (plays real fart audio file for maximum realism!)
cargo run -- -w fart -v 0.3

# Easter egg: Type "oppokokoppokosuttenten" (Japanese: おっぽこ　こっぽこ　すってんてん)
# to trigger a special fart sound effect! 🎉
```

## Installation

### For Developers
```bash
git clone <repository>
cd sound

# Build both versions
./run.sh build

# Or build manually:
cargo build --release --bin codebeats-gui    # GUI version
cargo build --release --bin codebeats        # CLI version
```

**Requirements:** Rust 1.70+, audio output device

### For End Users (Standalone Packages)

**No Rust installation required!** Download pre-built packages:

- **Windows**: Extract `codebeats-windows-x64.zip`, double-click `codebeats-gui.exe`
- **macOS**: Extract `codebeats-macos-x64.tar.gz`, double-click `codebeats-gui`  
- **Linux**: Extract `codebeats-linux-x64.tar.gz`, run `./codebeats-gui`

Each package includes both GUI and CLI versions with all assets bundled.

### Running the Applications

**Unified Script (Recommended):**
```bash
./codebeats          # Launch GUI
./codebeats cli      # Launch CLI
./codebeats help     # Show all options
```

**GUI Version:**
```bash
./codebeats gui
# or: cargo run --bin codebeats-gui
```
Opens a user-friendly configuration window where you can:
- Select language configurations from dropdowns
- Choose waveforms and adjust volume with sliders
- Enable verbose logging
- Start/stop CodeBeats with a single click
- View built-in help and documentation

**Command-Line Version:**
```bash
./codebeats cli -w natural -v 0.5
# or: cargo run --bin codebeats -- -w natural -v 0.5
```

### Dependencies
- `cpal` - Cross-platform audio library
- `device_query` - Keyboard input detection  
- `hound` - WAV file loading for audio samples
- `serde`/`serde_json` - Configuration file parsing
- `clap` - Command-line argument parsing
- `egui`/`eframe` - Cross-platform GUI framework (GUI version only)
- `tokio` - Async process management (GUI version only)
- `env_logger` - Logging (GUI version only)

## Key Features

- **Cross-platform GUI and CLI interfaces**
- Real-time polyphonic synthesis
- Language-specific musical scales
- ADSR envelope system
- Rate limiting for rapid typing
- Low-pass filtering option
- Cross-platform audio support
- Real audio sample playback (fart waveform uses actual WAV file)
- Easter egg: Hidden sequence detection for special sound effects
- **GUI features:**
  - Easy configuration with dropdowns and sliders
  - Real-time process management
  - Start/stop functionality
  - Built-in help and documentation

## Easter Eggs 🥚

### Japanese Fart Sequence
Type the romaji sequence `oppokokoppokosuttenten` (Japanese: **おっぽこ　こっぽこ　すってんてん**) anywhere in the program to trigger a special fart sound effect! 

- Works in any waveform mode
- Ignores spaces and non-letter keys
- Anti-spam protection prevents repeated triggering
- Use `--verbose` mode to see when it triggers

## Complete Solution Summary

This implementation provides a complete cross-platform GUI solution that preserves your existing command-line interface while adding user-friendly graphical configuration:

### ✅ What's Been Delivered

**Cross-Platform Desktop Application:**
- **GUI Interface**: Full graphical configuration using `egui` framework
- **CLI Interface**: Original command-line program completely preserved
- **Standalone Deployment**: Both interfaces packaged for distribution
- **Cross-Platform Support**: Windows, macOS, and Linux binaries

**Key Features:**
- Dropdown selection for all language configurations  
- Slider controls for volume and filter settings
- Waveform selection with descriptions
- Real-time process management (start/stop functionality)
- Built-in help and documentation
- Automatic discovery of configuration files

### 🚀 How to Use

**For End Users (No Rust Required):**
```bash
# Download and extract platform package, then:
./Start-CodeBeats-GUI.sh    # Double-click in file manager
```

**For Developers:**
```bash
./codebeats                    # Launch GUI
./codebeats cli -w cyberpunk   # Launch CLI with options
```

**Build Deployment Packages:**
```bash
./codebeats package         # Quick single-platform package
./codebeats package-all     # Multi-platform packages
```

### 🏗️ Architecture Benefits

- **Separation of Concerns**: GUI launches CLI processes, no code duplication
- **Consistency**: Identical behavior between GUI and CLI interfaces
- **Maintainability**: Single audio engine, multiple interfaces
- **Deployment Ready**: Standalone packages for end users
- **Developer Friendly**: Full source access and build tools

## Deployment & Distribution

### Quick Deployment (Recommended)

Create a ready-to-use package for your platform:

```bash
./codebeats package
```

This creates `releases/codebeats-[platform].tar.gz` with:
- Both GUI and CLI binaries
- All configuration files and audio assets
- User-friendly launcher scripts (`Start-CodeBeats-GUI.sh`)
- Simple instructions (`HOW-TO-RUN.txt`)

### Multi-Platform Building

Create packages for multiple platforms:

```bash
# Build for current platform
./codebeats build

# Build for specific platforms  
./codebeats build-windows    # Windows x64
./codebeats build-macos      # macOS x64
./codebeats build-linux      # Linux x64

# Build for all platforms
./codebeats build-all
```

### End User Experience

**No installation required!** Users just:

1. **Download**: Get the `.tar.gz` or `.zip` for their platform
2. **Extract**: Unzip/untar the package anywhere
3. **Run**: Double-click `Start-CodeBeats-GUI.sh` (or `.exe` on Windows)
4. **Enjoy**: Configure and start making music!

### Package Contents
- `codebeats-gui` + `Start-CodeBeats-GUI.sh` - Graphical interface
- `codebeats` + `Start-CodeBeats-CLI.sh` - Command-line interface  
- `language_configs/` - All programming language musical scales
- `effects/` - Audio sample files (fart sounds, etc.)
- `HOW-TO-RUN.txt` - Simple user instructions
- `README.md` - Complete documentation

### Distribution Formats
- **Windows**: `codebeats-windows-x64.zip` (~20MB)
- **macOS**: `codebeats-macos-x64.tar.gz` (~15MB)  
- **Linux**: `codebeats-linux-x64.tar.gz` (~18MB)

### Unified Tool

The `./codebeats` script provides all functionality in one place:
- **Run**: Launch GUI or CLI versions
- **Build**: Compile for current or multiple platforms  
- **Package**: Create deployment archives for distribution
- **Utilities**: Clean, test, and check code

See [DEPLOYMENT.md](DEPLOYMENT.md) for detailed deployment instructions and cross-compilation setup.

## Summary

CodeBeats now provides a complete desktop music synthesis solution with both command-line and graphical interfaces. The architecture maintains simplicity while offering professional deployment options for end users who don't need to install development tools.

**Perfect for:**
- **Developers**: Full-featured CLI with all synthesis options
- **End Users**: Easy-to-use GUI with one-click deployment
- **Distribution**: Professional standalone packages for all platforms

---

**Happy coding!** 🎵💻