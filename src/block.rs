use crate::constants::*;
use raylib::{
    core::math::Vector2,
    ffi::Rectangle,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

pub struct Block {
    position: Vector2,
    active: bool,
}

impl Block {
    pub fn new(position: Vector2) -> Block {
        Block {
            position,
            active: true,
        }
    }

    pub fn erase(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_v(self.position, BLOCK_SIZE, BLOCK_COLOR);
    }

    pub fn get_rect(&self) -> Rectangle {
        Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: BLOCK_SIDE as f32,
            height: BLOCK_SIDE as f32,
        }
    }
}
