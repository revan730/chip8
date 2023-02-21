pub trait Drawable {
    fn draw(&mut self, x: u8, y: u8, set: u8) -> bool;
    fn cls(&mut self);
    fn present(&mut self);
}
