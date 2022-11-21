use super::*;
use std::cell::RefCell;

mod effects;
mod piece_generator;
mod renderer;
mod score;

use effects::BoardEffects;
use piece_generator::PieceGenerator;
pub use renderer::Renderer;
pub use score::ScoreHandler;

pub type PlayingField = std::collections::VecDeque<Box<[Block; 10]>>;

pub struct Board {
    blocks: PlayingField,

    keybinds: keys::KeyBinds,

    piece_generator: PieceGenerator,
    falling_piece: Tetromino,
    swap_piece: Option<tetromino::Shape>,
    ghost_piece: Tetromino,

    /// Whether the piece has already been swapped in this move
    swapped: bool,

    last_update_time: std::time::Instant,
    ground_time: std::time::Instant,
    moves_on_ground: u32,
    on_ground: bool,

    /// * Some(x) => dead for x amount of time
    /// * None => alive
    pub death_time: Option<std::time::Instant>,

    pub victim: Option<Rc<RefCell<Board>>>,
    pub lines_received: std::collections::VecDeque<u32>,

    effects: BoardEffects,
    score: ScoreHandler,

    audio: Rc<RefCell<kira::manager::AudioManager>>,

    clear_sound: kira::sound::static_sound::StaticSoundData,
    drop_sound: kira::sound::static_sound::StaticSoundData,
}

impl Board {
    pub fn new(
        keybinds: keys::KeyBinds,
        rng: rand::rngs::SmallRng,
        audio: Rc<RefCell<kira::manager::AudioManager>>,
        roman: &crate::resource::ResourceManager,
    ) -> Self {
        let mut blocks = PlayingField::new();
        for _ in 0..32 {
            blocks.push_back(Box::new([Block::Air; 10]));
        }

        let mut piece_factory = PieceGenerator::new(rng);
        let falling_piece = Tetromino::new(piece_factory.next_piece());
        let ghost_piece = falling_piece.clone();

        let clear_sound = kira::sound::static_sound::StaticSoundData::from_cursor(
            std::io::Cursor::new((*roman.get_binary("clear.wav")).clone()),
            kira::sound::static_sound::StaticSoundSettings::default(),
        )
        .unwrap();

        let drop_sound = kira::sound::static_sound::StaticSoundData::from_cursor(
            std::io::Cursor::new((*roman.get_binary("drop.wav")).clone()),
            kira::sound::static_sound::StaticSoundSettings::default(),
        )
        .unwrap();

        let mut me = Self {
            blocks,
            falling_piece,
            piece_generator: piece_factory,
            last_update_time: std::time::Instant::now(),
            ground_time: std::time::Instant::now(),
            moves_on_ground: 0,
            on_ground: false,
            swapped: false,
            swap_piece: None,
            keybinds,
            ghost_piece,
            victim: None,
            lines_received: std::collections::VecDeque::new(),
            death_time: None,
            effects: BoardEffects::new(0.1, 0.5, 0.1, 0.5),
            score: ScoreHandler::new(),
            audio,
            clear_sound,
            drop_sound,
        };
        me.update_ghost();
        me
    }

    /// The update function should be run every frame. The board calculates its own timings
    pub fn update(
        &mut self,
        keys: &mut std::collections::HashMap<glutin::event::VirtualKeyCode, KeyTiming>,
    ) {
        self.effects.update();
        // exit immediately if we are dead
        if let Some(_) = self.death_time {
            return;
        }

        let now = std::time::Instant::now();

        // handle input and set soft drop
        let mut soft_drop = false;
        self.handle_input(keys, now, &mut soft_drop);

        // run gravity if timeout expired
        if now.duration_since(self.last_update_time)
            >= std::time::Duration::from_millis(if soft_drop {
                self.effects.velocity.y -= 0.01;
                20
            } else {
                1000
            })
        {
            self.soft_drop();
            self.last_update_time = now;
        }

        // land the piece if timeout expired and on ground
        if (now.duration_since(self.ground_time) > std::time::Duration::from_millis(500)
            || self.moves_on_ground > 10)
            && self.on_ground
        {
            self.land_piece();
        }
    }

