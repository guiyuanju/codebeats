# CodeBeats 🎵

A programming music simulator that transforms your coding workflow into a harmonious musical experience. Every keystroke becomes a note, creating beautiful melodies while you code.

## Features

- **Real-time Audio Synthesis**: Low-latency polyphonic sound generation
- **Multiple Waveforms**: Electronic, Natural piano, Saw, Square, and Cyberpunk synth
- **Smart Key Detection**: Supports both physical keys (`1`, `[`) and shifted keys (`!`, `{`)
- **Programming-Optimized Mapping**: Keyboard layout based on coding key frequency analysis
- **Language-Specific Scales**: Different musical scales for different programming languages
- **ADSR Envelope System**: Natural attack/decay/sustain/release for realistic sound
- **Smart Rate Limiting**: Volume gradually decreases for rapid successive key presses

## Quick Start

```bash
# Basic usage with default settings
cargo run

# With specific waveform and language
cargo run -- --waveform cyberpunk --language language_configs/rust.json --volume 0.7

# With low-pass filter to reduce harsh high frequencies
cargo run -- --filter-cutoff 800 --volume 0.7

# See all options
cargo run -- --help
```

## Waveforms

| Waveform | Description | Best For |
|----------|-------------|----------|
| `electronic` | Pure sine wave, clean sound | Default, general use |
| `natural` | Piano-like with harmonics | Warm, organic feel |
| `cyberpunk` | Blade Runner style analog synth | Atmospheric coding |
| `saw` | Bright sawtooth wave | Electronic music feel |
| `square` | Retro 8-bit sound | Nostalgic programming |
| `harmonic` | Mathematical harmonic series | Haskell, mathematical precision |
| `sine` | Pure sine wave (alias for electronic) | Clean, minimal sound |
| `sawtooth` | Sawtooth wave (alias for saw) | Bright electronic sound |
| `triangle` | Smooth triangle wave | Gentle electronic sound |

## Language Configurations

### Programming Languages

| Language | Scale | Character | Config File |
|----------|-------|-----------|-------------|
| General | Pentatonic | Balanced, pleasant | `general_programming_language.json` |
| Python | F Major | Warm, friendly | `python.json` |
| Rust | C Minor Pentatonic | Powerful, direct | `rust.json` |
| JavaScript | D Mixolydian | Modern, dynamic | `javascript.json` |
| C | A Natural Minor | Serious, precise | `c.json` |
| Go | G Major Pentatonic | Clean, simple | `go.json` |
| Haskell | Sophisticated Harmonic | Mathematical precision | `haskell.json` |
| Java | Structured Enterprise | Systematic, corporate | `java.json` |
| Clojure | Jazz-influenced | Lisp elegance with modern touches | `clojure.json` |
| Scheme | Classical Lisp | Minimalist, elegant | `scheme.json` |
| Emacs Lisp | Editor-optimized | Extensibility focus | `emacs-lisp.json` |

### Human Languages

| Language | Scale | Character | Config File |
|----------|-------|-----------|-------------|
| English | Letter Frequency Optimized | Based on English letter patterns | `english.json` |
| Chinese | Pinyin-optimized | Focus on Pinyin input patterns | `chinese.json` |
| Japanese | Romaji-optimized | Japanese phonetic patterns | `japanese.json` |

## Examples

```bash
# Cyberpunk Rust experience
cargo run -- -w cyberpunk -l language_configs/rust.json -v 0.8

# Quiet Python coding with gentle filtering
cargo run -- -w natural -l language_configs/python.json -v 0.3 --filter-cutoff 1000

# Functional Haskell with sophisticated harmonies
cargo run -- -l language_configs/haskell.json -v 0.6

# Jazz-influenced Clojure coding
cargo run -- -l language_configs/clojure.json -v 0.7

# English writing with natural sounds
cargo run -- -l language_configs/english.json -v 0.5

# Chinese/Pinyin input with optimized mapping
cargo run -- -l language_configs/chinese.json -v 0.6

# Japanese/Romaji input
cargo run -- -l language_configs/japanese.json -v 0.6

# Custom configuration with filtering
cargo run -- -c my_custom_config.json --filter-cutoff 600
```

## Shifted Key Support

The system intelligently detects shifted characters, creating richer musical experiences:

- **Number pairs**: `1`→C4 vs `!`→C6 (octave relationship)
- **Bracket pairs**: `[`→A5 vs `{`→A6 (harmonic step up)
- **Punctuation**: `;`→D6 vs `:`→D7 (emphasis relationship)

Try typing code with lots of symbols to hear the musical relationships:
```javascript
function test() { return [1, 2, 3]; }
if (x > 0) { print("Hello World!"); }
```

## Configuration Priority

1. **CLI argument** (`--waveform`) - highest priority
2. **Language config file** - medium priority  
3. **Built-in default** (electronic) - fallback

## Musical Design

The keyboard mappings use music theory principles:

- **Common keys** (E, T, A, O, I, N, S, R) get pleasant mid-range notes
- **Less common keys** get higher or lower notes to avoid frequency clashes
- **Shifted characters** typically play higher octaves of their base keys
- **Programming symbols** create harmonic relationships (brackets, operators)
- **Rate limiting** reduces volume by 30% for each rapid successive press within 500ms

## Commands

| Command | Description |
|---------|-------------|
| `cargo run` | Start with default settings |
| `cargo run -- --help` | Show all available options |

## Options

- `--waveform` / `-w`: Choose waveform (natural, electronic, saw, square, cyberpunk, harmonic, sine, sawtooth, triangle)
- `--language` / `-l`: Path to language configuration file
- `--config` / `-c`: Custom keyboard mapping file
- `--volume` / `-v`: Master volume (0.0-1.0, default: 1.0)
- `--filter-cutoff`: Low-pass filter cutoff frequency in Hz (200-8000, default: 1200)

## Requirements

- Rust 1.70+
- Audio output device
- Keyboard input

## Installation

```bash
git clone <repository>
cd sound
cargo build --release
cargo run -- --help
```

## Project Structure

```
src/
├── main.rs              # Main application and CLI
├── audio_engine.rs      # Audio synthesis and ADSR envelopes
├── keyboard_config.rs   # Keyboard configuration management
├── keyboard_mapping.rs  # Key-to-note mapping logic
└── waveforms.rs         # Waveform synthesis algorithms

language_configs/        # Pre-built language configurations
├── general_programming_language.json
├── python.json
├── rust.json
├── javascript.json
├── c.json
├── go.json
├── haskell.json
├── java.json
├── clojure.json
├── scheme.json
├── emacs-lisp.json
├── english.json
├── chinese.json
└── japanese.json
```

## Tips

- Use headphones or speakers for the best experience
- Start with lower volume (0.3-0.5) and adjust to preference
- Try different language configs to find your preferred musical style
- Experiment with shifted keys (`!@#$%^&*(){}:"<>?`) for richer melodies
- Use `--filter-cutoff` to reduce harsh high frequencies (try 800-1000Hz for gentler sound)
- All frequency mappings have been optimized for comfort - no more harsh high-pitched sounds!
- **Rate limiting** automatically prevents audio overload during rapid typing - volume decreases smoothly with repeated key presses

## Troubleshooting

- **No sound**: Check audio device and volume settings
- **"Unmapped key"**: Key isn't in current config (this is normal for some keys)
- **Too loud/quiet**: Use `--volume` flag to adjust
- **Config not loading**: Check file path and JSON syntax
- **Sound too harsh/shrill**: Lower the filter cutoff frequency (try `--filter-cutoff 800`)

---

**Happy coding and music making!** 🎵💻✨