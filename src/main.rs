use std::{env, fs, path};

use url;
use toml;
use tao::{
    event::Event,
    event_loop::{ControlFlow, EventLoop}
};

mod windows;
mod macos;

fn url2posix(url: url::Url) -> String {
    if let Ok(path) = url.to_file_path() {
        path.to_string_lossy().to_string()
    }
    else {
        String::new()
    }
}

fn main() {
    let platform = env::consts::OS;

    if platform != "windows" && platform != "macos" {
        return;
    }

    let config_path = path::PathBuf::from(
        match platform {
            "windows" => env::var("USERPROFILE").unwrap(),
            "macos"   => env::var("HOME").unwrap(),
            _         => unreachable!()
        }
    ).join(".throughtheterminal");

    let config_content = fs::read_to_string(&config_path)
        .expect(".throughtheterminal doesn't exist");

    let config: toml::Value = config_content.parse::<toml::Value>()
        .expect("failed to parse .throughtheterminal");

    let terminal = config.get("terminal")
        .and_then(toml::Value::as_str)
        .expect("terminal doesn't exist in .throughtheterminal")
        .to_string();

    let editor = config.get("editor")
        .and_then(toml::Value::as_str)
        .expect("editor doesn't exist in .throughtheterminal")
        .to_string();

    let method = config.get("method")
        .and_then(toml::Value::as_str)
        .expect("method doesn't exist in .throughtheterminal")
        .to_string();

    if platform == "windows" {
        let arguments: Vec<String> = env::args().collect();
        windows::windows(
            &terminal,
            &editor,
            &method,
            arguments[1].clone()
        );
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
                        macos::macos(
                            &terminal,
                            &editor,
                            &method,
                            url2posix(url)
                        );
                    }
                }

                *control_flow = ControlFlow::Exit;
            }
        });
    }
}
