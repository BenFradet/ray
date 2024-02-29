pub trait Drawable {
    fn draw(&self, frame: &mut [u8]) -> ();
}
