use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels;
use sdl2::render::WindowCanvas;
use simdnoise::NoiseBuilder;

pub struct Grid {
    cell_size: u16,

    screen_width: u16,
    screen_height: u16,

    width: u16,
    height: u16,

    angles_offset: (f32, f32),
    lengths_offset: (f32, f32),

    angles_offset_change: (f32, f32),
    lengths_offset_change: (f32, f32),

    angles: Vec<f32>,
    lengths: Vec<f32>,

    seed: i32,
}

impl Grid {
    pub fn new(cell_size: u16, seed: i32, screen_width: u16, screen_height: u16) -> Self {
        let width = (screen_width / cell_size) + 1;
        let height = (screen_height / cell_size) + 1;
        let seed = seed;
        Self {
            cell_size,
            screen_width,
            screen_height,
            width,
            height,
            angles_offset: (0., 0.),
            lengths_offset: (0., 0.),

            angles_offset_change: (0., 0.),
            lengths_offset_change: (0., 0.),

            angles: Self::flow_field_angles(seed, width, height, 0., 0.),
            lengths: Self::flow_field_lengths(seed, width, height, 0., 0.),

            seed,
        }
    }

    pub fn update(
        &mut self,
        cell_size: Option<u16>,
        seed: Option<i32>,
        screen_width: Option<u16>,
        screen_height: Option<u16>,
    ) {
        if let Some(new_cell_size) = cell_size {
            self.cell_size = new_cell_size;
            self.width = (self.screen_width / self.cell_size) + 1;
            self.height = (self.screen_height / self.cell_size) + 1;
        }
        if let Some(new_seed) = seed {
            self.seed = new_seed;
        }
        if let Some(new_screen_width) = screen_width {
            self.width = new_screen_width / self.cell_size;
        }
        if let Some(new_screen_height) = screen_height {
            self.height = new_screen_height / self.cell_size;
        }
        self.angles = Self::flow_field_angles(self.seed, self.width, self.height, 0., 0.);
        self.lengths = Self::flow_field_lengths(self.seed, self.width, self.height, 0., 0.);
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        self.draw_grid(canvas);
        //self.fill_grid_with_arc(canvas);
        self.fill_grid_with_vec(canvas);
    }

    #[allow(clippy::cast_possible_wrap)]
    fn draw_grid(&self, canvas: &mut WindowCanvas) {
        let color = pixels::Color::RGBA(255, 255, 255, 15);
        for square in 0..std::cmp::max(self.height, self.width) {
            let _ = canvas.hline(
                0,
                (self.width * self.cell_size) as i16,
                (square * self.cell_size) as i16,
                color,
            );
            let _ = canvas.vline(
                (square * self.cell_size) as i16,
                0,
                (self.height * self.cell_size) as i16,
                color,
            );
        }
    }

    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    fn _fill_grid_with_arc(&self, canvas: &mut WindowCanvas) {
        let color = pixels::Color::RGB(255, 255, 255);
        for y_cell in 0..self.height {
            for x_cell in 0..self.width {
                canvas
                    .arc(
                        (x_cell * self.cell_size + self.cell_size / 2) as i16,
                        (y_cell * self.cell_size + self.cell_size / 2) as i16,
                        (self.cell_size / 2) as i16,
                        0,
                        (self.angles[(y_cell * self.width + x_cell) as usize]) as i16,
                        color,
                    )
                    .unwrap();
            }
        }
    }

    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    fn fill_grid_with_vec(&self, canvas: &mut WindowCanvas) {
        let color = pixels::Color::RGBA(255, 255, 255, 100);
        for y_cell in 0..self.height {
            for x_cell in 0..self.width {
                let center_x = (x_cell * self.cell_size + self.cell_size / 2) as i16;
                let center_y = (y_cell * self.cell_size + self.cell_size / 2) as i16;
                let angle = self.angles[(y_cell * self.width + x_cell) as usize];
                let length = self.lengths[(y_cell * self.width + x_cell) as usize];

                if self.cell_size > 32 {
                    canvas
                        .thick_line(
                            center_x,
                            center_y,
                            center_x
                                + ((angle).cos() * length * f32::from(self.cell_size) / 2.) as i16,
                            center_y
                                + ((angle).sin() * length * f32::from(self.cell_size) / 2.) as i16,
                            2,
                            color,
                        )
                        .unwrap();
                } else {
                    canvas
                        .aa_line(
                            center_x,
                            center_y,
                            center_x
                                + ((angle).cos() * length * f32::from(self.cell_size) / 2.) as i16,
                            center_y
                                + ((angle).sin() * length * f32::from(self.cell_size) / 2.) as i16,
                            color,
                        )
                        .unwrap();
                }
            }
        }
    }

    pub fn tick(&mut self, _current_tick: f32) {
        self.angles_offset.0 += self.angles_offset_change.0;
        self.angles_offset.1 += self.angles_offset_change.1;
        self.lengths_offset.0 += self.lengths_offset_change.0;
        self.lengths_offset.1 += self.lengths_offset_change.1;
        self.angles = Self::flow_field_angles(
            self.seed,
            self.width,
            self.height,
            self.angles_offset.0,
            self.angles_offset.1,
        );
        self.lengths = Self::flow_field_lengths(
            self.seed,
            self.width,
            self.height,
            self.lengths_offset.0,
            self.lengths_offset.1,
        );
    }

    #[allow(clippy::pedantic)]
    pub fn get_vec_on_position(&self, x: f32, y: f32) -> (f32, f32) {
        let x_cell = x as u16 / self.cell_size;
        let y_cell = y as u16 / self.cell_size;
        (
            self.angles[(y_cell * self.width + x_cell) as usize],
            self.lengths[(y_cell * self.width + x_cell) as usize],
        )
    }

    pub fn set_offset_changes(&mut self, angles: (f32, f32), lengths: (f32, f32)) {
        self.angles_offset_change = angles;
        self.lengths_offset_change = lengths;
    }

    fn flow_field_angles(
        seed: i32,
        width: u16,
        height: u16,
        x_offset: f32,
        y_offset: f32,
    ) -> Vec<f32> {
        NoiseBuilder::gradient_2d_offset(x_offset, width.into(), y_offset, (height + 1).into())
            .with_seed(seed)
            .with_freq(0.02)
            .generate_scaled(0.0, std::f32::consts::TAU * 2.)
    }
    fn flow_field_lengths(
        seed: i32,
        width: u16,
        height: u16,
        x_offset: f32,
        y_offset: f32,
    ) -> Vec<f32> {
        NoiseBuilder::gradient_2d_offset(x_offset, width.into(), y_offset, (height + 1).into())
            .with_seed(seed - 1000)
            .with_freq(0.02)
            .generate_scaled(0.0, 1.0)
    }
}
