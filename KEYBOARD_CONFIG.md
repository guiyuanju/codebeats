# Keyboard Configuration Guide

## Overview

CodeBeats now supports fully customizable keyboard mappings! You can customize the sound for each key by editing a JSON configuration file.

## Quick Start

### 1. Generate Default Configuration

```bash
cargo run -- generate-config
```

This creates a `keyboard_config.json` file with the default programming-optimized mappings.

### 2. Edit Configuration File

Edit `keyboard_config.json` to customize your keyboard mappings:

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
    "S": {
      "note": "D4",
      "volume": 0.3,
      "description": "D note for letter S"
    }
  }
}
```

### 3. Start the Program

```bash
cargo run
```

The program will automatically load your custom configuration!

## Configuration File Format

### Basic Structure

```json
{
  "version": "1.0",
  "description": "Description of your configuration",
  "mappings": {
    "KeyName": {
      "note": "Note",
      "volume": 0.0-1.0,
      "description": "Optional description"
    }
  }
}
```

### Field Descriptions

- **version**: Configuration file version
- **description**: Configuration description  
- **mappings**: Key mappings object
  - **KeyName**: Key name (e.g., "A", "Space", "F1")
  - **note**: Note name (e.g., "C4", "F#5", "Bb3")
  - **volume**: Volume (0.0-1.0)
  - **description**: Optional description

## Note Format

Standard note format supported:

- **Basic notes**: C, D, E, F, G, A, B
- **Sharps**: C#, D#, F#, G#, A#
- **Flats**: Db, Eb, Gb, Ab, Bb  
- **Octaves**: 0-9 (e.g., C4 is middle C)

### Example Notes
- `C4` - Middle C (261.63 Hz)
- `A4` - Concert A (440 Hz)
- `C#5` - High C sharp
- `Bb3` - Low B flat

## Key Names

### Letter Keys
`A`, `B`, `C`, ..., `Z`

### Number Keys
`Key0`, `Key1`, `Key2`, ..., `Key9`

### Function Keys
`F1`, `F2`, `F3`, ..., `F12`

### Special Keys
- `Space` - Spacebar
- `Enter` - Enter key
- `Backspace` - Backspace
- `Tab` - Tab key
- `Escape` - Escape key
- `Delete` - Delete key

### Modifier Keys
- `LShift`, `RShift` - Left/Right Shift
- `LControl`, `RControl` - Left/Right Control
- `LAlt`, `RAlt` - Left/Right Alt
- `CapsLock` - Caps Lock

### Arrow Keys
- `Up`, `Down`, `Left`, `Right` - Arrow keys
- `Home`, `End` - Home/End keys
- `PageUp`, `PageDown` - Page Up/Down

### Symbol Keys
- `Semicolon` - Semicolon (;)
- `Comma` - Comma (,)
- `Dot` - Period (.)
- `Slash` - Forward slash (/)
- `BackSlash` - Backslash (\)
- `LeftBracket` - Left bracket ([)
- `RightBracket` - Right bracket (])
- `Apostrophe` - Apostrophe (')
- `Equal` - Equal (=)
- `Minus` - Minus (-)

## Example Configurations

### 1. Piano Layout

Use QWERTY row as white keys:

```bash
cp example_configs/piano_layout.json keyboard_config.json
```

### 2. Minimal Configuration

Configure only the most common keys:

```bash
cp example_configs/minimal.json keyboard_config.json
```

## Volume Recommendations

- **Common letters**: 0.2-0.3
- **Number keys**: 0.2-0.25  
- **Symbol keys**: 0.15-0.2
- **Space/Enter**: 0.1-0.15
- **Modifier keys**: 0.05-0.1

## Music Theory Tips

### Harmonious Intervals
- **Perfect fifth**: C4 + G4
- **Major third**: C4 + E4
- **Minor third**: C4 + Eb4

### Recommended Scales
- **C Major Pentatonic**: C, D, E, G, A
- **C Major Scale**: C, D, E, F, G, A, B
- **A Minor Scale**: A, B, C, D, E, F, G

## Troubleshooting

### Configuration File Not Loading

1. Check JSON syntax is correct
2. Ensure filename is `keyboard_config.json`
3. Check file permissions

### Invalid Notes

Make sure note format is correct:
- Correct: `C4`, `F#5`, `Bb3`  
- Incorrect: `c4`, `F5#`, `B-flat3`

### Keys Not Responding

1. Check key name spelling
2. Ensure volume is not 0
3. Verify system permissions

## Advanced Features

### Dynamic Reloading
Hot reloading is not currently supported, restart required.

### Multiple Configuration Files
You can create multiple configuration files and manually copy to `keyboard_config.json`.

### Custom Waveforms
Configuration files work independently of waveform types:

```bash
cargo run cyberpunk  # Custom keyboard mapping + cyberpunk waveform
```

## Contributing Configurations

Share your keyboard configurations! Add your config files to the `example_configs/` directory.

---

🎵 Enjoy your personalized coding music experience!