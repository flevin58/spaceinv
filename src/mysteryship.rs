use crate::constants::*;
use rand::Rng;
use raylib_ffi::{
    rl_str, DrawTextureV, GetScreenWidth, LoadImageFromMemory, LoadTextureFromImage, Rectangle,
    Texture2D, Vector2,
};

pub struct MysteryShip {
    texture: Texture2D,
    position: Vector2,
    speed: f32,
    active: bool,
}

impl MysteryShip {
    pub fn new() -> Self {
        let ship_data = include_bytes!("../assets/images/mystery.png");
        let texture = unsafe {
            let ship_image =
                LoadImageFromMemory(rl_str!(".png"), ship_data.as_ptr(), ship_data.len() as i32);
            LoadTextureFromImage(ship_image)
        };
        Self {
            texture,
            position: Vector2 { x: 0., y: 0. },
            speed: 0.,
            active: false,
        }
    }

    pub fn set_inactive(&mut self) {
        self.active = false;
    }

    pub fn update(&mut self) {
        if self.active {
            self.position.x += self.speed;
            if self.position.x > (WORLD_WIDTH - self.texture.width - OFFSETX / 2) as f32
                || self.position.x < (OFFSETX / 2) as f32
            {
                self.active = false;
            }
        }
    }

    pub fn draw(&self) {
        if self.active {
            unsafe {
                DrawTextureV(self.texture, self.position, COLOR_WHITE);
            }
        }
    }

    pub fn spawn(&mut self) {
        let swidth = unsafe { GetScreenWidth() };
        let side: i32 = rand::thread_rng().gen_range(0..1);
        self.position.y = MYSTERYSHIP_YPOS;
        if side == 0 {
            self.position.x = (OFFSETX / 2) as f32;
            self.speed = MYSTERYSHIP_SPEED;
        } else {
            self.position.x = (swidth - self.texture.width - OFFSETX / 2) as f32;
            self.speed = -MYSTERYSHIP_SPEED;
        }
        self.active = true;
    }

    pub fn get_rect(&self) -> Rectangle {
        let mut width: f32 = 0.;
        let mut height: f32 = 0.;

        if self.active {
            width = self.texture.width as f32;
            height = self.texture.height as f32;
        }

        Rectangle {
            x: self.position.x,
            y: self.position.y,
            width,
            height,
        }
    }
}
