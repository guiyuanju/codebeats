# CodeBeats 🎵

Transform your typing into music. Every keystroke becomes a note, creating melodies while you code or write.

## Quick Start

### Using the Run Script (Easiest)
```bash
# Launch GUI (default)
./run.sh

# Launch CLI with default settings
./run.sh cli

# Launch CLI with specific options
./run.sh cli -w cyberpunk -v 0.5 -l language_configs/python.json

# Build both versions
./run.sh build
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

### Running the Applications

**Run Script (Recommended):**
```bash
./run.sh          # Launch GUI
./run.sh cli      # Launch CLI
./run.sh help     # Show all options
```

**GUI Version:**
```bash
./run.sh gui
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
./run.sh cli -w natural -v 0.5
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

**Cross-Platform GUI Application:**
- Full graphical interface using `egui` framework
- Dropdown selection for all language configurations  
- Slider controls for volume and filter settings
- Waveform selection with descriptions
- Real-time process management (start/stop functionality)
- Built-in help documentation
- Status monitoring and feedback

**Preserved CLI Interface:**
- Original command-line program remains completely unchanged
- All existing functionality and options preserved
- GUI launches CLI processes with selected parameters
- No runtime modification features (as requested)

**Enhanced User Experience:**
- Simple run script (`./run.sh`) for easy access to both versions
- Automatic discovery of language configuration files
- User-friendly display names for all options
- Process health monitoring
- Cross-platform window management

### 🚀 How to Use

**For Beginners (GUI):**
```bash
./run.sh                    # Launch GUI
# or: cargo run --bin codebeats-gui
```

**For Advanced Users (CLI):**
```bash
./run.sh cli -w cyberpunk -v 0.5
# or: cargo run --bin codebeats -- -w cyberpunk -v 0.5
```

**Build Both Versions:**
```bash
./run.sh build
```

### 🏗️ Architecture Benefits

- **Separation of Concerns**: GUI is a launcher, not a replacement
- **Consistency**: Same behavior between GUI and CLI
- **Maintainability**: No code duplication in audio engine
- **Flexibility**: Advanced users can still use CLI directly
- **Simplicity**: Follows project rule of "prefer simple even stupid code"

The GUI serves as an intuitive configuration interface while preserving all the power and flexibility of your original command-line tool.

---

**Happy coding!** 🎵💻