use tao::{
    event::Event,
    event_loop::{ControlFlow, EventLoop}
};

pub fn get_file_path_macos() -> Vec<String> {
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

pub fn macos(file_path: &String) {
}
