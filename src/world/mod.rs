mod controller;
mod game;
mod grid;

use raylib::prelude::*;

use crate::{brick::Brick, common::Direction, config::Config};

use self::game::Game;

pub struct World {
    current_brick: Brick,
    stack: Vec<Brick>,
    game: Game,
    dimension: Dimension,
}

pub struct Dimension {
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
        self.dimension = Dimension {
            h_size: config.grid_size.horizontal_size as i32,
            v_size: config.grid_size.vertical_size as i32,
        }
    }

    pub fn render(&mut self, drawable: &mut RaylibDrawHandle, config: &Config) {
        grid::render(drawable, config);

        for brick in self.stack.iter() {
            brick.render(drawable, config);
        }

        self.current_brick.render(drawable, config);
    }

    pub fn update(&mut self, handle: &mut RaylibHandle) {
        if self.game.is_over() {
            println!("Game is over");
            return;
        }

        if !self.game.is_running() {
            println!("No more spawn bricks");
            return;
        }

        self.game.increase_ticks();

        self.on_pressed(handle);
        if self.game.should_fall() {
            self.fall_brick();
        }
    }

    fn fall_brick(&mut self) {
        match self.check_and_move(Direction::Down) {
            true => return, // after moving down sucessfully
            false => {
                self.stack.push(self.current_brick.clone());
                self.current_brick = Brick::random();

                if self.check_game_over() {
                    self.game.set_game_over();
                }
            }
        }
    }

    fn check_game_over(&self) -> bool {
        let dx_dy_on_top = (0, 0);
        let is_over = Brick::will_collide_all(&self.current_brick, &self.stack, dx_dy_on_top);

        is_over
    }

    fn is_safe_to_move(&self, direction: Direction) -> bool {
        let dx_dy = Direction::get_dxdy(direction);

        let within_boundary = self
            .current_brick
            .within_boundary(dx_dy, self.dimension.to_tuple());

        let mut collision = false;

        if within_boundary {
            collision = Brick::will_collide_all(&self.current_brick, &self.stack, dx_dy);
        }

        within_boundary && !collision
    }

    pub fn check_and_move(&mut self, direction: Direction) -> bool {
        if !self.is_safe_to_move(direction) {
            return false;
        }
        self.current_brick.move_by_direction(direction);

        true
    }
}

impl Default for World {
    fn default() -> Self {
        World {
            current_brick: Brick::random(),
            stack: vec![],
            game: Game::default(),
            dimension: Dimension::empty(),
        }
    }
}
