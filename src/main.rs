#![warn(clippy::all)]

extern crate sdl2;

mod grid;
use grid::Grid;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use std::time::Duration;

const SCREEN_WIDTH: u32 = 1366;
const SCREEN_HEIGHT: u32 = 768;

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
    let mut seed = 0i32;
    let mut cell_size = 32;
    let mut grid = Grid::new(cell_size, seed, SCREEN_WIDTH, SCREEN_HEIGHT);
    grid.draw(&mut canvas);
    canvas.present();

    let mut changes = (0., 0.);
    let mut global_time = 0.0;
    let mut events = sdl_context.event_pump()?;
    'main: loop {
        // Clear the screen
        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        // INPUT ----
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
                    if keycode == Keycode::Space {
                        seed += 1;
                        global_time = 0.0;
                        changes = (0., 0.);
                    }
                    if keycode == Keycode::Quote {
                        cell_size += 1;
                    } else if keycode == Keycode::Period {
                        let _some_other_stuff = ();
                        if cell_size >= 5 {
                            cell_size -= 1;
                        }
                    }
                    if keycode == Keycode::Comma {
                        changes.1 += -0.5;
                        changes.0 = 0.;
                    } else if keycode == Keycode::O {
                        changes.1 += 0.5;
                        changes.0 = 0.;
                    }

                    if keycode == Keycode::A {
                        changes.0 += -0.5;
                        changes.1 = 0.;
                    } else if keycode == Keycode::E {
                        changes.0 += 0.5;
                        changes.1 = 0.;
                    }
                }

                Event::MouseButtonDown { x: _, y: _, .. } => {
                    continue;
                }

                _ => {}
            }
        }

        // UPDATE ----
        grid.update(Some(cell_size), Some(seed), None, None);
        grid.set_offset_changes(changes, changes);
        global_time += 0.2;
        grid.tick(global_time);

        // DRAW ----
        grid.draw(&mut canvas);

        canvas.present();
        // Framerate limiter.
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
