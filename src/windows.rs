use std::process::Command;

pub fn windows(file_path: String) {
    let _ = Command::new("wt")
        .args(["--window", "0", "new-tab", "nvim", &file_path])
        .output();
}