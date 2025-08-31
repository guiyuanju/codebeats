# CodeBeats Implementation Details

## Architecture Overview

CodeBeats uses a simple, direct architecture focused on real-time audio synthesis and keyboard input processing.

## Core Components

### Audio Engine (`audio_engine.rs`)
- **AudioState**: Main audio synthesis engine with ADSR envelope system
- **NoteState**: Individual note management with phase tracking and envelope processing
- **ADSR Parameters**: Attack/Decay/Sustain/Release profiles for different waveforms
- **Rate Limiting**: Prevents harsh sounds from rapid key presses

### Keyboard System (`keyboard_mapping.rs`, `keyboard_config.rs`)
- **VirtualKeycode**: Handles both physical keys and shifted characters
- **KeyboardStateTracker**: Real-time shift state detection
- **KeyboardConfig**: JSON-based keyboard-to-note mapping configuration
- **Frequency Calculation**: Standard musical tuning (A4=440Hz)

### Waveform Generation (`waveforms.rs`)
- **Electronic**: Pure sine wave
- **Natural**: Piano-like with harmonics and subtle vibrato
- **Cyberpunk**: Multi-oscillator analog synth emulation
- **Saw/Square**: Classic electronic waveforms

## Key Technical Decisions

### 1. Simplified Structure
- Flattened module hierarchy (no deep nested folders)
- Direct function calls instead of complex wrapper structs
- Single main loop handling all input/output

### 2. Real-time Processing
- 10ms polling interval for responsive input
- Lock-free audio generation where possible
- Efficient HashMap lookups for key mappings

### 3. Musical Design
- Pentatonic scales to avoid dissonance
- Frequency analysis-based key assignments
- Volume normalization to prevent ear fatigue

## Data Flow

```
Keyboard Input → Shift Detection → Virtual Key → Config Lookup → Audio Synthesis → Output
     ↓              ↓                ↓            ↓              ↓              ↓
DeviceQuery → StateTracker → VirtualKeycode → frequency/vol → NoteState → Audio Device
```

## Configuration System

### Priority Order
1. CLI arguments (`--waveform`)
2. Language config files (`language_configs/*.json`)
3. Built-in defaults (programming-optimized)

### Config Structure
```json
{
  "version": "1.0",
  "description": "Language description",
  "waveform": "electronic",
  "mappings": {
    "KeyName": {
      "note": "C4",
      "volume": 0.3,
      "description": "Key description"
    }
  }
}
```

## Audio Processing

### ADSR Envelope
- **Attack**: Exponential rise (0.01-0.03s)
- **Decay**: Exponential fall to sustain level
- **Sustain**: Constant level while key held
- **Release**: Exponential fade when key released

### Polyphonic Synthesis
- Multiple notes can play simultaneously
- Each note has independent phase and envelope
- Automatic cleanup of finished notes

### Rate Limiting
- Tracks press frequency per key
- Reduces volume for rapid successive presses
- Prevents audio crackling and harsh sounds

## Performance Considerations

### Memory Usage
- HashMap storage for active notes (minimal overhead)
- No audio buffering (real-time generation)
- JSON configs loaded once at startup

### CPU Usage
- Simple waveform algorithms for low latency
- Efficient trigonometric calculations
- 44.1kHz sample rate with minimal processing per sample

### Audio Latency
- Direct CPAL stream output
- No intermediate buffering
- Typical latency: 5-20ms depending on audio driver

## Keyboard Mapping Strategy

### Frequency Analysis
Common programming keys get pleasant mid-range frequencies:
- **Most common**: E, T, A, O, I, N, S, R → C4-G4 range
- **Moderately common**: L, C, U, D, P, M → G4-C5 range  
- **Less common**: Function keys, symbols → Higher registers

### Shifted Key Handling
- Physical keys: Base octave (C4, D4, E4...)
- Shifted keys: Higher octave (C5, D5, E5...)
- Maintains harmonic relationships

### Musical Relationships
- Brackets: `[` (A5) → `{` (A6) - octave pairs
- Operators: `+`, `=` share G5 - mathematical unity
- Punctuation: `;` (D6) → `:` (D7) - emphasis relationship

## Error Handling

### Graceful Degradation
- Invalid waveforms fall back to electronic
- Missing config files use built-in defaults  
- Unmapped keys show helpful messages (not errors)

### Audio Robustness
- Continues playing if individual notes fail
- Automatic cleanup of problematic note states
- Volume clamping prevents speaker damage

## Testing Strategy

### Unit Tests
- Frequency calculation accuracy
- ADSR envelope behavior
- Rate limiting functionality
- Configuration loading/parsing

### Integration Testing
- Audio system initialization
- Keyboard input processing
- Real-time performance under load

## Future Extensibility

The simplified architecture makes it easy to:
- Add new waveforms (implement in `waveforms.rs`)
- Create language configs (JSON files in `language_configs/`)
- Modify musical relationships (edit default config)
- Add effects (extend `NoteState` processing)

## Build Dependencies

- **cpal**: Cross-platform audio library
- **device_query**: Keyboard input detection
- **serde/serde_json**: Configuration serialization
- **clap**: Command-line argument parsing

## Platform Support

- **macOS**: Full support with Core Audio
- **Windows**: Full support with WASAPI
- **Linux**: Full support with ALSA/PulseAudio

## Key Simplifications Made

1. **Removed wrapper structs**: AudioSystem, UIManager, KeyboardProcessor → direct functions
2. **Flattened modules**: audio/, keyboard/, waveform/ → single-level files  
3. **Simplified main loop**: Direct keyboard processing instead of complex state management
4. **Consolidated documentation**: Multiple .md files → README.md + IMPLEMENTATION.md
5. **Removed library interface**: No lib.rs, pure binary application
6. **Removed analysis tools**: scale_comparison.rs was demo code, not core functionality
7. **Removed debug utilities**: test_keys.rs was a simple debugging tool
8. **Streamlined CLI**: Removed subcommands, kept simple options only

This architecture prioritizes simplicity and maintainability while preserving all core functionality.