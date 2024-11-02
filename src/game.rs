use std::rc::Rc;
use std::{fs::File, io::Write};

use crate::alien::Alien;
use crate::assets::Assets;
use crate::constants::*;
use crate::context::Context;
use crate::laser::Laser;
use crate::mysteryship::MysteryShip;
use crate::obstacle::Obstacle;
use crate::spaceship::Spaceship;
use rand::Rng;

use raylib::ffi::Color;
use raylib::ffi::TraceLogLevel::*;
use raylib::{
    core::math::Vector2,
    ffi::KeyboardKey::*,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

#[derive(Clone, PartialEq)]
enum GameState {
    Running,
    GameOver,
    LevelUp,
    Paused,
    Quit,
}

pub struct Game {
    ctx: Rc<Context>,
    assets: Rc<Assets>,
    spaceship: Box<Spaceship>,
    lasers: Vec<Laser>,
    obstacles: Vec<Obstacle>,
    aliens: Vec<Box<Alien>>,
    aliens_direction: i32,
    alien_lasers: Vec<Laser>,
    time_alien_last_fired: f64,
    mysteryship: Box<MysteryShip>,
    mysteryship_spawn_interval: f64,
    time_last_spawned: f64,
    lives: usize,
    level: usize,
    score: usize,
    high_score: usize,
    state: GameState,
}

impl Game {
    pub fn new() -> Self {
        let (mut rl, thread) = raylib::init()
            .size(WORLD_WIDTH, WORLD_HEIGHT)
            .title(WINDOW_TITLE)
            .vsync()
            .build();

        rl.set_trace_log(LOG_ERROR);
        rl.set_target_fps(60);

        // INIT AUDIO
        // let audio = RaylibAudio::init_audio_device().expect("error initializing audio device");
        // if audio.is_audio_device_ready() {
        //     println!("Audio device ready to use!");
        // }

        let context = Rc::new(Context::new(rl, thread));
        let assets = Rc::new(Assets::new(context.clone()));

        let mut game = Game {
            ctx: context.clone(),
            assets: assets.clone(),
            spaceship: Box::new(Spaceship::new(assets.clone())),
            lasers: Vec::new(),
            obstacles: Vec::new(),
            aliens: Vec::new(),
            aliens_direction: 1,
            alien_lasers: Vec::new(),
            time_alien_last_fired: 0.,
            mysteryship: Box::new(MysteryShip::new(assets.clone())),
            mysteryship_spawn_interval: rand::thread_rng()
                .gen_range(MYSTERYSHIP_MIN_INTERVAL..MYSTERYSHIP_MAX_INTERVAL),
            time_last_spawned: 0.,
            lives: PLAYER_LIVES,
            level: 1,
            score: 0,
            high_score: 0,
            state: GameState::Running,
        };

        game.create_obstacles();
        game.create_aliens();
        game.load_high_score();

        game
    }

    pub fn run(&mut self) {
        while self.state != GameState::Quit {
            self.handle_input();
            self.update();
            self.draw();
        }
    }

    pub fn init_level(&mut self) {
        self.level += 1;
        self.aliens_direction = 1;
        self.mysteryship_spawn_interval =
            rand::thread_rng().gen_range(MYSTERYSHIP_MIN_INTERVAL..MYSTERYSHIP_MAX_INTERVAL);
        self.time_last_spawned = 0.0;
        self.time_alien_last_fired = 0.0;
        self.state = GameState::Running;
    }

    pub fn init_game(&mut self) {
        self.lives = PLAYER_LIVES;
        self.level = 0;
        self.score = 0;
        self.high_score = 0;
        self.load_high_score();
        self.reset_game();
        self.init_level();
    }

    pub fn reset_game(&mut self) {
        self.spaceship.reset();
        self.aliens.clear();
        self.alien_lasers.clear();
        self.obstacles.clear();
        self.create_obstacles();
        self.create_aliens();
    }

    pub fn create_obstacles(&mut self) {
        let rl = self.ctx.rl.borrow();
        // create the obstacles
        let gap = (rl.get_screen_width() as usize - (NUM_OBSTACLES * OBSTACLE_WIDTH))
            / (NUM_OBSTACLES + 1);
        for i in 0..NUM_OBSTACLES {
            let offset_x = (i + 1) * gap + i * OBSTACLE_WIDTH;
            let offset_y = rl.get_screen_height() as usize - OBSTACLE_PADDING - OFFSETY as usize;
            self.obstacles.push(Obstacle::new(offset_x, offset_y));
        }
    }

    pub fn create_aliens(&mut self) {
        for row in 0..ALIEN_ROWS {
            let alien_type = match row {
                0 => 3,
                1 | 2 => 2,
                _ => 1,
            };

            for col in 0..ALIEN_COLUMNS {
                let x = ALIEN_OFFSET_X + col * ALIEN_SIZE;
                let y = ALIEN_OFFSET_Y + row * ALIEN_SIZE;
                self.aliens.push(Alien::new(
                    self.assets.clone(),
                    alien_type,
                    Vector2::new(x as f32, y as f32),
                ));
            }
        }
    }

    pub fn reset(&mut self) {
        self.spaceship.reset();
        self.lasers.clear();

        // create the ostacles
        let rl = self.ctx.rl.borrow_mut();
        self.obstacles.clear();
        let gap = (WINDOW_WIDTH as usize - (NUM_OBSTACLES * OBSTACLE_WIDTH)) / (NUM_OBSTACLES + 1);
        for i in 0..NUM_OBSTACLES {
            let offset_x = (i + 1) * gap + i * OBSTACLE_WIDTH;
            let offset_y = WINDOW_HEIGHT as usize - OBSTACLE_PADDING;
            self.obstacles.push(Obstacle::new(offset_x, offset_y));
        }

        self.aliens.clear();
        // create the aliens
        for row in 0..ALIEN_ROWS {
            let alien_type = match row {
                0 => 3,
                1 | 2 => 2,
                _ => 1,
            };
            for col in 0..ALIEN_COLUMNS {
                let x = ALIEN_OFFSET_X + col * ALIEN_SIZE;
                let y = ALIEN_OFFSET_Y + row * ALIEN_SIZE;
                self.aliens.push(Alien::new(
                    self.assets.clone(),
                    alien_type,
                    Vector2::new(x as f32, y as f32),
                ));
            }
        }

        self.aliens_direction = 1;
        self.alien_lasers.clear();
        self.time_alien_last_fired = 0.;
        self.mysteryship_spawn_interval =
            rand::thread_rng().gen_range(MYSTERYSHIP_MIN_INTERVAL..MYSTERYSHIP_MAX_INTERVAL);
        self.time_last_spawned = 0.;
        self.lives = PLAYER_LIVES;
        self.state = GameState::Running;
    }

    pub fn check_for_highscore(&mut self) {
        if self.score > self.high_score {
            self.high_score = self.score;
            self.save_high_score();
        }
    }

    pub fn save_high_score(&self) {
        let mut out_file =
            File::create("highscore.txt").expect("could not create or open the highscore file");
        write!(out_file, "{}", self.high_score).expect("could not write high score to file");
    }

    pub fn load_high_score(&mut self) {
        let hs = std::fs::read_to_string("highscore.txt");
        match hs {
            Ok(value) => {
                self.high_score = value.parse::<usize>().unwrap();
            }
            Err(_) => {
                self.high_score = 0;
            }
        }
    }

    pub fn handle_input(&mut self) {
        let ctx = self.ctx.clone();
        let rl = ctx.rl.borrow();

        if rl.window_should_close() {
            self.state = GameState::Quit;
        }

        if self.state == GameState::GameOver {
            self.handle_game_over_input();
            return;
        }

        if self.state == GameState::LevelUp {
            self.handle_level_up_input();
            return;
        }

        if rl.is_key_down(KEY_LEFT) {
            self.spaceship.move_left();
        } else if rl.is_key_down(KEY_RIGHT) {
            self.spaceship.move_right();
        } else if rl.is_key_down(KEY_SPACE) {
            let laser = self.spaceship.fire_laser(self.ctx.clone());
            if laser.is_some() {
                self.lasers.push(laser.unwrap());
            }
        } else if rl.is_key_pressed(KEY_P) {
            if self.state == GameState::Paused {
                self.state = GameState::Running;
            } else if self.state == GameState::Running {
                self.state = GameState::Paused;
            }
        }
    }

    pub fn handle_game_over_input(&mut self) {
        let ctx = self.ctx.clone();
        let rl = ctx.rl.borrow();
        if rl.is_key_pressed(KEY_ESCAPE) {
            self.state = GameState::Quit;
        }
        if rl.is_key_pressed(KEY_ENTER) {
            self.reset_game();
            self.init_game();
        }
    }

    pub fn handle_level_up_input(&mut self) {
        let ctx = self.ctx.clone();
        let rl = ctx.rl.borrow();
        if rl.is_key_pressed(KEY_ENTER) {
            self.reset_game();
            self.init_level();
        }
    }

    pub fn move_aliens(&mut self) {
        let mut should_move_down = false;
        for alien in self.aliens.iter_mut() {
            if alien.has_overflowed_right() {
                self.aliens_direction = -1;
                should_move_down = true;
            }
            if alien.has_overflowed_left() {
                self.aliens_direction = 1;
                should_move_down = true;
            }
            alien.update(self.aliens_direction);
        }
        if should_move_down {
            self.move_down_aliens(ALIEN_DOWN_DISTANCE);
        }
    }

    pub fn move_down_aliens(&mut self, distance: usize) {
        for alien in self.aliens.iter_mut() {
            alien.move_down(distance);
        }
    }

    pub fn aliens_shoot_laser(&mut self, ctx: Rc<Context>) {
        let rl = ctx.rl.borrow();
        let current_time = rl.get_time();
        if current_time - self.time_alien_last_fired >= ALIEN_LASER_INTERVAL
            && !self.aliens.is_empty()
        {
            let random_index: usize = rand::thread_rng().gen_range(0..self.aliens.len());
            let alien = &self.aliens[random_index];
            let laser_pos = alien.get_laser_position();
            self.alien_lasers
                .push(Laser::new(laser_pos, ALIEN_LASER_SPEED));
            self.time_alien_last_fired = rl.get_time();
        }
    }

    pub fn check_for_collisions(&mut self) -> bool {
        // ================
        // spaceship lasers
        // ================
        for laser in self.lasers.iter_mut() {
            // check against aliens
            for alien in self.aliens.iter_mut() {
                unsafe {
                    if alien.is_active()
                        && raylib::ffi::CheckCollisionRecs(alien.get_rect(), laser.get_rect())
                    {
                        self.score += alien.get_score();
                        alien.set_inactive();
                        laser.set_inactive();
                        self.assets.play_explosion_sound();
                    }
                }
            }
            // check if obstacle is hit and damage it!
            for obstacle in self.obstacles.iter_mut() {
                for block in obstacle.blocks.iter_mut() {
                    unsafe {
                        if raylib::ffi::CheckCollisionRecs(block.get_rect(), laser.get_rect()) {
                            block.set_inactive();
                            laser.set_inactive();
                        }
                    }
                }
            }
            // check against mystery ship
            unsafe {
                if raylib::ffi::CheckCollisionRecs(self.mysteryship.get_rect(), laser.get_rect()) {
                    self.score += MYSTERYSHIP_SCORE;
                    self.mysteryship.set_inactive();
                    laser.set_inactive();
                    //self.assets.play_explosion_sound();
                }
            }
            // check against alien lasers (yep, we can destroy alien lasers!)
            // T.B.D.
        }

        // ================
        // alien lasers
        // ================
        for laser in self.alien_lasers.iter_mut() {
            // check if spaceship is hit
            unsafe {
                if raylib::ffi::CheckCollisionRecs(laser.get_rect(), self.spaceship.get_rect()) {
                    laser.set_inactive();
                    self.lives -= 1;
                    if self.lives == 0 {
                        // line below gives error. TBD.
                        // self.game_over();
                        self.state = GameState::GameOver;
                    }
                }
            }
            // check if obstacle is hit and damage it!
            for obstacle in self.obstacles.iter_mut() {
                for block in obstacle.blocks.iter_mut() {
                    unsafe {
                        if raylib::ffi::CheckCollisionRecs(block.get_rect(), laser.get_rect()) {
                            block.set_inactive();
                            laser.set_inactive();
                        }
                    }
                }
            }
        }
        // Patch to fix issue in line 318 (spaceship being hit and no lives left)
        if self.state == GameState::GameOver {
            self.game_over();
        }

        // ===========
        // alien ships
        // ===========
        for alien in self.aliens.iter() {
            // alien collision with obstacle
            for obstacle in self.obstacles.iter_mut() {
                for block in obstacle.blocks.iter_mut() {
                    unsafe {
                        if raylib::ffi::CheckCollisionRecs(block.get_rect(), alien.get_rect()) {
                            block.set_inactive();
                        }
                    }
                }
            }
            // alien collision with ship
            unsafe {
                if raylib::ffi::CheckCollisionRecs(self.spaceship.get_rect(), alien.get_rect()) {
                    return true;
                }
            }
        }
        false
    }

    pub fn update(&mut self) {
        // do nothing if game is over
        if self.state != GameState::Running {
            return;
        }

        // update the spaceship (currently does nothing)
        self.spaceship.update();

        // update all spaceship lasers
        for laser in self.lasers.iter_mut() {
            laser.update();
        }

        // remove all inactive spaceship lasers
        self.lasers.retain(|elem| elem.is_active());

        // remove all inactive blocks
        for obstacle in self.obstacles.iter_mut() {
            obstacle.remove_inactive_blocks();
        }

        // remove all inactive aliens
        self.aliens.retain(|elem| elem.is_active());
        if self.aliens.is_empty() {
            self.state = GameState::LevelUp;
        }

        // update the aliens
        self.move_aliens();

        // create alien lasers
        self.aliens_shoot_laser(self.ctx.clone());

        // update alien lasers
        for laser in self.alien_lasers.iter_mut() {
            laser.update();
        }

        // remove all inactive alien lasers
        self.alien_lasers.retain(|elem| elem.is_active());

        // update the mystery ship
        let ctx = self.ctx.clone();
        let rl = ctx.rl.borrow();
        let current_time = rl.get_time();
        if current_time - self.time_last_spawned > self.mysteryship_spawn_interval {
            self.mysteryship.spawn(self.ctx.clone());
            self.time_last_spawned = rl.get_time();
            self.mysteryship_spawn_interval =
                rand::thread_rng().gen_range(MYSTERYSHIP_MIN_INTERVAL..MYSTERYSHIP_MAX_INTERVAL)
        }

        self.mysteryship.update();

        let done = self.check_for_collisions();
        self.check_for_highscore();
        if done {
            self.game_over();
        }
    }

    pub fn draw(&mut self) {
        let ctx = self.ctx.clone();
        let mut rl = ctx.rl.borrow_mut();
        let thread = ctx.thread.borrow();
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(WINDOW_BKG_COLOR);
        d.draw_rectangle_rounded_lines(
            FRAME_RECT,
            FRAME_ROUNDNESS,
            FRAME_SEGMENTS,
            FRAME_THICKNESS,
            FRAME_COLOR,
        );
        d.draw_line_ex(
            Vector2 {
                x: GUI_LINE_X1,
                y: GUI_LINE_Y,
            },
            Vector2 {
                x: GUI_LINE_X2,
                y: GUI_LINE_Y,
            },
            GUI_LINE_THICKNESS,
            FRAME_COLOR,
        );
        if self.state != GameState::GameOver {
            d.draw_text_ex(
                self.assets.get_font(),
                format!("LEVEL {:0>2}", self.level).as_str(),
                LEVEL_POS,
                FONT_SIZE as f32,
                FONT_SPACING,
                FRAME_COLOR,
            );
        } else {
            d.draw_text_ex(
                self.assets.get_font(),
                "GAME OVER",
                LEVEL_POS,
                FONT_SIZE as f32,
                FONT_SPACING,
                FRAME_COLOR,
            );
        }
        d.draw_text_ex(
            self.assets.get_font(),
            "SCORE",
            GUI_SCORE_TEXT_POS,
            FONT_SIZE as f32,
            FONT_SPACING,
            FRAME_COLOR,
        );
        d.draw_text_ex(
            self.assets.get_font(),
            format!("{:0>5}", self.score).as_str(),
            GUI_SCORE_VALUE_POS,
            FONT_SIZE as f32,
            FONT_SPACING,
            FRAME_COLOR,
        );
        d.draw_text_ex(
            self.assets.get_font(),
            "HIGH SCORE",
            GUI_HIGH_SCORE_TEXT_POS,
            FONT_SIZE as f32,
            FONT_SPACING,
            FRAME_COLOR,
        );
        d.draw_text_ex(
            self.assets.get_font(),
            format!("{:0>5}", self.high_score).as_str(),
            GUI_HIGH_SCORE_VALUE_POS,
            FONT_SIZE as f32,
            FONT_SPACING,
            FRAME_COLOR,
        );

        // DRAW OTHER OBJECTS

        let mut x = GUI_LIVEIMG_X;
        for _ in 0..self.lives {
            self.spaceship.draw_at(&mut d, x, GUI_LIVEIMG_Y);
            x += GUI_LIVEIMG_INC;
        }

        for obstacle in self.obstacles.iter() {
            obstacle.draw(&mut d);
        }
        self.spaceship.draw(&mut d);
        for laser in self.lasers.iter_mut() {
            laser.draw(&mut d);
        }

        for alien in self.aliens.iter() {
            alien.draw(&mut d);
        }

        for laser in self.alien_lasers.iter_mut() {
            laser.draw(&mut d);
        }

        self.mysteryship.draw(&mut d);

        if self.state == GameState::LevelUp {
            self.level_up_draw(&mut d);
        }
    }

    fn center_text_at(
        &mut self,
        ctx: Rc<Context>,
        d: &mut RaylibDrawHandle,
        posx: i32,
        posy: i32,
        width: i32,
        text: &str,
    ) {
        const YELLOW: Color = Color {
            r: 243,
            g: 216,
            b: 63,
            a: 255,
        };

        let rl = ctx.rl.borrow_mut();
        let text_width = rl.measure_text(text, 34);
        let newx = posx + (width - text_width) / 2;
        d.draw_text(text, newx, posy, 34, YELLOW);
    }

    fn draw_dialog_box(
        &mut self,
        ctx: Rc<Context>,
        d: &mut RaylibDrawHandle,
        text1: &str,
        text2: &str,
        text3: &str,
        color: Color,
    ) {
        const RWIDTH: i32 = 500;
        const RHEIGHT: i32 = 200;
        let rposx = (WORLD_WIDTH - RWIDTH) / 2;
        const RPOSY: i32 = 100;
        d.draw_rectangle_gradient_h(rposx, RPOSY, RWIDTH, RHEIGHT, color, color);
        d.draw_rectangle_lines(rposx, RPOSY, RWIDTH, RHEIGHT, color);
        self.center_text_at(ctx.clone(), d, rposx, 150, RWIDTH, text1);
        self.center_text_at(ctx.clone(), d, rposx, 190, RWIDTH, text2);
        self.center_text_at(ctx.clone(), d, rposx, 230, RWIDTH, text3);
    }

    fn level_up_draw(&mut self, d: &mut RaylibDrawHandle) {
        self.draw_dialog_box(
            self.ctx.clone(),
            d,
            "CONGRATULATIONS",
            "YOU DEFEATED THE ALIENS",
            "PRESS ENTER FOR NEXT LEVEL",
            GREEN_COLOR,
        );
    }

    fn game_over_draw(&mut self, d: &mut RaylibDrawHandle) {
        self.draw_dialog_box(
            self.ctx.clone(),
            d,
            "GAME OVER",
            "PRESS ENTER TO RESTART",
            "PRESS ESC TO QUIT",
            RED_COLOR,
        );
    }

    fn game_over(&mut self) {
        self.state = GameState::GameOver;
        self.save_high_score();
    }
}
