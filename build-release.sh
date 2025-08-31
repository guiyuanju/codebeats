#!/bin/bash

# CodeBeats Release Build Script
# Builds standalone deployment packages for different platforms

set -e

PROJECT_NAME="codebeats"
GUI_BINARY="${PROJECT_NAME}-gui"
CLI_BINARY="${PROJECT_NAME}"

echo "🎵 CodeBeats Release Builder"
echo "=========================="

show_help() {
    echo ""
    echo "Usage: $0 [target]"
    echo ""
    echo "Targets:"
    echo "  local          Build for current platform (default)"
    echo "  windows        Build for Windows (x86_64-pc-windows-gnu)"
    echo "  macos          Build for macOS (x86_64-apple-darwin)"
    echo "  linux          Build for Linux (x86_64-unknown-linux-gnu)"
    echo "  all            Build for all platforms"
    echo "  clean          Clean all build artifacts"
    echo ""
    echo "Examples:"
    echo "  $0 local       # Build for current platform"
    echo "  $0 windows     # Cross-compile for Windows"
    echo "  $0 all         # Build for all platforms"
    echo ""
}

build_for_target() {
    local target=$1
    local target_name=$2

    echo "🔨 Building for $target_name ($target)..."

    # Add target if not already installed
    if [ "$target" != "local" ]; then
        rustup target add $target 2>/dev/null || true
    fi

    # Build both binaries
    if [ "$target" = "local" ]; then
        cargo build --release --bin $CLI_BINARY
        cargo build --release --bin $GUI_BINARY
        local build_dir="target/release"
    else
        cargo build --release --target $target --bin $CLI_BINARY
        cargo build --release --target $target --bin $GUI_BINARY
        local build_dir="target/$target/release"
    fi

    # Create deployment directory
    local deploy_dir="releases/${PROJECT_NAME}-${target_name}"
    mkdir -p "$deploy_dir"

    # Copy binaries
    if [ "$target" = "x86_64-pc-windows-gnu" ]; then
        cp "$build_dir/${CLI_BINARY}.exe" "$deploy_dir/"
        cp "$build_dir/${GUI_BINARY}.exe" "$deploy_dir/"
    else
        cp "$build_dir/$CLI_BINARY" "$deploy_dir/"
        cp "$build_dir/$GUI_BINARY" "$deploy_dir/"
    fi

    # Copy assets
    cp -r language_configs "$deploy_dir/"
    if [ -d "effects" ]; then
        cp -r effects "$deploy_dir/"
    fi

    # Copy documentation
    cp README.md "$deploy_dir/"
    cp CHANGELOG.md "$deploy_dir/" 2>/dev/null || true

    # Create platform-specific run script
    create_run_script "$deploy_dir" "$target"

    # Create archive
    echo "📦 Creating archive..."
    cd releases
    if [ "$target" = "x86_64-pc-windows-gnu" ]; then
        zip -r "${PROJECT_NAME}-${target_name}.zip" "${PROJECT_NAME}-${target_name}"
    else
        tar -czf "${PROJECT_NAME}-${target_name}.tar.gz" "${PROJECT_NAME}-${target_name}"
    fi
    cd ..

    echo "✅ Built $target_name: releases/${PROJECT_NAME}-${target_name}"
}

create_run_script() {
    local deploy_dir=$1
    local target=$2

    if [ "$target" = "x86_64-pc-windows-gnu" ]; then
        # Windows batch file
        cat > "$deploy_dir/run-gui.bat" << 'EOF'
@echo off
echo Starting CodeBeats GUI...
codebeats-gui.exe
pause
EOF
        cat > "$deploy_dir/run-cli.bat" << 'EOF'
@echo off
echo Starting CodeBeats CLI...
echo Press Ctrl+C to exit
codebeats.exe %*
pause
EOF
    else
        # Unix shell script
        cat > "$deploy_dir/run-gui.sh" << 'EOF'
#!/bin/bash
echo "Starting CodeBeats GUI..."
./codebeats-gui "$@"
EOF
        cat > "$deploy_dir/run-cli.sh" << 'EOF'
#!/bin/bash
echo "Starting CodeBeats CLI..."
echo "Press Ctrl+C to exit"
./codebeats "$@"
EOF
        chmod +x "$deploy_dir/run-gui.sh"
        chmod +x "$deploy_dir/run-cli.sh"
    fi
}

clean_build() {
    echo "🧹 Cleaning build artifacts..."
    cargo clean
    rm -rf releases
    echo "✅ Clean complete"
}

# Main script logic
case "${1:-local}" in
    "local")
        mkdir -p releases
        build_for_target "local" "$(uname -m)-$(uname -s | tr '[:upper:]' '[:lower:]')"
        ;;
    "windows")
        mkdir -p releases
        build_for_target "x86_64-pc-windows-gnu" "windows-x64"
        ;;
    "macos")
        mkdir -p releases
        build_for_target "x86_64-apple-darwin" "macos-x64"
        ;;
    "linux")
        mkdir -p releases
        build_for_target "x86_64-unknown-linux-gnu" "linux-x64"
        ;;
    "all")
        mkdir -p releases
        echo "🌍 Building for all platforms..."
        build_for_target "local" "$(uname -m)-$(uname -s | tr '[:upper:]' '[:lower:]')"
        build_for_target "x86_64-pc-windows-gnu" "windows-x64"
        build_for_target "x86_64-apple-darwin" "macos-x64"
        build_for_target "x86_64-unknown-linux-gnu" "linux-x64"
        echo ""
        echo "🎉 All builds complete!"
        echo "📦 Archives created in releases/ directory"
        ;;
    "clean")
        clean_build
        ;;
    "help"|"--help"|"-h")
        show_help
        ;;
    *)
        echo "❌ Unknown target: $1"
        show_help
        exit 1
        ;;
esac

echo ""
echo "🎵 Build complete! Happy music making! 🎵"
