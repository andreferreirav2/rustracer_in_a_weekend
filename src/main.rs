#![deny(clippy::all)]
#![forbid(unsafe_code)]

use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use std::time::{Instant};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod vec3;
use crate::vec3::Vec3;
use Vec3 as Color;
use Vec3 as Point3;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

struct World {
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Raytracer in a weekend")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let mut world = World::new();
    let mut draw_calls = 0;
    let mut draw_calls_duration = 0.0;
    let mut instant_of_last_call = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            let before_draw = Instant::now();
            world.draw(pixels.frame_mut());
            draw_calls_duration += before_draw.elapsed().as_secs_f64();

            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                *control_flow = ControlFlow::Exit;
                return;
            }

            // fps counter
            draw_calls += 1;
            if instant_of_last_call.elapsed().as_secs_f64() > 1.0 {
                println!("{:.2} render fps, {:.2} real fps", (draw_calls as f64) / draw_calls_duration, (draw_calls as f64) / instant_of_last_call.elapsed().as_secs_f64());
                instant_of_last_call = Instant::now();
                draw_calls_duration = 0.0;
                draw_calls = 0;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Update internal state and request a redraw
            world.update();
            window.request_redraw();
        }
    });
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

impl World {
    fn new() -> Self {
        Self {}
    }

    fn update(&mut self) {
    }

    /// Draw the `World` state to the frame buffer.
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as f64;
            let y = (i / WIDTH as usize) as f64;

            let color = Color{i: x / ((WIDTH-1) as f64), j: y / ((HEIGHT-1) as f64), k: 0.25};
            let rgba = color.as_rgba();

            pixel.copy_from_slice(&rgba);
        }
    }
}
