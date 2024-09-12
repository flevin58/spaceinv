use crate::constants::*;
use raylib::prelude::*;

pub struct Assets {
    font: Box<Font>,
    alien1_texture: Box<Texture2D>,
    alien2_texture: Box<Texture2D>,
    alien3_texture: Box<Texture2D>,
    mystery_texture: Box<Texture2D>,
    ship_texture: Box<Texture2D>,
}

impl Assets {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let font_data = include_bytes!("../assets/fonts/monogram.ttf");
        let font_res = rl.load_font_from_memory(thread, ".ttf", font_data, FONT_SIZE, None);

        let texture1 = rl.load_texture(&thread, ALIEN_TEXTURES[0]).unwrap();
        let texture2 = rl.load_texture(&thread, ALIEN_TEXTURES[1]).unwrap();
        let texture3 = rl.load_texture(&thread, ALIEN_TEXTURES[2]).unwrap();
        let mystery_texture = rl.load_texture(&thread, MYSTERYSHIP_TEXTURE).unwrap();
        let ship_texture = rl.load_texture(&thread, SPACESHIP_TEXTURE).unwrap();

        Assets {
            font: Box::new(font_res.unwrap()),
            alien1_texture: Box::new(texture1),
            alien2_texture: Box::new(texture2),
            alien3_texture: Box::new(texture3),
            mystery_texture: Box::new(mystery_texture),
            ship_texture: Box::new(ship_texture),
        }
    }

    pub fn get_font(&self) -> &Font {
        self.font.as_ref()
    }

    pub fn get_alien_texture(&self, alien_type: usize) -> &Texture2D {
        match alien_type {
            1 => self.alien1_texture.as_ref(),
            2 => self.alien2_texture.as_ref(),
            _ => self.alien3_texture.as_ref(),
        }
    }

    pub fn get_mystery_texture(&self) -> &Texture2D {
        self.mystery_texture.as_ref()
    }

    pub fn get_ship_texture(&self) -> &Texture2D {
        self.ship_texture.as_ref()
    }

    pub fn play_explosion_sound(&self) {
        // T.B.D.
    }
}
