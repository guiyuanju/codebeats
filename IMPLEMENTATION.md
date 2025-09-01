# Implementation Details

CodeBeats is a real-time audio synthesis application that converts keyboard input into musical notes.

## Architecture

### Core Components
- **AudioState** - Main synthesis engine with ADSR envelopes and polyphonic note management
- **KeyboardConfig** - JSON-based key-to-note mapping with language-specific musical scales
- **Waveforms** - 8 waveform types (natural, electronic, cyberpunk, harmonic, triangle, saw, square, fart)
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
- **Configuration Discovery**: Runtime scanning of language_configs directory
- **Cross-platform**: Native window management via eframe

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
- **Natural**: Piano with harmonics and vibrato
- **Electronic**: Pure sine wave
- **Cyberpunk**: Multi-oscillator analog synth with LFO
- **Fart**: Real audio file playback with sample rate conversion and linear interpolation
- **Others**: Standard mathematical waveforms (triangle, saw, square, harmonic)

## Audio File System
- **Location**: `effects/fart-quick-short.wav`
- **Format**: WAV (any sample rate, mono/stereo supported)
- **Processing**: Automatic sample rate conversion to 44.1kHz, stereo-to-mono mixing
- **Fallback**: Synthetic generation if file missing

## Build System

### Build Package Script
- **Script**: `./build_package` - Single command to build GUI releases for all platforms
- **Output**: Creates macOS .app bundle, Windows and Linux folders with startup scripts
- **Cross-Platform**: Attempts to build for macOS, Windows (x86_64-pc-windows-gnu), and Linux (x86_64-unknown-linux-gnu)
- **Packaging**: Automatically packages each platform with resources and launch scripts

### Binary Target
- **GUI Only**: `codebeats-gui` - Configuration launcher with log capture
- **Cross-Platform**: Builds for macOS (arm64), Windows (x86_64), and Linux (x86_64)
- **Console Behavior**: Hidden on Windows, native .app bundle on macOS

### Package Structure

**macOS .app Bundle:**
```
CodeBeats.app/
├── Contents/
│   ├── Info.plist          # Application metadata
│   ├── MacOS/
│   │   └── CodeBeats       # GUI executable
│   └── Resources/
│       ├── language_configs/
│       └── effects/
```

**Windows Package:**
```
CodeBeats-Windows/
├── CodeBeats.exe           # GUI executable
├── Start-CodeBeats.bat     # Startup script
├── README.txt              # User instructions
├── language_configs/
└── effects/
```

**Linux Package:**
```
CodeBeats-Linux/
├── CodeBeats               # GUI executable
├── start-codebeats.sh      # Startup script
├── README.txt              # User instructions
├── language_configs/
└── effects/
```

## Easter Egg System
- **Sequence**: `oppokokoppokosuttenten` (Japanese romaji)
- **Detection**: Real-time sliding window pattern matching
- **Features**: Anti-spam protection, works in any waveform mode
- **Implementation**: 50-character circular buffer with 21-character target sequence

## Testing
31 unit tests covering audio synthesis, configuration loading, waveform generation, sample playback, and Easter egg detection.