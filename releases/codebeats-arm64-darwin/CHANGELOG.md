# Changelog

## v2.0 - Simplified & Optimized (Latest)

### ✨ New Features
- **8 New Language Configurations**: Added Haskell, Java, Clojure, Scheme, Emacs Lisp, English, Chinese, Japanese
- **4 New Waveforms**: Harmonic, Sine, Triangle, Sawtooth for complete language config support
- **Simplified Terminal Output**: Clean, minimal logging for better user experience

### 🎵 Audio Improvements
- **Frequency Optimization**: Lowered 131 harsh high notes (>1000Hz) to comfortable ranges (65-880Hz)
- **Musical Harmony**: All language configs now use ear-friendly frequencies
- **8 Waveform Types**: Natural, Electronic, Cyberpunk, Harmonic, Triangle, Sine, Saw, Square

### 📚 Language Support
**Programming Languages (11 total)**:
- General, Python, Rust, JavaScript, C, Go
- Haskell (mathematical precision), Java (enterprise), Clojure (jazz), Scheme (minimalist), Emacs Lisp (editor)

**Human Languages (3 total)**:
- English (letter frequency), Chinese (Pinyin), Japanese (Romaji)

### 🔧 Technical
- **Simplified Logging**: Removed verbose key press/release messages
- **Clean Startup**: Minimal terminal output, focus on music
- **All Tests Passing**: 18 unit tests covering core functionality
- **Cross-Platform**: macOS, Windows, Linux via CPAL

### 📝 Documentation
- **Streamlined README**: Concise, focused on getting started quickly  
- **Simplified Implementation**: Technical details without overwhelming complexity
- **Better Examples**: Clear usage patterns for different scenarios

---

## v1.0 - Initial Release

- Basic audio synthesis with ADSR envelopes
- 6 programming language configurations
- Rate limiting and polyphonic support
- Real-time keyboard input processing