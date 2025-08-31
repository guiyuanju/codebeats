# Programming Language Musical Scales

CodeBeats supports different musical scales for each programming language, creating unique auditory experiences that match each language's design philosophy.

## Quick Start

```bash
# Try different language scales
cargo run config language_configs/rust.json        # C Minor Pentatonic - Power
cargo run config language_configs/javascript.json  # D Mixolydian - Modern  
cargo run config language_configs/c.json           # A Natural Minor - Serious
cargo run config language_configs/go.json          # G Major Pentatonic - Simple
cargo run config language_configs/python.json      # F Major - Friendly

# Compare all scales
cargo run compare-scales

# Interactive demo
./demo.sh
```

## Language Scales

### 🦀 Rust - C Minor Pentatonic
**Notes**: C - Eb - F - G - Bb  
**Character**: Powerful, direct, blues-inspired  
**Philosophy**: Systems programming power and control

- **Ownership operators** (`&`, `*`) use strong bass foundation
- **Lifetimes** (`'`) distinctive but comfortable (C6)
- **Core keywords** (`let`, `fn`, `match`) follow pentatonic progression
- **Frequency range**: C2-C6 (65-1047 Hz) - powerful but comfortable

### 🌐 JavaScript - D Mixolydian  
**Notes**: D - E - F# - G - A - B - C  
**Character**: Modern, jazzy, slightly unresolved  
**Philosophy**: Dynamic web development flexibility

- **Arrow functions** (`=>`) satisfying interval progressions
- **Template literals** (`` ` ``) unique augmented fourth (G#4)
- **Async features** use the mode's characteristic unresolved seventh
- **Frequency range**: C2-C6 (65-1047 Hz) - modern but comfortable

### ⚙️ C - A Natural Minor
**Notes**: A - B - C - D - E - F - G  
**Character**: Serious, methodical, precise  
**Philosophy**: Low-level programming precision

- **Pointer operations** follow natural minor intervals
- **System calls** emphasized through minor scale's focused nature
- **Memory management** reflected in methodical progressions
- **Frequency range**: C2-F6 (65-1397 Hz) - serious but sustainable

### 🐹 Go - G Major Pentatonic
**Notes**: G - A - B - D - E  
**Character**: Clean, simple, folk-like  
**Philosophy**: Minimalist efficiency

- **Simple syntax** matches pentatonic simplicity (5 notes only)
- **Clean intervals** reflect Go's straightforward design
- **Channel operations** use directional pentatonic flow
- **Frequency range**: C2-G6 (65-1568 Hz) - clean and pleasant

### 🐍 Python - F Major
**Notes**: F - G - A - Bb - C - D - E  
**Character**: Warm, friendly, accessible  
**Philosophy**: Readable, beginner-friendly programming

- **Significant whitespace** emphasized with structural bass
- **Readable syntax** follows warm major scale progressions
- **Bb characteristic note** gives F major its unique warmth
- **Frequency range**: F2-E6 (87-1319 Hz) - warm and welcoming

## Scale Comparison

| Language | Scale Type | Notes | Character | Best For |
|----------|------------|-------|-----------|----------|
| Rust | Minor Pentatonic | 5 | Powerful, direct | Systems programming |
| JavaScript | Mixolydian Mode | 7 | Modern, unresolved | Web development |
| C | Natural Minor | 7 | Serious, precise | Low-level programming |
| Go | Major Pentatonic | 5 | Clean, simple | Backend services |
| Python | Major Scale | 7 | Warm, friendly | Scripting, learning |

## Frequency Improvements

### Comfortable Range
- **Maximum frequency**: C6 (1047 Hz) - bright but not shrill
- **Sweet spot**: C3-C5 (130-523 Hz) - most pleasant range
- **Bass foundation**: C2 (65 Hz) - warm grounding
- **Smart reuse**: Rare keys reuse pleasant scale notes

### Audio Ergonomics
- ✅ No ear fatigue during extended sessions
- ✅ Pleasant harmonies within natural listening range
- ✅ Sustainable for daily programming use
- ✅ Headphone and speaker friendly

## Design Philosophy

### Scale Selection Logic
- **Pentatonic scales** (5 notes): Simple languages (Rust, Go)
- **Full scales** (7 notes): Expressive languages (JavaScript, C, Python)
- **Minor scales**: Control-focused languages (Rust, C)
- **Major scales**: Accessible languages (Go, Python)
- **Modal scales**: Unique paradigms (JavaScript)

### Musical Programming Match
- **Systems languages**: Serious scales for serious programming
- **Web languages**: Modern scales for creative development
- **Simple languages**: Simple scales for clarity
- **Complex languages**: Rich scales for full expression

## Usage Tips

### By Project Type
- **Systems/Performance**: Rust or C configurations
- **Web Development**: JavaScript configuration
- **Microservices**: Go configuration
- **Scripting/Automation**: Python configuration

### By Session Type
- **Deep Focus**: Minor scales (Rust, C)
- **Creative Coding**: Modal scale (JavaScript)
- **Rapid Development**: Major scales (Go, Python)
- **Learning**: Python (most accessible scale)

## Customization

Edit any language JSON file to adjust:
- **Note mappings**: Change which keys map to which scale degrees
- **Volume levels**: Adjust 0.1-0.4 range for personal preference
- **Descriptions**: Add your own contextual descriptions

Example customization:
```json
{
  "R": {
    "note": "C4",
    "volume": 0.15,
    "description": "Quieter Rust foundation"
  }
}
```

## Common Key Mappings

All languages share consistent mappings for:
- **Whitespace**: Soft bass notes (C2, F2) - structural foundation
- **Modifiers**: Barely audible (0.05 volume) - don't interfere
- **Navigation**: Gentle mid-range (C3-G3) - smooth movement
- **Function keys**: Comfortable high range (C6-G6) - clear feedback

---

**Result**: Each programming language creates a unique musical environment that enhances coding while staying comfortable for extended use. The scales reinforce language characteristics through carefully chosen musical relationships.