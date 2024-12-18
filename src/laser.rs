use crate::constants::*;
use ray::{Rectangle, Vector2};
use raylib_ffi as ray;

#[derive(Clone)]
pub struct Laser {
    position: Vector2,
    speed: f32,
    active: bool,
}

impl Laser {
    pub fn new(position: Vector2, speed: f32) -> Self {
        Laser {
            position,
            speed,
            active: true,
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn set_inactive(&mut self) {
        self.active = false;
    }

    pub fn update(&mut self) {
        if self.active {
            self.position.y += self.speed;
            if self.position.y > (WORLD_HEIGHT - OFFSETY) as f32 || self.position.y < 25. {
                self.active = false;
            }
        }
    }

    pub fn draw(&self) {
        unsafe {
            ray::DrawRectangleV(self.position, LASER_SIZE, LASER_COLOR);
        }
    }

    pub fn get_rect(&self) -> Rectangle {
        Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: LASER_WIDTH,
            height: LASER_HEIGHT,
        }
    }
}
