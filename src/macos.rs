use std::process::Command;

fn terminalapp(editor: &String, file_path: &String) {
    let _ = Command::new("osascript")
        .arg("-e")
        .arg(format!(
            r#"
            tell application "Terminal"
                activate
                do script "{} \"{}\""
            end tell
            "#,
            editor,
            file_path
        ))
        .output();
}

pub fn macos(terminal: &String, editor: &String, method: &String, file_path: String) {
    if terminal == "Terminal" {
        terminalapp(editor, &file_path);
    }
}
