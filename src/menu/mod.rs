use super::*;
use glam::{Mat4, Vec3};

struct MenuItem {
    texture: graphics::Texture,
    target: Screen,
    zoom: f32,
}

impl MenuItem {
    pub fn new(
        gl: Rc<gl33::GlFns>,
        texture: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        target: Screen,
    ) -> Self {
        Self {
            texture: graphics::Texture::from_image(gl, texture).unwrap(),
            target,
            zoom: 1.0,
        }
    }
}

pub struct Menu {
    shader: Rc<graphics::Shader>,
    item_model: graphics::Model,
    active_item: usize,
    items: Vec<MenuItem>,
    chosen: Option<crate::Screen>,
    spring_position: f32,
}

impl Menu {
    pub fn new(gl: Rc<gl33::GlFns>, roman: &crate::resource::ResourceManager) -> Self {
        let single = roman.get_image("single.png");
        let double = roman.get_image("double.png");
        let vert = roman.get_text("default.vert");
        let frag = roman.get_text("menu.frag");
        Self {
            shader: Rc::new(
                graphics::Shader::new(gl.clone(), &vert, &frag).expect("couldn't compile shader!"),
            ),
            item_model: graphics::Model::new(
                gl.clone(),
                &[
                    (-1.0, -0.25, 0.0),
                    (1.0, -0.25, 0.0),
                    (-1.0, 0.25, 0.0),
                    (1.0, -0.25, 0.0),
                    (1.0, 0.25, 0.0),
                    (-1.0, 0.25, 0.0),
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
            chosen: None,
            active_item: 0,
            items: vec![
                MenuItem::new(gl.clone(), &single, Screen::SingleGame),
                MenuItem::new(gl.clone(), &double, Screen::DoubleGame),
            ],
            spring_position: 0.0,
        }
    }
}

impl Playable for Menu {
    fn update(&mut self) {
        let target_position = 0.5 * self.active_item as f32;
        self.spring_position += (target_position - self.spring_position) * 0.1;

        for i in 0..self.items.len() {
            let target_zoom = if self.active_item == i { 1.2 } else { 1.0 };
            self.items[i].zoom += (target_zoom - self.items[i].zoom) * 0.1;
        }
    }

    fn draw(&mut self, screen_width: i32, screen_height: i32) {
        self.shader.bind();
        let aspect = screen_width as f32 / screen_height as f32;
        let mat = Mat4::from_scale(Vec3::new(1.0 / aspect, 1.0, 1.0))
            * Mat4::from_scale(Vec3::new(0.5, 0.5, 0.5));
        let mut transform = graphics::Transformer::new(self.shader.clone());
        transform.set(mat);

        transform.push();
        transform.transform(Mat4::from_translation(Vec3::new(
            0.0,
            self.spring_position,
            0.0,
        )));
        for i in 0..self.items.len() {
            self.items[i].texture.bind();
            transform.push();
            transform.transform(Mat4::from_scale(Vec3::new(
                self.items[i].zoom,
                self.items[i].zoom,
                self.items[i].zoom,
            )));
            self.item_model.render();
            transform.pop();
            transform.transform(Mat4::from_translation(Vec3::new(0.0, -0.5, 0.0)));
        }
        transform.pop();
    }

    fn input(&mut self, event: glutin::event::KeyboardInput) {
        if let glutin::event::ElementState::Released = event.state {
            return;
        }
        match event.virtual_keycode {
            Some(x) => match x {
                glutin::event::VirtualKeyCode::W | glutin::event::VirtualKeyCode::Up => {
                    if self.active_item > 0 {
                        self.active_item -= 1;
                    }
                }
                glutin::event::VirtualKeyCode::S | glutin::event::VirtualKeyCode::Down => {
                    if self.active_item < self.items.len() - 1 {
                        self.active_item += 1;
                    }
                }
                glutin::event::VirtualKeyCode::Space | glutin::event::VirtualKeyCode::Return => {
                    self.chosen = Some(self.items.swap_remove(self.active_item).target);
                }
                _ => (),
            },
            None => (),
        }
    }

    fn next_screen(&mut self) -> Option<crate::Screen> {
        self.chosen.take()
    }
}
