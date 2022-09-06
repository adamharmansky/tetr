use super::*;
use crate::graphics;
use glam::{Vec2, Vec4};
use graphics::Transformer;

/// The renderer renders the playing field, it contains the sahders, textures, models, ...
pub struct Renderer {
    block_shader: Rc<graphics::Shader>,
    block_color: graphics::UniformHandle,
    block_model: graphics::Model,
    block_texture: graphics::Texture,
    block_transform: graphics::Transformer,
    block_opacity: graphics::UniformHandle,

    board_model: graphics::Model,
    board_texture: graphics::Texture,

    misc_shader: Rc<graphics::Shader>,
    misc_transform: graphics::Transformer,
    misc_color: graphics::UniformHandle,
    texture_enable: graphics::UniformHandle,

    particle_model: graphics::Model,
    star_model: graphics::Model,

    star_texture: graphics::Texture,
}

impl Renderer {
    pub fn new(gl: Rc<gl33::GlFns>) -> Self {
        let block_shader = Rc::new(
            graphics::Shader::new(
                gl.clone(),
                include_str!("../../shaders/default.vert"),
                include_str!("../../shaders/block.frag"),
            )
            .expect("couldn't compile shader"),
        );
        let block_color = graphics::UniformHandle::new(block_shader.clone(), "kolor");
        let block_texture =
            graphics::Texture::load(gl.clone(), include_bytes!("../../assets/block.png")).unwrap();

        let block_model = graphics::Model::new(
            gl.clone(),
            &[
                (0.0, 0.0, 0.0),
                (1.0, 0.0, 0.0),
                (0.0, 1.0, 0.0),
                (1.0, 0.0, 0.0),
                (1.0, 1.0, 0.0),
                (0.0, 1.0, 0.0),
            ],
            &[
                (0.0, 1.0),
                (1.0, 1.0),
                (0.0, 0.0),
                (1.0, 1.0),
                (1.0, 0.0),
                (0.0, 0.0),
            ],
            &[(0.0, 0.0, -1.0); 6],
        )
        .unwrap();

        let star_model = graphics::Model::new(
            gl.clone(),
            &[
                (-1.0, -1.0, 0.0),
                (1.0, -1.0, 0.0),
                (-1.0, 1.0, 0.0),
                (1.0, -1.0, 0.0),
                (1.0, 1.0, 0.0),
                (-1.0, 1.0, 0.0),
            ],
            &[
                (0.0, 1.0),
                (1.0, 1.0),
                (0.0, 0.0),
                (1.0, 1.0),
                (1.0, 0.0),
                (0.0, 0.0),
            ],
            &[(0.0, 0.0, -1.0); 6],
        )
        .unwrap();

        let misc_shader = Rc::new(
            graphics::Shader::new(
                gl.clone(),
                include_str!("../../shaders/default.vert"),
                include_str!("../../shaders/texture_optional.frag"),
            )
            .expect("couldn't compile shader"),
        );

        let block_transform = Transformer::new(block_shader.clone());
        let block_opacity = graphics::UniformHandle::new(block_shader.clone(), "opacity");
        let misc_transform = Transformer::new(misc_shader.clone());
        let misc_color = graphics::UniformHandle::new(misc_shader.clone(), "kolor");
        let texture_enable = graphics::UniformHandle::new(misc_shader.clone(), "enable_texture");

        let particle_model = graphics::Model::new(
            gl.clone(),
            &[(-1.0, -1.0, 0.0), (0.0, 0.732, 0.0), (1.0, -1.0, 0.0)],
            &[],
            &[],
        )
        .unwrap();

        let board_texture =
            graphics::Texture::load(gl.clone(), include_bytes!("../../assets/board.png")).unwrap();
        let board_model = graphics::Model::new(
            gl.clone(),
            &[
                (-6.0, -1.0, 0.0),
                (16.0, -1.0, 0.0),
                (-6.0, 21.0, 0.0),
                (16.0, -1.0, 0.0),
                (16.0, 21.0, 0.0),
                (-6.0, 21.0, 0.0),
            ],
            &[
                (0.0, 1.0),
                (1.0, 1.0),
                (0.0, 0.0),
                (1.0, 1.0),
                (1.0, 0.0),
                (0.0, 0.0),
            ],
            &[],
        )
        .unwrap();

        let star_texture =
            graphics::Texture::load(gl.clone(), include_bytes!("../../assets/star.png")).unwrap();

        Self {
            block_shader,
            block_color,
            block_model,
            block_texture,
            board_model,
            board_texture,
            misc_shader,
            block_transform,
            block_opacity,
            misc_transform,
            misc_color,
            particle_model,
            texture_enable,
            star_model,
            star_texture,
        }
    }

