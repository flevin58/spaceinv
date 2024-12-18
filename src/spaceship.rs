use crate::constants::*;
use crate::laser::Laser;
use ray::{Rectangle, Texture2D, Vector2};
use raylib_ffi as ray;

#[derive(Clone)]
struct Bounds {
    min: f32,
    max: f32,
}

pub struct Spaceship {
    texture: Texture2D,
    position: Vector2,
    bounds: Bounds,
    last_fire_time: f64,
}

impl Spaceship {
    pub fn new() -> Self {
        let ship_data = include_bytes!("../assets/images/spaceship.png");
        let texture = unsafe {
            let ship_image = ray::LoadImageFromMemory(
                ray::rl_str!(".png"),
                ship_data.as_ptr(),
                ship_data.len() as i32,
            );
            ray::LoadTextureFromImage(ship_image)
        };
        let width = texture.width;
        let height = texture.height;

        let ship_x = (WORLD_WIDTH - width) as f32 / 2.;
        let ship_y = (WORLD_HEIGHT - height - SPACESHIP_YOFFSET) as f32;

        let ship_min = SPACESHIP_XOFFSET as f32;
        let ship_max = (WORLD_WIDTH - width - SPACESHIP_XOFFSET) as f32;

        Self {
            texture,
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
        let ship_x = (WORLD_WIDTH - self.texture.width) / 2;
        let ship_y = WORLD_HEIGHT - self.texture.height - SPACESHIP_YOFFSET;
        self.position.x = ship_x as f32;
        self.position.y = ship_y as f32;
    }

    // currently unused
    pub fn update(&mut self) {}

    pub fn draw(&self) {
        unsafe {
            ray::DrawTextureV(self.texture, self.position, COLOR_WHITE);
        }
    }

    pub fn draw_at(&self, x: f32, y: f32) {
        let pos = Vector2 { x, y };
        unsafe {
            ray::DrawTextureV(self.texture, pos, COLOR_WHITE);
        }
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

    pub fn fire_laser(&mut self) -> Option<Laser> {
        unsafe {
            if ray::GetTime() - self.last_fire_time >= LASER_TIME {
                let laser_pos = Vector2 {
                    x: self.position.x + (self.texture.width as f32 - LASER_WIDTH) / 2.,
                    y: self.position.y,
                };
                self.last_fire_time = ray::GetTime();

                Some(Laser::new(laser_pos, LASER_SPEED))
            } else {
                None
            }
        }
    }

    pub fn get_rect(&self) -> Rectangle {
        let width = self.texture.width as f32;
        let height = self.texture.height as f32;
        Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: width,
            height: height,
        }
    }
}
