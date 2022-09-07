use super::*;

pub struct Texture {
    gl: Rc<GlFns>,
    id: u32,
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteTextures(1, &self.id as _);
        }
    }
}

impl Texture {
    pub fn from_image(
        gl: Rc<GlFns>,
        img: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    ) -> Result<Self, String> {
        let mut id = 0u32;
        unsafe {
            gl.GenTextures(1, &mut id as _);
            gl.BindTexture(gl33::GL_TEXTURE_2D, id);
            gl.TexParameteri(
                gl33::GL_TEXTURE_2D,
                gl33::GL_TEXTURE_MIN_FILTER,
                gl33::GL_LINEAR.0 as _,
            );
            gl.TexParameteri(
                gl33::GL_TEXTURE_2D,
                gl33::GL_TEXTURE_MAG_FILTER,
                gl33::GL_LINEAR.0 as _,
            );
            gl.TexImage2D(
                gl33::GL_TEXTURE_2D,
                0,
                gl33::GL_RGBA.0 as _,
                img.width() as _,
                img.height() as _,
                0,
                gl33::GL_RGBA,
                gl33::GL_UNSIGNED_BYTE,
                img.as_ptr() as _,
            );
            gl.GenerateMipmap(gl33::GL_TEXTURE_2D);
        }
        Ok(Self { gl, id })
    }

    pub fn load(gl: Rc<GlFns>, data: &[u8]) -> Result<Self, String> {
        let cursor = std::io::Cursor::new(data);
        let img = image::io::Reader::new(cursor)
            .with_guessed_format()
            .ok()
            .ok_or(String::from("wrong file format"))?
            .decode()
            .ok()
            .ok_or(String::from("unable to load image"))?
            .into_rgba8();
        Self::from_image(gl, &img)
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.BindTexture(gl33::GL_TEXTURE_2D, self.id);
        }
    }
}
