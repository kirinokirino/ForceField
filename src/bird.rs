use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels;
use sdl2::render::WindowCanvas;

use crate::Grid;
use crate::SCREEN_HEIGHT;
use crate::SCREEN_WIDTH;
pub struct Bird {
    pub x: f32,
    pub y: f32,
    pub vel: (f32, f32),
}

impl Bird {
    pub const fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            vel: (0.0, 0.0),
        }
    }
    pub fn tick(&mut self, grid: &Grid) {
        if self.x < 0. {
            self.x = f32::from(SCREEN_WIDTH);
        } else if self.x > f32::from(SCREEN_WIDTH) {
            self.x = 0.;
        }
        if self.y < 0. {
            self.y = f32::from(SCREEN_HEIGHT);
        } else if self.y > f32::from(SCREEN_HEIGHT) {
            self.y = 0.;
        }
        let (angle, length) = grid.get_vec_on_position(self.x, self.y);
        let drag = 0.95;
        self.vel.0 += (angle.cos()) * length;
        self.vel.1 += (angle.sin()) * length;
        self.vel.0 *= drag;
        self.vel.1 *= drag;
        self.x += self.vel.0;
        self.y += self.vel.1;
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn draw(&self, canvas: &mut WindowCanvas) {
        let color = pixels::Color::RGBA(180, 220, 255, 255);
        let _ = canvas.aa_circle(self.x as i16, self.y as i16, 10_i16, color);
    }
}
