# 🎵 Waveform Configuration Guide

This document details the design philosophy, technical implementation, and configuration parameters for each waveform in the Piano Keyboard Sound Simulator.

## 🎹 Overview

Each waveform is designed for specific use cases and aesthetic preferences, combining unique synthesis techniques with tailored ADSR envelopes to create distinctive sonic characteristics.

---

## 1. 🎼 Natural Piano (Default)

### Design Philosophy
- **Goal**: Simulate acoustic piano characteristics
- **Inspiration**: Traditional grand piano with rich harmonic content
- **Use Case**: General programming, classical music feel, non-distracting background

### Technical Implementation
```rust
// Complex harmonic structure
let fundamental = base_phase.sin();                    // Base frequency (100%)
let harmonic2 = (base_phase * 2.0).sin() * 0.3;      // 2nd harmonic (30%)
let harmonic3 = (base_phase * 3.0).sin() * 0.15;     // 3rd harmonic (15%)
let harmonic4 = (base_phase * 4.0).sin() * 0.08;     // 4th harmonic (8%)
let harmonic5 = (base_phase * 5.0).sin() * 0.05;     // 5th harmonic (5%)

// Natural vibrato
let vibrato = (phase * 6.0 * 2.0 * PI).sin() * 0.002;
let modulated_phase = base_phase * (1.0 + vibrato);
```

### ADSR Configuration
```rust
attack_time: 0.02,    // 20ms - Quick piano hammer strike
decay_time: 0.1,      // 100ms - Natural decay
sustain_level: 0.7,   // 70% - Moderate sustain
release_time: 0.15,   // 150ms - Natural piano release
```

### Key Characteristics
- ✅ Rich harmonic content mimics real piano
- ✅ Subtle vibrato adds organic feel
- ✅ Quick attack for responsive playing
- ✅ Balanced sustain for chord work

### Recommended For
- General coding sessions
- Classical music enthusiasts
- Users who prefer natural, non-synthetic sounds
- Long typing sessions (non-fatiguing)

---

## 2. ⚡ Electronic (Pure Sine)

### Design Philosophy
- **Goal**: Clean, precise electronic tone
- **Inspiration**: Classic analog synthesizers, pure waveforms
- **Use Case**: Minimalist aesthetic, clean sound design, electronic music

### Technical Implementation
```rust
// Pure sine wave - no harmonics or modulation
base_phase.sin()
```

### ADSR Configuration
```rust
attack_time: 0.01,    // 10ms - Instant electronic response
decay_time: 0.05,     // 50ms - Short decay
sustain_level: 0.8,   // 80% - High sustain for pads
release_time: 0.1,    // 100ms - Clean cutoff
```

### Key Characteristics
- ✅ Mathematically pure sine wave
- ✅ No harmonics or overtones
- ✅ Instant attack for precision
- ✅ High sustain level for pad-like sounds

### Recommended For
- Electronic music production
- Sound design work
- Users who prefer clean, minimal tones
- Testing pure frequency relationships

---

## 3. ⚡ Saw Wave (Bright Electronic)

### Design Philosophy
- **Goal**: Bright, cutting electronic sound
- **Inspiration**: Classic analog synthesizer leads
- **Use Case**: Modern electronic music, energetic coding sessions

### Technical Implementation
```rust
// Linear sawtooth wave
2.0 * (phase - phase.floor()) - 1.0
```

### ADSR Configuration
```rust
attack_time: 0.005,   // 5ms - Punchy electronic attack
decay_time: 0.08,     // 80ms - Quick decay
sustain_level: 0.75,  // 75% - Good sustain
release_time: 0.12,   // 120ms - Clean electronic release
```

### Key Characteristics
- ✅ Rich in high-frequency harmonics
- ✅ Bright, cutting sound
- ✅ Very fast attack for rhythmic playing
- ✅ Classic analog synthesizer character

### Recommended For
- Electronic music production
- High-energy coding sessions
- Users who like bright, modern sounds
- Dance/EDM music contexts

---

## 4. 🟫 Square Wave (Retro Gaming)

### Design Philosophy
- **Goal**: 8-bit retro gaming nostalgia
- **Inspiration**: Classic arcade games, chip music
- **Use Case**: Game development, retro coding atmosphere

### Technical Implementation
```rust
// 50% duty cycle square wave
if (phase % 1.0) < 0.5 { 1.0 } else { -1.0 }
```

### ADSR Configuration
```rust
attack_time: 0.005,   // 5ms - Instant 8-bit response
decay_time: 0.08,     // 80ms - Quick decay
sustain_level: 0.75,  // 75% - Punchy sustain
release_time: 0.12,   // 120ms - Clean cutoff
```

### Key Characteristics
- ✅ Pure square wave with odd harmonics
- ✅ Classic 8-bit game sound
- ✅ Instant attack for rhythmic precision
- ✅ Nostalgic, retro character

### Recommended For
- Game development projects
- Retro/nostalgic coding sessions
- Chiptune music enthusiasts
- Users who grew up with 8-bit games

---

## 5. 🌃 Cyberpunk 2049 (Analog Synth)

