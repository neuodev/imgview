use clap::{Arg, Parser};
use thiserror::Error;
use winit::{
    error::OsError,
    event::{Event, KeyboardInput, WindowEvent},
    event_loop::EventLoop,
    window::{CursorIcon, WindowBuilder},
};

/// Simple program to view images
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Image path
    #[clap(short, long, value_parser, default_value = "grades.png")]
    image: String,
}

#[derive(Debug, Error)]
enum ErrorWrapper {
    #[error("Unable to create the main window")]
    WindowError(#[from] OsError),
}

fn main() -> Result<(), ErrorWrapper> {
    let args = Args::parse();
    // Load the image
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Img Viewer")
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
