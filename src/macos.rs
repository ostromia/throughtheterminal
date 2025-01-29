use std::process::Command;

fn ghostty(editor: &String, method: &String, file_path: &String) {
    let applescript = format!(
        r#"
            set isGhosttyRunning to false

            if application "Ghostty" is running then
                set isGhosttyRunning to true
            end if

            tell application "Ghostty"
                activate
            end tell

            tell application "System Events"
                keystroke "{}" using command down
            end tell

            delay 0.1

            tell application "System Events"
                keystroke "{} \"{}\""
                key code 36
            end tell
        "#,
        if method == "tab" { "t" } else { "n" },
        editor,
        file_path
    );

    let _ = Command::new("osascript")
        .arg("-e")
        .arg(applescript)
        .output();
}

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

fn wezterm(editor: &String, method: &String, file_path: &String) {
    let applescript = format!(r#"
        tell application "WezTerm"
            activate
        end tell

        tell application "System Events"
            click menu bar item 3 of menu bar 1 of application process "WezTerm"
            click menu item 1 of menu 1 of menu bar item 3 of menu bar 1 of application process "WezTerm"
            delay 0.1
            keystroke "{} \"{}\""
            key code 36
        end tell
        "#,
        editor,
        file_path
    );

    let _ = Command::new("osascript")
        .arg("-e")
        .arg(applescript)
        .output();
}

pub fn macos(terminal: &String, editor: &String, method: &String, file_path: String) {
    if terminal == "Terminal" {
        terminalapp(editor, method, &file_path);
    }

    else if terminal == "WezTerm" {
        wezterm(editor, method, &file_path);
    }

    else if terminal == "Ghostty" {
        ghostty(editor, method, &file_path);
    }
}
