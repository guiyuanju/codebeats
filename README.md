# CodeBeats 🎵

Transform your coding workflow into a harmonious musical experience. Every keystroke becomes a note, creating beautiful melodies while you code.

## ✨ Language-Specific Musical Scales

Each programming language uses a **different musical scale** that matches its philosophy:

- **🦀 Rust**: C Minor Pentatonic - Powerful blues for systems programming
- **🌐 JavaScript**: D Mixolydian - Modern jazz for dynamic web development  
- **⚙️ C**: A Natural Minor - Serious classical for low-level precision
- **🐹 Go**: G Major Pentatonic - Clean folk simplicity for efficient code
- **🐍 Python**: F Major - Warm pastoral for accessible programming

## Quick Start

```bash
# Installation
git clone <repository>
cd codebeats
cargo build --release

# Try different language scales
cargo run config language_configs/rust.json        # Powerful minor pentatonic
cargo run config language_configs/javascript.json  # Modern Mixolydian mode
cargo run config language_configs/python.json      # Warm major scale

# Interactive demo
./demo.sh

# Compare all scales
cargo run compare-scales
```

## Features

- **🎼 5 Unique Musical Scales** - Each language has its own tonal character
- **🎹 Smart Key Mapping** - Optimized for programming workflows
- **🎵 Multiple Waveforms** - Electronic, Natural, Saw, Square, Cyberpunk
- **🔊 Comfortable Audio** - No ear fatigue, frequency range optimized for extended use
- **⚡ Real-time Synthesis** - Low-latency audio with ADSR envelopes
- **🌍 Cross-Platform** - Works on macOS, Windows, and Linux

## Scale Comparison

| Language | Scale | Character | Best For |
|----------|-------|-----------|----------|
| Rust | C Minor Pentatonic | Powerful, direct | Systems programming |
| JavaScript | D Mixolydian | Modern, unresolved | Web development |
| C | A Natural Minor | Serious, precise | Low-level programming |
| Go | G Major Pentatonic | Clean, simple | Backend services |
| Python | F Major | Warm, friendly | Scripting, learning |

## Usage

### Basic Commands
```bash
cargo run                                    # Default electronic piano
cargo run natural                          # Natural piano sound
cargo run cyberpunk                        # Futuristic synth
cargo run config language_configs/rust.json # Rust-specific scale
```

### With Waveforms
```bash
cargo run natural config language_configs/python.json    # Warm piano + warm scale
cargo run cyberpunk config language_configs/rust.json    # Futuristic synth + powerful scale
cargo run electronic config language_configs/go.json     # Clean electronic + simple scale
```

### Demo and Analysis
```bash
./demo.sh                    # Interactive demo with all options
cargo run compare-scales     # Detailed musical analysis
cargo run generate-config   # Create custom configuration
```

## macOS Setup

Grant Accessibility permissions:
1. System Preferences → Security & Privacy → Accessibility
2. Add Terminal or your terminal app
3. Restart terminal and run CodeBeats

## Configuration

Create custom mappings by editing language JSON files:
```json
{
  "R": {
    "note": "C4",
    "volume": 0.2,
    "description": "Root note of scale"
  }
}
```

## Audio Specifications

- **Frequency Range**: 65 Hz (C2) to 1047 Hz (C6) - comfortable for extended use
- **Sample Rate**: 44.1 kHz
- **Latency**: < 10ms
- **Volume Range**: 0.05-0.4 (optimized for keyboard typing)

---

## Documentation

- **LANGUAGES.md** - Language scales and musical philosophy
- **CONFIG.md** - Keyboard configuration and customization guide

**Experience programming languages as music. Each language sounds as unique as it codes.** 🎹✨