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
    pub fn new(cell_size: u32, seed: i32, screen_width: u32, screen_height: u32) -> Self {
        let width = (screen_width / cell_size) as usize;
        let height = (screen_height / cell_size) as usize;
        let seed = seed;
        Self {
            cell_size,
            width,
            height,

            angles: Grid::flow_field_angles(seed, width, height, 0., 0.),
            lengths: Grid::flow_field_lengths(seed, width, height, 0., 0.),

            seed,
        }
    }

    pub fn update(
        &mut self,
        cell_size: Option<u32>,
        seed: Option<i32>,
        screen_width: Option<u32>,
        screen_height: Option<u32>,
    ) {
        if let Some(new_cell_size) = cell_size {
            self.cell_size = new_cell_size;
        }
        if let Some(new_seed) = seed {
            self.seed = new_seed;
        }
        if let Some(new_screen_width) = screen_width {
            self.width = (new_screen_width / self.cell_size) as usize;
        }
        if let Some(new_screen_height) = screen_height {
            self.height = (new_screen_height / self.cell_size) as usize;
        }
        self.angles = Grid::flow_field_angles(self.seed, self.width, self.height, 0., 0.);
        self.lengths = Grid::flow_field_lengths(self.seed, self.width, self.height, 0., 0.);
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        self.draw_grid(canvas);
        //self.fill_grid_with_arc(canvas);
        self.fill_grid_with_vec(canvas);
    }

    fn draw_grid(&self, canvas: &mut WindowCanvas) {
        let color = pixels::Color::RGBA(255, 255, 255, 15);
        for square in 0..std::cmp::max(self.height, self.width) {
            let _ = canvas.hline(
                0,
                (self.width * self.cell_size as usize) as i16,
                (square * self.cell_size as usize) as i16,
                color,
            );
            let _ = canvas.vline(
                (square * self.cell_size as usize) as i16,
                0,
                (self.height * self.cell_size as usize) as i16,
                color,
            );
        }
    }

    fn _fill_grid_with_arc(&self, canvas: &mut WindowCanvas) {
        let color = pixels::Color::RGB(255, 255, 255);
        for y_cell in 0..self.height {
            for x_cell in 0..self.width {
                canvas
                    .arc(
                        (x_cell as u32 * self.cell_size + self.cell_size / 2) as i16,
                        (y_cell as u32 * self.cell_size + self.cell_size / 2) as i16,
                        10,
                        0,
                        (self.angles[(y_cell * self.width + x_cell) as usize]) as i16,
                        color,
                    )
                    .unwrap();
            }
        }
    }
    fn fill_grid_with_vec(&self, canvas: &mut WindowCanvas) {
        let color = pixels::Color::RGBA(255, 255, 255, 100);
        for y_cell in 0..self.height {
            for x_cell in 0..self.width {
                let center_x = (x_cell as u32 * self.cell_size + self.cell_size / 2) as i16;
                let center_y = (y_cell as u32 * self.cell_size + self.cell_size / 2) as i16;
                let angle = self.angles[(y_cell * self.width + x_cell) as usize];
                let length = self.lengths[(y_cell * self.width + x_cell) as usize];

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

    pub fn tick(&mut self, current_tick: f32) {
        self.angles = Grid::flow_field_angles(
            self.seed,
            self.width,
            self.height,
            current_tick,
            current_tick,
        );
        self.lengths = Grid::flow_field_lengths(
            self.seed,
            self.width,
            self.height,
            current_tick,
            current_tick,
        );
    }

    fn flow_field_angles(
        seed: i32,
        width: usize,
        height: usize,
        x_offset: f32,
        y_offset: f32,
    ) -> Vec<f32> {
        NoiseBuilder::gradient_2d_offset(x_offset, width + 1, y_offset, height)
            .with_seed(seed)
            .with_freq(0.02)
            .generate_scaled(0.0, 359.9)
    }
    fn flow_field_lengths(
        seed: i32,
        width: usize,
        height: usize,
        x_offset: f32,
        y_offset: f32,
    ) -> Vec<f32> {
        NoiseBuilder::gradient_2d_offset(x_offset, width + 1, y_offset, height)
            .with_seed(seed)
            .with_freq(0.02)
            .generate_scaled(-10.0, 10.0)
    }
}
