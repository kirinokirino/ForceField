use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels;
use sdl2::render::WindowCanvas;
use simdnoise::NoiseBuilder;

pub struct Grid {
    cell_size: u32,
    width: usize,
    height: usize,

    angles: Vec<f32>,
    lengths: Vec<f32>,

    seed: i32,
}

impl Grid {
    pub fn new(cell_size: u32, seed: i32) -> Self {
        let width = (SCREEN_WIDTH / cell_size) as usize;
        let height = (SCREEN_HEIGHT / cell_size) as usize;
        let seed = seed;
        Self {
            cell_size,
            width,
            height,

            angles: Grid::flow_field_angles(width, height, 0., 0.),
            lengths: Grid::flow_field_lengths(width, height, 0., 0.),

            seed,
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        self.draw_grid(canvas);
        //self.fill_grid_with_arc(canvas);
        self.fill_grid_with_vec(canvas);
    }

    fn draw_grid(&self, canvas: &mut WindowCanvas) {
        let color = pixels::Color::RGBA(255, 255, 255, 15);
        for square in 0..std::cmp::max(SCREEN_HEIGHT, SCREEN_WIDTH) / self.cell_size {
            let _ = canvas.hline(
                0,
                SCREEN_WIDTH as i16,
                (square * self.cell_size) as i16,
                color,
            );
            let _ = canvas.vline(
                (square * self.cell_size) as i16,
                0,
                SCREEN_HEIGHT as i16,
                color,
            );
        }
    }

    fn fill_grid_with_arc(&self, canvas: &mut WindowCanvas) {
        let color = pixels::Color::RGB(255, 255, 255);
        for y_cell in 0..SCREEN_HEIGHT / self.cell_size {
            for x_cell in 0..SCREEN_WIDTH / self.cell_size {
                canvas
                    .arc(
                        (x_cell * self.cell_size + self.cell_size / 2) as i16,
                        (y_cell * self.cell_size + self.cell_size / 2) as i16,
                        10,
                        0,
                        (self.angles[(y_cell * SCREEN_WIDTH / self.cell_size + x_cell) as usize])
                            as i16,
                        color,
                    )
                    .unwrap();
            }
        }
    }
    fn fill_grid_with_vec(&self, canvas: &mut WindowCanvas) {
        let color = pixels::Color::RGBA(255, 255, 255, 100);
        for y_cell in 0..SCREEN_HEIGHT / self.cell_size {
            for x_cell in 0..SCREEN_WIDTH / self.cell_size {
                let center_x = (x_cell * self.cell_size + self.cell_size / 2) as i16;
                let center_y = (y_cell * self.cell_size + self.cell_size / 2) as i16;
                let angle = self.angles[(y_cell * SCREEN_WIDTH / self.cell_size + x_cell) as usize];
                let length =
                    self.lengths[(y_cell * SCREEN_WIDTH / self.cell_size + x_cell) as usize];

                canvas
                    .aa_line(
                        center_x,
                        center_y,
                        center_x + (angle.to_radians().cos() * length) as i16,
                        center_y + (angle.to_radians().sin() * length) as i16,
                        color,
                    )
                    .unwrap();
            }
        }
    }

    fn tick(&mut self) {}

    fn flow_field_angles(width: usize, height: usize, x_offset: f32, y_offset: f32) -> Vec<f32> {
        NoiseBuilder::gradient_2d_offset(x_offset, width + 1, y_offset, height)
            .with_seed(0)
            .with_freq(0.02)
            .generate_scaled(0.0, 359.9)
    }
    fn flow_field_lengths(width: usize, height: usize, x_offset: f32, y_offset: f32) -> Vec<f32> {
        NoiseBuilder::gradient_2d_offset(x_offset, width + 1, y_offset, height)
            .with_seed(0)
            .with_freq(0.02)
            .generate_scaled(-10.0, 10.0)
    }
}
