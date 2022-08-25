use clap::Parser;
use image::{imageops, DynamicImage, ImageError};
use pixels::{Pixels, SurfaceTexture};
use thiserror::Error;
use winit::{
    dpi::PhysicalSize,
    error::OsError,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

const SCREEN_PERCENT: u32 = 90;

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
    #[error("Unable to load the image")]
    IoError(#[from] std::io::Error),
    #[error("Unable to decode image")]
    ImageError(#[from] ImageError),
    #[error("Unable to calculate maximum screen size on your primary monitor")]
    NoPrimaryMonitor,
    #[error("Unable to view image pixels")]
    PixelsError(#[from] pixels::Error),
}

fn main() -> Result<(), ErrorWrapper> {
    let args = Args::parse();
    // Load the image
    let mut img = image::io::Reader::open(&args.image)?.decode()?;

    let event_loop = EventLoop::new();

    let primary_monitor = event_loop
        .primary_monitor()
        .ok_or(ErrorWrapper::NoPrimaryMonitor)?;

    let screen_size = primary_monitor.size();

    let max_screen_size = (
        screen_size.width * SCREEN_PERCENT / 100,
        screen_size.height * SCREEN_PERCENT / 100,
    );

    let horz_scale = calc_scale(max_screen_size.0, img.width());
    let vert_scale = calc_scale(max_screen_size.1, img.height());
    let scale = horz_scale.max(vert_scale);
    let window_inner_size = PhysicalSize::new(img.width() / scale, img.height() / scale);

    let window = WindowBuilder::new()
        .with_title("Img Viewer")
        .with_inner_size(window_inner_size)
        .build(&event_loop)?;

    let surface = SurfaceTexture::new(window_inner_size.width, window_inner_size.height, &window);
    let mut pixels = Pixels::new(img.width(), img.height(), surface)?;
    let img_bytes = img.as_mut_rgba8().unwrap().as_flat_samples();
    let img_bytes = img_bytes.as_slice();

    let pixels_bytes = pixels.get_frame();
    img_bytes.into_iter().enumerate().for_each(|(idx, p)| {
        pixels_bytes[idx] = *p;
    });

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();
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
                    println!("[{:?}] {:#?}", virtual_keycode, state);
                    if state == ElementState::Released || virtual_keycode.is_none() {
                        return;
                    }

                    let new_img = match virtual_keycode.unwrap() {
                        VirtualKeyCode::C => Some(img.fliph()),
                        VirtualKeyCode::X => Some(img.flipv()),
                        VirtualKeyCode::Z => Some(img.rotate180()),
                        _ => None,
                    };

                    if new_img.is_none() {
                        return;
                    }

                    img = new_img.unwrap();

                    let img_bytes = img.as_mut_rgba8().unwrap().as_flat_samples();
                    let img_bytes = img_bytes.as_slice();

                    let pixels_bytes = pixels.get_frame();
                    img_bytes.into_iter().enumerate().for_each(|(idx, p)| {
                        pixels_bytes[idx] = *p;
                    });

                    pixels.render().unwrap();
                }
                WindowEvent::Resized(size) => {
                    println!("[resize] {:?}", size);
                    resize(&mut pixels, &size);
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                let _ = pixels.render();
            }
            _ => {}
        }
    });
}

fn calc_scale(max_size: u32, curr_size: u32) -> u32 {
    if max_size >= curr_size {
        1
    } else {
        (curr_size as f32 / max_size as f32).ceil() as u32
    }
}

fn resize(pixels: &mut Pixels, size: &PhysicalSize<u32>) {
    pixels.resize_surface(size.width, size.height)
}
