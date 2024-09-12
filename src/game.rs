use std::{fs::File, io::Write};

use crate::constants::*;
use crate::laser::Laser;
use crate::mysteryship::MysteryShip;
use crate::obstacle::Obstacle;
use crate::spaceship::Spaceship;
use crate::{alien::Alien, assets::Assets};

use rand::Rng;

use raylib::{
    core::math::Vector2,
    ffi::KeyboardKey::*,
    prelude::{RaylibDraw, RaylibDrawHandle},
    RaylibHandle,
};

pub struct Game<'a> {
    spaceship: Spaceship<'a>,
    lasers: Vec<Laser>,
    obstacles: Vec<Obstacle>,
    aliens: Vec<Alien<'a>>,
    aliens_direction: i32,
    alien_lasers: Vec<Laser>,
    time_alien_last_fired: f64,
    mysteryship: MysteryShip<'a>,
    mysteryship_spawn_interval: f64,
    time_last_spawned: f64,
    lives: usize,
    running: bool,
    level: usize,
    score: usize,
    high_score: usize,
}

impl<'a> Game<'a> {
    pub fn new(rl: &mut RaylibHandle, assets: &'a Assets) -> Self {
        //let font_data = include_bytes!("../assets/fonts/monogram.ttf");
        //let font_res = rl.load_font_from_memory(thread, ".ttf", font_data, FONT_SIZE, None);

        let mut game = Game {
            spaceship: Spaceship::new(rl, assets),
            lasers: Vec::new(),
            obstacles: Vec::new(),
            aliens: Vec::new(),
            aliens_direction: 1,
            alien_lasers: Vec::new(),
            time_alien_last_fired: 0.,
            mysteryship: MysteryShip::new(assets),
            mysteryship_spawn_interval: rand::thread_rng()
                .gen_range(MYSTERYSHIP_MIN_INTERVAL..MYSTERYSHIP_MAX_INTERVAL),
            time_last_spawned: 0.,
            lives: PLAYER_LIVES,
            running: true,
            level: 1,
            score: 0,
            high_score: 0,
        };

        // create the obstacles
        let gap = (rl.get_screen_width() as usize - (NUM_OBSTACLES * OBSTACLE_WIDTH))
            / (NUM_OBSTACLES + 1);
        for i in 0..NUM_OBSTACLES {
            let offset_x = (i + 1) * gap + i * OBSTACLE_WIDTH;
            let offset_y = rl.get_screen_height() as usize - OBSTACLE_PADDING - OFFSETY as usize;
            game.obstacles.push(Obstacle::new(offset_x, offset_y));
        }

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
                game.aliens.push(Alien::new(
                    assets,
                    alien_type,
                    Vector2::new(x as f32, y as f32),
                ));
            }
        }

        game.load_high_score();
        //game.music.play_stream();

        game
    }

    pub fn reset(&mut self, rl: &mut RaylibHandle, assets: &'a Assets) {
        self.spaceship.reset(rl);
        self.lasers.clear();

        // create the ostacles
        self.obstacles.clear();
        let gap = (rl.get_screen_width() as usize - (NUM_OBSTACLES * OBSTACLE_WIDTH))
            / (NUM_OBSTACLES + 1);
        for i in 0..NUM_OBSTACLES {
            let offset_x = (i + 1) * gap + i * OBSTACLE_WIDTH;
            let offset_y = rl.get_screen_height() as usize - OBSTACLE_PADDING;
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
                    assets,
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
        self.running = true;
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

    pub fn handle_input(&mut self, rl: &mut RaylibHandle, assets: &'a Assets) {
        // on game over we can reset the game!
        if !self.running {
            if rl.is_key_down(KEY_ENTER) {
                self.reset(rl, assets);
            }
            return;
        }

        if rl.is_key_down(KEY_LEFT) {
            self.spaceship.move_left();
        } else if rl.is_key_down(KEY_RIGHT) {
            self.spaceship.move_right();
        } else if rl.is_key_down(KEY_SPACE) {
            let laser = self.spaceship.fire_laser(&rl);
            if laser.is_some() {
                self.lasers.push(laser.unwrap());
            }
        }
    }

    pub fn move_aliens(&mut self, rl: &mut RaylibHandle) {
        let mut should_move_down = false;
        for alien in self.aliens.iter_mut() {
            if alien.has_overflowed_right(rl) {
                self.aliens_direction = -1;
                should_move_down = true;
            }
            if alien.has_overflowed_left() {
                self.aliens_direction = 1;
                should_move_down = true;
            }
            alien.update(rl, self.aliens_direction);
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

    pub fn aliens_shoot_laser(&mut self, rl: &mut RaylibHandle) {
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
                        return true;
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

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        // do nothing if game is over
        if !self.running {
            return;
        }

        // update the spaceship (currently does nothing)
        self.spaceship.update(rl);

        // update all spaceship lasers
        for laser in self.lasers.iter_mut() {
            laser.update(rl);
        }

        // remove all inactive spaceship lasers
        self.lasers.retain(|elem| elem.is_active());

        // remove all inactive blocks
        for obstacle in self.obstacles.iter_mut() {
            obstacle.remove_inactive_blocks();
        }

        // remove all inactive aliens
        self.aliens.retain(|elem| elem.is_active());

        // update the aliens
        self.move_aliens(rl);

        // create alien lasers
        self.aliens_shoot_laser(rl);

        // update alien lasers
        for laser in self.alien_lasers.iter_mut() {
            laser.update(rl);
        }

        // remove all inactive alien lasers
        self.alien_lasers.retain(|elem| elem.is_active());

        // update the mystery ship
        let current_time = rl.get_time();
        if current_time - self.time_last_spawned > self.mysteryship_spawn_interval {
            self.mysteryship.spawn(rl);
            self.time_last_spawned = rl.get_time();
            self.mysteryship_spawn_interval =
                rand::thread_rng().gen_range(MYSTERYSHIP_MIN_INTERVAL..MYSTERYSHIP_MAX_INTERVAL)
        }

        self.mysteryship.update(rl);

        let done = self.check_for_collisions();
        self.check_for_highscore();
        if done {
            self.game_over();
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, assets: &Assets) {
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
        if self.running {
            d.draw_text_ex(
                assets.get_font(),
                format!("LEVEL {:0>2}", self.level).as_str(),
                LEVEL_POS,
                FONT_SIZE as f32,
                FONT_SPACING,
                FRAME_COLOR,
            );
        } else {
            d.draw_text_ex(
                assets.get_font(),
                "GAME OVER",
                LEVEL_POS,
                FONT_SIZE as f32,
                FONT_SPACING,
                FRAME_COLOR,
            );
        }
        d.draw_text_ex(
            assets.get_font(),
            "SCORE",
            GUI_SCORE_TEXT_POS,
            FONT_SIZE as f32,
            FONT_SPACING,
            FRAME_COLOR,
        );
        d.draw_text_ex(
            assets.get_font(),
            format!("{:0>5}", self.score).as_str(),
            GUI_SCORE_VALUE_POS,
            FONT_SIZE as f32,
            FONT_SPACING,
            FRAME_COLOR,
        );
        d.draw_text_ex(
            assets.get_font(),
            "HIGH SCORE",
            GUI_HIGH_SCORE_TEXT_POS,
            FONT_SIZE as f32,
            FONT_SPACING,
            FRAME_COLOR,
        );
        d.draw_text_ex(
            assets.get_font(),
            format!("{:0>5}", self.high_score).as_str(),
            GUI_HIGH_SCORE_VALUE_POS,
            FONT_SIZE as f32,
            FONT_SPACING,
            FRAME_COLOR,
        );

        let mut x = GUI_LIVEIMG_X;
        for _ in 0..self.lives {
            self.spaceship.draw_at(d, x, GUI_LIVEIMG_Y);
            x += GUI_LIVEIMG_INC;
        }

        for obstacle in self.obstacles.iter() {
            obstacle.draw(d);
        }
        self.spaceship.draw(d);
        for laser in self.lasers.iter_mut() {
            laser.draw(d);
        }

        for alien in self.aliens.iter() {
            alien.draw(d);
        }

        for laser in self.alien_lasers.iter_mut() {
            laser.draw(d);
        }

        self.mysteryship.draw(d);
    }

    fn game_over(&mut self) {
        self.running = false;
    }
}
