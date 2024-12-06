use ray::{Color, Rectangle, Vector2};
use raylib_ffi as ray;

// ALIEN TYPES
pub const ALIEN1: usize = 0;
pub const ALIEN2: usize = 1;
pub const ALIEN3: usize = 2;

// COLORS

pub const COLOR_WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
};
// GAME CONSTANTS

pub const WINDOW_WIDTH: i32 = 750;
pub const WINDOW_HEIGHT: i32 = 700;
pub const OFFSETX: i32 = 50;
pub const OFFSETY: i32 = 100;
pub const WORLD_WIDTH: i32 = WINDOW_WIDTH + OFFSETX;
pub const WORLD_HEIGHT: i32 = WINDOW_HEIGHT + OFFSETY;
pub const WINDOW_TITLE: &str = "Rust Space Invaders";

pub const PLAYER_LIVES: usize = 3;

pub const WINDOW_BKG_COLOR: Color = Color {
    r: 29,
    g: 29,
    b: 27,
    a: 255,
};

// WINDOWS GUI CONSTANTS

pub const FONT_SIZE: i32 = 34;
pub const FONT_SPACING: f32 = 2.;

pub const GUI_LIVEIMG_X: f32 = 50.;
pub const GUI_LIVEIMG_Y: f32 = 745.;
pub const GUI_LIVEIMG_INC: f32 = 50.;

pub const LEVEL_POS: Vector2 = Vector2 { x: 570., y: 740. };

pub const FRAME_ROUNDNESS: f32 = 0.18;
pub const FRAME_THICKNESS: f32 = 2.;
pub const FRAME_SEGMENTS: i32 = 20;
const FRAME_PADDING: f32 = 10.;
const FRAME_SIDE: f32 = (WINDOW_WIDTH + OFFSETX) as f32 - FRAME_PADDING;

pub const FRAME_RECT: Rectangle = Rectangle {
    x: FRAME_PADDING,
    y: FRAME_PADDING,
    width: FRAME_SIDE - FRAME_PADDING,
    height: FRAME_SIDE - FRAME_PADDING,
};

pub const GUI_LINE_Y: f32 = 730.;
pub const GUI_LINE_X1: f32 = 25.;
pub const GUI_LINE_X2: f32 = 775.;
pub const GUI_LINE_THICKNESS: f32 = 2.;
pub const GUI_SCORE_TEXT_POS: Vector2 = Vector2 { x: 50., y: 15. };
pub const GUI_SCORE_VALUE_POS: Vector2 = Vector2 { x: 50., y: 40. };
pub const GUI_HIGH_SCORE_TEXT_POS: Vector2 = Vector2 { x: 570., y: 15. };
pub const GUI_HIGH_SCORE_VALUE_POS: Vector2 = Vector2 { x: 655., y: 40. };

pub const FRAME_COLOR: Color = Color {
    r: 243,
    g: 216,
    b: 63,
    a: 255,
};

pub const GREEN_COLOR: Color = Color {
    r: 8,
    g: 160,
    b: 69,
    a: 255,
};

pub const RED_COLOR: Color = Color {
    r: 133,
    g: 1,
    b: 1,
    a: 255,
};

// LASER CONSTANTS

pub const LASER_TIME: f64 = 0.35;
pub const LASER_SPEED: f32 = -6.;
pub const LASER_WIDTH: f32 = 4.;
pub const LASER_HEIGHT: f32 = 15.;
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
pub const SPACESHIP_YOFFSET: i32 = OFFSETY;
pub const SPACESHIP_XOFFSET: i32 = OFFSETX / 2;

// MYSTERYSHIP CONSTANTS

pub const MYSTERYSHIP_SPEED: f32 = 3.;
pub const MYSTERYSHIP_YPOS: f32 = 90.;
pub const MYSTERYSHIP_MIN_INTERVAL: f64 = 10.;
pub const MYSTERYSHIP_MAX_INTERVAL: f64 = 20.;
pub const MYSTERYSHIP_SCORE: usize = 500;

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

pub const ALIEN_SCORES: [usize; 3] = [100, 200, 300];

pub const ALIEN_ROWS: usize = 5;
pub const ALIEN_COLUMNS: usize = 11;
pub const ALIEN_SIZE: usize = 55;
pub const ALIEN_OFFSET_X: usize = 75;
pub const ALIEN_OFFSET_Y: usize = 110;
pub const ALIEN_DOWN_DISTANCE: usize = 4;
pub const ALIEN_LASER_SPEED: f32 = 6.;
pub const ALIEN_LASER_INTERVAL: f64 = 0.35;

// SOUND CONSTANTS

//pub const SOUND_LASER: &str = "assets/sounds/laser_pew.ogg";
//pub const SOUND_EXPLOSION: &str = "assets/sounds/rumble.ogg";
//pub const GAME_MUSIC: &str = "assets/sounds/music.ogg";
