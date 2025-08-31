# Implementation Details

## Architecture

CodeBeats uses a simple, direct architecture for real-time audio synthesis and keyboard input.

### Core Components

- **AudioState** - Main synthesis engine with ADSR envelopes and note management
- **KeyboardConfig** - JSON-based key-to-note mapping with language-specific scales
- **Waveforms** - 8 waveform types (natural, electronic, cyberpunk, harmonic, etc.)
- **VirtualKeycode** - Handles both physical keys and shifted characters

### Audio System

- **Sample Rate**: 44.1kHz direct CPAL stream output
- **ADSR Envelopes**: Attack/Decay/Sustain/Release with waveform-specific parameters
- **Polyphonic**: Multiple notes can play simultaneously with independent envelopes
- **Rate Limiting**: Volume reduction (0.7^n) for rapid successive key presses

### Key Design Decisions

1. **Simplified Structure**: Single-level modules, direct function calls
2. **Real-time Processing**: 10ms polling, lock-free audio where possible
3. **Musical Comfort**: All frequencies optimized to 65-880Hz range (no harsh high notes)
4. **Language Optimization**: Each config targets specific language patterns

## Configuration System

### JSON Structure
```json
{
  "version": "2.0",
  "description": "Language description",
  "waveform": "natural",
  "mappings": {
    "KeyName": {
      "note": "C4",
      "volume": 0.3,
      "description": "Key usage description"
    }
  }
}
```

### Priority Order
1. CLI arguments (highest)
2. Language config files
3. Built-in defaults

## Logging System

### Verbose Mode
- **CLI Flag**: `--verbose` enables detailed terminal logging
- **Key Events**: Shows key press/release with note information
- **Configuration**: Displays config loading status and audio settings
- **Format**: Emoji-prefixed messages for visual clarity
  - 🎵 Key press with note/frequency info
  - 🔇 Key release events
  - ✓ Successful operations
  - ✗ Error conditions
  - ⚪ Unmapped keys

### Example Verbose Output
```
✓ Loaded keyboard config from: language_configs/python.json
🔊 Audio settings: volume=0.8, filter=1200Hz
🎵 CodeBeats - Python Programming Language (natural)
🎹 Verbose logging enabled
Press Ctrl+C to exit
🎵 Key: d → D4 (293.7Hz, vol: 0.24)
🎵 Key: e → E4 (329.6Hz, vol: 0.27)
🔇 Key: d → D4 (released)
⚪ Key: Escape (unmapped)
```

## Waveform Implementation

- **Natural**: Piano with harmonics and vibrato
- **Electronic/Sine**: Pure sine wave
- **Cyberpunk**: Multi-oscillator analog synth with LFO
- **Harmonic**: Mathematical harmonic series with golden ratio
- **Triangle/Saw/Square**: Classic electronic waveforms

## Language Configurations

### Programming Languages (11 configs)
- Keyword frequency analysis determines note assignments
- Common symbols create harmonic relationships
- Each language has unique musical character

### Human Languages (3 configs)  
- Letter frequency optimization (English: E-T-A-O-I-N-S-H-R)
- Input method patterns (Chinese: Pinyin, Japanese: Romaji)
- Phonetic considerations for vowels/consonants

## Key Optimizations

- **Frequency Comfort**: All 131 high notes (>1000Hz) reduced to ≤880Hz
- **Rate Limiting**: Prevents audio overload during rapid typing
- **ADSR Tuning**: Waveform-specific envelope parameters
- **Memory Efficiency**: HashMap-based note tracking with automatic cleanup

## Testing

18 unit tests cover:
- Audio synthesis and ADSR behavior
- Keyboard mapping and frequency calculation  
- Configuration loading and parsing
- Waveform generation and validation

## Platform Support

Cross-platform via CPAL:
- macOS: Core Audio
- Windows: WASAPI  
- Linux: ALSA/PulseAudio