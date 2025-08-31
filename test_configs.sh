#!/bin/bash

# Configuration Testing Script for CodeBeats
# Tests different keyboard configurations to demonstrate language-specific mappings

echo "🎵 CodeBeats Configuration Testing Script"
echo "========================================="
echo

# Function to run a config for a few seconds
test_config() {
    local config_file=$1
    local description=$2
    local duration=${3:-5}

    echo "🔧 Testing: $description"
    echo "📁 Config: $config_file"
    echo "⏱️  Duration: ${duration} seconds"
    echo "👆 Type some code to hear the differences..."
    echo

    # Start the program in background
    cargo run config "$config_file" 2>/dev/null &
    local pid=$!

    # Wait for specified duration
    sleep $duration

    # Kill the program
    kill $pid 2>/dev/null
    wait $pid 2>/dev/null

    echo "✅ Test completed"
    echo "---"
    echo
}

echo "This script will test different programming language configurations."
echo "Each test runs for 5 seconds - try typing different code patterns!"
echo
read -p "Press Enter to continue..."
echo

# Test default configuration
echo "🎯 Testing Default Programming Configuration"
test_config "keyboard_config.json" "Default programming-optimized mapping" 7

# Test language-specific configurations
test_config "language_configs/rust.json" "Rust systems programming (try: let mut x = &y; fn main() {})" 8
test_config "language_configs/javascript.json" "JavaScript web development (try: const fn = () => {}; console.log())" 8
test_config "language_configs/go.json" "Go simple efficiency (try: func main() { if err := nil })" 8
test_config "language_configs/c.json" "C foundation (try: int *ptr = &var; #include <stdio.h>)" 8

# Test example configurations
test_config "example_configs/piano_layout.json" "Piano layout (try: QWERTY row = white keys)" 6
test_config "example_configs/minimal.json" "Minimal configuration (only essential keys)" 6

echo "🎊 All configuration tests completed!"
echo
echo "💡 Usage tips:"
echo "  • Different languages emphasize different syntax patterns"
echo "  • Rust config highlights ownership operators (&, *, ->) in bass tones"
echo "  • JavaScript config makes arrow functions (=>) leap to higher octaves"
echo "  • Go config uses clean intervals matching the language philosophy"
echo "  • C config uses deep bass for pointer operations and bright tones for preprocessor"
echo
echo "🚀 To use any configuration permanently:"
echo "  cargo run config language_configs/rust.json"
echo "  cargo run cyberpunk config language_configs/javascript.json"
echo
echo "📝 To create your own configuration:"
echo "  cargo run -- generate-config"
echo "  # Then edit keyboard_config.json"
echo
echo "🎵 Happy coding with musical feedback!"
