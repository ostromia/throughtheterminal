use std::process::Command;

fn terminalapp(editor: &String, method: &String, file_path: &String) {
    let applescript: String;

    if method == "tab" {
        applescript = format!(
            r#"
            tell application "Terminal"
                activate
                do script "{} \"{}\""
            end tell
            "#,
            editor,
            file_path
        );
    }
    else {
        applescript = format!(
            r#"
            tell application "Terminal"
                do script "{} \"{}\""
                activate
            end tell
            "#,
            editor,
            file_path
        );
    }

    let _ = Command::new("osascript")
        .arg("-e")
        .arg(applescript)
        .output();
}

pub fn macos(terminal: &String, editor: &String, method: &String, file_path: String) {
    if terminal == "Terminal" {
        terminalapp(editor, method, &file_path);
    }
}
