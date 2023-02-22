use std::cell::{RefCell};

use crate::drawable::Drawable;
use crate::constants::{SCREEN_WIDTH, SCREEN_HEIGHT, PIXEL_COLOR};


use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

pub struct SDLScreen {
    canvas: Canvas<Window>,
    texture: RefCell<Texture<'static>>,
    fb: [u32; SCREEN_HEIGHT * SCREEN_WIDTH],
    update_needed: bool,
}

impl Drawable for SDLScreen {
    fn draw(&mut self, x: u8, y: u8, set: u8) -> bool {
        // Using preallocated texture we update it on each DRW opcode so we don't waste time on it
        // on actual GPU draw

        let mut pixel_value: u32 = 0;
        if set > 0 {
            pixel_value = PIXEL_COLOR;
        }

        let collision = self.fb[(y as usize * SCREEN_WIDTH) + x as usize] & pixel_value;
        self.fb[(y as usize * SCREEN_WIDTH) + x as usize] ^= pixel_value;

        let texture = self.texture.get_mut();

        let raw_data = unsafe {
            std::slice::from_raw_parts(self.fb.as_ptr() as *const u8, self.fb.len() * 4)
        };

        texture.update(None, raw_data, SCREEN_WIDTH * 4).unwrap();

        self.update_needed = true;


        collision > 0
    }

    fn cls(&mut self) {
        self.fb.iter_mut().for_each(|m| *m = 0);
        let texture = self.texture.get_mut();

        let raw_data = unsafe {
            std::slice::from_raw_parts(self.fb.as_ptr() as *const u8, self.fb.len() * 4)
        };

        texture.update(None, raw_data, SCREEN_WIDTH * 4).unwrap();

        self.update_needed = true;
    }

    fn present(&mut self) {
        if self.update_needed {
            self.canvas.copy(self.texture.get_mut(), None, None).unwrap();
            self.canvas.present();
            self.update_needed = false;
        }
    }
}

impl SDLScreen {
    pub fn new(canvas: Canvas<Window>) -> SDLScreen {
        let texture_creator = canvas.texture_creator();
        let texture =  texture_creator.create_texture_target(canvas.texture_creator().default_pixel_format(), SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .unwrap();

        let texture = unsafe{
            std::mem::transmute::<_, Texture<'static>>(texture)
        };

        SDLScreen { canvas, texture: RefCell::new(texture), fb: [0; SCREEN_HEIGHT * SCREEN_WIDTH], update_needed: false }
    }
}