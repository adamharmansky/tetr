use super::*;
use std::cell::RefCell;

pub struct GraphicsHandle {
    pub gl: Rc<GlFns>,
    shaders: Vec<Rc<RefCell<Shader>>>,
}

impl GraphicsHandle {
    pub fn new(gl: GlFns) -> Self {
        Self {
            gl: Rc::new(gl),
            shaders: Vec::new(),
        }
    }

    /// Bind a new shader and save the old one
    pub fn push_shader(&mut self, shader: Rc<RefCell<Shader>>) {
        {
            let s = shader.borrow();
            s.bind(&self.gl);
        }
        self.shaders.push(shader);
    }

    /// Unbind the current shader and set the old one
    pub fn pop_shader(&mut self) {
        self.shaders.pop();
        if let Some(x) = self.shaders.last() {
            x.borrow().bind(&self.gl);
        }
    }

    pub fn set_uniform<T: shader::SetAsUniform + 'static>(&mut self, name: &'static str, value: T) {
        self.shaders
            .last_mut()
            .unwrap()
            .borrow_mut()
            .uniform_set(&self.gl, name, value);
    }

    pub fn push_uniform<T: shader::SetAsUniform + 'static>(
        &mut self,
        name: &'static str,
        value: T,
    ) {
        self.shaders
            .last_mut()
            .unwrap()
            .borrow_mut()
            .uniform_push(&self.gl, name, value);
    }

    pub fn pop_uniform<T: shader::SetAsUniform + 'static>(&mut self, name: &'static str) {
        self.shaders
            .last_mut()
            .unwrap()
            .borrow_mut()
            .uniform_pop(&self.gl, name);
    }
}

pub struct GraphicsHandleBorrow<'a> {
    handle: &'a mut GraphicsHandle,
}

impl<'a> Drop for GraphicsHandleBorrow<'a> {
    fn drop(&mut self) {
        self.handle.pop_shader()
    }
}
