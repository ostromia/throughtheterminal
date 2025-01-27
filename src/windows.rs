use std::process::Command;

fn windows_terminal(editor: &String, file_path: &String) {
    let _ = Command::new("wt")
        .args(["--window", "0", "new-tab", editor, &file_path])
        .output();
}

pub fn windows(terminal: &String, editor: &String, method: &String, file_path: String) {
    if terminal == "Windows Terminal" {
        windows_terminal(editor, &file_path);
    }
}
