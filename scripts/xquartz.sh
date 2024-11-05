#!/bin/bash

# Function to check if XQuartz is already running
check_xquartz() {
    pgrep -x "Xquartz" > /dev/null
    return $?
}

# Function to check if X11 is accepting connections
check_x11() {
    /opt/X11/bin/xhost &> /dev/null
    return $?
}

echo "Setting up XQuartz..."

# Add X11 bin to PATH if not already there
if [[ ":$PATH:" != *":/opt/X11/bin:"* ]]; then
    export PATH="/opt/X11/bin:$PATH"
fi

# Start XQuartz if not running
if ! check_xquartz; then
    echo "Starting XQuartz..."
    open -a XQuartz
    
    # Wait for XQuartz to start (adjust sleep time if needed)
    sleep 3
fi

# Set DISPLAY if not set
if [ -z "$DISPLAY" ]; then
    echo "Setting DISPLAY variable..."
    export DISPLAY=:0
fi

# Check and set X11 permissions
if ! check_x11; then
    echo "Setting up X11 permissions..."
    /opt/X11/bin/xhost + > /dev/null
fi

echo "XQuartz setup complete!"
echo "DISPLAY=$DISPLAY"
echo "X11 permissions:"
/opt/X11/bin/xhost