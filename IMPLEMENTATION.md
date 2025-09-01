# Implementation Details

CodeBeats is a real-time audio synthesis application that converts keyboard input into musical notes.

## Architecture

### Core Components
- **AudioState** - Main synthesis engine with ADSR envelopes and polyphonic note management
- **KeyboardConfig** - JSON-based key-to-note mapping with language-specific musical scales
- **Waveforms** - 25+ waveform types with unique sonic characteristics and ADSR parameters
- **GUI Module** - Cross-platform graphical interface using egui/eframe with real-time log display

### Audio System
- **Sample Rate**: 44.1kHz direct CPAL stream output
- **ADSR Envelopes**: Attack/Decay/Sustain/Release with waveform-specific parameters
- **Polyphonic**: Multiple simultaneous notes with independent envelopes
- **Rate Limiting**: Volume reduction (0.7^n) for rapid successive keypresses
- **Audio Samples**: Real WAV file playback for fart waveform with automatic sample rate conversion

## GUI Features

### Design Philosophy
The GUI launches CLI processes rather than reimplementing the audio engine. This ensures:
- Consistent behavior between GUI and CLI
- Simplified maintenance
- Single audio codebase

### Technical Implementation
- **Framework**: egui 0.24 for immediate mode GUI
- **Process Management**: Spawns CLI with stdout/stderr capture
- **Log Display**: Multi-threaded output capture with Arc<Mutex<Vec<String>>>
- **Dynamic Configuration Loading**: Runtime CLI integration for options discovery
- **Cross-platform**: Native window management via eframe

### Dynamic Options Loading
- **Waveforms**: GUI calls `cargo run --bin codebeats list-waveforms` to get all available waveforms
- **Language Configs**: GUI calls `cargo run --bin codebeats list-configs` to discover configuration files  
- **CLI Output Parsing**: Parses CLI output format "  name - description" for dropdown population
- **Fallback Strategy**: Uses hardcoded options if CLI calls fail
- **Refresh Functionality**: 🔄 Refresh button allows manual reload without restart
- **Capitalization**: Automatically capitalizes first letter of config names for display

### Platform-Specific Features
- **macOS**: Native .app bundle creation with Info.plist (no terminal windows)
- **Windows**: `#![windows_subsystem = "windows"]` attribute to hide console (native builds only)
- **Linux**: Standard executable with shell script launcher

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
      "volume": 0.8,
      "description": "Key mapping description"
    }
  }
}
```

### Waveform System

### Synthesis Engine
CodeBeats features 8 carefully selected waveforms, each implementing unique audio generation algorithms:

- **Natural**: Piano with harmonics and subtle vibrato modulation for organic piano sound
- **Electronic**: Pure sine wave generation for clean, precise tones  
- **Cyberpunk**: Multi-oscillator analog synthesis with LFO modulation, detuning, and analog-style soft clipping
- **Saw**: Classic sawtooth wave with bright harmonic content for electronic music
- **Square**: Retro 8-bit style square wave with sharp transitions
- **Triangle**: Smooth triangular wave for mellow electronic sounds
- **Fart**: Realistic fart synthesis with low-frequency emphasis (40-150Hz), body resonance simulation, formant filtering, and gentle turbulence
- **Bass**: Deep bass with sub-harmonic emphasis, rich harmonic series, and analog-style saturation for powerful low-end

### ADSR Envelope System
Waveform-specific envelope parameters optimized for each sound character:
- **Natural/Bass**: Organic attack/release with slightly slower decay for depth
- **Electronic/Triangle**: Clean electronic envelopes with precise timing
- **Saw/Square**: Sharp attack for electronic percussion and bright sounds
- **Cyberpunk**: Analog-style envelopes with longer sustain for warm synthesis
- **Fart**: Quick attack with natural sustain and organic release timing

## Language Configuration System

### Programming Languages (16 configurations)
Each programming language uses a musically appropriate scale and maps keys based on keyword frequency:

- **C** (E major, guitar waveform): Performance-focused with class/struct emphasis
- **C++** (E major, guitar waveform): Template and performance-oriented mappings
- **C#** (D major, FM waveform): .NET-focused with LINQ and async/await prominence
- **TypeScript** (A major, Moog waveform): Type-focused with interface and generic emphasis
- **Swift** (G major, bell waveform): iOS/macOS development with protocol orientation
- **PHP** (C major, organ waveform): Web development focus with $ variable prefix emphasis
- **Ruby** (F# major, pluck waveform): Elegant syntax with symbol and block parameter focus
- **Kotlin** (Ab major, ambient waveform): JVM/Android focus with nullable types
- **Python** (F major, natural waveform): Readable syntax with def/class/import emphasis
- **Rust** (Original configuration): Memory safety and performance focus
- **JavaScript** (Original configuration): Web development and async programming
- **Java** (Original configuration): Enterprise and object-oriented programming
- **Go** (Original configuration): Concurrency and simplicity focus
- **Clojure** (Original configuration): Functional programming with Lisp syntax
- **Haskell** (Original configuration): Pure functional programming
- **Scheme** (Original configuration): Minimalist Lisp dialect
- **Emacs Lisp** (Original configuration): Emacs extension programming
- **General Programming** (Original configuration): Universal programming patterns

### Human Languages (6 configurations)
Letter frequency-based musical mappings with distinctive waveforms:

- **English** (Original configuration): Standard QWERTY optimization
- **Chinese** (Original configuration): Character-based input patterns
- **Japanese** (Original configuration): Hiragana/Katakana/Kanji patterns
- **Spanish** (B major, choir waveform): Romance language vowel emphasis with ñ character
- **French** (Db major, saxophone waveform): French phonetics with accent grave support
- **German** (Bb major, theremin waveform): Germanic language patterns with umlaut characters

### Musical Scale Selection
Each language uses a carefully chosen musical scale:
- Major scales for positive, productive languages (C, D, E, F, G, A, B major)
- Flat keys (Db, Bb, Ab major) for languages with unique cultural character
- Sharp keys (F#, C# major) for technical/modern languages

### Key Mapping Strategy
1. **Frequency Analysis**: Most common letters mapped to fundamental scale degrees
2. **Keyword Optimization**: Programming keywords mapped to harmonious intervals
3. **Linguistic Character**: Special characters (ñ, ü, ß) get distinctive note assignments
4. **Volume Scaling**: More frequent keys have slightly higher volumes for musical balance
5. **Octave Spreading**: Shift+key combinations mapped to higher octaves for harmonic richness

The system creates natural musical phrases as users type, with each language producing characteristic melodic patterns that reflect the structure and rhythm of the code or text being written.
      "volume": 0.3,
      "description": "Key usage description"
    }
  }
}
```

