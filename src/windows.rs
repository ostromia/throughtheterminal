use std::process::Command;

fn windows_terminal(editor: &String, method: &String, file_path: &String) {
    let window_index = if method == "tab" { 0 } else { -1 };

    let _ = Command::new("wt")
        .args(["--window", &window_index.to_string(), "new-tab", editor, &file_path])
        .output();
}

pub fn windows(terminal: &String, editor: &String, method: &String, file_path: String) {
    if terminal == "Windows Terminal" {
        windows_terminal(editor, method, &file_path);
    }
}
