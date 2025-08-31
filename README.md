# CodeBeats 🎵

CodeBeats is a programming-optimized keyboard music simulator that transforms your coding workflow into a harmonious musical experience. Every keystroke becomes a note, creating beautiful melodies while you code.

## Features

- **🎹 Smart Keyboard Mapping**: Every key mapped to musical notes based on coding frequency
- **⌨️ Programming-Optimized**: Common keys use pleasant pentatonic scales, quiet enough not to distract
- **🎵 Multiple Waveforms**: Electronic (default), Natural piano, Saw, Square, and Cyberpunk synth
- **🔊 Volume Intelligence**: Frequently used keys are quieter to maintain coding flow
- **⚡ Real-time Audio**: Low-latency audio synthesis with ADSR envelopes
- **🌍 Cross-Platform**: Works on macOS, Windows, and Linux
- **🎚️ Command-line Control**: Set your preferred waveform at startup
- **⌘ Mac Command Key Support**: Auto-detects and maps Mac Command keys (⌘) to gentle bass notes
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
# Default (Electronic waveform)
cargo run

# Or choose a specific waveform
cargo run natural      # Natural piano with harmonics
cargo run electronic   # Pure sine wave (default)
cargo run saw          # Bright sawtooth wave
cargo run square       # Retro 8-bit square wave
cargo run cyberpunk    # Blade Runner 2049 style analog synth
```

This starts the global keyboard listener. Every key you press will play a musical note in real-time.

### 🆕 New Features

#### Mac Command Key Support ⌘
- Mac Command keys are automatically detected and mapped to subtle bass notes
- Works alongside other modifier keys without disrupting your workflow
- Compatible with Windows keys and Meta keys on other platforms

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
- **Focus Mode**: Common keys are intentionally quiet to not break concentration
- **Waveform Switching**: Press F8-F12 to change sound in real-time

### 🆕 Enhanced Vim Experience
- **Normal navigation**: `j`, `k`, `h`, `l` play pleasant notes
- **Rapid navigation**: `jjjj` automatically gets quieter to prevent ear fatigue
- **Mixed movement**: Each key tracked independently for optimal experience
- **No configuration needed**: Rate limiting works automatically

### Command Key Usage
- **Mac**: Command key (⌘) plays subtle D2 bass note
- **Windows**: Windows key works similarly  
- **Detection**: Command keys are automatically detected when pressed

## Contributing

The codebase balances simplicity with rich functionality:
- Core audio engine and waveform synthesis in `main.rs`
- Advanced ADSR envelope system
- Multiple waveform types with distinct characteristics  
- Easy to understand architecture
- Extensible waveform system for adding new sounds
- See `WAVEFORM_GUIDE.md` for detailed technical documentation

## License

[Add your license here]