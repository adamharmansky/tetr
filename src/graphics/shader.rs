use super::*;

pub struct Shader {
    gl: Rc<GlFns>,
    id: u32,
}

pub struct UniformHandle {
    shader: Rc<Shader>,
    id: u32,
}

impl Shader {
    pub fn new(gl: Rc<GlFns>, vscode: &str, fscode: &str) -> Option<Self> {
        let vs = create_shader(&gl, vscode, gl33::GL_VERTEX_SHADER)?;
        let fs = create_shader(&gl, fscode, gl33::GL_FRAGMENT_SHADER)?;
        let program = gl.CreateProgram();
        if program == 0 {
            return None;
        }
        gl.AttachShader(program, vs);
        gl.AttachShader(program, fs);
        gl.LinkProgram(program);
        unsafe {
            let mut success = 0;
            gl.GetProgramiv(program, gl33::GL_LINK_STATUS, &mut success);
            if success == 0 {
                return None;
            }
        }
        gl.DeleteShader(vs);
        gl.DeleteShader(fs);
        Some(Shader { gl, id: program })
    }

    pub fn bind(&self) {
        self.gl.UseProgram(self.id);
    }
}

impl UniformHandle {
    pub fn new(shader: Rc<Shader>, name: &str) -> Self {
        Self {
            shader: shader.clone(),
            id: unsafe {
                let mut name = String::from(name);
                name += "\0";
                shader
                    .gl
                    .GetUniformLocation(shader.id, name.as_bytes().as_ptr() as _)
                    as _
            },
        }
    }

    /// Set the uniform.
    ///
    /// *side effect*: binds the associated shader
    pub fn set<T>(&self, value: T)
    where
        T: SetAsUniform,
    {
        unsafe {
            self.shader.bind();
            value.set_as_uniform(&self.shader.gl, self.id)
        };
    }
}

pub trait SetAsUniform {
    unsafe fn set_as_uniform(&self, _gl: &Rc<GlFns>, _id: u32) {}
}

impl SetAsUniform for i32 {
    unsafe fn set_as_uniform(&self, gl: &Rc<GlFns>, id: u32) {
        gl.Uniform1i(id as _, *self);
    }
}

impl SetAsUniform for f32 {
    unsafe fn set_as_uniform(&self, gl: &Rc<GlFns>, id: u32) {
        gl.Uniform1f(id as _, *self);
    }
}

impl SetAsUniform for Vec2 {
    unsafe fn set_as_uniform(&self, gl: &Rc<GlFns>, id: u32) {
        gl.Uniform2f(id as _, self.x, self.y);
    }
}

impl SetAsUniform for Vec3 {
    unsafe fn set_as_uniform(&self, gl: &Rc<GlFns>, id: u32) {
        gl.Uniform3f(id as _, self.x, self.y, self.z);
    }
}

impl SetAsUniform for Vec4 {
    unsafe fn set_as_uniform(&self, gl: &Rc<GlFns>, id: u32) {
        gl.Uniform4f(id as _, self.x, self.y, self.z, self.w);
    }
}

impl SetAsUniform for Mat4 {
    unsafe fn set_as_uniform(&self, gl: &Rc<GlFns>, id: u32) {
        gl.UniformMatrix4fv(id as _, 1, false as _, self.to_cols_array().as_ptr() as _);
    }
}

fn create_shader(gl: &Rc<GlFns>, code: &str, t: gl33::ShaderType) -> Option<u32> {
    let s = gl.CreateShader(t);
    if s == 0 {
        return None;
    }
    unsafe {
        gl.ShaderSource(s, 1, &code.as_ptr().cast(), &(code.len() as _));
        gl.CompileShader(s);
        let mut success = 0;
        gl.GetShaderiv(s, gl33::GL_COMPILE_STATUS, &mut success);
        if success == 0 {
            return None;
        }
    };
    Some(s)
}
