// src/bridge/cli.rs

use std::process::Command;
use std::env;

pub fn run_colcon_build() -> Result<String, String> {
    let current_dir = env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?;
    
    let output = Command::new("bash")
        .arg("-c")
        .arg("colcon build --symlink-install")
        .current_dir(&current_dir)
        .output()
        .map_err(|e| format!("Failed to execute colcon: {}", e))?;

    let mut result = format!("Building in: {}\n", current_dir.display());
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        result.push_str(&stdout);
        Ok(result)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        result.push_str(&stderr);
        Err(result)
    }
}