use super::*;
use std::collections::HashMap;

struct Uniform {
    id: u32,
    values: Vec<Box<dyn shader::SetAsUniform>>,
}

pub struct Shader {
    gl: Rc<GlFns>,
    id: u32,
    uniforms: HashMap<&'static str, Uniform>,
    vs: u32,
    fs: u32,
}

impl Drop for Shader {
    fn drop(&mut self) {
        self.gl.DeleteShader(self.vs);
        self.gl.DeleteShader(self.fs);
        self.gl.DeleteProgram(self.id);
    }
}

impl Shader {
    pub fn new(handle: &GraphicsHandle, vscode: &str, fscode: &str) -> Option<Self> {
        let vs = create_shader(&handle.gl, vscode, gl33::GL_VERTEX_SHADER)?;
        let fs = create_shader(&handle.gl, fscode, gl33::GL_FRAGMENT_SHADER)?;
        let program = handle.gl.CreateProgram();
        if program == 0 {
            return None;
        }
        handle.gl.AttachShader(program, vs);
        handle.gl.AttachShader(program, fs);
        handle.gl.LinkProgram(program);
        unsafe {
            let mut success = 0;
            handle
                .gl
                .GetProgramiv(program, gl33::GL_LINK_STATUS, &mut success);
            if success == 0 {
                return None;
            }
        }
        Some(Shader {
            gl: handle.gl.clone(),
            id: program,
            uniforms: HashMap::new(),
            vs,
            fs,
        })
    }

    pub fn bind(&self, gl: &GlFns) {
        gl.UseProgram(self.id);
    }

    fn get_uniform<'a>(&'a mut self, gl: &GlFns, name: &'static str) -> &'a mut Uniform {
        self.uniforms.entry(name).or_insert_with(|| unsafe {
            let mut name = String::from(name);
            name += "\0";
            let name = name.as_bytes().as_ptr();
            let id = gl.GetUniformLocation(self.id, name as _) as u32;
            Uniform {
                id,
                values: Vec::new(),
            }
        })
    }

    pub fn uniform_set<T: SetAsUniform + 'static>(
        &mut self,
        gl: &GlFns,
        name: &'static str,
        value: T,
    ) {
        let uni = self.get_uniform(gl, name);
        unsafe {
            value.set_as_uniform(gl, uni.id);
        }
        if let Some(x) = uni.values.last_mut() {
            *x = Box::new(value);
        } else {
            uni.values.push(Box::new(value));
        }
    }

    pub fn uniform_push<T: SetAsUniform + 'static>(
        &mut self,
        gl: &GlFns,
        name: &'static str,
        value: T,
    ) {
        let uni = self.get_uniform(gl, name);
        unsafe {
            value.set_as_uniform(gl, uni.id);
        }
        uni.values.push(Box::new(value));
    }

    pub fn uniform_pop(&mut self, gl: &GlFns, name: &'static str) {
        let uni = self.get_uniform(gl, name);
        let value = uni.values.pop().unwrap();
        unsafe {
            value.set_as_uniform(gl, uni.id);
        }
    }
}

pub trait SetAsUniform {
    unsafe fn set_as_uniform(&self, _gl: &GlFns, _id: u32) {}
}

impl SetAsUniform for i32 {
    unsafe fn set_as_uniform(&self, gl: &GlFns, id: u32) {
        gl.Uniform1i(id as _, *self);
    }
}

impl SetAsUniform for f32 {
    unsafe fn set_as_uniform(&self, gl: &GlFns, id: u32) {
        gl.Uniform1f(id as _, *self);
    }
}

impl SetAsUniform for Vec2 {
    unsafe fn set_as_uniform(&self, gl: &GlFns, id: u32) {
        gl.Uniform2f(id as _, self.x, self.y);
    }
}

impl SetAsUniform for Vec3 {
    unsafe fn set_as_uniform(&self, gl: &GlFns, id: u32) {
        gl.Uniform3f(id as _, self.x, self.y, self.z);
    }
}

impl SetAsUniform for Vec4 {
    unsafe fn set_as_uniform(&self, gl: &GlFns, id: u32) {
        gl.Uniform4f(id as _, self.x, self.y, self.z, self.w);
    }
}

impl SetAsUniform for Mat4 {
    unsafe fn set_as_uniform(&self, gl: &GlFns, id: u32) {
        gl.UniformMatrix4fv(id as _, 1, false as _, self.to_cols_array().as_ptr() as _);
    }
}

impl SetAsUniform for bool {
    unsafe fn set_as_uniform(&self, gl: &GlFns, id: u32) {
        gl.Uniform1i(id as _, (*self) as _);
    }
}

fn create_shader(gl: &GlFns, code: &str, t: gl33::ShaderType) -> Option<u32> {
    let s = gl.CreateShader(t);
    if s == 0 {
        return None;
    }
    unsafe {
        gl.ShaderSource(s, 1, &code.as_bytes().as_ptr() as _, &(code.len() as _));
        gl.CompileShader(s);
        let mut success = 0;
        gl.GetShaderiv(s, gl33::GL_COMPILE_STATUS, &mut success);
        if success == 0 {
            return None;
        }
    };
    Some(s)
}
