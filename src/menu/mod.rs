use super::*;
use glam::{Mat4, Vec3};

struct MenuItem {
    texture: graphics::Texture,
    target: Screen,
    zoom: f32,
}

impl MenuItem {
    pub fn new(
        gh: &mut graphics::GraphicsHandle,
        texture: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        target: Screen,
    ) -> Self {
        Self {
            texture: graphics::Texture::from_image(gh, texture).unwrap(),
            target,
            zoom: 1.0,
        }
    }
}

pub struct Menu {
    shader: Rc<RefCell<graphics::Shader>>,
    item_model: graphics::Model,
    active_item: usize,
    items: Vec<MenuItem>,
    chosen: Option<crate::Screen>,
    spring_position: f32,
}

impl Menu {
    pub fn new(
        gh: &mut graphics::GraphicsHandle,
        roman: &crate::resource::ResourceManager,
    ) -> Self {
        let single = roman.get_image("single.png");
        let double = roman.get_image("double.png");
        let vert = roman.get_text("default.vert");
        let frag = roman.get_text("menu.frag");
        let shader = Rc::new(RefCell::new(
            graphics::Shader::new(gh, &vert, &frag).expect("couldn't compile shader!"),
        ));
        Self {
            shader,
            item_model: graphics::Model::new(
                gh,
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
                MenuItem::new(gh, &single, Screen::SingleGame),
                MenuItem::new(gh, &double, Screen::DoubleGame),
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

    fn draw(&mut self, gh: &mut graphics::GraphicsHandle, screen_width: i32, screen_height: i32) {
        gh.push_shader(self.shader.clone());
        let aspect = screen_width as f32 / screen_height as f32;
        let mut mat = Mat4::from_scale(Vec3::new(1.0 / aspect, 1.0, 1.0))
            * Mat4::from_scale(Vec3::new(0.5, 0.5, 0.5));

        mat *= Mat4::from_translation(Vec3::new(0.0, self.spring_position, 0.0));
        for i in 0..self.items.len() {
            self.items[i].texture.bind(gh);
            gh.set_uniform(
                "view",
                mat * Mat4::from_scale(Vec3::new(
                    self.items[i].zoom,
                    self.items[i].zoom,
                    self.items[i].zoom,
                )),
            );
            self.item_model.render(gh);
            mat *= Mat4::from_translation(Vec3::new(0.0, -0.5, 0.0));
        }
        gh.pop_shader();
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
