use crate::{assets::Assets, constants::*};
use rand::Rng;

use raylib::{
    color::Color,
    core::math::Vector2,
    ffi::Rectangle,
    prelude::{RaylibDraw, RaylibDrawHandle, RaylibHandle},
};

pub struct MysteryShip<'m> {
    assets: &'m Assets,
    position: Vector2,
    speed: f32,
    active: bool,
}

impl<'m> MysteryShip<'m> {
    pub fn new(assets: &'m Assets) -> Self {
        MysteryShip {
            assets,
            position: Vector2::zero(),
            speed: 0.,
            active: false,
        }
    }

    pub fn set_inactive(&mut self) {
        self.active = false;
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        if self.active {
            self.position.x += self.speed;
            if self.position.x
                > (rl.get_screen_width() - self.assets.get_mystery_texture().width - OFFSETX / 2)
                    as f32
                || self.position.x < (OFFSETX / 2) as f32
            {
                self.active = false;
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        if self.active {
            d.draw_texture_v(
                self.assets.get_mystery_texture(),
                self.position,
                Color::WHITE,
            );
        }
    }

    pub fn spawn(&mut self, rl: &mut raylib::RaylibHandle) {
        let side: i32 = rand::thread_rng().gen_range(0..1);
        self.position.y = MYSTERYSHIP_YPOS;
        if side == 0 {
            self.position.x = (OFFSETX / 2) as f32;
            self.speed = MYSTERYSHIP_SPEED;
        } else {
            self.position.x = (rl.get_screen_width()
                - self.assets.get_mystery_texture().width
                - OFFSETX / 2) as f32;
            self.speed = -MYSTERYSHIP_SPEED;
        }
        self.active = true;
    }

    pub fn get_rect(&self) -> Rectangle {
        let mut width: f32 = 0.;
        let mut height: f32 = 0.;

        if self.active {
            width = self.assets.get_mystery_texture().width as f32;
            height = self.assets.get_mystery_texture().height as f32;
        }

        Rectangle {
            x: self.position.x,
            y: self.position.y,
            width,
            height,
        }
    }
}
