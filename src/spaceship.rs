use raylib::{
    color::Color,
    core::math::Vector2,
    misc::AsF32,
    prelude::{RaylibDraw, RaylibDrawHandle},
    texture::Texture2D,
    RaylibHandle, RaylibThread,
};

use crate::constants::*;
use crate::laser::Laser;

struct Bounds {
    min: f32,
    max: f32,
}

pub struct Spaceship {
    image: Texture2D,
    position: Vector2,
    bounds: Bounds,
    last_fire_time: f64,
}

impl Spaceship {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let ship_texture = rl.load_texture(&thread, SPACESHIP_TEXTURE).unwrap();

        let ship_x = (rl.get_screen_width().as_f32() - ship_texture.width.as_f32()) / 2.;
        let ship_y = rl.get_screen_height().as_f32() - ship_texture.height.as_f32();

        let ship_min = 0.;
        let ship_max = rl.get_screen_width().as_f32() - ship_texture.width.as_f32();

        Spaceship {
            image: ship_texture,
            position: Vector2 {
                x: ship_x,
                y: ship_y,
            },
            bounds: Bounds {
                min: ship_min,
                max: ship_max,
            },
            last_fire_time: 0.,
        }
    }

    // currently unused
    pub fn update(&mut self, _rl: &mut RaylibHandle) {}

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture_v(&self.image, self.position, Color::WHITE);
    }

    pub fn move_left(&mut self) {
        self.position.x -= SPACESHIP_SPEED;
        if self.position.x < self.bounds.min {
            self.position.x = self.bounds.min;
        }
    }

    pub fn move_right(&mut self) {
        self.position.x += SPACESHIP_SPEED;
        if self.position.x > self.bounds.max {
            self.position.x = self.bounds.max;
        }
    }

    pub fn fire_laser(&mut self, rl: &RaylibHandle) -> Option<Laser> {
        if rl.get_time() - self.last_fire_time >= LASER_TIME {
            let laser_pos = Vector2 {
                x: self.position.x + (self.image.width.as_f32() - LASER_WIDTH) / 2.,
                y: self.position.y,
            };
            self.last_fire_time = rl.get_time();

            Some(Laser::new(laser_pos, LASER_SPEED))
        } else {
            None
        }
    }
}
