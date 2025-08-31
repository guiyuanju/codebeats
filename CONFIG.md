# Configuration Guide

## Quick Start

```bash
# Use language-specific scales
cargo run config language_configs/rust.json        # C Minor Pentatonic
cargo run config language_configs/javascript.json  # D Mixolydian
cargo run config language_configs/python.json      # F Major
cargo run config language_configs/go.json          # G Major Pentatonic
cargo run config language_configs/c.json           # A Natural Minor

# Generate custom configuration
cargo run generate-config

# Combine with waveforms
cargo run cyberpunk config language_configs/rust.json
```

## Configuration File Format

```json
{
  "version": "1.0",
  "description": "Custom keyboard layout",
  "mappings": {
    "A": {
      "note": "C4",
      "volume": 0.3,
      "description": "Middle C for letter A"
    },
    "Space": {
      "note": "C2",
      "volume": 0.1,
      "description": "Quiet bass foundation"
    }
  }
}
```

## Key Names

### Letters and Numbers
- **Letters**: A, B, C, ..., Z
- **Numbers**: Key0, Key1, ..., Key9

### Special Keys
- **Basic**: Space, Enter, Backspace, Tab, Escape
- **Modifiers**: LShift, RShift, LControl, RControl, LAlt, RAlt
- **Functions**: F1, F2, ..., F12
- **Navigation**: Up, Down, Left, Right, Home, End, PageUp, PageDown

### Symbols (with Shift handling)
- **Semicolon**: `;` and `:` (Shift+;)
- **Comma**: `,` and `<` (Shift+,)
- **Dot**: `.` and `>` (Shift+.)
- **Slash**: `/` and `?` (Shift+/)
- **LeftBracket**: `[` and `{` (Shift+[)
- **RightBracket**: `]` and `}` (Shift+])
- **Equal**: `=` and `+` (Shift+=)
- **Minus**: `-` and `_` (Shift+-)
- **Apostrophe**: `'` and `"` (Shift+')
- **Grave**: `` ` `` and `~` (Shift+`)
- **BackSlash**: `\` and `|` (Shift+\)

## Note Format

### Supported Notes
- **Natural**: C, D, E, F, G, A, B
- **Sharps**: C#, D#, F#, G#, A#
- **Flats**: Db, Eb, Gb, Ab, Bb
- **Octaves**: 0-9 (C4 = middle C = 261.63 Hz)

### Frequency Guidelines
- **Comfortable range**: C2 (65 Hz) to C6 (1047 Hz)
- **Sweet spot**: C3-C5 (130-523 Hz)
- **Avoid**: Notes above C6 (too shrill for extended use)

## Volume Guidelines

- **Whitespace** (Space, Tab): 0.05-0.15 - barely audible structure
- **Common letters**: 0.15-0.25 - comfortable for frequent use
- **Operators/symbols**: 0.25-0.35 - clear feedback
- **Rare keys**: 0.2-0.3 - balanced presence
- **Modifiers**: 0.05 - almost silent

## Shift Key Behavior

The system automatically handles both normal and shifted characters for symbol keys:

- **Single mapping**: Define once, works for both characters
- **Example**: `Semicolon` mapping handles both `;` and `:` (Shift+;)
- **Automatic**: No need to define separate shifted mappings
- **Consistent**: Same musical note for related characters

## Language Scale Philosophy

### Scale Types
- **Pentatonic** (5 notes): Simple, universal - Rust, Go
- **Heptatonic** (7 notes): Complex, expressive - JavaScript, C, Python
- **Minor scales**: Serious, focused programming
- **Major scales**: Accessible, friendly programming
- **Modal scales**: Unique character - JavaScript Mixolydian

### Implementation Strategy
- **Core letters**: Map to primary scale degrees
- **Rare letters**: Reuse pleasant scale notes
- **Operators**: Use characteristic scale intervals
- **Frequency comfort**: Stay within C2-C6 range

## Creating Custom Configurations

### 1. Start with Language Base
Copy an existing language config and modify:
```bash
cp language_configs/python.json my_custom.json
# Edit my_custom.json
cargo run config my_custom.json
```

### 2. Key Mapping Strategy
- Map frequent keys to comfortable middle range (C3-C5)
- Use scale-appropriate intervals for harmony
- Keep volumes moderate (0.2-0.3) for sustainability

### 3. Testing
```bash
cargo run config my_custom.json
# Type various patterns to test comfort and harmony
```

## Example Configurations

### Minimal Setup
```json
{
  "mappings": {
    "A": {"note": "C4", "volume": 0.2},
    "S": {"note": "D4", "volume": 0.2},
    "D": {"note": "E4", "volume": 0.2},
    "F": {"note": "F4", "volume": 0.2},
    "Space": {"note": "C2", "volume": 0.1}
  }
}
```

### Programming Optimized
```json
{
  "mappings": {
    "Semicolon": {"note": "G3", "volume": 0.2, "description": "Statement end"},
    "LeftBracket": {"note": "C3", "volume": 0.3, "description": "Block start"},
    "RightBracket": {"note": "C4", "volume": 0.3, "description": "Block end"},
    "Equal": {"note": "D4", "volume": 0.25, "description": "Assignment"}
  }
}
```

---

**Result**: Simple, powerful configuration system that makes every programming language sound unique while staying comfortable for extended coding sessions.