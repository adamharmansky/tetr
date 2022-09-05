use super::*;

pub struct Transformer {
    stack: Vec<Mat4>,
    transform_uniform: UniformHandle,
    current_matrix: Mat4,
}

impl Transformer {
    pub fn new(shader: Rc<Shader>) -> Self {
        let transform_uniform = UniformHandle::new(shader, "view");
        Self {
            transform_uniform,
            stack: Vec::new(),
            current_matrix: Mat4::IDENTITY,
        }
    }

    #[allow(unused)]
    pub fn push(&mut self) {
        self.stack.push(self.current_matrix);
    }

    #[allow(unused)]
    pub fn pop(&mut self) {
        self.current_matrix = self
            .stack
            .pop()
            .expect("bottom of transformer stack reached!");
        self.update();
    }

    #[allow(unused)]
    pub fn transform(&mut self, mat: Mat4) {
        self.current_matrix *= mat;
        self.update();
    }

    #[allow(unused)]
    pub fn set(&mut self, mat: Mat4) {
        self.current_matrix = mat;
        self.update();
    }

    #[allow(unused)]
    pub fn get(&mut self) -> Mat4 {
        self.current_matrix
    }

    fn update(&mut self) {
        self.transform_uniform.set(self.current_matrix);
    }
}
