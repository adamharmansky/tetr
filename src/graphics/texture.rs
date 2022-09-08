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
        handle: &GraphicsHandle,
        img: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    ) -> Result<Self, String> {
        let mut id = 0u32;
        unsafe {
            handle.gl.GenTextures(1, &mut id as _);
            handle.gl.BindTexture(gl33::GL_TEXTURE_2D, id);
            handle.gl.TexParameteri(
                gl33::GL_TEXTURE_2D,
                gl33::GL_TEXTURE_MIN_FILTER,
                gl33::GL_LINEAR.0 as _,
            );
            handle.gl.TexParameteri(
                gl33::GL_TEXTURE_2D,
                gl33::GL_TEXTURE_MAG_FILTER,
                gl33::GL_LINEAR.0 as _,
            );
            handle.gl.TexImage2D(
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
            handle.gl.GenerateMipmap(gl33::GL_TEXTURE_2D);
        }
        Ok(Self {
            gl: handle.gl.clone(),
            id,
        })
    }

    pub fn load(handle: &GraphicsHandle, data: &[u8]) -> Result<Self, String> {
        let cursor = std::io::Cursor::new(data);
        let img = image::io::Reader::new(cursor)
            .with_guessed_format()
            .ok()
            .ok_or(String::from("wrong file format"))?
            .decode()
            .ok()
            .ok_or(String::from("unable to load image"))?
            .into_rgba8();
        Self::from_image(handle, &img)
    }

    pub fn bind(&self, gh: &GraphicsHandle) {
        unsafe {
            gh.gl.BindTexture(gl33::GL_TEXTURE_2D, self.id);
        }
    }
}
