use super::*;

pub struct Model {
    gl: Rc<GlFns>,
    vao: u32,
    vertices: u32,
    texcoords: u32,
    normals: u32,
    count: u32,
}

impl Drop for Model {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteBuffers(1, &self.vertices);
            self.gl.DeleteBuffers(1, &self.texcoords);
            self.gl.DeleteBuffers(1, &self.normals);
            self.gl.DeleteVertexArrays(1, &self.vao);
        }
    }
}

impl Model {
    pub fn new(
        handle: &GraphicsHandle,
        vertices: &[(f32, f32, f32)],
        texcoords: &[(f32, f32)],
        normals: &[(f32, f32, f32)],
    ) -> Option<Self> {
        let mut vao = 0u32;
        let vert_vbo;
        let tex_vbo;
        let norm_vbo;
        unsafe {
            handle.gl.GenVertexArrays(1, &mut vao);
            if vao == 0 {
                return None;
            };
            handle.gl.BindVertexArray(vao);
            vert_vbo = gen_vbo(handle, vertices, 0, 3, false, gl33::GL_FLOAT)?;
            tex_vbo = gen_vbo(handle, texcoords, 1, 2, false, gl33::GL_FLOAT)?;
            norm_vbo = gen_vbo(handle, normals, 2, 3, false, gl33::GL_FLOAT)?;
        }
        Some(Self {
            gl: handle.gl.clone(),
            vao,
            vertices: vert_vbo,
            texcoords: tex_vbo,
            normals: norm_vbo,
            count: vertices.len() as _,
        })
    }

    pub fn render(&self, handle: &GraphicsHandle) {
        unsafe {
            handle.gl.BindVertexArray(self.vao);
            handle.gl.DrawArrays(gl33::GL_TRIANGLES, 0, self.count as _);
        }
    }
}

unsafe fn gen_vbo<T>(
    handle: &GraphicsHandle,
    data: &[T],
    index: u32,
    size: u32,
    normalized: bool,
    data_type: gl33::VertexAttribPointerType,
) -> Option<u32> {
    let mut vbo = 0u32;
    handle.gl.GenBuffers(1, &mut vbo);
    if vbo == 0 {
        return None;
    };
    handle.gl.BindBuffer(gl33::GL_ARRAY_BUFFER, vbo);
    handle.gl.BufferData(
        gl33::GL_ARRAY_BUFFER,
        (data.len() * std::mem::size_of::<T>()) as _,
        data.as_ptr() as _,
        gl33::GL_STATIC_DRAW,
    );
    handle.gl.VertexAttribPointer(
        index,
        size as _,
        data_type,
        normalized as _,
        std::mem::size_of::<T>() as _,
        0 as *const _,
    );
    handle.gl.EnableVertexAttribArray(index);
    Some(vbo)
}
