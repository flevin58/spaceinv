use crate::constants::*;
use crate::log;
use raylib_ffi::IsSoundPlaying;
use raylib_ffi::LoadSoundFromWave;
use raylib_ffi::LoadWaveFromMemory;
use raylib_ffi::PlaySound;
use raylib_ffi::SetMusicVolume;
use raylib_ffi::Sound;
use raylib_ffi::{
    rl_str, Font, LoadFontFromMemory, LoadMusicStreamFromMemory, Music, PlayMusicStream,
    UnloadFont, UnloadMusicStream, UpdateMusicStream,
};

#[derive(Clone)]
pub struct Assets {
    font: Font,
    music: Music,
    laser: Sound,
    alien_explosion: Sound,
    ship_explosion: Sound,
    mystery_sound: Sound,
}

impl Drop for Assets {
    fn drop(&mut self) {
        unsafe {
            UnloadFont(self.font);
            UnloadMusicStream(self.music);
        }
    }
}

macro_rules! embed_ogg_sound {
    ($snd:expr) => {{
        let sound_data = include_bytes!($snd);
        let sound_wav = LoadWaveFromMemory(
            rl_str!(".ogg"),
            sound_data.as_ptr(),
            sound_data.len() as i32,
        );
        LoadSoundFromWave(sound_wav)
    }};
}

macro_rules! embed_ttf_font {
    ($fname:expr, $size: expr) => {{
        let font_data = include_bytes!($fname);
        LoadFontFromMemory(
            rl_str!(".ttf"),
            font_data.as_ptr(),
            font_data.len() as i32,
            $size,
            0 as *mut i32,
            0,
        )
    }};
}

impl Assets {
    pub fn new() -> Self {
        unsafe {
            // Embedded music
            let music_data = include_bytes!("../assets/sounds/music.ogg");
            let music_res = LoadMusicStreamFromMemory(
                rl_str!(".ogg"),
                music_data.as_ptr(),
                music_data.len() as i32,
            );

            Assets {
                font: embed_ttf_font!("../assets/fonts/monogram.ttf", FONT_SIZE),
                music: music_res,
                laser: embed_ogg_sound!("../assets/sounds/laser.ogg"),
                alien_explosion: embed_ogg_sound!("../assets/sounds/alien_explosion.ogg"),
                ship_explosion: embed_ogg_sound!("../assets/sounds/ship_explosion.ogg"),
                mystery_sound: embed_ogg_sound!("../assets/sounds/mystery.ogg"),
            }
        }
    }

    pub fn get_font(&self) -> Font {
        self.font
    }

    pub fn play_music(&self) {
        unsafe {
            SetMusicVolume(self.music, 0.8);
            PlayMusicStream(self.music)
        };
    }

    pub fn update_music(&self) {
        unsafe {
            UpdateMusicStream(self.music);
        }
    }

    pub fn play_laser_sound(&self) {
        unsafe {
            PlaySound(self.laser);
        }
    }

    pub fn play_alien_explosion_sound(&self) {
        unsafe {
            PlaySound(self.alien_explosion);
        }
    }

    pub fn play_mystery_explosion_sound(&self) {
        unsafe {
            PlaySound(self.alien_explosion);
        }
    }

    pub fn play_ship_explosion_sound(&self) {
        unsafe {
            PlaySound(self.ship_explosion);
        }
    }

    pub fn play_mystery_sound(&self) {
        unsafe {
            if !IsSoundPlaying(self.mystery_sound) {
                PlaySound(self.mystery_sound);
            }
        }
    }
}
