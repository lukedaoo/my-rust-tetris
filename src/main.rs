use my_rust_tetris::{config::Config, world::World};
use raylib::prelude::*;

fn main() {
    let config = Config::default();
    let (mut rl, thread) = raylib::init()
        .size(*config.width() as i32, *config.height_with_margin() as i32)
        .title(&config.title())
        .vsync()
        .build();

    rl.set_target_fps(config.fps);

    let mut world = World::default();
    world.update_dimemsion(&config);
    world.start();

    while !rl.window_should_close() {
        world.update(&mut rl);
        let mut drawable = rl.begin_drawing(&thread);

        drawable.clear_background(Color::BLACK);
        drawable.draw_fps(10, 10);
        world.render(&mut drawable, &config);
    }
}
