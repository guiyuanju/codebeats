# CodeBeats 🎵

Programming music simulator with language-specific scales and real-time audio synthesis.

## Quick Start

```bash
# Basic usage
cargo run

# With options
cargo run -- --waveform cyberpunk --language language_configs/rust.json --volume 0.7

# Help
cargo run -- --help
```

## Options

- `--waveform`: natural, electronic, saw, square, cyberpunk (overrides language config)
- `--language`: Path to language config (python.json, rust.json, etc.)
- `--config`: Custom keyboard config file (optional - built-in default available)
- `--volume`: Master volume (0.0-1.0)

## Configuration System

### Built-in Default Configuration
- **No setup required** - programming-optimized keyboard mapping included
- Uses electronic waveform and pentatonic scale by default
- Automatically loads if no config files are specified

### Waveform Priority
1. **CLI argument** (`--waveform`) - highest priority
2. **Language config** (specified in .json files) - medium priority  
3. **Built-in default** (electronic) - lowest priority

## Language Scales

| Language | Scale | Waveform | Character |
|----------|-------|----------|-----------|
| Python | F Major | Natural | Warm, friendly |
| Rust | C Minor Pentatonic | Electronic | Powerful, direct |
| JavaScript | D Mixolydian | Cyberpunk | Modern, dynamic |
| C | A Natural Minor | Saw | Serious, precise |
| Go | G Major Pentatonic | Square | Clean, simple |

## Commands

```bash
# Compare language-specific scales
cargo run -- compare-scales

# No configuration needed - built-in defaults work out of the box
cargo run
```

## Examples

```bash
# Cyberpunk Rust
cargo run -- -w cyberpunk -l language_configs/rust.json -v 0.8

# Quiet Python
cargo run -- -w natural -l language_configs/python.json -v 0.3

# Full volume
cargo run -- -w electronic -l language_configs/rust.json -v 1.0

# Custom config
cargo run -- -c my_config.json -v 0.5
```
