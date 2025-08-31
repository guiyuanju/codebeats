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
cargo build --release
```

**Requirements:** Rust 1.70+, audio output device

### Dependencies
- `cpal` - Cross-platform audio library
- `device_query` - Keyboard input detection
- `hound` - WAV file loading for audio samples
- `serde`/`serde_json` - Configuration file parsing
- `clap` - Command-line argument parsing

## Key Features

- Real-time polyphonic synthesis
- Language-specific musical scales
- ADSR envelope system
- Rate limiting for rapid typing
- Low-pass filtering option
- Cross-platform audio support
- Real audio sample playback (fart waveform uses actual WAV file)
- Easter egg: Hidden sequence detection for special sound effects

## Easter Eggs 🥚

### Japanese Fart Sequence
Type the romaji sequence `oppokokoppokosuttenten` (Japanese: **おっぽこ　こっぽこ　すってんてん**) anywhere in the program to trigger a special fart sound effect! 

- Works in any waveform mode
- Ignores spaces and non-letter keys
- Anti-spam protection prevents repeated triggering
- Use `--verbose` mode to see when it triggers

---

**Happy coding!** 🎵💻