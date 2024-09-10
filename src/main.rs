mod alien;
mod block;
mod constants;
mod game;
mod laser;
mod obstacle;
mod spaceship;

use constants::*;
use game::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title(WINDOW_TITLE)
        .vsync()
        .build();

    rl.set_target_fps(60);

    let mut game = Game::new(&mut rl, &thread);

    while !rl.window_should_close() {
        game.handle_input(&mut rl);
        game.update(&mut rl);
        let mut d = rl.begin_drawing(&thread);
        game.draw(&mut d);
    }
}
