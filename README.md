# CodeBeats 🎵

Transform your typing into music. Every keystroke becomes a note, creating melodies while you code or write.

## Quick Start

```bash
# Run with default settings
cargo run

# Use a specific language configuration
cargo run -- -l language_configs/python.json

# Adjust volume and waveform
cargo run -- -w cyberpunk -v 0.5
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
```

## Installation

```bash
git clone <repository>
cd sound
cargo build --release
```

**Requirements:** Rust 1.70+, audio output device

## Key Features

- Real-time polyphonic synthesis
- Language-specific musical scales
- ADSR envelope system
- Rate limiting for rapid typing
- Low-pass filtering option
- Cross-platform audio support

---

**Happy coding!** 🎵💻