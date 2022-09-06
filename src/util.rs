pub enum Screen {
    Menu,
    SingleGame,
    DoubleGame,
}

pub trait Playable {
    fn update(&mut self);
    fn draw(&mut self, screen_width: i32, screen_height: i32);
    fn input(&mut self, event: glutin::event::KeyboardInput);
    fn next_screen(&mut self) -> Option<Screen> {
        None
    }
}
