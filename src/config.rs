const DEFAULT_H_SIZE: u32 = 10;
const DEFAULT_V_SIZE: u32 = 20;

#[derive(Clone)]
pub struct Config {
    pub window_size: WindowSize,
    pub grid_size: GridSize,
    pub title: String,
    pub fps: u32,
}

#[derive(Copy, Clone)]
pub struct WindowSize {
    width: u32,
    height: u32,
    height_with_margin: u32,
}

#[derive(Copy, Clone)]
pub struct GridSize {
    pub horizontal_size: u32,
    pub vertical_size: u32,
    pub acutal_width: f32,
    pub grid_canvas_l: f32,
    pub grid_canvas_r: f32,
    pub margin_top: f32,
    pub margin_bottom: f32,
}

impl Default for Config {
    fn default() -> Self {
        const WIDTH: u32 = 1600;
        const HEIGHT: u32 = 900;
        const MARGIN_TOP: u32 = 90;
        const MARGIN_BOTTOM: u32 = 45;
        const FPS: u32 = 60;

        let window_size = WindowSize {
            width: WIDTH,
            height: HEIGHT,
            height_with_margin: HEIGHT + MARGIN_TOP + MARGIN_BOTTOM,
        };

        let acutal_width = window_size.width as f32 * (9_f32 / 32_f32);
        let grid_canvas_l = (window_size.width as f32 - acutal_width) / 2_f32;
        let grid_canvas_r = grid_canvas_l + acutal_width;

        let grid_size = GridSize {
            horizontal_size: DEFAULT_H_SIZE,
            vertical_size: DEFAULT_V_SIZE,
            acutal_width,
            grid_canvas_l,
            grid_canvas_r,
            margin_top: MARGIN_TOP as f32,
            margin_bottom: MARGIN_BOTTOM as f32,
        };

        Config {
            fps: FPS,
            title: "My Tetris".to_string(),
            window_size,
            grid_size,
        }
    }
}

impl Config {
    pub fn title(&self) -> &String {
        &self.title
    }
    pub fn grid_size(&self) -> &GridSize {
        &self.grid_size
    }

    pub fn window_size(&self) -> &WindowSize {
        &self.window_size
    }

    pub fn height(&self) -> &u32 {
        &self.window_size.height
    }

    pub fn width(&self) -> &u32 {
        &self.window_size.width
    }

    pub fn height_with_margin(&self) -> &u32 {
        &self.window_size.height_with_margin
    }

    pub fn dimension(&self) -> (i32, i32) {
        (
            self.grid_size.horizontal_size as i32,
            self.grid_size.vertical_size as i32,
        )
    }
}
