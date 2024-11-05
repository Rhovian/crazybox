#!/bin/bash

# Check if running on macOS host
if [[ "$OSTYPE" == "darwin"* ]]; then
    # Note: this script should run on the Mac host, not in container
    echo "Setting up X11 forwarding for macOS..."
    
    # Check if XQuartz is installed
    if ! command -v xquartz &> /dev/null && ! [ -d "/Applications/XQuartz.app" ]; then
        echo "XQuartz is not installed. Please install it with: brew install --cask xquartz"
        exit 1
    }

    # Start XQuartz if not running
    if ! pgrep -x "Xquartz" > /dev/null; then
        echo "Starting XQuartz..."
        open -a XQuartz
        
        # Wait for XQuartz to start
        sleep 3
    fi

    # Get the current DISPLAY number
    export DISPLAY=:0

    # Try to set xhost permissions
    if command -v xhost &> /dev/null; then
        xhost + localhost
        echo "X11 forwarding set up successfully"
    else
        echo "Warning: xhost command not found. X11 forwarding might not work properly"
    fi
else
    echo "Not running on macOS, skipping X11 setup"
fi