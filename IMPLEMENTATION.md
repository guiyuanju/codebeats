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
🎵 CodeBeats - Python Programming Language (fart)
Warning: Could not load fart sample: No such file or directory (os error 2)
🎹 Verbose logging enabled
Press Ctrl+C to exit
🎵 Key: d → D4 (293.7Hz, vol: 0.24) [sample playback]
🎵 Key: e → E4 (329.6Hz, vol: 0.27) [sample playback]
⚪ Key: Escape (unmapped)
```

## Waveform Implementation

- **Natural**: Piano with harmonics and vibrato
- **Electronic/Sine**: Pure sine wave
- **Cyberpunk**: Multi-oscillator analog synth with LFO
- **Harmonic**: Mathematical harmonic series with golden ratio
- **Triangle/Saw/Square**: Classic electronic waveforms
- **Fart**: Real audio file playback using effects/fart-quick-short.wav with proper sample rate conversion (24kHz→44.1kHz), stereo-to-mono mixing, linear interpolation, volume control, and automatic fallback to synthetic generation if file is missing

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
- **Audio File Playback**: Fart waveform uses real WAV file samples with automatic sample rate conversion and linear interpolation
- **Rate Limiting**: Prevents audio overload during rapid typing
- **ADSR Tuning**: Waveform-specific envelope parameters (fart bypasses ADSR for direct playback)
- **Memory Efficiency**: HashMap-based note tracking with automatic cleanup
- **Sample Management**: Automatic cleanup of finished audio sample playbacks
- **Graceful Fallback**: If fart audio file is missing, falls back to synthetic generation

## Audio File Requirements

### Fart Sample Format
- **Location**: `effects/fart-quick-short.wav`
- **Current File**: 24kHz stereo, 16-bit, 0.377s duration
- **Format**: WAV file (any sample rate, mono or stereo)
- **Bit Depth**: 16-bit or 32-bit integer, or 32-bit float
- **Sample Rate Conversion**: Automatic conversion from file sample rate to system sample rate (44.1kHz)
- **Duration**: Recommended 0.5-2 seconds for quick fart sounds
- **Stereo Handling**: Stereo files are automatically mixed to mono for playback
- **Fallback**: If file is missing, synthetic fart generation is used automatically

## Easter Eggs

### Japanese Sequence Detection
- **Target Sequence**: `oppokokoppokosuttenten` (romaji for "おっぽこ　こっぽこ　すってんてん")
- **Trigger Action**: Plays fart audio sample regardless of current waveform
- **Implementation**: Real-time input sequence detection with circular buffer
- **Features**:
  - Ignores spaces and non-letter keys
  - Anti-spam protection (prevents repeated triggering)
  - Works in any waveform mode
  - 50-character input history buffer
  - Sequence length: 21 characters
- **Detection Algorithm**: Sliding window pattern matching on keyboard input history

## Testing

31 unit tests cover:
- Audio synthesis and ADSR behavior
- Audio sample loading and playback functionality
- Sample interpolation and timing accuracy
- Keyboard mapping and frequency calculation  
- Configuration loading and parsing
- Waveform generation and validation (including fart sample playback)
- Audio sample file loading with sample rate conversion and error handling
- Input sequence detection for Easter eggs (6 comprehensive tests)

## Platform Support

Cross-platform via CPAL:
- macOS: Core Audio
- Windows: WASAPI  
- Linux: ALSA/PulseAudio