### Language Configurations
- **Programming Languages**: 11 configs optimized for keyword frequency and symbols
- **Human Languages**: 3 configs optimized for letter frequency patterns
- **Musical Scales**: Each language uses different musical scales and note ranges

## Waveform Implementation
- **Natural**: Piano with harmonics and vibrato modulation
- **Electronic**: Pure sine wave for clean tones
- **Cyberpunk**: Multi-oscillator analog synth with detuning and LFO modulation
- **Saw**: Bright sawtooth wave with rich harmonic content
- **Square**: Classic 8-bit square wave
- **Triangle**: Smooth triangular wave
- **Fart**: Realistic synthesis with body resonance, formant filtering, and gentle turbulence
- **Bass**: Deep bass with sub-harmonic emphasis and analog saturation

## Audio File System
- **Fart Audio Sample**: `effects/fart-quick-short.wav` (optional)
- **Format**: WAV (any sample rate, mono/stereo supported)
- **Processing**: Automatic sample rate conversion to 44.1kHz, stereo-to-mono mixing
- **Fallback**: Synthetic fart generation if file missing (default implementation)
- **Note**: All other waveforms use pure synthesis algorithms

## Build System

### Build Package Script
- **Script**: `./build_package` - Single command to build GUI releases for all platforms
- **Output**: Creates macOS .app bundle, Windows and Linux folders with startup scripts
- **Cross-Platform**: Attempts to build for macOS, Windows (x86_64-pc-windows-gnu), and Linux (x86_64-unknown-linux-gnu)
- **Packaging**: Automatically packages each platform with resources and launch scripts

## GUI-CLI Integration

### Dynamic Configuration Discovery
The GUI automatically discovers available options by calling CLI commands:

```rust
// Waveform discovery
let output = std::process::Command::new(cli_exe)
    .args(&args)
    .arg("list-waveforms")
    .output();

// Language config discovery  
let output = std::process::Command::new(cli_exe)
    .args(&args)
    .arg("list-configs")
    .output();
```

### CLI Output Parsing
- **Waveform Format**: `"  natural      - Piano-like with harmonics"`
- **Config Format**: `"  cpp                            - cpp.json"`
- **Parser Logic**: Splits on " - " separator, trims whitespace, capitalizes display names
- **Error Handling**: Falls back to hardcoded lists if CLI calls fail

### Benefits
- **Always Current**: GUI automatically reflects new configuration files
- **No Maintenance**: No need to update hardcoded lists when adding configs
- **Consistency**: GUI and CLI always show the same available options
- **User Experience**: Refresh button allows real-time updates

## Easter Egg System
- **Sequence**: `oppokokoppokosuttenten` (Japanese romaji)
- **Detection**: Real-time sliding window pattern matching
- **Features**: Anti-spam protection, works in any waveform mode
- **Implementation**: 50-character circular buffer with 21-character target sequence
