use std::process::Command;

use windows::Win32::{
    Foundation::{
        BOOL,
        HANDLE,
        HWND,
        LPARAM
    },
    System::{
        ProcessStatus::GetModuleFileNameExW,
        Threading::{
            OpenProcess,
            PROCESS_ALL_ACCESS,
            PROCESS_QUERY_INFORMATION
        },
    },
    UI::WindowsAndMessaging::{
        EnumWindows,
        GetWindowTextW,
        GetWindowThreadProcessId,
        IsWindowVisible,
        SetForegroundWindow
    }
};

struct Process {
    pid: u32,
    hwnd: HWND,
    window_title: String,
    module_file_name: String
}

fn windows_terminal(editor: &String, method: &String, file_path: &String) {
    let window_index = if method == "tab" { 0 } else { -1 };

    let _ = Command::new("wt")
        .args(["--window", &window_index.to_string(), "new-tab", editor, &file_path])
        .output();
}

unsafe fn get_window_text(hwnd: HWND) -> String {
    let mut buffer: [u16; 512] = [0; 512];
    let length = GetWindowTextW(hwnd, &mut buffer);
    return String::from_utf16_lossy(&buffer[..length as usize]);
}

unsafe fn get_process_id(hwnd: HWND) -> u32 {
    let mut pid = 0;
    GetWindowThreadProcessId(hwnd, Some(&mut pid));
    return pid;
}

unsafe fn get_module_file_name(pid: u32) -> String {
    let process_handle: HANDLE = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_ALL_ACCESS, false, pid).unwrap();
    let mut path: [u16; 512] = [0; 512];
    let length = GetModuleFileNameExW(Some(process_handle), None, &mut path);
    return String::from_utf16_lossy(&path[..length as usize]);
}

unsafe extern "system" fn enum_windows_callback(hwnd: HWND, _lparam: LPARAM) -> BOOL {
    if IsWindowVisible(hwnd).as_bool() {
        let window_title = get_window_text(hwnd);
        let pid = get_process_id(hwnd);
        let module_file_name = get_module_file_name(pid);

        if module_file_name.contains("wezterm-gui.exe") {
            let process = Process {
                pid,
                hwnd,
                window_title: window_title.clone(),
                module_file_name: module_file_name.clone(),
            };

            let process_list: &mut Vec<Process> = &mut *(_lparam.0 as *mut Vec<Process>);
            process_list.push(process);
        }
    }

    return BOOL(1);
}

fn wezterm(editor: &String, method: &String, file_path: &String) {
    let mut processes: Vec<Process> = Vec::new();

    unsafe {
        let _ = EnumWindows(Some(enum_windows_callback), LPARAM(&mut processes as *mut _ as isize));
    }

    for process in &processes {
        println!(
            "PID: {}, HWND: {:?}, Title: {}, Module: {}",
            process.pid, process.hwnd, process.window_title, process.module_file_name
        );
    }

}

pub fn windows(terminal: &String, editor: &String, method: &String, file_path: String) {
    if terminal == "Windows Terminal" {
        windows_terminal(editor, method, &file_path);
    }
    else if terminal == "WezTerm" {
        wezterm(editor, method, &file_path);
    }
}
