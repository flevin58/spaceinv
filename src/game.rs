use crate::alien::Alien;
use crate::constants::*;
use crate::mysteryship::MysteryShip;
use crate::spaceship::Spaceship;
use crate::{laser::Laser, obstacle::Obstacle};

use rand::Rng;

use raylib::{
    core::math::Vector2,
    ffi::{KeyboardKey::*, Rectangle},
    prelude::{RaylibDraw, RaylibDrawHandle},
    RaylibHandle, RaylibThread,
};

pub struct Game<'a> {
    spaceship: Spaceship,
    lasers: Vec<Laser>,
    obstacles: Vec<Obstacle<'a>>,
    aliens: Vec<Alien>,
    aliens_direction: i32,
    alien_lasers: Vec<Laser>,
    time_alien_last_fired: f64,
    mysteryship: MysteryShip,
    mysteryship_spawn_interval: f64,
    time_last_spawned: f64,
}

impl<'a> Game<'a> {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut game = Game {
            spaceship: Spaceship::new(rl, thread),
            lasers: Vec::new(),
            obstacles: Vec::new(),
            aliens: Vec::new(),
            aliens_direction: 1,
            alien_lasers: Vec::new(),
            time_alien_last_fired: 0.,
            mysteryship: MysteryShip::new(rl, thread),
            mysteryship_spawn_interval: rand::thread_rng()
                .gen_range(MYSTERYSHIP_MIN_INTERVAL..MYSTERYSHIP_MAX_INTERVAL),
            time_last_spawned: 0.,
        };

        // create the ostacles
        let gap = (rl.get_screen_width() as usize - (NUM_OBSTACLES * OBSTACLE_WIDTH))
            / (NUM_OBSTACLES + 1);
        for i in 0..NUM_OBSTACLES {
            let offset_x = (i + 1) * gap + i * OBSTACLE_WIDTH;
            let offset_y = rl.get_screen_height() as usize - OBSTACLE_PADDING;
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
                    rl,
                    thread,
                    alien_type,
                    Vector2::new(x as f32, y as f32),
                ));
            }
        }

        game
    }

    pub fn handle_input(&mut self, rl: &mut RaylibHandle) {
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

    pub fn check_for_collisions(&mut self, rl: &mut RaylibHandle) {
        // spaceship lasers
        for laser in self.lasers.iter_mut() {
            for alien in self.aliens.iter_mut() {
                unsafe {
                    if alien.is_alive()
                        && raylib::ffi::CheckCollisionRecs(alien.get_rect(), laser.get_rect())
                    {
                        alien.erase();
                        laser.erase();
                    }
                }
            }
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        // update the spaceship (currently does nothing)
        self.spaceship.update(rl);

        // update all spaceship lasers
        for laser in self.lasers.iter_mut() {
            laser.update(rl);
        }

        // remove all inactive spaceship lasers
        self.lasers.retain(|elem| elem.is_active());

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

        self.check_for_collisions(rl);
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.clear_background(WINDOW_BKG_COLOR);
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
}
