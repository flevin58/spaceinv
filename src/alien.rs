use raylib::{
    color::Color,
    core::math::Vector2,
    misc::AsF32,
    prelude::{RaylibDraw, RaylibDrawHandle},
    texture::Texture2D,
    RaylibHandle, RaylibThread,
};

use crate::constants::*;

pub struct Alien {
    position: Vector2,
    image: Texture2D,
}

impl Alien {
    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        kind: usize,
        position: Vector2,
    ) -> Self {
        let asset_files: [&str; 3] = [ALIEN1_TEXTURE, ALIEN2_TEXTURE, ALIEN3_TEXTURE];

        Alien {
            position,
            image: rl.load_texture(&thread, asset_files[kind - 1]).unwrap(),
        }
    }

    pub fn has_overflowed_right(&self, rl: &RaylibHandle) -> bool {
        if self.position.x as i32 + self.image.width > rl.get_screen_width() {
            true
        } else {
            false
        }
    }

    pub fn has_overflowed_left(&self) -> bool {
        if self.position.x < 0. {
            true
        } else {
            false
        }
    }

    pub fn move_down(&mut self, distance: usize) {
        self.position.y += distance as f32;
    }

    pub fn update(&mut self, _rl: &mut RaylibHandle, direction: i32) {
        self.position.x += direction.as_f32();
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture_v(&self.image, self.position, Color::WHITE);
    }
}
