use std::env;
use std::process::Command;

use url::Url;

use tao::{event::Event, event_loop::{ControlFlow, EventLoop}};

fn url2posix(url: Url) -> String {
    if let Ok(path) = url.to_file_path() {
        path.to_string_lossy().to_string()
    } else {
        String::new()
    }
}

fn windows(file_path: String) {
    Command::new("wt")
        .args(["--window", "0", "new-tab", "nvim", &file_path]);
}

fn macos(file_path: String) {
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

fn main() {
    let platform = env::consts::OS;

    if platform == "windows" {
        let arguments: Vec<String> = env::args().collect();
        windows(arguments[1].clone());
    }

    else if platform == "macos" {
        let event_loop = EventLoop::new();
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            if let Event::Opened { urls } = event {

                for url in urls {
                    if platform == "windows" {

                    }
                    else if platform == "macos" {
                        macos(url2posix(url));
                    }
                }

                *control_flow = ControlFlow::Exit;
            }
        });
    }
}
