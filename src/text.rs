use crate::graphics;
use std::{cell::RefCell, rc::Rc};
extern crate freetype;
use crate::graphics::GraphicsHandle;
use glam::{Mat4, Vec3, Vec4};

struct Letter {
    width: f32,
    height: f32,
    texture: graphics::Texture,
    metrics: freetype::GlyphMetrics,
}

pub struct Font {
    face: freetype::Face,
    characters: std::collections::HashMap<char, Letter>,
    size: u32,
}

pub struct TextRenderer {
    lib: freetype::Library,
    shader: Rc<RefCell<graphics::Shader>>,
    model: graphics::Model,
}

impl TextRenderer {
    pub fn new(
        gh: &mut GraphicsHandle,
        roman: &crate::resource::ResourceManager,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let vs = roman.get_text("default.vert");
        let fs = roman.get_text("texture.frag");
        Ok(Self {
            lib: freetype::Library::init()?,
            shader: Rc::new(RefCell::new(graphics::Shader::new(gh, &vs, &fs).unwrap())),
            model: graphics::Model::new(
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
            .unwrap(),
        })
    }

    /// Draw a string, transformed by matrix `mat`, starting from point 0.0
    ///
    /// 1 unit = 1 px
    pub fn draw(
        &self,
        gh: &mut GraphicsHandle,
        font: &mut Font,
        mut mat: Mat4,
        color: Vec4,
        text: &str,
    ) {
        for i in text.chars() {
            let letter = font.get_char(gh, i);
            gh.bind(self.shader.clone());
            letter.texture.bind(gh);
            gh.set_uniform("color", color);
            gh.set_uniform(
                "view",
                mat * Mat4::from_translation(Vec3::new(
                    letter.metrics.horiBearingX as f32 / 64.0,
                    letter.metrics.horiBearingY as f32 / 64.0 - letter.metrics.height as f32 / 64.0,
                    0.0,
                )) * Mat4::from_scale(Vec3::new(letter.width, letter.height, 1.0)),
            );
            self.model.render(gh);
            gh.unbind();
            mat *= Mat4::from_translation(Vec3::new(
                letter.metrics.horiAdvance as f32 / 64.0,
                0.0,
                0.0,
            ));
        }
    }

    /// Get the width of a string in pixels
    pub fn get_width(&self, gh: &mut GraphicsHandle, font: &mut Font, text: &str) -> f32 {
        let mut w = 0.0;
        for i in text.chars() {
            let letter = font.get_char(gh, i);
            w += letter.metrics.horiAdvance as f32 / 64.0;
        }
        w
    }
}

impl Font {
    pub fn new(
        renderer: &TextRenderer,
        data: Rc<Vec<u8>>,
        size: u32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let face = renderer.lib.new_memory_face(data, 0)?;
        face.set_pixel_sizes(size, size)?;
        Ok(Self {
            face,
            characters: std::collections::HashMap::new(),
            size,
        })
    }

    fn get_char<'a>(&'a mut self, gh: &mut GraphicsHandle, position: char) -> &'a Letter {
        self.characters
            .entry(position)
            .or_insert_with(|| Self::load(&self.face, self.size, gh, position))
    }

    fn load(face: &freetype::Face, size: u32, gh: &mut GraphicsHandle, position: char) -> Letter {
        face.set_pixel_sizes(size, size).unwrap();
        face.load_char(position as usize, freetype::face::LoadFlag::RENDER)
            .unwrap();
        let glyph = face.glyph();
        let bmp = glyph.bitmap();

        let width = bmp.width();
        let height = bmp.rows();
        let b = bmp.buffer();
        let pitch = bmp.pitch();
        let metrics = glyph.metrics();

        let mut texture = graphics::Texture::from_image(
            gh,
            &image::ImageBuffer::<image::Rgba<u8>, Vec<u8>>::from_fn(
                width as _,
                height as _,
                |x, y| {
                    image::Rgba::<u8>::from([
                        255,
                        255,
                        255,
                        b[(x as i32 + y as i32 * pitch) as usize],
                    ])
                },
            ),
        )
        .unwrap();

        texture.fontify(gh);
        Letter {
            width: width as _,
            height: height as _,
            texture,
            metrics,
        }
    }
}
