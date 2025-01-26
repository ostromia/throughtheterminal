use std::env;

mod windows;
mod macos;

fn main() {
    let platform = env::consts::OS;

    if platform == "windows" {
        windows::windows(&windows::get_file_path_windows());
    }
    else if platform == "macos" {
        for i in macos::get_file_path_macos().iter() {
            macos::macos(i);
        }
    }
    else {
        return;
    }
}