    fn handle_input(
        &mut self,
        keys: &mut std::collections::HashMap<glutin::event::VirtualKeyCode, KeyTiming>,
        now: std::time::Instant,
        soft_drop: &mut bool,
    ) {
        for (key, timing) in keys {
            let key = match self.keybinds.decode(*key) {
                Some(x) => x,
                None => continue,
            };
            let mut run = false;
            *timing = match timing {
                KeyTiming::None => match key {
                    keys::Key::SoftDrop => {
                        *soft_drop = true;
                        KeyTiming::None
                    }
                    keys::Key::Left | keys::Key::Right => {
                        run = true;
                        KeyTiming::Delayed(now)
                    }
                    _ => {
                        run = true;
                        KeyTiming::Single
                    }
                },
                KeyTiming::Delayed(t) => {
                    if now.duration_since(*t) >= std::time::Duration::from_millis(150) {
                        run = true;
                        KeyTiming::Repeat(now)
                    } else {
                        KeyTiming::Delayed(*t)
                    }
                }
                KeyTiming::Repeat(t) => {
                    if now.duration_since(*t) >= std::time::Duration::from_millis(20) {
                        run = true;
                        KeyTiming::Repeat(now)
                    } else {
                        KeyTiming::Repeat(*t)
                    }
                }
                KeyTiming::Single => KeyTiming::Single,
            };

            if run {
                if self.on_ground {
                    self.moves_on_ground += 1;
                }
                match key {
                    keys::Key::Left => self.move_left(),
                    keys::Key::Right => self.move_right(),
                    keys::Key::RotateCW => self.rotate_cw(),
                    keys::Key::RotateCCW => self.rotate_ccw(),
                    keys::Key::HardDrop => self.hard_drop(),
                    keys::Key::Swap => self.swap(),
                    _ => (),
                }
                self.ground_time = std::time::Instant::now();
            }
        }
    }

    fn rotate_cw(&mut self) {
        self.falling_piece.rotate_cw(&self.blocks);
        self.test_ground();
        self.update_ghost();
    }

    fn rotate_ccw(&mut self) {
        self.falling_piece.rotate_ccw(&self.blocks);
        self.test_ground();
        self.update_ghost();
    }

    fn soft_drop(&mut self) {
        self.falling_piece
            .translate(BlockPos::new(0, -1), &self.blocks);
        self.test_ground();
    }

    fn move_left(&mut self) {
        if self
            .falling_piece
            .translate(BlockPos::new(-1, 0), &self.blocks)
        {
            self.effects.velocity.x -= 0.03;
        }
        self.test_ground();
        self.update_ghost();
    }

    fn move_right(&mut self) {
        if self
            .falling_piece
            .translate(BlockPos::new(1, 0), &self.blocks)
        {
            self.effects.velocity.x += 0.03;
        }
        self.test_ground();
        self.update_ghost();
    }

    fn hard_drop(&mut self) {
        self.effects.velocity.y -= 0.15;
        loop {
            self.hard_drop_fly_particles();
            if self
                .falling_piece
                .translate(BlockPos::new(0, -1), &self.blocks)
            {
                break;
            }
        }
        self.audio
            .borrow_mut()
            .play(self.drop_sound.clone())
            .unwrap();
        self.land_piece();
    }

    fn hard_drop_fly_particles(&mut self) {
        let shape = self.falling_piece.get_shape();
        for x in 0..4 {
            for y in 0..4 {
                if let Block::Block { .. } = shape[x][y] {
                    if rand::random::<u32>() % 10 == 0 {
                        self.effects.particles.push(effects::Particle::new(
                            glam::Vec2::new(
                                (self.falling_piece.position.x + x as i32) as f32
                                    + rand::random::<f32>(),
                                (self.falling_piece.position.y + y as i32) as f32
                                    + rand::random::<f32>(),
                            ),
                            glam::Vec2::new(0.0, 0.1 * rand::random::<f32>()),
                            glam::Vec2::new(0.0, 0.01),
                            0.2 * rand::random::<f32>(),
                            0.005,
                            effects::ParticleModel::random_color(),
                        ));
                    }
                }
            }
        }
    }

    fn swap(&mut self) {
        if self.swapped {
            return;
        }
        let new_piece = self
            .swap_piece
            .take()
            .or_else(|| Some(self.piece_generator.next_piece()))
            .unwrap();
        self.swap_piece = Some(self.falling_piece.shape);
        self.falling_piece = Tetromino::new(new_piece);
        self.swapped = true;
        self.update_ghost();
    }

    fn land_piece(&mut self) {
        // test whether there is a block above out piece
        // this is used for score calculation and must be tested BEFORE the block has been landed
        let covered = self.test_translation(BlockPos::new(0, 1));

        let mut top = 0;

        // convert the piece into blocks
        let piece = self.falling_piece.get_shape();
        for y in 0..4 {
            for x in 0..4 {
                if let Block::Block { .. } = piece[x][y] {
                    let board_x = self.falling_piece.position.x + x as i32;
                    let board_y = self.falling_piece.position.y + y as i32;
                    if board_y - 19 > top {
                        top = board_y;
                    }
                    self.blocks[board_y as usize][board_x as usize] = piece[x as usize][y as usize];
                }
            }
        }

        self.land_aftermath(self.falling_piece.position.y, top, covered);
        self.land_particles();

        // draw a new piece and reset everything
        self.falling_piece = Tetromino::new(self.piece_generator.next_piece());
        self.on_ground = false;
        self.moves_on_ground = 0;
        self.swapped = false;
        self.update_ghost();
    }

    fn land_aftermath(&mut self, mut piece_position: i32, mut piece_top: i32, covered: bool) {
        // add a little bump for landing the piece
        self.effects.velocity.y -= 0.075;

        let mut lines_cleared = 0;

        // scan through lines to find filled lines
        // scan from the top so that we don't skip any lines
        'l: for y in (piece_position.clamp(0, 32)..(piece_position + 4).clamp(0, 32)).rev() {
            for x in 0..10 {
                if let Block::Air = self.blocks[y as usize][x] {
                    continue 'l;
                }
            }
            lines_cleared += 1;
            self.remove_line(y as _);
            piece_position -= 1;
            piece_top -= 1;
        }

