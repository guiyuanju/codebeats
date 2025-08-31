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

- `--waveform`: natural, electronic, saw, square, cyberpunk
- `--language`: Path to language config (python.json, rust.json, etc.)
- `--config`: Custom keyboard config file
- `--volume`: Master volume (0.0-1.0)

## Language Scales

| Language | Scale | Character |
|----------|-------|-----------|
| Python | F Major | Warm, friendly |
| Rust | C Minor Pentatonic | Powerful, direct |
| JavaScript | D Mixolydian | Modern, dynamic |
| C | A Natural Minor | Serious, precise |
| Go | G Major Pentatonic | Clean, simple |

## Commands

```bash
# Generate default config
cargo run -- generate-config

# Compare scales
cargo run -- compare-scales
```

## Examples

```bash
# Cyberpunk Rust
cargo run -- -w cyberpunk -l language_configs/rust.json -v 0.8

# Quiet Python
cargo run -- -w natural -l language_configs/python.json -v 0.3

# Custom config
cargo run -- -c my_config.json -v 0.5
```
