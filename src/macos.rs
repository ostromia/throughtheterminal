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

fn wezterm(editor: &String, method: &String, file_path: &String) {
    let applescript: String;

    if method == "tab" {
        applescript = format!(r#"
            set isWeztermRunning to false

            if application "WezTerm" is running then
                set isWeztermRunning to true
            end if

            tell application "WezTerm"
                activate
            end tell

            if isWeztermRunning then
                tell application "System Events"
                    click menu bar item 3 of menu bar 1 of application process "WezTerm"
                    click menu item 1 of menu 1 of menu bar item 3 of menu bar 1 of application process "WezTerm"
                end tell
            end if

            delay 0.1

            tell application "System Events"
                keystroke "{} \"{}\""
                key code 36
            end tell
            "#,
            editor,
            file_path
        );
    }
    else {
        applescript = format!(r#"
            set isWeztermRunning to false

            if application "WezTerm" is running then
                set isWeztermRunning to true
            end if

            tell application "WezTerm"
                activate
            end tell

            if isWeztermRunning then
                tell application "System Events"
                    click menu bar item 3 of menu bar 1 of application process "WezTerm"
                    click menu item 2 of menu 1 of menu bar item 3 of menu bar 1 of application process "WezTerm"
                end tell
            end if

            delay 0.5

            tell application "System Events"
                keystroke "{} \"{}\""
                key code 36
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
    else if terminal == "WezTerm" {
        wezterm(editor, method, &file_path);
    }
}
