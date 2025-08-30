# Piano Keyboard Sound Simulator

A Rust-based piano simulator that maps your keyboard keys to musical notes, creating a harmonious typing experience optimized for programming workflows.

## Features

- **Real-time Keyboard Mapping**: Every key on your keyboard is mapped to a musical note
- **Programming-Optimized**: Common programming keys are mapped to pleasant pentatonic scales
- **Volume-Aware**: Key frequency determines volume (common keys are quieter)
- **Cross-Platform Audio**: Uses `cpal` for low-latency audio output
- **Simple Testing Framework**: Built-in tests for audio functionality

## Quick Start

### Prerequisites

- Rust (latest stable version)
- Audio output device
- On macOS: Accessibility permissions for global key detection

### Installation

```bash
git clone <repository>
cd sound
cargo build --release
```

### Usage

#### Interactive Mode (Real-time keyboard piano)
```bash
cargo run
```
This starts the global keyboard listener. Every key you press will play a musical note.

#### Testing Mode
```bash
# Run all tests
cargo run test

# Test specific functionality
cargo run scale          # Play C major scale
cargo run chords         # Play chord progression
cargo run arpeggio       # Play arpeggio pattern
cargo run octaves        # Test different octaves
cargo run keyboard       # Test keyboard mapping

# Play specific notes/chords
cargo run note C4 1000                    # Play C4 for 1 second
cargo run chord C4,E4,G4 800             # Play C major chord for 800ms
```

## Keyboard Mapping

The keyboard is mapped to create pleasant harmonies during programming:

### Common Letters (Pentatonic Scale - Pleasant & Harmonious)
- **Most frequent**: `E=E4`, `T=G4`, `A=C4`, `O=D4`, `I=A4` - Middle range, moderate volume
- **Very common**: `N=E5`, `S=G5`, `H=C5`, `R=D5` - Higher octave
- **Common**: `L=F4`, `U=A3`, `D=F5`, `C=B4`, `M=B3` - Mixed range

### Numbers (Same harmonic pattern as letters)
- `0-9` mapped to the same notes as common letters for consistency

### Programming Symbols (Gentle harmonics)
- `;`, `[`, `]`, `,`, `.`, `/`, `\`, `'`, `=`, `-` - All in comfortable range

### Special Keys (Quieter to avoid disruption)
- **Space, Enter, Tab, Backspace** - Very quiet bass notes
- **Arrow keys, Home/End** - Comfortable low range
- **Modifiers (Shift, Ctrl, Alt)** - Barely audible bass
- **Function keys** - Bright harmonics for special actions

## Technical Details

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

## Testing Framework

The built-in testing framework provides:

### Available Tests
- **Scale Test**: Plays C major scale to verify note accuracy
- **Chord Test**: Tests polyphonic capabilities with basic chord progressions
- **Arpeggio Test**: Tests rapid note transitions
- **Octave Test**: Verifies frequency calculation across octaves
- **Keyboard Test**: Validates key-to-note mapping

### Custom Tests
```bash
# Play any note with custom duration
cargo run note <note> [duration_ms]
# Examples:
cargo run note C4 500
cargo run note F#5 1000
cargo run note Bb3 800

# Play any chord with custom duration  
cargo run chord <note1,note2,note3> [duration_ms]
# Examples:
cargo run chord C4,E4,G4 1000
cargo run chord F4,A4,C5,E5 1500
```

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
- **Minimal Complexity**: Simple testing interface replaces complex DSL
- **Real-time Performance**: Low latency for responsive audio feedback

## Project Structure

```
sound/
├── src/
│   ├── main.rs         # Main application with audio engine and tests
│   └── test_keys.rs    # Utility for testing key detection
├── Cargo.toml          # Dependencies
└── README.md           # This file
```

## Contributing

The codebase is designed to be simple and focused:
- Core audio functionality in `main.rs`
- Minimal testing framework for validation
- No complex DSL or scripting - just direct function calls
- Easy to extend with new test cases or keyboard mappings

## License

[Add your license here]