# Testing LazyROS with a Real ROS2 Package

## Quick Start

1. **Build lazyros** (if you haven't already):
   ```bash
   cd /home/tristan/Desktop/lazyros
   cargo build --release
   ```

2. **Navigate to your ROS2 workspace root**:
   ```bash
   cd ~/your_ros2_workspace  # or wherever your ROS2 workspace is
   ```

3. **Run lazyros**:
   ```bash
   /home/tristan/Desktop/lazyros/target/release/lazyros
   # Or if you've installed it:
   lazyros
   ```

4. **Press 'b'** to build and install your packages

5. **Press 'q'** to quit

## Example: Testing with a Simple Package

If you don't have a ROS2 workspace yet, you can create a test one:

```bash
# Create a test workspace
mkdir -p ~/test_ros2_ws/src
cd ~/test_ros2_ws/src

# Create a simple package (or use an existing one)
ros2 pkg create --build-type ament_cmake test_package

# Go back to workspace root
cd ~/test_ros2_ws

# Run lazyros
/home/tristan/Desktop/lazyros/target/release/lazyros

# Press 'b' to build
```

## What to Expect

- The TUI will show "Building package with colcon..." when you press 'b'
- Build output will appear in the logs area
- On success: "Build successful!" followed by the last few lines of output
- On failure: "Build failed:" followed by the error message

## Troubleshooting

- **"colcon: command not found"**: Make sure ROS2 is sourced:
  ```bash
  source /opt/ros/humble/setup.bash  # or your ROS2 distro
  ```

- **Build runs but no output**: The build might be very fast or cached. Try cleaning first:
  ```bash
  colcon build --cmake-clean-cache
  ```

- **Permission issues**: Make sure you have write permissions in the workspace directory

