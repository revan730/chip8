use std::cell::{RefCell, Ref};

use crate::drawable::Drawable;
use crate::constants::{SCREEN_WIDTH, SCREEN_HEIGHT};


use sdl2::rect::Point;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::pixels::Color;

pub struct SDLScreen {
    canvas: Canvas<Window>,
    texture: RefCell<Texture<'static>>,
    framebuffer: [[u8; SCREEN_HEIGHT]; SCREEN_WIDTH],
    update_needed: bool,
}

impl Drawable for SDLScreen {
    fn draw(&mut self, x: u8, y: u8, set: u8) -> bool {
        // Using preallocated texture we update it on each DRW opcode so we don't waste time on it
        // on actual GPU draw
        let collision = self.framebuffer[x as usize][y as usize] & set;
        self.framebuffer[x as usize][y as usize] ^= set;

        let texture = self.texture.get_mut();
        self.canvas.with_texture_canvas(texture, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
            texture_canvas.clear();
            texture_canvas.set_draw_color(Color::RGB(0, 255, 0));
            for x in 0..SCREEN_WIDTH {
                for y in 0..SCREEN_HEIGHT {
                    if self.framebuffer[x][y] == 1 {
                        texture_canvas.draw_point(Point::new(x as i32, y as i32));
                    }
                }
            }
        });

        self.update_needed = true;


        collision > 0
    }

    fn cls(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    fn present(&mut self) {
        if self.update_needed {
            self.canvas.copy(self.texture.get_mut(), None, None);
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

        SDLScreen { canvas, texture: RefCell::new(texture), framebuffer: [[0; SCREEN_HEIGHT]; SCREEN_WIDTH], update_needed: false }
    }
}