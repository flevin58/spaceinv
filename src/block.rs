use crate::constants::*;
use raylib::math::Vector2;
use raylib::prelude::*;

pub struct Block {
    position: Vector2,
}

impl Block {
    pub fn new(position: Vector2) -> Block {
        Block { position }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_v(self.position, BLOCK_SIZE, BLOCK_COLOR);
    }
}
