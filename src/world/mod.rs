mod controller;
mod game;
mod grid;
mod mechanism;
mod textable;

use raylib::prelude::*;

use crate::{
    brick::{brick_factory::BrickFactory, brick_type::BrickType, Brick},
    config::Config,
    input::game_controls::GameControls,
};

use self::{game::Game, textable::Textable};

pub struct World {
    current_brick: Brick,
    preview_brick: Brick,
    next_brick_type: BrickType,
    stack: Vec<Brick>,
    game_control: GameControls,
    game: Game,
    dimension: Dimension,
    line_cleared_text: Textable,
    score_text: Textable,
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

    pub fn h(&self) -> i32 {
        self.h_size
    }

    pub fn v(&self) -> i32 {
        self.v_size
    }

    pub fn to_tuple(&self) -> (i32, i32) {
        (self.h(), self.v())
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
        grid::render_incomming_brick(drawable, config);

        self.render_imcoming_brick(drawable, config);

        for brick in self.stack.iter() {
            brick.render(drawable, config);
        }

        self.preview_brick.render_alpha(drawable, config);
        self.current_brick.render(drawable, config);

        if self.game.is_running() {
            let line_cleared_res = format!("Line cleared: {}", self.game.line_cleared());
            self.line_cleared_text.draw(&line_cleared_res, drawable);

            let score_res = format!("Score: {}", self.game.score());
            self.score_text.draw(&score_res, drawable);
        }
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
        self.fall_preview_brick();

        self.game_control.update(handle);
        self.on_pressed();

        if self.game.should_fall() {
            self.fall_brick();
        }
        let row_has_full_pieces = self.find_rows_has_full_pieces();
        self.clear_rows_and_fall_other_pieces_down(&row_has_full_pieces);

        self.game.update(row_has_full_pieces.len() as i32);
    }

    pub fn reset(&mut self) {
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

    fn render_imcoming_brick(&self, drawable: &mut RaylibDrawHandle, config: &Config) {
        let start_coords = BrickType::get_start_coords_for_incoming_brick(self.next_brick_type);
        let next_brick = Brick::spawn(start_coords, self.next_brick_type);
        next_brick.render(drawable, config);
    }
}

impl Default for World {
    fn default() -> Self {
        let random_brick = BrickFactory::random_brick_type();
        let current_brick = Brick::spawn_by_brick_type(random_brick);
        let preview_brick = current_brick.clone();

        let line_cleared = Textable {
            name: "line_cleared".to_string(),
            location: (150, 200),
            font_size: 30,
            color: Color::PINK,
        };

        let score = Textable {
            name: "score".to_string(),
            location: (150, 250),
            font_size: 30,
            color: Color::GREEN,
        };

        World {
            current_brick,
            preview_brick,
            next_brick_type: BrickFactory::random_brick_type(),
            game_control: GameControls::default(),
            stack: vec![],
            game: Game::default(),
            dimension: Dimension::empty(),
            line_cleared_text: line_cleared,
            score_text: score,
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
