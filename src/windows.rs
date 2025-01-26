use std::env;

pub fn get_file_path_windows() -> String {
    let arguments: Vec<String> = env::args().collect();
    return arguments[1].clone();
}

pub fn windows(file_path: &String) {
}