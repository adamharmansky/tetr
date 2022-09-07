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

#[derive(std::fmt::Debug)]
pub struct StringError(pub String);

impl std::error::Error for StringError {}
impl std::fmt::Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}
