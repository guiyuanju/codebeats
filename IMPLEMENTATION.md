# CodeBeats Implementation Details

## Architecture Overview

CodeBeats uses a simple, direct architecture focused on real-time audio synthesis and keyboard input processing.

## Core Components

### Audio Engine (`audio_engine.rs`)
- **AudioState**: Main audio synthesis engine with ADSR envelope system
- **NoteState**: Individual note management with phase tracking and envelope processing
- **ADSR Parameters**: Attack/Decay/Sustain/Release profiles for different waveforms
- **Sustain Duration**: Configurable minimum note duration after key release
- **Low-Pass Filter**: Simple one-pole filter to reduce harsh high frequencies

### Keyboard System (`keyboard_mapping.rs`, `keyboard_config.rs`)
- **VirtualKeycode**: Handles both physical keys and shifted characters
- **KeyboardStateTracker**: Real-time shift state detection with press/release virtual keycode mapping
- **KeyboardConfig**: JSON-based keyboard-to-note mapping configuration
- **Frequency Calculation**: Standard musical tuning (A4=440Hz)

### Waveform Generation (`waveforms.rs`)
- **Electronic**: Pure sine wave
- **Natural**: Piano-like with harmonics and subtle vibrato
- **Cyberpunk**: Multi-oscillator analog synth emulation
- **Saw/Square**: Classic electronic waveforms
- **Harmonic**: Mathematical harmonic series with precise overtones and golden ratio influence
- **Sine**: Pure sine wave (alias for Electronic)
- **Sawtooth**: Sawtooth wave (alias for Saw)
- **Triangle**: Smooth triangle wave for gentle electronic sound

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

### 4. Sustain Duration Design
- Optional minimum note duration independent of key press duration
- Non-blocking implementation - doesn't affect input responsiveness
- Per-note tracking allows overlapping sustained notes
- Graceful fallback when sustain duration is 0 (default behavior)

### 5. Low-Pass Filter Design
- Simple one-pole filter applied to each note individually
- Configurable cutoff frequency (200-8000Hz, default: 1200Hz)
- Per-note filter state for independent processing
- Smooth frequency response without audible artifacts
- Default 1200Hz cutoff reduces harsh high frequencies while preserving musical content
- Configurable via `--filter-cutoff` parameter (200-8000Hz range)
- Per-note filter state prevents audio artifacts during note transitions

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
  "version": "2.0",
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

### Language-Specific Design Principles

#### Programming Languages
- **Haskell**: Sophisticated harmonic structure with mathematical precision using advanced chord progressions
- **Java**: Structured enterprise scale with systematic harmonies reflecting corporate development patterns
- **Clojure**: Jazz-influenced harmonies with Lisp elegance, featuring minor sevenths and ninths
- **Scheme**: Classical Lisp minimalist scale focusing on elegant simplicity and pure functional concepts
- **Emacs Lisp**: Editor-optimized scale with extensibility focus, emphasizing Control and Meta key usage

#### Human Languages  
- **English**: Optimized for English letter frequencies (E-T-A-O-I-N-S-H-R pattern) with comfortable mid-range mapping
- **Chinese**: Pinyin input pattern optimization focusing on common initials (Z-S-D-L-G) and finals (A-O-E-I-U-V)
- **Japanese**: Romaji input patterns emphasizing vowel sounds (A-I-U-E-O) and common consonant combinations (K-S-T-N-H-M)

#### Musical Scale Characteristics
- **Programming Languages**: Use structured scales (major, minor, pentatonic) matching language paradigms
- **Human Languages**: Emphasize phonetic frequency patterns with harmonic relationships between vowels and consonants
- **Waveform Selection**: Each language config includes recommended waveforms (harmonic, sine, triangle, etc.)

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

### Smart Rate Limiting
- **RateLimiter struct**: Tracks recent press times for each key using HashMap
- **Sliding window**: 500ms time window for detecting rapid presses
- **Exponential volume reduction**: Each rapid successive press multiplies volume by 0.7
- **Per-key tracking**: Each key has independent rate limiting history
- **Automatic cleanup**: Old press times are automatically removed from memory
- **Formula**: `volume_multiplier = 0.7^(rapid_press_count)`
- **Example**: 1st press = 100%, 2nd rapid press = 70%, 3rd = 49%, 4th = 34%

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

### Keyboard State Tracking Fix
- **Problem**: When pressing Shift+9 (creating "(") then releasing Shift before releasing 9, the sound would continue forever
- **Root Cause**: Virtual keycode determination was based on current shift state, causing press and release events to use different key IDs
- **Solution**: `KeyboardStateTracker` now remembers which virtual keycode was used for each physical key press and uses the same virtual keycode for the corresponding release
- **Implementation**: 
  - Added `pressed_virtual_keys` HashMap to track virtual keycodes
  - Separate methods for press (`get_virtual_keycode_for_press`) and release (`get_virtual_keycode_for_release`) events
  - **Timing Fix**: The release method removes the virtual keycode from tracking (not the update method) to ensure proper sequencing
  - This ensures every key press gets a matching release with the same virtual keycode, regardless of modifier key release order

