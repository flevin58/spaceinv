use crate::constants::*;
use rand::Rng;

use raylib::{
    color::Color,
    core::math::Vector2,
    ffi::Rectangle,
    misc::AsF32,
    prelude::{RaylibDraw, RaylibDrawHandle},
    texture::Texture2D,
    RaylibHandle, RaylibThread,
};

pub struct MysteryShip {
    image: Texture2D,
    position: Vector2,
    speed: f32,
    alive: bool,
}

impl MysteryShip {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mystery_texture = rl.load_texture(&thread, MYSTERYSHIP_TEXTURE).unwrap();

        MysteryShip {
            image: mystery_texture,
            position: Vector2::zero(),
            speed: 0.,
            alive: false,
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        if self.alive {
            self.position.x += self.speed;
            if self.position.x > rl.get_screen_width().as_f32() || self.position.x < 0. {
                self.alive = false;
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        if self.alive {
            d.draw_texture_v(&self.image, self.position, Color::WHITE);
        }
    }

    pub fn spawn(&mut self, rl: &mut raylib::RaylibHandle) {
        let side: i32 = rand::thread_rng().gen_range(0..1);
        self.position.y = MYSTERYSHIP_YPOS;
        if side == 0 {
            self.position.x = 0.;
            self.speed = MYSTERYSHIP_SPEED;
        } else {
            self.position.x = rl.get_screen_width().as_f32() - self.image.width.as_f32();
            self.speed = -MYSTERYSHIP_SPEED;
        }
        self.alive = true;
    }

    pub fn get_rect(&self) -> Rectangle {
        let mut width: f32 = 0.;
        let mut height: f32 = 0.;

        if self.alive {
            width = self.image.width.as_f32();
            height = self.image.height.as_f32();
        }

        Rectangle {
            x: self.position.x,
            y: self.position.y,
            width,
            height,
        }
    }
}
