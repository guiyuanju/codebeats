#!/bin/bash

# CodeBeats Launcher Script
# Provides easy access to both CLI and GUI versions

set -e

show_help() {
    echo "CodeBeats Launcher 🎵"
    echo ""
    echo "Usage: $0 [option] [cli-args...]"
    echo ""
    echo "Options:"
    echo "  gui          Launch GUI configuration interface (default)"
    echo "  cli          Launch command-line interface"
    echo "  build        Build both versions"
    echo "  build-gui    Build GUI version only"
    echo "  build-cli    Build CLI version only"
    echo "  help         Show this help"
    echo ""
    echo "Examples:"
    echo "  $0                           # Launch GUI"
    echo "  $0 gui                       # Launch GUI"
    echo "  $0 cli                       # Launch CLI with default settings"
    echo "  $0 cli -w cyberpunk -v 0.5   # Launch CLI with specific options"
    echo "  $0 build                     # Build both versions"
    echo ""
    echo "For CLI options, run: $0 cli --help"
}

case "${1:-gui}" in
    "gui"|"")
        echo "🎵 Starting CodeBeats GUI..."
        cargo run --bin codebeats-gui
        ;;
    "cli")
        shift  # Remove 'cli' from arguments
        echo "🎵 Starting CodeBeats CLI..."
        if [ $# -eq 0 ]; then
            cargo run --bin codebeats
        else
            cargo run --bin codebeats -- "$@"
        fi
        ;;
    "build")
        echo "🔨 Building both CodeBeats versions..."
        cargo build --release --bin codebeats
        cargo build --release --bin codebeats-gui
        echo "✓ Built target/release/codebeats (CLI)"
        echo "✓ Built target/release/codebeats-gui (GUI)"
        ;;
    "build-gui")
        echo "🔨 Building CodeBeats GUI..."
        cargo build --release --bin codebeats-gui
        echo "✓ Built target/release/codebeats-gui"
        ;;
    "build-cli")
        echo "🔨 Building CodeBeats CLI..."
        cargo build --release --bin codebeats
        echo "✓ Built target/release/codebeats"
        ;;
    "help"|"--help"|"-h")
        show_help
        ;;
    *)
        echo "❌ Unknown option: $1"
        echo ""
        show_help
        exit 1
        ;;
esac
