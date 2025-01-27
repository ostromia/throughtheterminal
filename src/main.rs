use std::env;

use url::Url;
use tao::{event::Event, event_loop::{ControlFlow, EventLoop}};

mod windows;
mod macos;

fn url2posix(url: Url) -> String {
    if let Ok(path) = url.to_file_path() {
        path.to_string_lossy().to_string()
    }
    else {
        String::new()
    }
}

fn main() {
    let platform = env::consts::OS;

    if platform == "windows" {
        let arguments: Vec<String> = env::args().collect();
        windows::windows(arguments[1].clone());
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
                        macos::macos(url2posix(url));
                    }
                }

                *control_flow = ControlFlow::Exit;
            }
        });
    }
}
