pub enum Screen {
    Menu,
    SingleGame,
    DoubleGame,
    Exit,
}

pub trait Playable {
    fn update(&mut self);
    fn draw(
        &mut self,
        gh: &mut crate::graphics::GraphicsHandle,
        screen_width: i32,
        screen_height: i32,
    );
    fn input(&mut self, event: glutin::event::KeyboardInput);
    fn next_screen(&mut self) -> Option<Screen> {
        None
    }
}
