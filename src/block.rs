use crate::constants::*;
use ray::{Rectangle, Vector2};
use raylib_ffi as ray;

#[derive(Clone)]
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

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn set_inactive(&mut self) {
        self.active = false;
    }

    pub fn draw(&self) {
        unsafe {
            ray::DrawRectangleV(self.position, BLOCK_SIZE, BLOCK_COLOR);
        }
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
