use crate::context::Context;
use raylib::{
    color::Color,
    core::math::Vector2,
    ffi::Rectangle,
    prelude::{RaylibDraw, RaylibDrawHandle},
};
use std::rc::Rc;

use crate::assets::Assets;
use crate::constants::*;
use crate::laser::Laser;

#[derive(Clone)]
struct Bounds {
    min: f32,
    max: f32,
}

pub struct Spaceship {
    assets: Rc<Assets>,
    position: Vector2,
    bounds: Bounds,
    last_fire_time: f64,
}

impl Spaceship {
    pub fn new(assets: Rc<Assets>) -> Self {
        let width = assets.get_ship_texture().width;
        let height = assets.get_ship_texture().height;

        let ship_x = (WORLD_WIDTH - width) as f32 / 2.;
        let ship_y = (WORLD_HEIGHT - height - SPACESHIP_YOFFSET) as f32;

        let ship_min = SPACESHIP_XOFFSET as f32;
        let ship_max = (WORLD_WIDTH - width - SPACESHIP_XOFFSET) as f32;

        Self {
            assets,
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

    pub fn reset(&mut self) {
        // put back the spacehip at the center
        let ship_x = (WORLD_WIDTH - self.assets.get_ship_texture().width) / 2;
        let ship_y = WORLD_HEIGHT - self.assets.get_ship_texture().height - SPACESHIP_YOFFSET;
        self.position.x = ship_x as f32;
        self.position.y = ship_y as f32;
    }

    // currently unused
    pub fn update(&mut self) {}

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture_v(self.assets.get_ship_texture(), self.position, Color::WHITE);
    }

    pub fn draw_at(&self, d: &mut RaylibDrawHandle, x: f32, y: f32) {
        let pos = Vector2::new(x, y);
        d.draw_texture_v(self.assets.get_ship_texture(), pos, Color::WHITE);
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

    pub fn fire_laser(&mut self, ctx: Rc<Context>) -> Option<Laser> {
        let rl = ctx.rl.borrow();

        if rl.get_time() - self.last_fire_time >= LASER_TIME {
            let laser_pos = Vector2 {
                x: self.position.x
                    + (self.assets.get_ship_texture().width as f32 - LASER_WIDTH) / 2.,
                y: self.position.y,
            };
            self.last_fire_time = rl.get_time();

            Some(Laser::new(laser_pos, LASER_SPEED))
        } else {
            None
        }
    }

    pub fn get_rect(&self) -> Rectangle {
        let width = self.assets.get_ship_texture().width as f32;
        let height = self.assets.get_ship_texture().height as f32;
        Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: width,
            height: height,
        }
    }
}
