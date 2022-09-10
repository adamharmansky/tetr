use glam::Mat4;
use glam::Vec3;
use rand::prelude::*;
use std::rc::Rc;

mod background;
mod block;
mod board;
mod keys;
mod tetromino;
mod util;

use block::Block;
use board::Board;
use tetromino::Tetromino;
use util::BlockPos;

use std::cell::RefCell;

#[derive(Clone, Copy)]
pub enum KeyTiming {
    /// The key always applies (like soft drop) but doesn't repeat
    None,
    Delayed(std::time::Instant),
    Repeat(std::time::Instant),
    /// The key applies on key press, and it's been applied
    Single,
}

pub struct Game {
    boards: Vec<Rc<RefCell<Board>>>,
    renderer: board::Renderer,
    keys_pressed: std::collections::HashMap<glutin::event::VirtualKeyCode, KeyTiming>,
    exiting: bool,
    background: background::Background,
}

pub enum GameMode {
    Single,
    Double,
}

impl Game {
    pub fn new(
        gh: &mut crate::graphics::GraphicsHandle,
        roman: &crate::resource::ResourceManager,
        tr: Rc<crate::text::TextRenderer>,
        mode: GameMode,
    ) -> Self {
        let rng = SmallRng::from_entropy();
        let boards = match mode {
            GameMode::Double => {
                let left = Rc::new(RefCell::new(Board::new(
                    keys::KeyBinds::left(),
                    rng.clone(),
                )));
                let right = Rc::new(RefCell::new(Board::new(keys::KeyBinds::right(), rng)));
                {
                    left.borrow_mut().victim = Some(right.clone());
                    right.borrow_mut().victim = Some(left.clone());
                }
                vec![left, right]
            }
            GameMode::Single => vec![Rc::new(RefCell::new(Board::new(
                keys::KeyBinds::single(),
                rng,
            )))],
        };
        Self {
            renderer: board::Renderer::new(gh, roman, tr),
            boards,
            keys_pressed: std::collections::HashMap::new(),
            exiting: false,
            background: background::Background::new(gh, roman),
        }
    }
}

impl crate::Playable for Game {
    fn draw(
        &mut self,
        gh: &mut crate::graphics::GraphicsHandle,
        screen_width: i32,
        screen_height: i32,
    ) {
        let aspect = screen_width as f32 / screen_height as f32;
        let mat = Mat4::from_scale(Vec3::new(1.0 / aspect, 1.0, 1.0))
            * Mat4::from_scale(Vec3::new(0.75, 0.75, 0.75));

        self.background.draw(gh, screen_width, screen_height);

        for i in 0..self.boards.len() {
            let mat =
                mat * Mat4::from_translation(Vec3::new(
                    0.6 - 2.2 * 0.5 * self.boards.len() as f32 + 2.2 * i as f32,
                    -1.0,
                    0.0,
                )) * Mat4::from_scale(Vec3::new(0.1, 0.1, 0.1));

            self.renderer.draw(gh, &self.boards[i].borrow(), mat);
        }

        // self.board.draw(gl, mat);
    }

    fn update(&mut self) {
        for i in &mut self.boards {
            i.borrow_mut().update(&mut self.keys_pressed);
        }
    }

    fn input(&mut self, input: glutin::event::KeyboardInput) {
        match input.state {
            glutin::event::ElementState::Pressed => {
                if let Some(x) = input.virtual_keycode {
                    if let glutin::event::VirtualKeyCode::Escape = x {
                        self.exiting = true
                    }
                    if !self.keys_pressed.contains_key(&x) {
                        self.keys_pressed.insert(x, KeyTiming::None);
                    }
                }
            }
            glutin::event::ElementState::Released => {
                if let Some(x) = input.virtual_keycode {
                    self.keys_pressed.remove(&x);
                }
            }
        }
    }

    fn next_screen(&mut self) -> Option<crate::Screen> {
        if self.exiting {
            return Some(crate::Screen::Menu);
        }
        for i in &mut self.boards {
            if let Some(x) = i.borrow().death_time {
                if std::time::Instant::now().duration_since(x)
                    > std::time::Duration::from_millis(1000)
                {
                    return Some(crate::Screen::Menu);
                }
            }
        }
        None
    }
}
