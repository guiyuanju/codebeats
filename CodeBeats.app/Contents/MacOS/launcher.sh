#!/bin/bash

# Get the directory where the app is located
APP_DIR="$(cd "$(dirname "$0")" && pwd)"
RESOURCES_DIR="$APP_DIR/../Resources"

# Set environment variables for resource paths
export CODEBEATS_RESOURCES_DIR="$RESOURCES_DIR"
export PATH="$APP_DIR:$PATH"

# Change to resources directory so relative paths work
cd "$RESOURCES_DIR" 2>/dev/null || cd "$APP_DIR"

# Launch the GUI
exec "$APP_DIR/CodeBeats"
