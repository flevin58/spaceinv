use crate::constants::*;
use crate::context::*;
use raylib::prelude::*;
use std::rc::Rc;

pub struct Assets {
    font: Box<Font>,
    alien1_texture: Box<Texture2D>,
    alien2_texture: Box<Texture2D>,
    alien3_texture: Box<Texture2D>,
    mystery_texture: Box<Texture2D>,
    ship_texture: Box<Texture2D>,
}

impl Assets {
    pub fn new(ctx: Rc<Context>) -> Self {
        let mut rl = ctx.rl.borrow_mut();
        let thread = ctx.thread.borrow();

        // The font is embedded in the binary
        let font_data = include_bytes!("../assets/fonts/monogram.ttf");
        let font_res = rl.load_font_from_memory(&thread, ".ttf", font_data, FONT_SIZE, None);

        // The alien textures are embedded in the binary
        let alien1_data = include_bytes!("../assets/images/alien_1.png");
        let alien1_image = Image::load_image_from_mem(".png", alien1_data).unwrap();
        let alien1_texture = rl.load_texture_from_image(&thread, &alien1_image).unwrap();
        let alien2_data = include_bytes!("../assets/images/alien_2.png");
        let alien2_image = Image::load_image_from_mem(".png", alien2_data).unwrap();
        let alien2_texture = rl.load_texture_from_image(&thread, &alien2_image).unwrap();
        let alien3_data = include_bytes!("../assets/images/alien_3.png");
        let alien3_image = Image::load_image_from_mem(".png", alien3_data).unwrap();
        let alien3_texture = rl.load_texture_from_image(&thread, &alien3_image).unwrap();

        // The mystery ship texture is embedded in the binary
        let mystery_data = include_bytes!("../assets/images/mystery.png");
        let mystery_image = Image::load_image_from_mem(".png", mystery_data).unwrap();
        let mystery_texture = rl.load_texture_from_image(&thread, &mystery_image).unwrap();

        // The mystery ship texture is embedded in the binary
        let ship_data = include_bytes!("../assets/images/spaceship.png");
        let ship_image = Image::load_image_from_mem(".png", ship_data).unwrap();
        let ship_texture = rl.load_texture_from_image(&thread, &ship_image).unwrap();

        Assets {
            font: Box::new(font_res.unwrap()),
            alien1_texture: Box::new(alien1_texture),
            alien2_texture: Box::new(alien2_texture),
            alien3_texture: Box::new(alien3_texture),
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

impl Drop for Assets {
    fn drop(&mut self) {
        todo!()
    }
}
