# Piano Keyboard Sound Simulator

A minimal Rust-based piano simulator that maps your keyboard keys to musical notes, creating a harmonious typing experience optimized for programming workflows.

## Features

- **Real-time Keyboard Mapping**: Every key on your keyboard is mapped to a musical note
- **Programming-Optimized**: Common programming keys are mapped to pleasant pentatonic scales
- **Volume-Aware**: Key frequency determines volume (common keys are quieter)
- **Cross-Platform Audio**: Uses `cpal` for low-latency audio output
- **Ultra-Minimal**: Pure core functionality, no complex features

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

```bash
cargo run
```

This starts the global keyboard listener. Every key you press will play a musical note in real-time.

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
│   ├── main.rs         # Complete application (< 300 lines)
│   └── test_keys.rs    # Utility for testing key detection
├── Cargo.toml          # Dependencies
└── README.md           # This file
```

The entire core functionality is contained in a single, focused file with no external complexity.

## Usage Tips

- **Volume Control**: Use your system volume to adjust overall loudness
- **Key Combinations**: Hold multiple keys for chords
- **Programming Flow**: Let the music enhance your coding rhythm
- **Focus Mode**: Common keys are intentionally quiet to not break concentration

## Contributing

The codebase is intentionally minimal:
- All functionality in `main.rs` 
- No testing framework or complex features
- Easy to understand and modify
- Direct mapping functions for easy customization

## License

[Add your license here]