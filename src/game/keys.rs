use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum Key {
    Left,
    Right,
    RotateCW,
    RotateCCW,
    SoftDrop,
    HardDrop,
    Swap,
}

pub struct KeyBinds {
    keys: HashMap<glutin::event::VirtualKeyCode, Key>,
}

impl KeyBinds {
    pub fn single() -> Self {
        Self {
            keys: HashMap::from([
                (glutin::event::VirtualKeyCode::Left, Key::Left),
                (glutin::event::VirtualKeyCode::Right, Key::Right),
                (glutin::event::VirtualKeyCode::Up, Key::RotateCW),
                (glutin::event::VirtualKeyCode::LControl, Key::RotateCCW),
                (glutin::event::VirtualKeyCode::Down, Key::SoftDrop),
                (glutin::event::VirtualKeyCode::Space, Key::HardDrop),
                (glutin::event::VirtualKeyCode::LShift, Key::Swap),
            ]),
        }
    }

    pub fn left() -> Self {
        Self {
            keys: HashMap::from([
                (glutin::event::VirtualKeyCode::A, Key::Left),
                (glutin::event::VirtualKeyCode::D, Key::Right),
                (glutin::event::VirtualKeyCode::W, Key::RotateCW),
                (glutin::event::VirtualKeyCode::LControl, Key::RotateCCW),
                (glutin::event::VirtualKeyCode::S, Key::SoftDrop),
                (glutin::event::VirtualKeyCode::Space, Key::HardDrop),
                (glutin::event::VirtualKeyCode::LShift, Key::Swap),
            ]),
        }
    }

    pub fn right() -> Self {
        Self {
            keys: HashMap::from([
                (glutin::event::VirtualKeyCode::Numpad1, Key::Left),
                (glutin::event::VirtualKeyCode::Numpad3, Key::Right),
                (glutin::event::VirtualKeyCode::Numpad5, Key::RotateCW),
                (glutin::event::VirtualKeyCode::NumpadComma, Key::RotateCCW),
                (glutin::event::VirtualKeyCode::Numpad2, Key::SoftDrop),
                (glutin::event::VirtualKeyCode::NumpadEnter, Key::HardDrop),
                (glutin::event::VirtualKeyCode::Numpad0, Key::Swap),
            ]),
        }
    }

    pub fn decode(&self, k: glutin::event::VirtualKeyCode) -> Option<Key> {
        self.keys.get(&k).cloned()
    }
}
