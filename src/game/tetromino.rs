use super::board::PlayingField;
use super::Block;
use crate::game::util::BlockPos;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Shape {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

#[derive(Debug, Clone)]
pub struct Tetromino {
    /// Position of the top left corner
    pub position: BlockPos,
    pub shape: Shape,
    /// Every piece has 4 rotation states.
    /// Rotations go clockwise from the start state
    rotation_state: u8,
}

/// Generates a 2D 4x4 [x][y] shape from an array of chars.
/// Also inverts `y` so that the blocks look natually on screen.
macro_rules! block_shape {
    ($c:expr, [$([$($x:expr),*]),*]) => {{
        let mut data = [[Block::Air;4];4];
        let mut x = 0;
        let mut y = 0;
        $($(if $x == '#' { data[x][3-y] = Block::Block{color: $c}; }; x += 1;)* {&x}; y += 1; x = 0; )*
        // remove annoyting "unused assignment" warnings
        {&x};
        {&y};
        data
    }}
}

impl Tetromino {
    pub fn new(shape: Shape) -> Self {
        Self {
            position: BlockPos::new(3, 20),
            shape,
            rotation_state: 0,
        }
    }

    /// Attempt to move the block in a given direction
    ///
    /// # Return value
    ///
    /// Returns `true` when the translation was unsuccessful
    pub fn translate(&mut self, direction: BlockPos, game: &PlayingField) -> bool {
        self.position += direction;
        if self.obstructed(game) {
            self.position -= direction;
            true
        } else {
            false
        }
    }

    /// Test whether the block intersects with something on the playing field
    pub fn obstructed(&self, game: &PlayingField) -> bool {
        let shape = self.get_shape();
        for x in 0..4 {
            for y in 0..4 {
                if let Block::Block { .. } = shape[x][y] {
                    if x as i32 + self.position.x >= 0
                        && x as i32 + self.position.x < 10
                        && y as i32 + self.position.y >= 0
                    {
                        if let Block::Block { .. } = game[(self.position.y + y as i32) as usize]
                            [(self.position.x + x as i32) as usize]
                        {
                            // We intersect with a block
                            return true;
                        }
                    } else {
                        // We are outside of the playing field
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Attempt to rotate the block clockwise
    ///
    /// # Return value
    ///
    /// Returns `true` when the rotation was unsuccessful
    pub fn rotate_cw(&mut self, game: &PlayingField) -> bool {
        self.rotation_state += 1;
        self.rotation_state %= 4;

        // test default rotation
        if !self.obstructed(game) {
            return false;
        }

        for i in match self.shape {
            Shape::J | Shape::L | Shape::S | Shape::T | Shape::Z => match self.rotation_state {
                0 => [
                    BlockPos::new(-1, 0),
                    BlockPos::new(-1, -1),
                    BlockPos::new(0, 2),
                    BlockPos::new(-1, 2),
                ],
                1 => [
                    BlockPos::new(-1, 0),
                    BlockPos::new(-1, 1),
                    BlockPos::new(0, -2),
                    BlockPos::new(-1, -2),
                ],
                2 => [
                    BlockPos::new(1, 0),
                    BlockPos::new(1, -1),
                    BlockPos::new(0, 2),
                    BlockPos::new(1, 2),
                ],
                3 => [
                    BlockPos::new(1, 0),
                    BlockPos::new(1, 1),
                    BlockPos::new(0, -2),
                    BlockPos::new(1, -2),
                ],
                _ => panic!(),
            },
            Shape::I => match self.rotation_state {
                0 => [
                    BlockPos::new(1, 0),
                    BlockPos::new(-2, 0),
                    BlockPos::new(1, -2),
                    BlockPos::new(-2, 1),
                ],
                1 => [
                    BlockPos::new(-2, 0),
                    BlockPos::new(1, 0),
                    BlockPos::new(-2, -1),
                    BlockPos::new(1, 2),
                ],
                2 => [
                    BlockPos::new(-1, 0),
                    BlockPos::new(2, 0),
                    BlockPos::new(-1, 2),
                    BlockPos::new(2, -1),
                ],
                3 => [
                    BlockPos::new(2, 0),
                    BlockPos::new(-1, 0),
                    BlockPos::new(2, 1),
                    BlockPos::new(-1, -2),
                ],
                _ => panic!(),
            },
            Shape::O => [
                BlockPos::new(0, 0),
                BlockPos::new(0, 0),
                BlockPos::new(0, 0),
                BlockPos::new(0, 0),
            ],
        } {
            if !self.translate(i, game) {
                return false;
            }
        }
        self.rotation_state += 3;
        self.rotation_state %= 4;
        true
    }

    /// Attempt to rotate the block counter-clockwise
    ///
    /// # Return value
    ///
    /// Returns `true` when the rotation was unsuccessful
    pub fn rotate_ccw(&mut self, game: &PlayingField) -> bool {
        self.rotation_state += 3;
        self.rotation_state %= 4;

        // test default rotation
        if !self.obstructed(game) {
            return false;
        }

        for i in match self.shape {
            Shape::J | Shape::L | Shape::S | Shape::T | Shape::Z => match self.rotation_state {
                0 => [
                    BlockPos::new(1, 0),
                    BlockPos::new(1, -1),
                    BlockPos::new(0, 2),
                    BlockPos::new(1, 2),
                ],
                1 => [
                    BlockPos::new(-1, 0),
                    BlockPos::new(-1, 1),
                    BlockPos::new(0, -2),
                    BlockPos::new(-1, -2),
                ],
                2 => [
                    BlockPos::new(-1, 0),
                    BlockPos::new(-1, -1),
                    BlockPos::new(0, 2),
                    BlockPos::new(-1, 2),
                ],
                3 => [
                    BlockPos::new(1, 0),
                    BlockPos::new(1, 1),
                    BlockPos::new(0, -2),
                    BlockPos::new(1, -2),
                ],
                _ => panic!(),
            },
            Shape::I => match self.rotation_state {
                0 => [
                    BlockPos::new(2, 0),
                    BlockPos::new(-1, 0),
                    BlockPos::new(2, 1),
                    BlockPos::new(-1, -2),
                ],
                1 => [
                    BlockPos::new(1, 0),
                    BlockPos::new(-2, 0),
                    BlockPos::new(1, -2),
                    BlockPos::new(-2, 1),
                ],
                2 => [
                    BlockPos::new(-2, 0),
                    BlockPos::new(1, 0),
                    BlockPos::new(-2, 1),
                    BlockPos::new(1, 2),
                ],
                3 => [
                    BlockPos::new(-1, 0),
                    BlockPos::new(2, 0),
                    BlockPos::new(-1, 2),
                    BlockPos::new(2, -1),
                ],
                _ => panic!(),
            },
            Shape::O => [
                BlockPos::new(0, 0),
                BlockPos::new(0, 0),
                BlockPos::new(0, 0),
                BlockPos::new(0, 0),
            ],
        } {
            if !self.translate(i, game) {
                return false;
            }
        }
        self.rotation_state += 1;
        self.rotation_state %= 4;
        true
    }

    const COLORS: [(f32, f32, f32); 7] = [
        (0.0, 1.0, 1.0), // I
        (0.0, 0.0, 1.0), // J
        (1.0, 0.5, 0.0), // L
        (1.0, 1.0, 0.0), // O
        (0.0, 1.0, 0.0), // S
        (1.0, 0.0, 1.0), // T
        (1.0, 0.0, 0.0), // Z
    ];

    /// Generate the shape of the tetromino out of blocks
    pub fn get_shape(&self) -> [[Block; 4]; 4] {
        match self.shape {
            Shape::I => match self.rotation_state {
                0 => {
                    block_shape!(
                        Self::COLORS[0],
                        [
                            [' ', ' ', ' ', ' '],
                            ['#', '#', '#', '#'],
                            [' ', ' ', ' ', ' '],
                            [' ', ' ', ' ', ' ']
                        ]
                    )
                }
                1 => {
                    block_shape!(
                        Self::COLORS[0],
                        [
                            [' ', ' ', '#', ' '],
                            [' ', ' ', '#', ' '],
                            [' ', ' ', '#', ' '],
                            [' ', ' ', '#', ' ']
                        ]
                    )
                }
                2 => {
                    block_shape!(
                        Self::COLORS[0],
                        [
                            [' ', ' ', ' ', ' '],
                            [' ', ' ', ' ', ' '],
                            ['#', '#', '#', '#'],
                            [' ', ' ', ' ', ' ']
                        ]
                    )
                }
                3 => {
                    block_shape!(
                        Self::COLORS[0],
                        [
                            [' ', '#', ' ', ' '],
                            [' ', '#', ' ', ' '],
                            [' ', '#', ' ', ' '],
                            [' ', '#', ' ', ' ']
                        ]
                    )
                }
                _ => panic!(),
            },
            Shape::J => match self.rotation_state {
                0 => {
                    block_shape!(
                        Self::COLORS[1],
                        [
                            ['#', ' ', ' ', ' '],
                            ['#', '#', '#', ' '],
                            [' ', ' ', ' ', ' '],
                            [' ', ' ', ' ', ' ']
                        ]
                    )
                }
                1 => {
                    block_shape!(
                        Self::COLORS[1],
                        [
                            [' ', '#', '#', ' '],
                            [' ', '#', ' ', ' '],
                            [' ', '#', ' ', ' '],
                            [' ', ' ', ' ', ' ']
                        ]
                    )
                }
                2 => {
                    block_shape!(
                        Self::COLORS[1],
                        [
                            [' ', ' ', ' ', ' '],
                            ['#', '#', '#', ' '],
                            [' ', ' ', '#', ' '],
                            [' ', ' ', ' ', ' ']
                        ]
                    )
                }
                3 => {
                    block_shape!(
                        Self::COLORS[1],
                        [
                            [' ', '#', ' ', ' '],
                            [' ', '#', ' ', ' '],
                            ['#', '#', ' ', ' '],
                            [' ', ' ', ' ', ' ']
                        ]
                    )
                }
                _ => panic!(),
            },
            Shape::L => match self.rotation_state {
                0 => {
                    block_shape!(
                        Self::COLORS[2],
                        [
                            [' ', ' ', '#', ' '],
                            ['#', '#', '#', ' '],
                            [' ', ' ', ' ', ' '],
                            [' ', ' ', ' ', ' ']
                        ]
                    )
                }
                1 => {
                    block_shape!(
                        Self::COLORS[2],
                        [
                            [' ', '#', ' ', ' '],
                            [' ', '#', ' ', ' '],
                            [' ', '#', '#', ' '],
                            [' ', ' ', ' ', ' ']
                        ]
                    )
                }
                2 => {
                    block_shape!(
                        Self::COLORS[2],
                        [
                            [' ', ' ', ' ', ' '],
                            ['#', '#', '#', ' '],
                            ['#', ' ', ' ', ' '],
                            [' ', ' ', ' ', ' ']
                        ]
                    )
                }
                3 => {
                    block_shape!(
                        Self::COLORS[2],
                        [
                            ['#', '#', ' ', ' '],
                            [' ', '#', ' ', ' '],
                            [' ', '#', ' ', ' '],
                            [' ', ' ', ' ', ' ']
                        ]
                    )
                }
                _ => panic!(),
            },
            Shape::O => match self.rotation_state {
                0 => {
                    block_shape!(
                        Self::COLORS[3],
                        [
                            [' ', '#', '#', ' '],
                            [' ', '#', '#', ' '],
                            [' ', ' ', ' ', ' '],
                            [' ', ' ', ' ', ' ']
                        ]
                    )
                }
                1 => {
                    block_shape!(
                        Self::COLORS[3],
                        [
                            [' ', '#', '#', ' '],
                            [' ', '#', '#', ' '],
                            [' ', ' ', ' ', ' '],
                            [' ', ' ', ' ', ' ']
                        ]
                    )
                }
                2 => {
                    block_shape!(
                        Self::COLORS[3],
                        [
                            [' ', '#', '#', ' '],
                            [' ', '#', '#', ' '],
                            [' ', ' ', ' ', ' '],
                            [' ', ' ', ' ', ' ']
                        ]
                    )
                }
                3 => {
                    block_shape!(
                        Self::COLORS[3],
                        [
                            [' ', '#', '#', ' '],
                            [' ', '#', '#', ' '],
                            [' ', ' ', ' ', ' '],
                            [' ', ' ', ' ', ' ']
                        ]
                    )
                }
                _ => panic!(),
            },
            Shape::S => match self.rotation_state {
                0 => {
                    block_shape!(
                        Self::COLORS[4],
                        [
                            [' ', '#', '#', ' '],
                            ['#', '#', ' ', ' '],
                            [' ', ' ', ' ', ' '],
                            [' ', ' ', ' ', ' ']
                        ]
                    )
                }
                1 => {
                    block_shape!(
                        Self::COLORS[4],
                        [
                            [' ', '#', ' ', ' '],
                            [' ', '#', '#', ' '],
                            [' ', ' ', '#', ' '],
                            [' ', ' ', ' ', ' ']
                        ]
                    )
                }
                2 => {
                    block_shape!(
                        Self::COLORS[4],
                        [
                            [' ', ' ', ' ', ' '],
                            [' ', '#', '#', ' '],
                            ['#', '#', ' ', ' '],
                            [' ', ' ', ' ', ' ']
                        ]
                    )
                }
                3 => {
                    block_shape!(
                        Self::COLORS[4],
                        [
                            ['#', ' ', ' ', ' '],
                            ['#', '#', ' ', ' '],
                            [' ', '#', ' ', ' '],
                            [' ', ' ', ' ', ' ']
                        ]
                    )
                }
                _ => panic!(),
            },
            Shape::T => match self.rotation_state {
                0 => {
                    block_shape!(
                        Self::COLORS[5],
                        [
                            [' ', '#', ' ', ' '],
                            ['#', '#', '#', ' '],
                            [' ', ' ', ' ', ' '],
                            [' ', ' ', ' ', ' ']
                        ]
                    )
                }
                1 => {
                    block_shape!(
                        Self::COLORS[5],
                        [
                            [' ', '#', ' ', ' '],
                            [' ', '#', '#', ' '],
                            [' ', '#', ' ', ' '],
                            [' ', ' ', ' ', ' ']
                        ]
                    )
                }
                2 => {
                    block_shape!(
                        Self::COLORS[5],
                        [
                            [' ', ' ', ' ', ' '],
                            ['#', '#', '#', ' '],
                            [' ', '#', ' ', ' '],
                            [' ', ' ', ' ', ' ']
                        ]
                    )
                }
                3 => {
                    block_shape!(
                        Self::COLORS[5],
                        [
                            [' ', '#', ' ', ' '],
                            ['#', '#', ' ', ' '],
                            [' ', '#', ' ', ' '],
                            [' ', ' ', ' ', ' ']
                        ]
                    )
                }
                _ => panic!(),
            },
            Shape::Z => match self.rotation_state {
                0 => {
                    block_shape!(
                        Self::COLORS[6],
                        [
                            ['#', '#', ' ', ' '],
                            [' ', '#', '#', ' '],
                            [' ', ' ', ' ', ' '],
                            [' ', ' ', ' ', ' ']
                        ]
                    )
                }
                1 => {
                    block_shape!(
                        Self::COLORS[6],
                        [
                            [' ', ' ', '#', ' '],
                            [' ', '#', '#', ' '],
                            [' ', '#', ' ', ' '],
                            [' ', ' ', ' ', ' ']
                        ]
                    )
                }
                2 => {
                    block_shape!(
                        Self::COLORS[6],
                        [
                            [' ', ' ', ' ', ' '],
                            ['#', '#', ' ', ' '],
                            [' ', '#', '#', ' '],
                            [' ', ' ', ' ', ' ']
                        ]
                    )
                }
                3 => {
                    block_shape!(
                        Self::COLORS[6],
                        [
                            [' ', '#', ' ', ' '],
                            ['#', '#', ' ', ' '],
                            ['#', ' ', ' ', ' '],
                            [' ', ' ', ' ', ' ']
                        ]
                    )
                }
                _ => panic!(),
            },
        }
    }
}
