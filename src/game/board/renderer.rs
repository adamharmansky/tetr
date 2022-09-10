use super::*;
use crate::graphics;
use crate::text;
use glam::{Vec2, Vec4};

/// The renderer renders the playing field, it contains the sahders, textures, models, ...
pub struct Renderer {
    block_shader: Rc<RefCell<graphics::Shader>>,
    misc_shader: Rc<RefCell<graphics::Shader>>,

    block_model: graphics::Model,
    block_texture: graphics::Texture,

    board_model: graphics::Model,
    board_texture: graphics::Texture,

    particle_model: graphics::Model,

    star_model: graphics::Model,
    star_texture: graphics::Texture,

    tr: Rc<text::TextRenderer>,
    font: text::Font,
}

impl Renderer {
    pub fn new(
        gh: &mut crate::graphics::GraphicsHandle,
        roman: &crate::resource::ResourceManager,
        tr: Rc<text::TextRenderer>,
    ) -> Self {
        let default_vert = roman.get_text("default.vert");

        let block_frag = roman.get_text("block.frag");
        let block_shader = Rc::new(RefCell::new(
            graphics::Shader::new(gh, &default_vert, &block_frag).expect("couldn't compile shader"),
        ));

        let misc_frag = roman.get_text("texture_optional.frag");
        let misc_shader = Rc::new(RefCell::new(
            graphics::Shader::new(gh, &default_vert, &misc_frag).expect("couldn't compile shader"),
        ));

        let block_model = graphics::Model::new(
            gh,
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
            gh,
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

        let particle_model = graphics::Model::new(
            gh,
            &[(-1.0, -1.0, 0.0), (0.0, 0.732, 0.0), (1.0, -1.0, 0.0)],
            &[],
            &[],
        )
        .unwrap();

        let board_model = graphics::Model::new(
            gh,
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

        let star_texture = roman.get_image("star.png");
        let star_texture = graphics::Texture::from_image(gh, &star_texture).unwrap();
        let block_texture = roman.get_image("block.png");
        let block_texture = graphics::Texture::from_image(gh, &block_texture).unwrap();
        let board_texture = roman.get_image("board.png");
        let board_texture = graphics::Texture::from_image(gh, &board_texture).unwrap();

        let font = text::Font::new(&tr, roman.get_binary("teko-light.ttf"), 200).unwrap();

        Self {
            block_shader,
            block_model,
            block_texture,
            board_model,
            board_texture,
            misc_shader,
            particle_model,
            star_model,
            star_texture,
            tr,
            font,
        }
    }

    pub fn draw(
        &mut self,
        gh: &mut crate::graphics::GraphicsHandle,
        board: &Board,
        mut mat: glam::Mat4,
    ) {
        let now = std::time::Instant::now();
        let death_animation = if let Some(x) = board.death_time {
            now.duration_since(x).as_millis() as f32 / 1000.0
        } else {
            0.0
        } as f32;
        let opacity = 1.0 - death_animation;
        let death_fall = death_animation * death_animation * 10.0;

        mat *= Mat4::from_translation(Vec3::new(0.0, -death_fall, 0.0));
        mat *= Mat4::from_translation(Vec3::new(5.0, 10.0, 0.0));
        mat *= Mat4::from_scale(Vec3::new(
            board.effects.scale,
            board.effects.scale,
            board.effects.scale,
        ));
        mat *= Mat4::from_translation(Vec3::new(-5.0, -10.0, 0.0));
        mat *= Mat4::from_translation(Vec3::new(
            board.effects.position.x,
            board.effects.position.y,
            0.0,
        ));

        {
            gh.bind(self.misc_shader.clone());

            // draw the meter of pieces to be added
            {
                gh.set_uniform("enable_texture", false);
                gh.set_uniform("color", glam::Vec4::new(1.0, 0.0, 0.0, opacity));
                let mut mat = mat * Mat4::from_translation(Vec3::new(-1.0, 0.05, 0.0));
                for i in &board.lines_received {
                    gh.set_uniform(
                        "view",
                        mat * Mat4::from_scale(Vec3::new(1.0, *i as f32 - 0.1, 0.9)),
                    );
                    self.block_model.render(gh);
                    mat *= Mat4::from_translation(Vec3::new(0.0, *i as f32, 0.0));
                }
            }

            // draw the board
            {
                gh.set_uniform("enable_texture", true);
                gh.set_uniform("color", Vec4::new(1.0, 1.0, 1.0, opacity));
                gh.set_uniform("view", mat);
                self.board_texture.bind(gh);
                self.board_model.render(gh);
            }

            gh.unbind();
        }

        {
            gh.bind(self.block_shader.clone());
            self.block_texture.bind(gh);
            // draw the placed pieces
            for i in 0..24 {
                for j in 0..10 {
                    if let Block::Block { color } = board.blocks[i][j] {
                        gh.set_uniform(
                            "view",
                            mat * glam::Mat4::from_translation(Vec3::new(j as f32, i as f32, 0.0)),
                        );
                        gh.set_uniform(
                            "color",
                            glam::Vec4::new(color.0, color.1, color.2, opacity),
                        );
                        self.block_model.render(gh);
                    }
                }
            }

            // draw the ghost piece
            self.draw_piece(
                gh,
                mat * glam::Mat4::from_translation(Vec3::new(
                    board.ghost_piece.position.x as f32,
                    board.ghost_piece.position.y as f32,
                    0.0,
                )),
                &board.ghost_piece,
                true,
            );

            // draw the falling piece
            self.draw_piece(
                gh,
                mat * glam::Mat4::from_translation(Vec3::new(
                    board.falling_piece.position.x as f32,
                    board.falling_piece.position.y as f32,
                    0.0,
                )),
                &board.falling_piece,
                false,
            );

            // draw the queue
            {
                let mut mat = mat * Mat4::from_translation(Vec3::new(12.5, 17.5, 0.0));
                for i in 0..5 {
                    self.draw_shape(gh, mat, board.piece_generator.queue[i], false);
                    mat *= Mat4::from_translation(Vec3::new(0.0, -3.0, 0.0));
                }
            }

            // draw the swap piece
            if let Some(x) = board.swap_piece {
                self.draw_shape(
                    gh,
                    mat * Mat4::from_translation(Vec3::new(-2.5, 17.5, 0.0)),
                    x,
                    board.swapped,
                );
            }

            gh.unbind();
        }

        // draw the info text
        if let Some(x) = &board.effects.info {
            let width = self.tr.get_width(gh, &mut self.font, x.text.as_str());
            let size = 1.0 + now.duration_since(x.time).as_millis() as f32 / 4000.0;
            self.tr.draw(
                gh,
                &mut self.font,
                mat * Mat4::from_translation(Vec3::new(-1.2 - width * 0.01 * size, 14.0, 0.0))
                    * Mat4::from_scale(Vec3::new(0.01, 0.01, 0.01) * size),
                Vec4::new(
                    1.0,
                    1.0,
                    1.0,
                    1.0 - now.duration_since(x.time).as_millis() as f32 / 1000.0,
                ),
                x.text.as_str(),
            );
        }

        self.tr.draw(
            gh,
            &mut self.font,
            mat * Mat4::from_translation(Vec3::new(10.2, 2.3, 0.0))
                * Mat4::from_scale(Vec3::new(0.008, 0.01, 0.01)),
            Vec4::new(1.0, 1.0, 1.0, if board.score.combo < 2 { 0.1 } else { 1.0 }),
            format!("COMBO×{}", board.score.combo).as_str(),
        );

        self.tr.draw(
            gh,
            &mut self.font,
            mat * Mat4::from_translation(Vec3::new(10.2, 0.3, 0.0))
                * Mat4::from_scale(Vec3::new(0.01, 0.01, 0.01)),
            Vec4::new(1.0, 1.0, 1.0, if board.score.b2b < 2 { 0.1 } else { 1.0 }),
            format!("B2B×{}", board.score.b2b).as_str(),
        );

        // draw the particles
        {
            gh.bind(self.misc_shader.clone());
            for i in &board.effects.particles {
                gh.set_uniform(
                    "view",
                    mat * Mat4::from_translation(Vec3::new(i.position.x, i.position.y, 0.0))
                        * Mat4::from_scale(Vec3::new(i.size, i.size, 1.0)),
                );
                match i.model {
                    effects::ParticleModel::Colorful(color) => {
                        gh.set_uniform("color", color);
                        gh.set_uniform("enable_texture", false);
                        self.particle_model.render(gh);
                    }
                    effects::ParticleModel::Star => {
                        gh.set_uniform("color", Vec4::new(1.0, 1.0, 1.0, 1.0));
                        gh.set_uniform("enable_texture", true);
                        self.star_texture.bind(gh);
                        self.star_model.render(gh);
                    }
                }
            }
            gh.unbind();
        }
    }

    fn draw_piece(
        &mut self,
        gh: &mut crate::graphics::GraphicsHandle,
        mat: Mat4,
        piece: &Tetromino,
        shadow: bool,
    ) {
        let shape = piece.get_shape();
        self.draw_blocks(gh, mat, &shape, shadow);
    }

    fn draw_shape(
        &mut self,
        gh: &mut crate::graphics::GraphicsHandle,
        mat: Mat4,
        shape: tetromino::Shape,
        shadow: bool,
    ) {
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

        self.draw_blocks(
            gh,
            mat * Mat4::from_translation(Vec3::new(-com.x, -com.y, 0.0)),
            &shape,
            shadow,
        );
    }

    fn draw_blocks(
        &mut self,
        gh: &mut crate::graphics::GraphicsHandle,
        mat: Mat4,
        shape: &[[Block; 4]; 4],
        shadow: bool,
    ) {
        for y in 0..4 {
            for x in 0..4 {
                if let Block::Block { color } = shape[x][y] {
                    gh.set_uniform(
                        "view",
                        mat * glam::Mat4::from_translation(Vec3::new(x as f32, y as f32, 0.0)),
                    );
                    gh.set_uniform(
                        "color",
                        if shadow {
                            glam::Vec4::new(0.5, 0.5, 0.5, 0.5)
                        } else {
                            glam::Vec4::new(color.0, color.1, color.2, 1.0)
                        },
                    );
                    self.block_model.render(gh);
                }
            }
        }
    }
}