### Design Philosophy
- **Goal**: Blade Runner 2049 atmospheric soundscape
- **Inspiration**: Vangelis, Hans Zimmer, analog synthesizer warmth
- **Use Case**: Cyberpunk programming, sci-fi projects, atmospheric coding

### Technical Implementation
```rust
// Multi-oscillator analog synthesizer
let saw = 2.0 * (phase - phase.floor()) - 1.0;    // Main sawtooth
let sub_bass = (base_phase * 0.5).sin() * 0.4;    // Sub-octave bass

// LFO modulation for analog character
let lfo = (time * 0.3).sin();                     // Slow LFO
let lfo2 = (time * 0.7).sin();                    // Medium LFO

// PWM (Pulse Width Modulation)
let pulse_width = 0.5 + lfo * 0.1;
let pulse = if (phase % 1.0) < pulse_width { 1.0 } else { -1.0 };

// Oscillator mixing
let mixed = saw * 0.6 + pulse * 0.3 + sub_bass;

// Analog-style low-pass filter
let cutoff_mod = 0.7 + lfo2 * 0.2;
let filtered = mixed * cutoff_mod;

// Warm analog distortion
let saturated = soft_clipping(filtered * 1.2);

// Chorus effect (detuning)
let detune1 = (base_phase * 1.003).sin() * 0.15;
let detune2 = (base_phase * 0.997).sin() * 0.15;

saturated + detune1 + detune2
```

### ADSR Configuration
```rust
attack_time: 0.08,    // 80ms - Slow analog synth pad attack
decay_time: 0.3,      // 300ms - Long decay for atmosphere
sustain_level: 0.6,   // 60% - Soft sustain
release_time: 0.4,    // 400ms - Long analog tail
```

### Key Characteristics
- ✅ Complex multi-oscillator architecture
- ✅ LFO modulation for organic movement
- ✅ PWM for analog synthesizer authenticity
- ✅ Soft distortion for warmth
- ✅ Chorus effect for spatial width
- ✅ Slow attack/long release for atmospheric pads

### Recommended For
- Cyberpunk/sci-fi coding projects
- Night-time programming sessions
- Atmospheric, immersive coding
- Users who love Blade Runner soundtracks

---

## 🎛️ Real-time Switching

### Function Key Mapping
- **F8** → Cyberpunk 2049
- **F9** → Natural Piano
- **F10** → Electronic
- **F11** → Saw Wave
- **F12** → Square Wave

### Command Line Options
```bash
cargo run natural    # Natural piano (default)
cargo run electronic # Pure electronic
cargo run saw        # Bright sawtooth
cargo run square     # Retro square wave
cargo run cyberpunk  # Blade Runner 2049 style
```

---

## 🔧 Technical Architecture

### Waveform Generation Pipeline
1. **Phase Calculation** → Independent per note
2. **Waveform Synthesis** → Type-specific algorithms
3. **ADSR Envelope** → Volume shaping
4. **Output Mixing** → Polyphonic combination

### Performance Characteristics
- **CPU Usage**: Natural > Cyberpunk > Saw/Square > Electronic
- **Memory Usage**: All waveforms have identical memory footprint
- **Latency**: <10ms for all waveforms

### Extensibility
The waveform system is designed for easy extension:
1. Add new enum variant in `Waveform`
2. Implement synthesis in `generate_sample()`
3. Define ADSR parameters in `set_waveform()`
4. Add UI controls and command line options

---

## 🎵 Musical Theory Integration

### Harmonic Content Analysis
| Waveform | Fundamental | 2nd | 3rd | 4th | 5th | Character |
|----------|-------------|-----|-----|-----|-----|-----------|
| Natural | 100% | 30% | 15% | 8% | 5% | Warm, piano-like |
| Electronic | 100% | 0% | 0% | 0% | 0% | Pure, mathematical |
| Saw | 100% | 50% | 33% | 25% | 20% | Bright, buzzy |
| Square | 100% | 0% | 33% | 0% | 20% | Hollow, 8-bit |
| Cyberpunk | Complex modulated harmonic structure | Atmospheric, evolving |

### Frequency Response
- **Natural**: Balanced across spectrum with slight mid-range emphasis
- **Electronic**: Perfect flat response
- **Saw**: High-frequency emphasis, bright character
- **Square**: Mid-range hollow with strong odd harmonics
- **Cyberpunk**: Dynamic response with filter modulation

---

## 💡 Usage Tips

### Choosing the Right Waveform
- **Long coding sessions**: Natural (least fatiguing)
- **Creative work**: Cyberpunk (inspiring atmosphere)
- **Testing/debugging**: Electronic (clear, precise)
- **Game development**: Square (thematic match)
- **Music production**: Saw (versatile, bright)

### Performance Optimization
- Use Electronic for maximum performance
- Natural and Cyberpunk are most CPU-intensive
- All waveforms support full polyphony

### Customization Potential
- ADSR parameters can be made user-configurable
- LFO rates and depths can be exposed as controls
- Filter cutoff frequencies can be made adjustable
- Additional effects (reverb, delay) can be added per waveform

This guide serves as both documentation and reference for understanding the artistic and technical decisions behind each waveform implementation.