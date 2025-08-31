# Deployment Guide

This document explains how to build and deploy CodeBeats as standalone packages that users can run without installing Rust or Cargo.

## Quick Deployment

### Build for Current Platform
```bash
# Build standalone package for your current platform
./build-release.sh local
```

### Build for All Platforms
```bash
# Build for Windows, macOS, and Linux
./build-release.sh all
```

## Platform-Specific Builds

### Windows (x86_64)
```bash
# Cross-compile for Windows
rustup target add x86_64-pc-windows-gnu
./build-release.sh windows
```

### macOS (x86_64)
```bash
# Cross-compile for macOS (requires macOS SDK)
rustup target add x86_64-apple-darwin
./build-release.sh macos
```

### Linux (x86_64)
```bash
# Cross-compile for Linux
rustup target add x86_64-unknown-linux-gnu
./build-release.sh linux
```

## Deployment Package Structure

Each deployment package contains:

```
codebeats-platform/
├── codebeats              # CLI binary (codebeats.exe on Windows)
├── codebeats-gui          # GUI binary (codebeats-gui.exe on Windows)
├── language_configs/      # All language configuration files
│   ├── python.json
│   ├── rust.json
│   └── ...
├── effects/               # Audio sample files (if present)
│   └── fart-quick-short.wav
├── README.md              # Documentation
├── run-gui.sh/.bat        # Platform-specific GUI launcher
└── run-cli.sh/.bat        # Platform-specific CLI launcher
```

## User Instructions

### For End Users - GUI Version (Recommended)

**Windows:**
1. Extract the zip file
2. Double-click `codebeats-gui.exe` or `run-gui.bat`

**macOS/Linux:**
1. Extract the tar.gz file
2. Double-click `codebeats-gui` or run `./run-gui.sh`

### For End Users - CLI Version

**Windows:**
```cmd
# Extract zip file, then open Command Prompt in the folder
codebeats.exe --help
codebeats.exe -w cyberpunk -v 0.5
```

**macOS/Linux:**
```bash
# Extract tar.gz file, then open terminal in the folder
./codebeats --help
./codebeats -w cyberpunk -v 0.5
```

## Development vs Deployment Architecture

### Development Mode
- GUI spawns CLI via `cargo run --bin codebeats`
- Assets loaded from source directory structure
- Requires Rust/Cargo installation

### Deployment Mode
- GUI looks for CLI binary in same directory
- Assets loaded relative to executable location
- Completely standalone - no dependencies

### Binary Discovery Logic
1. Look for CLI binary in same directory as GUI binary
2. Fallback to current working directory
3. Show error if CLI binary not found

### Asset Discovery Logic
1. Look for `language_configs/` relative to GUI binary location
2. Fallback to current working directory
3. Use built-in defaults if configs not found

## Cross-Compilation Requirements

### Windows Cross-Compilation (from macOS/Linux)
```bash
# Install Windows target
rustup target add x86_64-pc-windows-gnu

# macOS: Install mingw-w64
brew install mingw-w64

# Linux: Install mingw-w64
sudo apt-get install gcc-mingw-w64-x86-64
```

### macOS Cross-Compilation
```bash
# Install macOS target
rustup target add x86_64-apple-darwin

# Requires macOS SDK (only available on macOS)
```

### Linux Cross-Compilation
```bash
# Install Linux target
rustup target add x86_64-unknown-linux-gnu

# Usually works without additional dependencies
```

## Release Workflow

### 1. Build All Platforms
```bash
./build-release.sh all
```

### 2. Test Packages
```bash
# Test the local build
cd releases/codebeats-*-local/
./run-gui.sh  # or double-click codebeats-gui
```

### 3. Upload to Distribution
```bash
ls releases/
# codebeats-linux-x64.tar.gz
# codebeats-macos-x64.tar.gz  
# codebeats-windows-x64.zip
```

## Distribution Checklist

- [ ] GUI binary launches without errors
- [ ] CLI binary works independently
- [ ] Language configs are discovered correctly
- [ ] Audio files are included (if present)
- [ ] Documentation is included
- [ ] Platform-specific launchers work
- [ ] No Rust/Cargo dependencies required
- [ ] Archive extracts cleanly

## Troubleshooting

### "CLI binary not found"
- Ensure both `codebeats` and `codebeats-gui` are in the same directory
- Check file permissions (executable bit on Unix systems)
- Verify binary names match platform expectations (`.exe` on Windows)

### "Language configs not found"
- Ensure `language_configs/` directory is in the same folder as binaries
- Check that JSON files are valid and readable
- GUI will use built-in defaults if configs are missing

### Audio issues
- Ensure system audio is working
- Check volume settings in GUI
- Audio files (like fart sample) are optional - synthetic fallback is used

### Cross-compilation fails
- Install required toolchains for target platforms
- Some targets may require additional system dependencies
- macOS targets typically require building on macOS

## Security Considerations

- Binaries are not code-signed (add signing for production)
- Users may need to allow execution on macOS/Windows (security warnings)
- Consider notarization for macOS distribution
- Virus scanners may flag cross-compiled binaries (false positives)

## File Sizes

Typical deployment package sizes:
- **CLI binary**: ~4-8MB (includes audio synthesis)
- **GUI binary**: ~15-25MB (includes UI framework)  
- **Language configs**: ~50KB total
- **Audio samples**: ~100KB (if included)
- **Total package**: ~20-35MB per platform

## Performance Notes

- Release builds are significantly faster than debug builds
- Direct binary execution is much faster than `cargo run`
- GUI startup time: ~1-3 seconds
- CLI startup time: ~100-500ms
- Audio latency: ~10-50ms depending on system