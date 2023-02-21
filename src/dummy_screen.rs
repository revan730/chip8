use crate::drawable::Drawable;

pub struct DummyScreen {}

impl Drawable for DummyScreen {
    fn draw(&mut self, x: u8, y: u8, set: u8) -> bool {
        println!("Drawing pixel at {} {}, value {}", x, y, set);

        false
    }

    fn cls(&mut self) {
        println!("Clearing screen");
    }

    fn present(&mut self) {
        // Left for compatibility
    }
}
