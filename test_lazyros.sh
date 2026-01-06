#!/bin/bash
# Helper script to test lazyros with a ROS2 workspace

# Check if lazyros is built
if [ ! -f "target/release/lazyros" ] && [ ! -f "target/debug/lazyros" ]; then
    echo "Building lazyros..."
    cargo build --release
fi

# Check if a workspace directory was provided
if [ -z "$1" ]; then
    echo "Usage: $0 <ros2_workspace_path>"
    echo ""
    echo "Example:"
    echo "  $0 ~/my_ros2_ws"
    echo ""
    echo "Or run from your workspace directory:"
    echo "  cd ~/my_ros2_ws"
    echo "  $(pwd)/target/release/lazyros"
    exit 1
fi

WORKSPACE="$1"

# Check if workspace exists
if [ ! -d "$WORKSPACE" ]; then
    echo "Error: Workspace directory '$WORKSPACE' does not exist"
    exit 1
fi

# Check if it looks like a ROS2 workspace
if [ ! -d "$WORKSPACE/src" ]; then
    echo "Warning: '$WORKSPACE' doesn't have a 'src' directory."
    echo "This might not be a ROS2 workspace."
    read -p "Continue anyway? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Source ROS2 if available (try common locations)
if [ -f "/opt/ros/humble/setup.bash" ]; then
    source /opt/ros/humble/setup.bash
elif [ -f "/opt/ros/foxy/setup.bash" ]; then
    source /opt/ros/foxy/setup.bash
elif [ -f "/opt/ros/galactic/setup.bash" ]; then
    source /opt/ros/galactic/setup.bash
fi

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Change to workspace and run lazyros
echo "Running lazyros in workspace: $WORKSPACE"
echo "Press 'b' to build, 'q' to quit"
echo ""

cd "$WORKSPACE"

# Run lazyros from the script's directory
if [ -f "$SCRIPT_DIR/target/release/lazyros" ]; then
    "$SCRIPT_DIR/target/release/lazyros"
elif [ -f "$SCRIPT_DIR/target/debug/lazyros" ]; then
    "$SCRIPT_DIR/target/debug/lazyros"
else
    echo "Error: lazyros binary not found. Run 'cargo build --release' first."
    exit 1
fi

