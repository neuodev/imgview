use winit::{
    error::OsError,
    event::{Event, KeyboardInput, WindowEvent},
    event_loop::EventLoop,
    window::{CursorIcon, WindowBuilder},
};

fn main() -> Result<(), OsError> {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("ImgPre")
        .build(&event_loop)?;

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();
        window.set_cursor_icon(CursorIcon::Grab);
        match event {
            Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => {
                    println!("Close reqested!");
                    control_flow.set_exit();
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state,
                            virtual_keycode,
                            ..
                        },
                    ..
                } => {
                    println!("[{:?}] {:#?}", virtual_keycode, state)
                }
                WindowEvent::ScaleFactorChanged {
                    scale_factor,
                    new_inner_size,
                } => {
                    println!("[scale-factor] {}", scale_factor)
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {}
            _ => {}
        }
    });
}
