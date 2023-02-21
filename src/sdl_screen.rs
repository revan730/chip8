use crate::drawable::Drawable;
use crate::constants::{SCREEN_WIDTH, SCREEN_HEIGHT};


use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;

pub struct SDLScreen {
    canvas: Canvas<Window>,
    framebuffer: [[u8; SCREEN_HEIGHT]; SCREEN_WIDTH],
}

impl Drawable for SDLScreen {
    fn draw(&mut self, x: u8, y: u8, set: u8) -> bool { // TODO: Refactor u8 -> usize
        let collision = self.framebuffer[x as usize][y as usize] & set;
        self.framebuffer[x as usize][y as usize] ^= set;

        collision > 0
    }

    fn cls(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    fn present(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        // TODO: For future - instead of u8 2d array Point array can be used for better performance ???

        let mut points_set: Vec<Point> = Vec::with_capacity(SCREEN_WIDTH * SCREEN_HEIGHT);

        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                if self.framebuffer[x][y] == 1 {
                    points_set.push(Point::new(x as i32, y as i32));
                }
            }
        }

        self.canvas.set_draw_color(Color::RGB(0, 255, 0));
        self.canvas.draw_points(&points_set[..]).unwrap();

        self.canvas.present();
    }
}

impl SDLScreen {
    pub fn new(canvas: Canvas<Window>) -> SDLScreen {
        SDLScreen { canvas, framebuffer: [[0; SCREEN_HEIGHT]; SCREEN_WIDTH] }
    }
}