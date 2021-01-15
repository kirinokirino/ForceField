#![warn(clippy::all)]

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use sdl2::render::{Canvas, Texture, WindowCanvas};
use std::time::Duration;

use sdl2::gfx::primitives::DrawRenderer;

const SCREEN_WIDTH: u32 = 1366;
const SCREEN_HEIGHT: u32 = 768;
const GRID_SIZE: u32 = 32;

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("ForceField", SCREEN_WIDTH, SCREEN_HEIGHT)
        .opengl() // this line DOES NOT enable opengl, but allows you to create/get an OpenGL context from your window.
        .fullscreen()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .present_vsync()
        .accelerated()
        .build()
        .expect("could not make a canvas");

    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    draw_grid(&mut canvas, GRID_SIZE);
    canvas.present();

    let mut events = sdl_context.event_pump()?;
    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if keycode == Keycode::Escape {
                        break 'main;
                    }
                }

                Event::MouseButtonDown { x, y, .. } => {
                    draw_axis(&mut canvas, x as i16, y as i16);
                    canvas.present();
                }

                _ => {}
            }
        }
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn draw_axis(canvas: &mut WindowCanvas, x: i16, y: i16) {
    let color = pixels::Color::RGB(255, 255, 255);
    let _ = canvas.hline(0, SCREEN_WIDTH as i16, y as i16, color);
    let _ = canvas.vline(x as i16, 0, SCREEN_HEIGHT as i16, color);
}

fn draw_grid(canvas: &mut WindowCanvas, cell_size: u32) {
    let color = pixels::Color::RGB(255, 255, 255);
    for square in 0..std::cmp::max(SCREEN_HEIGHT, SCREEN_WIDTH) / cell_size {
        let _ = canvas.hline(0, SCREEN_WIDTH as i16, (square * cell_size) as i16, color);
        let _ = canvas.vline((square * cell_size) as i16, 0, SCREEN_HEIGHT as i16, color);
    }
}
