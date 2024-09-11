use raylib::{
    color::Color,
    core::math::Vector2,
    ffi::Rectangle,
    misc::AsF32,
    prelude::{RaylibDraw, RaylibDrawHandle},
    texture::Texture2D,
    RaylibHandle, RaylibThread,
};

use crate::constants::*;

pub struct Alien {
    position: Vector2,
    image: Texture2D,
    alive: bool,
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
            alive: true,
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

    pub fn get_laser_position(&self) -> Vector2 {
        let laser_x = self.position.x + self.image.width.as_f32() / 2.;
        let laser_y = self.position.y + self.image.height.as_f32();
        Vector2::new(laser_x, laser_y)
    }

    pub fn move_down(&mut self, distance: usize) {
        self.position.y += distance as f32;
    }

    pub fn erase(&mut self) {
        self.alive = false;
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn update(&mut self, _rl: &mut RaylibHandle, direction: i32) {
        self.position.x += direction.as_f32();
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        if self.alive {
            d.draw_texture_v(&self.image, self.position, Color::WHITE);
        }
    }

    pub fn get_rect(&self) -> Rectangle {
        Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: self.image.width.as_f32(),
            height: self.image.height.as_f32(),
        }
    }
}
