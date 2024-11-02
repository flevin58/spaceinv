use crate::block::Block;
use crate::constants::*;
use raylib::prelude::*;

#[derive(Clone)]
pub struct Obstacle {
    pub blocks: Vec<Block>,
}

impl Obstacle {
    pub fn new(x: usize, y: usize) -> Obstacle {
        let mut obs = Obstacle {
            // position: Vector2::new(x, y),
            blocks: Vec::new(),
        };
        for row in 0..GRID_HEIGHT {
            for col in 0..GRID_WIDTH {
                if OBSTACLE_GRID[row][col] == 1 {
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

    pub fn remove_inactive_blocks(&mut self) {
        self.blocks.retain(|elem| elem.is_active());
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for block in self.blocks.iter() {
            block.draw(d);
        }
    }
}
