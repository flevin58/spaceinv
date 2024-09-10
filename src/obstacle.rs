use crate::block::Block;
use crate::constants::*;
use raylib::prelude::*;

pub struct Obstacle<'a> {
    // position: Vector2,
    blocks: Vec<Block>,
    grid: [&'a [u8; GRID_WIDTH]; GRID_HEIGHT],
}

impl<'a> Obstacle<'a> {
    pub fn new(x: usize, y: usize) -> Obstacle<'a> {
        let mut obs = Obstacle {
            // position: Vector2::new(x, y),
            blocks: Vec::new(),
            grid: OBSTACLE_GRID,
        };
        for row in 0..GRID_HEIGHT {
            for col in 0..GRID_WIDTH {
                if obs.grid[row][col] == 1 {
                    let pos_x = x + col * BLOCK_SIDE;
                    let pos_y = y + row * BLOCK_SIDE;
                    let block = Block::new(Vector2 {
                        x: pos_x as f32,
                        y: pos_y as f32,
                    });
                    obs.blocks.push(block);
                }
            }
        }

        obs
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for block in self.blocks.iter() {
            block.draw(d);
        }
    }
}
