# 🎵 Shifted Key Detection Demo

## Overview

Your sound system now supports **shifted key detection**! This means you can have different musical notes for:
- `1` vs `!` (Shift+1)
- `2` vs `@` (Shift+2)
- `[` vs `{` (Shift+[)
- And many more!

## What's New

### ✨ Enhanced Key Detection
- **Physical Keys**: Regular keypresses (like `a`, `1`, `[`)
- **Shifted Keys**: Shift combinations (like `!`, `@`, `{`)
- **Real-time Tracking**: Automatically detects when Shift is held down

### 🎹 Musical Harmony Design

The new configuration uses musical theory to create pleasing sounds:

#### Number Row - Octave Pairs
- `1` → C4 (root note) | `!` → C6 (bright octave)
- `2` → D4 (second) | `@` → D6 (high harmony)
- `3` → E4 (third) | `#` → E6 (bright accent)
- `4` → F4 (fourth) | `$` → F6 (high fourth)
- `5` → G4 (fifth) | `%` → G6 (perfect fifth high)

#### Programming Symbols - Chord Extensions
- `[` → A5 (programming) | `{` → A6 (block opening)
- `]` → B5 (programming) | `}` → B6 (block closing)
- `;` → D6 (statement) | `:` → D7 (emphasis)
- `'` → E6 (quote) | `"` → E7 (string delimiter)

#### Special Characters - Musical Effects
- `*` → C7 (bright star note - high volume)
- `?` → A7 (inquiry tone - questioning feel)
- `|` → C7 (pipe flow - strong accent)
- `~` → B7 (wave motion - flowing sound)

## How to Test

### 1. Run the Program
```bash
cargo run
```

### 2. Try These Key Combinations

**Numbers with and without Shift:**
- Type: `1234567890`
- Then: `!@#$%^&*()`
- Notice how shifted versions are higher and brighter!

**Programming Brackets:**
- Type: `[]{}`
- The opening `{` and closing `}` create harmonic pairs

**Punctuation Pairs:**
- Type: `;:` (statement vs emphasis)
- Type: `'"` (quote vs string)
- Type: `,<.>/?` (comparisons and questions)

### 3. Create Musical Phrases

**Chord Progression:**
- `1!` (C4→C6 octave jump)
- `5%` (G4→G6 perfect fifth)
- `4$` (F4→F6 fourth)
- `1!` (return to root)

**Programming Melody:**
- Type code with brackets: `function() { return [1, 2, 3]; }`
- Notice how `{}` and `[]` create musical structure!

**Question and Answer:**
- Type: `what?` (ends with questioning A7)
- Type: `yes!` (ends with emphatic C6)

## Technical Details

### Architecture
- **KeyboardStateTracker**: Monitors shift key states
- **VirtualKeycode**: Represents both physical and shifted keys
- **Enhanced Mapping**: String-based key identification supports shifted characters

### Musical Theory Applied
- **Pentatonic Foundation**: C-D-E-G-A scale for pleasant harmony
- **Octave Doubling**: Shifted keys often play higher octaves of base notes
- **Chord Extensions**: Related keys create harmonic intervals
- **Volume Dynamics**: Shifted keys are slightly louder for emphasis

### Configuration Structure
```json
{
  "A": { "note": "C4", "volume": 0.3 },
  "Exclamation": { "note": "C6", "volume": 0.35 }
}
```

Physical key `A` and shifted key `Exclamation` (Shift+1) have different mappings!

## Benefits

### For Coding
- **Syntax Highlighting**: Different sounds for `()` vs `{}`
- **Punctuation Awareness**: Distinct tones for `,` vs `<`
- **Error Prevention**: Different sounds help distinguish similar characters

### For Music
- **Richer Harmony**: Double the available notes
- **Dynamic Expression**: Shifted keys provide emphasis
- **Chord Progressions**: Related keys create musical relationships

### For Accessibility
- **Audio Feedback**: Confirm shift state through sound
- **Pattern Recognition**: Learn typing patterns through melody
- **Muscle Memory**: Associate key positions with musical intervals

## Next Steps

1. **Experiment**: Try different typing patterns to discover melodies
2. **Customize**: Edit `keyboard_config.json` to create your own musical mappings
3. **Code Music**: Write code that sounds beautiful as you type!

---

*Happy coding and music making! 🎵💻*