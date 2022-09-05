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
    pub fn load(gl: Rc<GlFns>, data: &[u8]) -> Result<Self, String> {
        let cursor = std::io::Cursor::new(data);
        let img = image::io::Reader::new(cursor)
            .with_guessed_format()
            .ok()
            .ok_or(String::from("wrong file format"))?
            .decode()
            .ok()
            .ok_or(String::from("unable to load image"))?;
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
                match img {
                    image::DynamicImage::ImageLuma8(_) => gl33::GL_RED,
                    image::DynamicImage::ImageLumaA8(_) => gl33::GL_RG,
                    image::DynamicImage::ImageRgb8(_) => gl33::GL_RGB,
                    image::DynamicImage::ImageRgba8(_) => gl33::GL_RGBA,
                    _ => return Err(String::from("Wrong color format")),
                },
                gl33::GL_UNSIGNED_BYTE,
                img.as_bytes().as_ptr() as _,
            );
            gl.GenerateMipmap(gl33::GL_TEXTURE_2D);
        }
        Ok(Self { gl, id })
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.BindTexture(gl33::GL_TEXTURE_2D, self.id);
        }
    }
}
