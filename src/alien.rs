use crate::constants::{self, *};
use ray::{Texture2D, Vector2};
use raylib_ffi as ray;

pub struct Alien {
    texture: [Texture2D; 3],
    kind: usize,
    position: ray::Vector2,
    active: bool,
    score: usize,
}

impl Alien {
    pub fn new(kind: usize, position: ray::Vector2) -> Box<Alien> {
        // Alien1
        let alien1_data = include_bytes!("../assets/images/alien_1.png");
        let texture1 = unsafe {
            let alien1_image = ray::LoadImageFromMemory(
                ray::rl_str!(".png"),
                alien1_data.as_ptr(),
                alien1_data.len() as i32,
            );
            ray::LoadTextureFromImage(alien1_image)
        };

        // Alien2
        let alien2_data = include_bytes!("../assets/images/alien_2.png");
        let texture2 = unsafe {
            let alien2_image = ray::LoadImageFromMemory(
                ray::rl_str!(".png"),
                alien2_data.as_ptr(),
                alien2_data.len() as i32,
            );
            ray::LoadTextureFromImage(alien2_image)
        };

        // Alien3
        let alien3_data = include_bytes!("../assets/images/alien_3.png");
        let texture3 = unsafe {
            let alien3_image = ray::LoadImageFromMemory(
                ray::rl_str!(".png"),
                alien3_data.as_ptr(),
                alien3_data.len() as i32,
            );
            ray::LoadTextureFromImage(alien3_image)
        };

        Box::new(Alien {
            texture: [texture1, texture2, texture3],
            kind,
            position,
            active: true,
            score: ALIEN_SCORES[kind],
        })
    }

    pub fn get_score(&self) -> usize {
        self.score
    }

    pub fn has_overflowed_right(&self) -> bool {
        self.position.x as i32 + self.texture[self.kind].width > WINDOW_WIDTH - OFFSETX / 2
    }

    pub fn has_overflowed_left(&self) -> bool {
        (self.position.x as i32) < (OFFSETX / 2)
    }

    pub fn get_laser_position(&self) -> Vector2 {
        let width = self.texture[self.kind].width as f32;
        let height = self.texture[self.kind].height as f32;
        ray::Vector2 {
            x: self.position.x + width / 2.,
            y: self.position.y + height,
        }
    }

    pub fn move_down(&mut self, distance: usize) {
        self.position.y += distance as f32;
    }

    pub fn set_inactive(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn update(&mut self, direction: i32) {
        self.position.x += direction as f32;
    }

    pub fn draw(&self) {
        unsafe {
            ray::DrawTextureV(
                self.texture[self.kind],
                self.position,
                constants::COLOR_WHITE,
            );
        }
    }

    pub fn get_rect(&self) -> ray::Rectangle {
        ray::Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: self.texture[self.kind].width as f32,
            height: self.texture[self.kind].height as f32,
        }
    }
}
