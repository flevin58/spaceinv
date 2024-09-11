//! A space invaders game in Rust an Raylib
//! Based on this video: https://youtu.be/TGo3Oxdpr5o
//!
//! Rust implementation by Fernando Levin (flevin58@gmail.com)
mod alien;
mod block;
mod constants;
mod game;
mod laser;
mod mysteryship;
mod obstacle;
mod spaceship;

use constants::*;
use game::*;
use raylib::ffi::TraceLogLevel::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH + OFFSETX, WINDOW_HEIGHT + OFFSETY)
        .title(WINDOW_TITLE)
        .vsync()
        .build();

    rl.set_trace_log(LOG_ERROR);

    rl.set_target_fps(60);

    let mut game = Game::new(&mut rl, &thread);

    while !rl.window_should_close() {
        game.handle_input(&mut rl, &thread);
        game.update(&mut rl);
        let mut d = rl.begin_drawing(&thread);
        game.draw(&mut d);
    }
}
