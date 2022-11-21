#[derive(Clone, Copy, Debug)]
pub enum Block {
    Air,
    Block { color: (f32, f32, f32) },
}
