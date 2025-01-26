use std::env;
use std::process::Command;

use tao::{
    event::Event,
    event_loop::{ControlFlow, EventLoop}
};

fn get_file_path_windows() -> String {
    let arguments: Vec<String> = env::args().collect();
    return arguments[1].clone();
}

fn get_file_path_macos() -> Vec<String> {
    let mut result = Vec::new();
    let event_loop = EventLoop::new();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::Opened { urls } = event {
            result.extend(urls.iter().map(|url| url.to_string()));
            *control_flow = ControlFlow::Exit;
        }
    });
}

fn windows(file_path: String) {
}

fn macos(file_path: &String) {
}

fn main() {
    let platform = env::consts::OS;

    if platform == "windows" {
        windows(get_file_path_windows());
    }
    else if platform == "macos" {
        for i in get_file_path_macos().iter() {
            macos(i);
        }
    }
    else {
        return;
    }
}
