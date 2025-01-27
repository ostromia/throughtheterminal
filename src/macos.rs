use std::process::Command;

pub fn macos(file_path: String) {
    let _ = Command::new("osascript")
        .arg("-e")
        .arg(format!(
            r#"
            tell application "Terminal"
                activate
                do script "nvim \"{}\""
            end tell
            "#,
        file_path
        ))
        .output();
}