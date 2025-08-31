#!/bin/bash

# CodeBeats Quick Deployment Script
# Creates ready-to-distribute packages for end users

set -e

echo "🎵 CodeBeats Quick Deploy"
echo "========================"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ] || [ ! -f "src/main.rs" ]; then
    echo "❌ Error: Run this script from the CodeBeats project root directory"
    exit 1
fi

# Create releases directory
mkdir -p releases

echo "🔨 Building optimized release binaries..."
cargo build --release --bin codebeats
cargo build --release --bin codebeats-gui

echo "📦 Creating deployment package..."

# Determine platform
PLATFORM=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)
PACKAGE_NAME="codebeats-${PLATFORM}-${ARCH}"
DEPLOY_DIR="releases/${PACKAGE_NAME}"

# Clean and create deployment directory
rm -rf "$DEPLOY_DIR"
mkdir -p "$DEPLOY_DIR"

# Copy binaries
echo "📋 Copying binaries..."
cp "target/release/codebeats" "$DEPLOY_DIR/"
cp "target/release/codebeats-gui" "$DEPLOY_DIR/"

# Copy assets
echo "📂 Copying configuration files..."
cp -r language_configs "$DEPLOY_DIR/"

if [ -d "effects" ]; then
    echo "🔊 Copying audio files..."
    cp -r effects "$DEPLOY_DIR/"
fi

# Copy documentation
echo "📖 Copying documentation..."
cp README.md "$DEPLOY_DIR/"
[ -f "CHANGELOG.md" ] && cp CHANGELOG.md "$DEPLOY_DIR/"
[ -f "LICENSE" ] && cp LICENSE "$DEPLOY_DIR/"

# Create user-friendly launchers
echo "🚀 Creating launcher scripts..."

# GUI launcher
cat > "$DEPLOY_DIR/Start-CodeBeats-GUI.sh" << 'EOF'
#!/bin/bash
echo "🎵 Starting CodeBeats GUI..."
echo "   Close this window to exit CodeBeats"
echo ""
./codebeats-gui
EOF
chmod +x "$DEPLOY_DIR/Start-CodeBeats-GUI.sh"

# CLI launcher
cat > "$DEPLOY_DIR/Start-CodeBeats-CLI.sh" << 'EOF'
#!/bin/bash
echo "🎵 Starting CodeBeats CLI..."
echo "   Press Ctrl+C to exit"
echo ""
./codebeats
EOF
chmod +x "$DEPLOY_DIR/Start-CodeBeats-CLI.sh"

# Create user instructions
cat > "$DEPLOY_DIR/HOW-TO-RUN.txt" << 'EOF'
🎵 CodeBeats - How to Run

EASY WAY (Recommended):
  Double-click: Start-CodeBeats-GUI.sh
  This opens a user-friendly configuration window.

COMMAND LINE:
  Double-click: Start-CodeBeats-CLI.sh
  Or open terminal here and run: ./codebeats --help

WHAT'S INCLUDED:
  - codebeats-gui     = Graphical interface
  - codebeats         = Command-line interface
  - language_configs/ = Programming language settings
  - effects/          = Audio sample files
  - README.md         = Full documentation

QUICK START:
  1. Double-click "Start-CodeBeats-GUI.sh"
  2. Choose your programming language
  3. Select a waveform (try "cyberpunk" or "natural")
  4. Click "Start CodeBeats"
  5. Start typing to make music!

For more options, see README.md
EOF

# Make binaries executable (safety check)
chmod +x "$DEPLOY_DIR/codebeats"
chmod +x "$DEPLOY_DIR/codebeats-gui"

# Create archive
echo "🗜️  Creating distribution archive..."
cd releases
tar -czf "${PACKAGE_NAME}.tar.gz" "${PACKAGE_NAME}"
cd ..

# Calculate sizes
BINARY_SIZE=$(du -sh "$DEPLOY_DIR" | cut -f1)
ARCHIVE_SIZE=$(du -sh "releases/${PACKAGE_NAME}.tar.gz" | cut -f1)

echo ""
echo "✅ Deployment package created successfully!"
echo ""
echo "📊 Package Information:"
echo "   Platform: ${PLATFORM}-${ARCH}"
echo "   Folder size: ${BINARY_SIZE}"
echo "   Archive size: ${ARCHIVE_SIZE}"
echo ""
echo "📦 Ready to distribute:"
echo "   📁 releases/${PACKAGE_NAME}/"
echo "   🗜️  releases/${PACKAGE_NAME}.tar.gz"
echo ""
echo "🎯 User Instructions:"
echo "   1. Send them the .tar.gz file"
echo "   2. Tell them to extract it"
echo "   3. Tell them to double-click 'Start-CodeBeats-GUI.sh'"
echo ""
echo "🎵 Happy music making!"
