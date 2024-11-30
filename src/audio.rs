use raylib::audio::*;
use raylib_sys::TraceLogLevel;
use std::rc::Rc;
use crate::context::Context;

pub struct Audio<'a> {
    music: Rc<Music<'a>>,
}

impl<'a> Audio<'a> {
    pub fn new(ctx: Rc<Context>) -> Self {
        // INIT AUDIO
        let audio = RaylibAudio::init_audio_device().expect("error initializing audio device");
        if audio.is_audio_device_ready() {
            let rl = ctx
            rl.trace_log(TraceLogLevel::LOG_INFO, "Audio device ready to use!");

            // LOAD THE MUSIC
            let music_ogg = include_bytes!("../assets/sounds/music.ogg");
            let music = audio
                .new_music_from_memory(".ogg", music_ogg)
                .expect("Error loading music.ogg");
            music.play_stream();
        }
        Self {
            music: Rc::new()
        }
    }
}