### Simplified Audio System
- **Clean ADSR Envelopes**: Standard attack/decay/sustain/release with proven parameters
- **Basic Rate Limiting**: Volume reduction for rapid key presses prevents audio overload
- **Simple Polyphonic Synthesis**: Multiple notes play simultaneously without complex interference management
- **Minimal Feature Set**: Focus on core functionality for reliable, crackling-free audio experience

### Frequency Optimization
- **Problem**: Many keys were mapped to excessively high frequencies (>1000Hz) causing ear discomfort
- **Solution**: Systematic frequency reduction across all language configurations
- **Implementation**:
  - Reduced 128 high-frequency notes across all 6 language configs
  - Lowered notes by 1-2 octaves while preserving musical relationships
  - Applied pleasant mid-range alternatives (220-880Hz) for comfort
- **Results**: All configs now have comfortable frequency ranges while maintaining musical harmony

### Language Configuration Expansion
- **Programming Languages**: Added 5 new language configs (Haskell, Java, Clojure, Scheme, Emacs Lisp) with unique musical characteristics
- **Human Languages**: Added 3 new language configs (English, Chinese, Japanese) optimized for natural language patterns
- **Musical Diversity**: Each config uses different scales, waveforms, and harmonic structures matching language characteristics
- **Frequency Optimization**: All new configs use comfortable frequency ranges (avoiding harsh high frequencies)
- **Cultural Sensitivity**: Human language configs respect phonetic patterns and input method conventions

### Waveform Implementation Expansion
- **Harmonic Waveform**: Added sophisticated mathematical harmonic series with golden ratio influence for Haskell
- **Sine/Sawtooth/Triangle**: Added alias and new waveforms to support all language configuration requirements
- **ADSR Integration**: All new waveforms properly integrated with envelope system using appropriate parameters
- **Comprehensive Testing**: All 8 waveforms tested and validated for audio generation and parsing

### Testing Infrastructure  
- **Test Methods Added**: Added `get_mapping`, `set_mapping`, and `remove_mapping` methods to `KeyboardConfig` for comprehensive testing
- **Test Coverage**: All 20 unit tests now pass, covering ADSR envelopes, keyboard mapping, frequency calculation, configuration management, sustain duration, and low-pass filtering
- **Test Cleanup**: Removed redundant tests (`test_piano_layout`, `test_mapping_operations`, `test_rate_limiter`) to focus on core functionality
- **Code Cleanup**: Removed unused rate limiting system that was never properly implemented
- **JSON Validation**: All 11 language configuration files validated for correct JSON syntax and no duplicate keys
- **JSON Cleanup**: Fixed duplicate key issues in `c.json` (20 duplicates) and `javascript.json` (3 duplicates)
- **Frequency Optimization**: Systematically reduced 128 harsh high-frequency notes to comfortable ranges across all configs
- **Note Harmonies**: Multiple keys intentionally map to the same notes to create musical harmonies (this is by design, not an error)
- **Rate Limiting Implementation**: Added `RateLimiter` struct with sliding window press tracking and exponential volume reduction
- **Volume Protection**: Automatic volume reduction prevents audio overload during rapid typing sessions
- **Simplified Architecture**: Removed complex sustain duration and anti-crackling features for reliable, clean audio output
- **Focus on Core Features**: Prioritized stable polyphonic synthesis with effective rate limiting over advanced features

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

## Language Configuration Coverage

### Programming Languages (8 total)
1. **General** - Pentatonic, balanced programming
2. **Python** - F Major, warm and friendly  
3. **Rust** - C Minor Pentatonic, powerful and direct
4. **JavaScript** - D Mixolydian, modern and dynamic
5. **C** - A Natural Minor, serious and precise
6. **Go** - G Major Pentatonic, clean and simple
7. **Haskell** - Sophisticated harmonic, mathematical precision
8. **Java** - Structured enterprise, systematic harmonies
9. **Clojure** - Jazz-influenced, Lisp elegance
10. **Scheme** - Classical Lisp, minimalist elegance
11. **Emacs Lisp** - Editor-optimized, extensibility focus

### Human Languages (3 total)
1. **English** - Letter frequency optimized for English patterns
2. **Chinese** - Pinyin input optimization for Chinese phonetics
3. **Japanese** - Romaji input patterns for Japanese phonetics

## Key Simplifications Made

1. **Removed wrapper structs**: AudioSystem, UIManager, KeyboardProcessor → direct functions
2. **Flattened modules**: audio/, keyboard/, waveform/ → single-level files  
3. **Simplified main loop**: Direct keyboard processing instead of complex state management
4. **Consolidated documentation**: Multiple .md files → README.md + IMPLEMENTATION.md
5. **Removed library interface**: No lib.rs, pure binary application
6. **Removed analysis tools**: scale_comparison.rs was demo code, not core functionality
7. **Removed debug utilities**: test_keys.rs was a simple debugging tool
8. **Streamlined CLI**: Removed subcommands, kept simple options only
9. **Expanded language support**: Added 8 new language configurations covering major programming languages and human languages

This architecture prioritizes simplicity and maintainability while providing comprehensive language support and preserving all core functionality.