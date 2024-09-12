//! A space invaders game in Rust an Raylib
//! Based on this video: https://youtu.be/TGo3Oxdpr5o
//!
//! Rust implementation by Fernando Levin (flevin58@gmail.com)
//!
//! TBD: Sound assets are not implemented.
//!
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
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH + OFFSETX, WINDOW_HEIGHT + OFFSETY)
        .title(WINDOW_TITLE)
        .vsync()
        .build();

    rl.set_trace_log(LOG_ERROR);
    rl.set_target_fps(60);

    print!("Initializing audio device ... ");
    let audio = RaylibAudio::init_audio_device().expect("error initializing audio device");
    println!("done!");

    if audio.is_audio_device_ready() {
        println!("Audio device ready to use!");
    }

    /************************************************

    let explosion_sound: Sound<'static> = audio
        .new_sound(SOUND_EXPLOSION)
        .expect("error loading explosion sound");

    let laser_sound: Sound<'static> = audio
        .new_sound(SOUND_LASER)
        .expect("error loading explosion sound");


    ************************************************/

    let assets = Assets::new(&mut rl, &thread);

    let mut game = Game::new(&mut rl, &assets);

    while !rl.window_should_close() {
        game.handle_input(&mut rl, &assets);
        game.update(&mut rl);
        let mut d = rl.begin_drawing(&thread);
        game.draw(&mut d, &assets);
    }
}
