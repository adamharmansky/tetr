use crate::game::tetromino::*;
use rand::prelude::*;

pub struct PieceGenerator {
    pub queue: std::collections::VecDeque<Shape>,
    pack: Vec<Shape>,
    rng: SmallRng,
}

impl PieceGenerator {
    pub fn new(mut rng: rand::rngs::SmallRng) -> Self {
        let mut pack = Self::pack(&mut rng);
        let mut queue = std::collections::VecDeque::<Shape>::new();
        for _ in 0..5 {
            queue.push_back(pack.pop().unwrap());
        }
        Self { pack, queue, rng }
    }

    fn pack(rng: &mut SmallRng) -> Vec<Shape> {
        let mut orig = vec![
            Shape::I,
            Shape::J,
            Shape::L,
            Shape::O,
            Shape::S,
            Shape::T,
            Shape::Z,
        ];
        let mut new = Vec::<Shape>::new();
        for i in (1..8).rev() {
            new.push(orig.swap_remove((rng.next_u32() % i) as usize));
        }
        new
    }

    pub fn next_piece(&mut self) -> Shape {
        self.queue.push_back(self.pack.pop().unwrap());
        if self.pack.len() == 0 {
            self.pack = Self::pack(&mut self.rng)
        }
        self.queue.pop_front().unwrap()
    }
}