        if lines_cleared > 0 {
            self.audio
                .borrow_mut()
                .play(self.clear_sound.clone().with_modified_settings(|_| {
                    kira::sound::static_sound::StaticSoundSettings::new()
                        .playback_rate(kira::PlaybackRate::Semitones(self.score.combo as _))
                }))
                .unwrap();
        }

        let (mut lines_to_send, message) =
            self.score
                .analyze(lines_cleared, self.falling_piece.shape, covered);

        if let Some(x) = message {
            self.effects.info = Some(effects::InfoText {
                text: x,
                time: std::time::Instant::now(),
            });
        }

        self.effects.velocity.y -= 0.1 * lines_to_send as f32;

        // If there are pending lines, block the amount of lines we would normally send
        if let Some(mut x) = self.lines_received.pop_front() {
            if x > lines_to_send {
                x -= lines_to_send;
                lines_to_send = 0;
                self.insert_cheese(x as _);
                piece_top += x as i32;
                self.effects.velocity.y += 0.1 * x as f32;
            } else {
                lines_to_send -= x;
            }
        }

        // Die if we have reached the top
        if piece_top >= 20 {
            self.death_time = Some(std::time::Instant::now());
            return;
        }

        // Send lines only if we didn't die
        if let Some(v) = &mut self.victim {
            if lines_to_send > 0 {
                v.borrow_mut().lines_received.push_back(lines_to_send);
            }
        }
    }

    fn land_particles(&mut self) {
        let shape = self.falling_piece.get_shape();
        for x in 0..4 {
            for y in 0..4 {
                if let Block::Block { .. } = shape[x][y] {
                    self.effects.particles.push(effects::Particle::new(
                        glam::Vec2::new(
                            (self.falling_piece.position.x + x as i32) as f32
                                + rand::random::<f32>(),
                            (self.falling_piece.position.y + y as i32) as f32
                                + rand::random::<f32>(),
                        ),
                        glam::Vec2::new(0.0, 0.0),
                        glam::Vec2::new(0.0, 0.001),
                        0.3 * rand::random::<f32>(),
                        0.005,
                        effects::ParticleModel::Star,
                    ));
                }
            }
        }
    }

    fn test_ground(&mut self) {
        let old_ground = self.on_ground;
        self.on_ground = self.test_translation(BlockPos::new(0, -1));
        if self.on_ground && !old_ground {
            self.ground_time = std::time::Instant::now();
        }
    }

    /// Test whether a translation of a falling tetromino will **fail**
    ///
    /// # Return value
    ///
    /// Returns whether the translation has **failed**.
    fn test_translation(&mut self, dir: BlockPos) -> bool {
        let result = self.falling_piece.translate(dir, &self.blocks);
        // move the piece back up
        if !result {
            self.falling_piece
                .translate(BlockPos::new(-dir.x, -dir.y), &self.blocks);
        }
        result
    }

    /// Update the position of the ghost piece
    fn update_ghost(&mut self) {
        self.ghost_piece = self.falling_piece.clone();
        while !self
            .ghost_piece
            .translate(BlockPos::new(0, -1), &self.blocks)
        {}
    }

    /// Remove line n
    fn remove_line(&mut self, n: usize) {
        self.line_clear_particles(n);
        // The row should be full if the loop passed through everything
        let mut boks = Box::new([Block::Air; 10]);
        for i in n..30 {
            std::mem::swap(&mut self.blocks[i + 1], &mut boks);
            std::mem::swap(&mut self.blocks[i], &mut boks);
            std::mem::swap(&mut self.blocks[i + 1], &mut boks);
        }
        std::mem::swap(&mut self.blocks[31], &mut boks);
        std::mem::swap(&mut self.blocks[30], &mut boks);
    }

    fn line_clear_particles(&mut self, y: usize) {
        for x in 0..10 {
            self.effects.particles.push(effects::Particle::new(
                glam::Vec2::new(
                    x as f32 + rand::random::<f32>(),
                    y as f32 + rand::random::<f32>(),
                ),
                glam::Vec2::new((rand::random::<f32>() - 0.5) * 0.1, 0.1),
                glam::Vec2::new(0.0, -0.01),
                0.4 * rand::random::<f32>(),
                0.005,
                effects::ParticleModel::Star,
            ));
        }
    }

    /// Insert n lines of "cheese" at the bottom of the game
    fn insert_cheese(&mut self, n: usize) {
        let spot = rand::random::<usize>() % 10;
        for _ in 0..n {
            self.blocks.push_front({
                let mut line = Box::new(
                    [Block::Block {
                        color: (0.3, 0.3, 0.3),
                    }; 10],
                );
                line[spot] = Block::Air;
                line
            });
            self.blocks.pop_back();
        }
    }
}
