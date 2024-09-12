//! A space invaders game in Rust an Raylib
//! Based on this video: https://youtu.be/TGo3Oxdpr5o
//!
//! Rust implementation by Fernando Levin (flevin58@gmail.com)
mod alien;
mod assets;
mod block;
mod constants;
mod game;
mod laser;
mod mysteryship;
mod obstacle;
mod spaceship;

use assets::Assets;
use constants::*;
use game::*;
use raylib::ffi::TraceLogLevel::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH + OFFSETX, WINDOW_HEIGHT + OFFSETY)
        .title(WINDOW_TITLE)
        .vsync()
        .build();

    let assets = Assets::new(&mut rl, &thread);

    rl.set_trace_log(LOG_ERROR);

    rl.set_target_fps(60);

    let mut game = Game::new(&mut rl, &assets);

    while !rl.window_should_close() {
        game.handle_input(&mut rl, &assets);
        game.update(&mut rl);
        let mut d = rl.begin_drawing(&thread);
        game.draw(&mut d, &assets);
    }
}
