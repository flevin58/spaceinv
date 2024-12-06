use std::os::raw::c_int;
use std::{fs::File, io::Write};

use crate::alien::Alien;
use crate::assets::Assets;
//use crate::audio::Audio;
use crate::constants::*;
//use crate::context::Context;
use crate::laser::Laser;
use crate::log;
use crate::mysteryship::MysteryShip;
use crate::obstacle::Obstacle;
use crate::spaceship::Spaceship;
use rand::Rng;

use raylib_ffi::{
    enums::{KeyboardKey, TraceLogLevel},
    rl_str, BeginDrawing, CheckCollisionRecs, ClearBackground, CloseAudioDevice, Color, DrawLineEx,
    DrawRectangleGradientH, DrawRectangleLines, DrawRectangleRoundedLinesEx, DrawText, DrawTextEx,
    EndDrawing, GetTime, InitAudioDevice, InitWindow, IsKeyDown, IsKeyPressed, MeasureText,
    SetTargetFPS, SetTraceLogLevel, Vector2, WindowShouldClose,
};
use raylib_ffi::{CloseWindow, SetMusicVolume};

#[derive(Clone, PartialEq)]
enum GameState {
    Running,
    GameOver,
    LevelUp,
    Paused,
    Quit,
}

pub struct Game {
    assets: Box<Assets>,
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

impl Drop for Game {
    fn drop(&mut self) {
        log::info("Game is dropping !!!");
        unsafe {
            CloseWindow();
            CloseAudioDevice();
        }
    }
}

impl Game {
    pub fn new() -> Self {
        unsafe {
            InitAudioDevice();
            InitWindow(WORLD_WIDTH, WORLD_HEIGHT, rl_str!(WINDOW_TITLE));
            SetTargetFPS(60);
            SetTraceLogLevel(TraceLogLevel::Error as i32);
        }

        let mut game = Game {
            assets: Box::new(Assets::new()),
            spaceship: Box::new(Spaceship::new()),
            lasers: Vec::new(),
            obstacles: Vec::new(),
            aliens: Vec::new(),
            aliens_direction: 1,
            alien_lasers: Vec::new(),
            time_alien_last_fired: 0.,
            mysteryship: Box::new(MysteryShip::new()),
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
        self.assets.play_music();
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
        // create the obstacles
        const GAP: usize =
            (WORLD_WIDTH as usize - (NUM_OBSTACLES * OBSTACLE_WIDTH)) / (NUM_OBSTACLES + 1);
        for i in 0..NUM_OBSTACLES {
            let offset_x = (i + 1) * GAP + i * OBSTACLE_WIDTH;
            const OFFSET_Y: usize = WORLD_HEIGHT as usize - OBSTACLE_PADDING - OFFSETY as usize;
            self.obstacles.push(Obstacle::new(offset_x, OFFSET_Y));
        }
    }

    pub fn create_aliens(&mut self) {
        for row in 0..ALIEN_ROWS {
            let alien_type = match row {
                0 => ALIEN3,
                1 | 2 => ALIEN2,
                _ => ALIEN1,
            };

            for col in 0..ALIEN_COLUMNS {
                let x = ALIEN_OFFSET_X + col * ALIEN_SIZE;
                let y = ALIEN_OFFSET_Y + row * ALIEN_SIZE;
                self.aliens.push(Alien::new(
                    alien_type,
                    Vector2 {
                        x: x as f32,
                        y: y as f32,
                    },
                ));
            }
        }
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
        unsafe {
            if WindowShouldClose() {
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

            // For debug purposes!!!
            if IsKeyPressed(KeyboardKey::G as c_int) {
                self.state = GameState::GameOver;
                log::info("GameOver invoked by keyboard!");
                return;
            }
            if IsKeyPressed(KeyboardKey::L as c_int) {
                self.state = GameState::LevelUp;
                log::info("LevelUp invoked by keyboard!");
                return;
            }

            // Handle movement and laser fire
            if self.state == GameState::Running {
                if IsKeyDown(KeyboardKey::Left as c_int) {
                    self.spaceship.move_left();
                } else if IsKeyDown(KeyboardKey::Right as c_int) {
                    self.spaceship.move_right();
                } else if IsKeyDown(KeyboardKey::Space as c_int) {
                    let laser = self.spaceship.fire_laser();
                    if laser.is_some() {
                        self.assets.play_laser_sound();
                        self.lasers.push(laser.unwrap());
                    }
                }
            }

            // Handle pause/resume
            if IsKeyPressed(KeyboardKey::P as c_int) {
                if self.state == GameState::Paused {
                    self.state = GameState::Running;
                } else if self.state == GameState::Running {
                    self.state = GameState::Paused;
                }
            }
        }
    }

    pub fn handle_game_over_input(&mut self) {
        unsafe {
            if IsKeyPressed(KeyboardKey::Escape as c_int) {
                self.state = GameState::Quit;
            }
            if IsKeyPressed(KeyboardKey::Enter as c_int) {
                self.reset_game();
                self.init_game();
            }
        }
    }

    pub fn handle_level_up_input(&mut self) {
        unsafe {
            if IsKeyPressed(KeyboardKey::Enter as c_int) {
                self.reset_game();
                self.init_level();
            }
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

    pub fn aliens_shoot_laser(&mut self) {
        let current_time = unsafe { GetTime() };
        if current_time - self.time_alien_last_fired >= ALIEN_LASER_INTERVAL
            && !self.aliens.is_empty()
        {
            let random_index: usize = rand::thread_rng().gen_range(0..self.aliens.len());
            let alien = &self.aliens[random_index];
            let laser_pos = alien.get_laser_position();
            self.alien_lasers
                .push(Laser::new(laser_pos, ALIEN_LASER_SPEED));
            self.time_alien_last_fired = unsafe { GetTime() };
        }
    }

    pub fn check_for_collisions(&mut self) -> bool {
        // ================
        // spaceship lasers
        // ================
        for laser in self.lasers.iter_mut() {
            // check against aliens
            for alien in self.aliens.iter_mut() {
                if alien.is_active()
                    && unsafe { CheckCollisionRecs(alien.get_rect(), laser.get_rect()) }
                {
                    self.score += alien.get_score();
                    alien.set_inactive();
                    laser.set_inactive();
                    self.assets.play_alien_explosion_sound();
                }
            }
            // check if obstacle is hit and damage it!
            for obstacle in self.obstacles.iter_mut() {
                for block in obstacle.blocks.iter_mut() {
                    if unsafe { CheckCollisionRecs(block.get_rect(), laser.get_rect()) } {
                        block.set_inactive();
                        laser.set_inactive();
                    }
                }
            }
            // check against mystery ship
            if unsafe { CheckCollisionRecs(self.mysteryship.get_rect(), laser.get_rect()) } {
                self.score += MYSTERYSHIP_SCORE;
                self.mysteryship.set_inactive();
                laser.set_inactive();
                self.assets.play_mystery_explosion_sound();
            }
            // check against alien lasers (yep, we can destroy alien lasers!)
            // T.B.D.
        }

        // ================
        // alien lasers
        // ================
        for laser in self.alien_lasers.iter_mut() {
            // check if spaceship is hit
            if unsafe { CheckCollisionRecs(laser.get_rect(), self.spaceship.get_rect()) } {
                self.assets.play_ship_explosion_sound();
                laser.set_inactive();
                self.lives -= 1;
                if self.lives == 0 {
                    self.state = GameState::GameOver;
                }
            }
            // check if obstacle is hit and damage it!
            for obstacle in self.obstacles.iter_mut() {
                for block in obstacle.blocks.iter_mut() {
                    if unsafe { CheckCollisionRecs(block.get_rect(), laser.get_rect()) } {
                        block.set_inactive();
                        laser.set_inactive();
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
                    if unsafe { CheckCollisionRecs(block.get_rect(), alien.get_rect()) } {
                        block.set_inactive();
                    }
                }
            }
            // alien collision with ship
            if unsafe { CheckCollisionRecs(self.spaceship.get_rect(), alien.get_rect()) } {
                return true;
            }
        }
        false
    }

    pub fn update(&mut self) {
        // do nothing if game is over
        if self.state != GameState::Running {
            return;
        }

        // Update the music
        self.assets.update_music();

        // Update the spaceship (currently does nothing)
        self.spaceship.update();

        // Update all spaceship lasers
        for laser in self.lasers.iter_mut() {
            laser.update();
        }

        // Remove all inactive spaceship lasers
        self.lasers.retain(|elem| elem.is_active());

        // Remove all inactive blocks
        for obstacle in self.obstacles.iter_mut() {
            obstacle.remove_inactive_blocks();
        }

        // Remove all inactive aliens
        self.aliens.retain(|elem| elem.is_active());
        if self.aliens.is_empty() {
            self.state = GameState::LevelUp;
        }

        // Update the aliens
        self.move_aliens();

        // Create alien lasers
        self.aliens_shoot_laser();

        // Update alien lasers
        for laser in self.alien_lasers.iter_mut() {
            laser.update();
        }

        // Remove all inactive alien lasers
        self.alien_lasers.retain(|elem| elem.is_active());

        // Update the mystery ship
        let current_time = unsafe { GetTime() };
        if current_time - self.time_last_spawned > self.mysteryship_spawn_interval {
            self.mysteryship.spawn();
            self.time_last_spawned = unsafe { GetTime() };
            self.mysteryship_spawn_interval =
                rand::thread_rng().gen_range(MYSTERYSHIP_MIN_INTERVAL..MYSTERYSHIP_MAX_INTERVAL)
        }

        if self.mysteryship.is_active() {
            self.assets.play_mystery_sound();
            self.mysteryship.update();
        }

        let done = self.check_for_collisions();
        self.check_for_highscore();
        if done {
            self.game_over();
        }
    }

    pub fn draw(&mut self) {
        unsafe {
            BeginDrawing();
            ClearBackground(WINDOW_BKG_COLOR);
            DrawRectangleRoundedLinesEx(
                FRAME_RECT,
                FRAME_ROUNDNESS,
                FRAME_SEGMENTS,
                FRAME_THICKNESS,
                FRAME_COLOR,
            );
            DrawLineEx(
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
                DrawTextEx(
                    self.assets.get_font(),
                    rl_str!(format!("LEVEL {:0>2}", self.level).as_str()),
                    LEVEL_POS,
                    FONT_SIZE as f32,
                    FONT_SPACING,
                    FRAME_COLOR,
                );
            } else {
                DrawTextEx(
                    self.assets.get_font(),
                    rl_str!("GAME OVER"),
                    LEVEL_POS,
                    FONT_SIZE as f32,
                    FONT_SPACING,
                    FRAME_COLOR,
                );
            }
            DrawTextEx(
                self.assets.get_font(),
                rl_str!("SCORE"),
                GUI_SCORE_TEXT_POS,
                FONT_SIZE as f32,
                FONT_SPACING,
                FRAME_COLOR,
            );
            DrawTextEx(
                self.assets.get_font(),
                rl_str!(format!("{:0>5}", self.score).as_str()),
                GUI_SCORE_VALUE_POS,
                FONT_SIZE as f32,
                FONT_SPACING,
                FRAME_COLOR,
            );
            DrawTextEx(
                self.assets.get_font(),
                rl_str!("HIGH SCORE"),
                GUI_HIGH_SCORE_TEXT_POS,
                FONT_SIZE as f32,
                FONT_SPACING,
                FRAME_COLOR,
            );
            DrawTextEx(
                self.assets.get_font(),
                rl_str!(format!("{:0>5}", self.high_score).as_str()),
                GUI_HIGH_SCORE_VALUE_POS,
                FONT_SIZE as f32,
                FONT_SPACING,
                FRAME_COLOR,
            );

            // DRAW OTHER OBJECTS

            let mut x = GUI_LIVEIMG_X;
            for _ in 0..self.lives {
                self.spaceship.draw_at(x, GUI_LIVEIMG_Y);
                x += GUI_LIVEIMG_INC;
            }

            for obstacle in self.obstacles.iter() {
                obstacle.draw();
            }
            self.spaceship.draw();
            for laser in self.lasers.iter_mut() {
                laser.draw();
            }

            for alien in self.aliens.iter() {
                alien.draw();
            }

            for laser in self.alien_lasers.iter_mut() {
                laser.draw();
            }

            self.mysteryship.draw();

            if self.state == GameState::GameOver {
                self.game_over_draw();
            }

            if self.state == GameState::LevelUp {
                self.level_up_draw();
            }

            EndDrawing();
        }
    }

    fn center_text_at(&mut self, posx: i32, posy: i32, width: i32, text: &str) {
        const YELLOW: Color = Color {
            r: 243,
            g: 216,
            b: 63,
            a: 255,
        };

        let text_width = unsafe { MeasureText(rl_str!(text), 34) };
        let newx = posx + (width - text_width) / 2;
        unsafe { DrawText(rl_str!(text), newx, posy, 34, YELLOW) };
    }

    fn draw_dialog_box(&mut self, text1: &str, text2: &str, text3: &str, color: Color) {
        const RWIDTH: i32 = 600;
        const RHEIGHT: i32 = 200;
        const RPOSX: i32 = (WORLD_WIDTH - RWIDTH) / 2;
        const RPOSY: i32 = 100;
        unsafe {
            DrawRectangleGradientH(RPOSX, RPOSY, RWIDTH, RHEIGHT, color, color);
            DrawRectangleLines(RPOSX, RPOSY, RWIDTH, RHEIGHT, color);
        }
        self.center_text_at(RPOSX, 150, RWIDTH, text1);
        self.center_text_at(RPOSX, 190, RWIDTH, text2);
        self.center_text_at(RPOSX, 230, RWIDTH, text3);
    }

    fn level_up_draw(&mut self) {
        self.draw_dialog_box(
            "CONGRATULATIONS",
            "YOU DEFEATED THE ALIENS",
            "PRESS ENTER FOR NEXT LEVEL",
            GREEN_COLOR,
        );
    }

    fn game_over_draw(&mut self) {
        self.draw_dialog_box(
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
