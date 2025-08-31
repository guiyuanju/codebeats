# CodeBeats 🎵

CodeBeats is a programming-optimized keyboard music simulator that transforms your coding workflow into a harmonious musical experience. Every keystroke becomes a note, creating beautiful melodies while you code.

## Features

- **🎹 Customizable Keyboard Mapping**: Fully configurable keyboard-to-sound mappings via JSON files
- **⌨️ Programming-Optimized**: Default config uses pleasant pentatonic scales, quiet enough not to distract
- **🎵 Multiple Waveforms**: Electronic (default), Natural piano, Saw, Square, and Cyberpunk synth
- **🔊 Volume Intelligence**: Configurable volume levels for each key
- **⚡ Real-time Audio**: Low-latency audio synthesis with ADSR envelopes
- **🌍 Cross-Platform**: Works on macOS, Windows, and Linux
- **🎚️ Command-line Control**: Set your preferred waveform at startup
- **📝 Easy Configuration**: Generate and edit JSON config files to customize any key mapping
- **🎛️ Smart Rate Limiting**: Prevents high-pitched sounds from rapid key presses (perfect for vim users!)

## Quick Start

### Prerequisites

- Rust (latest stable version)
- Audio output device
- On macOS: Accessibility permissions for global key detection

### Installation

```bash
git clone <repository>
cd codebeats
cargo build --release
```

### Usage

```bash
# Default (Electronic waveform with default keyboard mapping)
cargo run

# Or choose a specific waveform
cargo run natural      # Natural piano with harmonics
cargo run electronic   # Pure sine wave (default)
cargo run saw          # Bright sawtooth wave
cargo run square       # Retro 8-bit square wave
cargo run cyberpunk    # Blade Runner 2049 style analog synth

# Keyboard configuration commands
cargo run -- generate-config           # Generate default keyboard_config.json
cargo run config <config_file>         # Use specific config file
cargo run config example_configs/piano_layout.json  # Use piano layout

# Language-specific configurations
cargo run config language_configs/rust.json        # Rust systems programming
cargo run config language_configs/javascript.json  # Web development  
cargo run config language_configs/go.json          # Go simplicity
cargo run config language_configs/c.json           # C foundation
```

This starts the global keyboard listener. Every key you press will play a musical note in real-time based on your configuration.

### 🆕 New Features

