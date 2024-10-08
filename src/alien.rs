use raylib::{
    color::Color,
    core::math::Vector2,
    ffi::Rectangle,
    misc::AsF32,
    prelude::{RaylibDraw, RaylibDrawHandle},
    RaylibHandle,
};

use crate::{assets::Assets, constants::*};

pub struct Alien<'a> {
    assets: &'a Assets,
    kind: usize,
    position: Vector2,
    active: bool,
    score: usize,
}

impl<'a> Alien<'a> {
    pub fn new(assets: &'a Assets, kind: usize, position: Vector2) -> Self {
        Alien {
            assets,
            kind,
            position,
            active: true,
            score: ALIEN_SCORES[kind - 1],
        }
    }

    pub fn get_score(&self) -> usize {
        self.score
    }

    pub fn has_overflowed_right(&self, rl: &RaylibHandle) -> bool {
        //if self.position.x as i32 + self.image.width > rl.get_screen_width() - OFFSETX / 2 {
        if self.position.x as i32 + self.assets.get_alien_texture(self.kind).width
            > rl.get_screen_width() - OFFSETX / 2
        {
            true
        } else {
            false
        }
    }

    pub fn has_overflowed_left(&self) -> bool {
        if (self.position.x as i32) < OFFSETX / 2 {
            true
        } else {
            false
        }
    }

    pub fn get_laser_position(&self) -> Vector2 {
        let width = self.assets.get_alien_texture(self.kind).width.as_f32();
        let height = self.assets.get_alien_texture(self.kind).height.as_f32();
        let laser_x = self.position.x + width / 2.;
        let laser_y = self.position.y + height;
        Vector2::new(laser_x, laser_y)
    }

    pub fn move_down(&mut self, distance: usize) {
        self.position.y += distance as f32;
    }

    pub fn set_inactive(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn update(&mut self, _rl: &mut RaylibHandle, direction: i32) {
        self.position.x += direction.as_f32();
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture_v(
            &self.assets.get_alien_texture(self.kind),
            self.position,
            Color::WHITE,
        );
    }

    pub fn get_rect(&self) -> Rectangle {
        Rectangle {
            x: self.position.x,
            y: self.position.y,
            width: self.assets.get_alien_texture(self.kind).width.as_f32(),
            height: self.assets.get_alien_texture(self.kind).height.as_f32(),
        }
    }
}
