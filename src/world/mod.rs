mod controller;
mod game;
mod grid;
mod mechanism;

use raylib::prelude::*;

use crate::{
    brick::{brick_factory::BrickFactory, Brick},
    config::Config,
    input::game_controls::GameControls,
};

use self::game::Game;

pub struct World {
    current_brick: Brick,
    preview_brick: Brick,
    stack: Vec<Brick>,
    game_control: GameControls,
    game: Game,
    dimension: Dimension,
}

#[derive(Clone)]
pub enum FocusOn {
    CurrentBrick,
    PreviewBrick,
}

struct Dimension {
    h_size: i32,
    v_size: i32,
}

impl Dimension {
    pub fn empty() -> Self {
        Dimension {
            h_size: 0,
            v_size: 0,
        }
    }

    pub fn to_tuple(&self) -> (i32, i32) {
        (self.h_size, self.v_size)
    }
}

impl World {
    pub fn update_dimemsion(&mut self, config: &Config) {
        let dim = config.dimension();
        self.dimension = Dimension {
            h_size: dim.0 as i32,
            v_size: dim.1 as i32,
        }
    }

    pub fn render(&mut self, drawable: &mut RaylibDrawHandle, config: &Config) {
        grid::render(drawable, config);

        for brick in self.stack.iter() {
            brick.render(drawable, config);
        }

        self.preview_brick.render_alpha(drawable, config);

        self.current_brick.render(drawable, config);
    }

    pub fn update(&mut self, handle: &mut RaylibHandle) {
        if !self.game.is_running() {
            self.on_stopped();
            return;
        }

        if self.game.is_over() {
            self.reset();
            return;
        }

        if self.game.is_paused() {
            self.on_paused(handle);
            return;
        }

        self.game.increase_ticks();

        //
        self.fall_preview_brick();

        self.game_control.update(handle);
        self.on_pressed();

        if self.game.should_fall() {
            self.fall_brick();
        }
        let row_has_full_pieces = self.find_rows_has_full_pieces();
        self.clear_rows_and_fall_other_pieces_down(&row_has_full_pieces);
    }

    pub fn reset(&mut self) {
        println!("Reset");
        self.current_brick = Brick::random();
        self.game_control = GameControls::default();
        self.stack = vec![];
        self.game = Game::default();
    }

    fn check_game_over(&self) -> bool {
        let dx_dy_on_top = (0, 0);
        let is_over = Brick::will_collide_all(&self.current_brick, &self.stack, dx_dy_on_top);

        is_over
    }
}

impl Default for World {
    fn default() -> Self {
        let random_brick = BrickFactory::random_brick_type();
        let current_brick = Brick::spawn_by_brick_type(random_brick);
        let preview_brick = current_brick.clone();
        World {
            current_brick,
            preview_brick,
            game_control: GameControls::default(),
            stack: vec![],
            game: Game::default(),
            dimension: Dimension::empty(),
        }
    }
}

impl World {
    pub fn start(&mut self) {
        self.game.start();
    }
    pub fn stop(&mut self) {
        self.game.stop();
    }

    pub fn current_brick(&self) -> &Brick {
        &self.current_brick
    }

    pub fn preview_brick(&self) -> &Brick {
        &self.preview_brick
    }
}
