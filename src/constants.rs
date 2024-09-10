use raylib::color::Color;
use raylib::math::Vector2;

// GAME CONSTANTS

pub const WINDOW_WIDTH: i32 = 750;
pub const WINDOW_HEIGHT: i32 = 700;
pub const WINDOW_TITLE: &str = "Rust Space Invaders";
pub const WINDOW_BKG_COLOR: Color = Color {
    r: 29,
    g: 29,
    b: 27,
    a: 255,
};

// LASER CONSTANTS

pub const LASER_TIME: f64 = 0.35;
pub const LASER_SPEED: f32 = -6.;
pub const LASER_WIDTH: f32 = 4.;
pub const LASER_HEIGHT: f32 = 5.;
pub const LASER_SIZE: Vector2 = Vector2 {
    x: LASER_WIDTH,
    y: LASER_HEIGHT,
};
pub const LASER_COLOR: Color = Color {
    r: 243,
    g: 216,
    b: 63,
    a: 255,
};

// SPACESHIP CONSTANTS

pub const SPACESHIP_SPEED: f32 = 7.;

// BLOCK CONSTANTS

pub const BLOCK_COLOR: Color = Color {
    r: 243,
    g: 216,
    b: 63,
    a: 255,
};

pub const BLOCK_SIDE: usize = 3;

pub const BLOCK_SIZE: Vector2 = Vector2 {
    x: BLOCK_SIDE as f32,
    y: BLOCK_SIDE as f32,
};

// GRID CONSTANTS

pub const GRID_WIDTH: usize = 23;
pub const GRID_HEIGHT: usize = 13;

// OSTACLE CONSTANTS

pub const NUM_OBSTACLES: usize = 4;
pub const OBSTACLE_WIDTH: usize = GRID_WIDTH * BLOCK_SIDE;
pub const OBSTACLE_PADDING: usize = 100;

#[rustfmt::skip]
pub const OBSTACLE_GRID: [&[u8; GRID_WIDTH]; GRID_HEIGHT ] = [
    &[0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0],
    &[0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0],
    &[0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0],
    &[0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0],
    &[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    &[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    &[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    &[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    &[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    &[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    &[1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1],
    &[1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1],
    &[1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1],
];

// ALIENS CONSTANTS

pub const ALIEN_ROWS: usize = 5;
pub const ALIEN_COLUMNS: usize = 11;
pub const ALIEN_SIZE: usize = 55;
pub const ALIEN_OFFSET_X: usize = 75;
pub const ALIEN_OFFSET_Y: usize = 110;
pub const ALIEN_DOWN_DISTANCE: usize = 4;

pub const ALIEN1_TEXTURE: &str = "assets/sprites/alien_1.png";
pub const ALIEN2_TEXTURE: &str = "assets/sprites/alien_2.png";
pub const ALIEN3_TEXTURE: &str = "assets/sprites/alien_3.png";
