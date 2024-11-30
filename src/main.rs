// A space invaders game in Rust an Raylib
// Based on this video: https://youtu.be/TGo3Oxdpr5o
//
// Rust implementation by Fernando Levin (flevin58@gmail.com)
//
// TBD: Sound assets are not implemented.
//

mod alien;
mod assets;
mod block;
mod constants;
mod context;
mod game;
mod laser;
mod mysteryship;
mod obstacle;
mod spaceship;
//mod audio;

use game::*;

fn main() {
    let mut game = Game::new();
    game.run();
}
