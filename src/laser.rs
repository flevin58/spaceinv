use crate::constants::*;
use raylib::prelude::*;

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

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        if self.active {
            self.position.y += self.speed;
            if self.position.y > rl.get_screen_height().as_f32() || self.position.y < 0. {
                self.active = false;
                println!("Laser inactive");
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        if self.active {
            d.draw_rectangle_v(self.position, LASER_SIZE, LASER_COLOR);
        }
    }
}
