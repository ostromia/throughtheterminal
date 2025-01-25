use std::env;

use std::thread;
use std::time::Duration;

fn windows(file_path: String) {
    println!("Windows machine detected\n");
    println!("file path: {}", file_path);
    thread::sleep(Duration::new(10, 0));
}

fn macos(file_path: String) {
    println!("MacOS machine detected\n");
    println!("file path: {}", file_path);
    thread::sleep(Duration::new(10, 0));
}

fn main() {
    let platform = env::consts::OS;
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() >= 2 {
        let file_path = arguments[1].clone();

        if platform == "windows" {
            windows(file_path);
        }
        else if platform == "macos" {
            macos(file_path);
        }
    }
}
