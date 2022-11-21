use super::*;
use std::cell::RefCell;

pub struct GraphicsHandle {
    pub gl: Rc<GlFns>,
    shader: Option<Rc<RefCell<Shader>>>,
}

impl GraphicsHandle {
    pub fn new(gl: GlFns) -> Self {
        Self {
            gl: Rc::new(gl),
            shader: None,
        }
    }

    /// Bind a new shader and unbind the old one
    pub fn bind(&mut self, shader: Rc<RefCell<Shader>>) {
        {
            let s = shader.borrow();
            s.bind(&self.gl);
        }
        self.shader = Some(shader);
    }

    /// Unbind the current shader
    pub fn unbind(&mut self) {
        self.shader = None;
        self.gl.UseProgram(0);
    }

    pub fn set_uniform<T: shader::SetAsUniform + 'static>(&mut self, name: &'static str, value: T) {
        self.shader
            .as_ref()
            .unwrap()
            .borrow_mut()
            .uniform(&self.gl, name, value);
    }
}

pub struct GraphicsHandleBorrow<'a> {
    handle: &'a mut GraphicsHandle,
}

impl<'a> Drop for GraphicsHandleBorrow<'a> {
    fn drop(&mut self) {
        self.handle.unbind()
    }
}