    pub fn draw(&mut self, board: &Board, mut position: glam::Mat4) {
        let death_animation = if let Some(x) = board.death_time {
            std::time::Instant::now().duration_since(x).as_millis() as f32 / 1000.0
        } else {
            0.0
        } as f32;
        let opacity = 1.0 - death_animation;
        let death_fall = death_animation * death_animation * 10.0;

        position *= Mat4::from_translation(Vec3::new(0.0, -death_fall, 0.0));
        position *= Mat4::from_translation(Vec3::new(5.0, 10.0, 0.0));
        position *= Mat4::from_scale(Vec3::new(
            board.effects.scale,
            board.effects.scale,
            board.effects.scale,
        ));
        position *= Mat4::from_translation(Vec3::new(-5.0, -10.0, 0.0));
        position *= Mat4::from_translation(Vec3::new(
            board.effects.position.x,
            board.effects.position.y,
            0.0,
        ));

        self.misc_shader.bind();

        // draw the meter of pieces to be added
        self.texture_enable.set(false);
        self.misc_color.set(glam::Vec4::new(1.0, 0.0, 0.0, opacity));
        self.misc_transform.set(position);
        self.misc_transform
            .transform(Mat4::from_translation(Vec3::new(-1.0, 0.05, 0.0)));
        for i in &board.lines_received {
            self.misc_transform.push();
            self.misc_transform
                .transform(Mat4::from_scale(Vec3::new(1.0, *i as f32 - 0.1, 0.9)));
            self.block_model.render();
            self.misc_transform.pop();
            self.misc_transform
                .transform(Mat4::from_translation(Vec3::new(0.0, *i as f32, 0.0)));
        }

        // draw the board
        self.misc_color.set(Vec4::new(1.0, 1.0, 1.0, opacity));
        self.texture_enable.set(true);
        self.misc_transform.set(position);
        self.board_texture.bind();
        self.board_model.render();

        // draw the placed pieces
        self.block_shader.bind();
        self.block_transform.set(position);

        self.block_opacity.set(opacity);
        self.block_texture.bind();
        for i in 0..24 {
            for j in 0..10 {
                self.block_transform.push();
                self.block_transform
                    .transform(glam::Mat4::from_translation(Vec3::new(
                        j as f32, i as f32, 0.0,
                    )));
                if let Block::Block { color } = board.blocks[i][j] {
                    self.block_color
                        .set(glam::Vec4::new(color.0, color.1, color.2, 1.0));
                    self.block_model.render();
                }
                self.block_transform.pop();
            }
        }

        // draw the ghost piece
        self.block_transform.push();
        self.block_transform
            .transform(glam::Mat4::from_translation(Vec3::new(
                board.ghost_piece.position.x as f32,
                board.ghost_piece.position.y as f32,
                0.0,
            )));
        self.draw_piece(&board.ghost_piece, true);
        self.block_transform.pop();

        // draw the falling piece
        self.block_transform.push();
        self.block_transform
            .transform(glam::Mat4::from_translation(Vec3::new(
                board.falling_piece.position.x as f32,
                board.falling_piece.position.y as f32,
                0.0,
            )));
        self.draw_piece(&board.falling_piece, false);
        self.block_transform.pop();

        // draw the queue
        self.block_transform.push();
        self.block_transform
            .transform(Mat4::from_translation(Vec3::new(12.5, 17.5, 0.0)));
        for i in 0..5 {
            self.draw_shape(board.piece_generator.queue[i], false);
            self.block_transform
                .transform(Mat4::from_translation(Vec3::new(0.0, -3.0, 0.0)));
        }
        self.block_transform.pop();

        // draw the swap piece
        if let Some(x) = board.swap_piece {
            self.block_transform.push();
            self.block_transform
                .transform(Mat4::from_translation(Vec3::new(-2.5, 17.5, 0.0)));
            self.draw_shape(x, board.swapped);
            self.block_transform.pop();
        }

        // draw the particles
        self.misc_shader.bind();
        self.misc_transform.set(position);
        for i in &board.effects.particles {
            self.misc_transform.push();
            self.misc_transform
                .transform(Mat4::from_translation(Vec3::new(
                    i.position.x,
                    i.position.y,
                    0.0,
                )));
            self.misc_transform
                .transform(Mat4::from_scale(Vec3::new(i.size, i.size, 1.0)));
            match i.model {
                effects::ParticleModel::Colorful(color) => {
                    self.misc_color.set(color);
                    self.texture_enable.set(false);
                    self.particle_model.render();
                }
                effects::ParticleModel::Star => {
                    self.misc_color.set(Vec4::new(1.0, 1.0, 1.0, 1.0));
                    self.texture_enable.set(true);
                    self.star_texture.bind();
                    self.star_model.render();
                }
            }
            self.misc_transform.pop();
        }
    }

    fn draw_piece(&mut self, piece: &Tetromino, shadow: bool) {
        let shape = piece.get_shape();
        self.draw_blocks(&shape, shadow);
    }

    fn draw_shape(&mut self, shape: tetromino::Shape, shadow: bool) {
        let piece = Tetromino::new(shape);
        let shape = piece.get_shape();
        let mut com = glam::Vec2::new(0.0, 0.0);

        for x in 0..4 {
            for y in 0..4 {
                if let Block::Block { .. } = shape[x][y] {
                    // we utilize the fact that all tetrominos have 4 blocks
                    com += Vec2::new(x as f32 + 0.5, y as f32 + 0.5) * 0.25;
                }
            }
        }

        self.block_transform.push();
        self.block_transform
            .transform(Mat4::from_translation(Vec3::new(-com.x, -com.y, 0.0)));
        self.draw_blocks(&shape, shadow);
        self.block_transform.pop();
    }

    fn draw_blocks(&mut self, shape: &[[Block; 4]; 4], shadow: bool) {
        for y in 0..4 {
            for x in 0..4 {
                if let Block::Block { color } = shape[x][y] {
                    self.block_transform.push();
                    self.block_transform
                        .transform(glam::Mat4::from_translation(Vec3::new(
                            x as f32, y as f32, 0.0,
                        )));
                    self.block_color.set(if shadow {
                        glam::Vec4::new(0.5, 0.5, 0.5, 0.5)
                    } else {
                        glam::Vec4::new(color.0, color.1, color.2, 1.0)
                    });
                    self.block_model.render();
                    self.block_transform.pop();
                }
            }
        }
    }
}