#### Fully Customizable Keyboard Mapping 📝
- **JSON Configuration**: Edit `keyboard_config.json` to customize any key mapping
- **Multiple Layouts**: Switch between different keyboard configurations
- **Easy Setup**: `cargo run -- generate-config` creates a customizable config file
- **Live Examples**: Pre-built configs for piano layout, minimal setup, and more
- **Note Flexibility**: Map any key to any musical note (C4, F#5, Bb3, etc.)
- **Volume Control**: Set individual volume levels for each key (0.0-1.0)

#### Smart Rate Limiting for Vim Users 🎛️
- **Problem**: Rapid key presses (like `jjjj` for vim navigation) create annoying high-pitched sounds
- **Solution**: Progressive volume reduction for rapid same-key presses
  - 1st press: 100% volume
  - 2nd rapid press: 70% volume  
  - 3rd rapid press: 40% volume
  - 4th-5th rapid press: 20% volume
  - 6+ rapid presses: Silent until rate slows down
- **Recovery**: Returns to normal volume after 1 second of no activity
- **Per-key**: Each key has independent rate limiting

#### Hold Duration Reduction 🕒
- **Problem**: Holding down keys (like holding 'j' to scroll) also creates repetitive sounds
- **Solution**: Progressive volume reduction for long key holds
  - 0-0.5s: Normal volume (100%)
  - 0.5-1s: Slightly reduced (80%)
  - 1-2s: More reduced (60%)
  - 2-3s: Significant reduction (40%)
  - 3-5s: Very quiet (20%)
  - 5s+: Almost silent (10%)
- **Status updates**: Shows hold duration every 2 seconds for long-held keys



### Real-time Waveform Switching

While the program is running, press these keys to change waveforms:
- **F8** - Cyberpunk 2049 (atmospheric analog synth)
- **F9** - Natural Piano (rich harmonics)
- **F10** - Electronic (pure sine wave)
- **F11** - Saw Wave (bright electronic)
- **F12** - Square Wave (retro gaming)

## 📝 Keyboard Configuration

### Quick Setup
```bash
# Generate default configuration file
cargo run -- generate-config

# Edit the generated keyboard_config.json file
# Then run normally - config is loaded automatically
cargo run
```

### Configuration File Format
```json
{
  "version": "1.0",
  "description": "My custom keyboard layout",
  "mappings": {
    "A": {
      "note": "C4",
      "volume": 0.3,
      "description": "Middle C for letter A"
    },
    "Space": {
      "note": "C3",
      "volume": 0.1,
      "description": "Quiet bass note"
    }
  }
}
```

### Using Different Configurations
```bash
# Use a specific config file
cargo run config example_configs/piano_layout.json

# Combine with waveforms
cargo run cyberpunk config example_configs/minimal.json
```

### Available Example Configurations
- **`example_configs/piano_layout.json`**: Traditional piano layout with QWERTY row as white keys
- **`example_configs/minimal.json`**: Simple configuration with just essential keys
- **Default**: Programming-optimized layout with pentatonic scales

### Language-Specific Configurations
- **`language_configs/rust.json`**: Rust systems programming - Emphasis on ownership operators (&, *), lifetimes ('), and macros (!)
- **`language_configs/javascript.json`**: Web development harmony - Arrow functions (=>), template literals (`), and object syntax
- **`language_configs/go.json`**: Go simplicity - Clean intervals for Go's minimalist syntax, distinctive := operator
- **`language_configs/c.json`**: C foundation - Deep bass for pointers (*), bright preprocessor (#), structural braces

### Supported Note Format
- **Basic notes**: C, D, E, F, G, A, B
- **Sharps**: C#, D#, F#, G#, A#
- **Flats**: Db, Eb, Gb, Ab, Bb
- **Octaves**: 0-9 (e.g., C4 = middle C, A4 = 440Hz)

### Key Names Reference
- **Letters**: A, B, C, ..., Z
- **Numbers**: Key0, Key1, ..., Key9
- **Special**: Space, Enter, Backspace, Tab, Escape
- **Modifiers**: LShift, RShift, LControl, RControl, LAlt, RAlt
- **Functions**: F1, F2, ..., F12
- **Navigation**: Up, Down, Left, Right, Home, End, PageUp, PageDown
- **Symbols**: Semicolon, Comma, Dot, Slash, LeftBracket, RightBracket, etc.

## 🎵 Waveform Types

### 1. 🎹 Natural Piano (Default)
- **Character**: Warm, acoustic piano-like
- **Technology**: Complex harmonic series with subtle vibrato
- **Best For**: General coding, classical feel, long sessions
- **ADSR**: Quick attack (20ms), natural decay, moderate sustain

### 2. ⚡ Electronic 
- **Character**: Clean, pure, mathematical
- **Technology**: Pure sine wave, no harmonics
- **Best For**: Electronic music, sound design, precision work
- **ADSR**: Instant attack (10ms), clean sustain, short release

### 3. 🔥 Saw Wave
- **Character**: Bright, cutting, modern
- **Technology**: Linear sawtooth with rich high frequencies  
- **Best For**: EDM production, high-energy coding
- **ADSR**: Punchy attack (5ms), quick decay, clean release

### 4. 🟫 Square Wave
- **Character**: Retro, 8-bit gaming nostalgia
- **Technology**: 50% duty cycle square wave
- **Best For**: Game development, chiptune music
- **ADSR**: Instant attack, punchy sustain, classic 8-bit feel

### 5. 🌃 Cyberpunk 2049
- **Character**: Atmospheric, sci-fi, analog warmth
- **Technology**: Multi-oscillator synth with LFO, PWM, chorus
- **Best For**: Sci-fi projects, night coding, immersive atmosphere
- **ADSR**: Slow pad attack (80ms), long atmospheric release (400ms)

## Default Keyboard Mapping

The default configuration creates pleasant harmonies during programming:

### Programming-Optimized Layout
- **Common letters**: Use pentatonic scales in middle range with moderate volume
- **Numbers**: Mapped to the same harmonic pattern as letters for consistency
- **Programming symbols**: Gentle harmonics in comfortable frequency ranges
- **Special keys**: Quieter bass notes to avoid disruption
- **Modifiers**: Very quiet bass tones that don't interfere with workflow

### Customization Philosophy
- **Frequency-based**: Common coding keys get pleasant, non-disruptive notes
- **Volume-aware**: Frequently used keys are quieter to maintain focus
- **Harmonically designed**: Keys that are often pressed together create pleasant chords
- **Fully customizable**: Every aspect can be changed via the configuration file

*Note: The exact mappings are fully customizable. Use `cargo run -- generate-config` to see and modify the complete default layout.*

## ⚙️ Technical Details

### Audio System
- **Sample Rate**: Uses system default (typically 44.1kHz)
- **Waveform**: Pure sine waves for clean tones
- **Polyphony**: Unlimited simultaneous notes
- **Latency**: Low-latency real-time audio using `cpal`

### Note Calculation
- Uses standard A4=440Hz tuning
- Supports full piano range (C1 to C8)
- Mathematical frequency calculation: `f = 440 * 2^((n-69)/12)`
- Supports both sharps (#) and flats (b) notation

### Dependencies
- `cpal`: Cross-platform audio library
- `device_query`: Global keyboard input detection

## macOS Setup

For global keyboard detection on macOS:

1. Go to **System Preferences > Security & Privacy > Privacy > Accessibility**
2. Click the lock and enter your password
3. Add your terminal application (Terminal.app, iTerm2, etc.)
4. Restart the program

## Design Philosophy

This simulator is designed for:
- **Coding Ambiance**: Create pleasant background tones while programming
- **Non-Disruptive**: Common keys are quieter to maintain focus
- **Musically Harmonic**: Uses music theory to ensure pleasant combinations
- **Ultra-Minimal**: Pure core functionality without any complexity
- **Real-time Performance**: Low latency for responsive audio feedback

## Project Structure

```
sound/
├── src/
│   ├── main.rs         # Complete application (~500 lines)
│   └── test_keys.rs    # Utility for testing key detection
├── Cargo.toml          # Dependencies
├── README.md           # This file
└── WAVEFORM_GUIDE.md   # Detailed waveform documentation
```

## 🎛️ Advanced Features

### ADSR Envelope System
Each waveform has carefully tuned Attack/Decay/Sustain/Release parameters:
- **Attack**: How quickly the note reaches full volume
- **Decay**: How quickly it drops to sustain level  
- **Sustain**: The held volume level
- **Release**: How quickly it fades after key release

### Anti-Aliasing & Quality
- Independent phase tracking for each note
- Exponential envelope curves for natural sound
- Soft saturation for analog warmth (Cyberpunk mode)
- No audio crackling or popping

### Performance
- Real-time audio with <10ms latency
- Unlimited polyphony (play as many keys simultaneously as you want)
- Efficient synthesis optimized for coding workflows
- Low CPU usage even with complex waveforms

## Usage Tips

### Waveform Selection
- **Long coding sessions**: Natural (least fatiguing)
- **Creative/artistic work**: Cyberpunk (inspiring atmosphere)  
- **Testing/debugging**: Electronic (clear, precise feedback)
- **Game development**: Square (thematic 8-bit match)
- **Music production**: Saw (versatile, bright character)

### General Usage
- **Volume Control**: Use your system volume to adjust overall loudness
- **Key Combinations**: Hold multiple keys for chords
- **Programming Flow**: Let the music enhance your coding rhythm
- **Custom Layouts**: Create different configs for different projects or moods
- **Waveform Switching**: Press F8-F12 to change sound in real-time

### Configuration Tips
- **Volume Levels**: 0.2-0.3 for common keys, 0.1-0.15 for special keys, 0.05-0.1 for modifiers
- **Note Selection**: Use pentatonic scales (C, D, E, G, A) for harmonious combinations
- **Testing**: Press keys while editing config to hear changes immediately
- **Backup**: Keep multiple config files for different use cases

### 🆕 Enhanced Vim Experience
- **Customizable**: Map vim navigation keys (j, k, h, l) to your preferred notes
- **Rate limiting**: Rapid key presses automatically get quieter
- **Independent tracking**: Each key has its own rate limiting
- **Easy setup**: Default config already optimized for coding workflows

## Contributing

The codebase balances simplicity with rich functionality:
- Core audio engine and waveform synthesis in `main.rs`
- Configurable keyboard mapping system in `src/keyboard/`
- JSON-based configuration with full validation
- Advanced ADSR envelope system
- Multiple waveform types with distinct characteristics  
- Easy to understand architecture
- Extensible configuration system
- Example configurations for different use cases
- Language-specific configurations optimized for syntax patterns
- See `KEYBOARD_CONFIG.md` for configuration documentation
- See `LANGUAGE_CONFIGS.md` for language-specific design philosophy

## License

[Add your license here]