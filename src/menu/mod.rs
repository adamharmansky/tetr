use super::*;
use crate::text;
use glam::{Mat4, Vec3, Vec4};
use std::rc::Rc;

struct MenuItem {
    target: Screen,
    zoom: f32,
    text: String,
    icon: graphics::Texture,
    color: Vec4,
}

impl MenuItem {
    pub fn new(target: Screen, text: String, icon: graphics::Texture, color: Vec4) -> Self {
        Self {
            text,
            target,
            zoom: 1.0,
            icon,
            color,
        }
    }
}

pub struct Menu {
    shader: Rc<RefCell<graphics::Shader>>,
    square: graphics::Model,
    background: graphics::Texture,
    scroll: f32,
    color: Vec4,

    active_item: usize,
    items: Vec<MenuItem>,
    chosen: Option<crate::Screen>,
    spring_position: f32,
    tr: Rc<text::TextRenderer>,
    font: text::Font,
}

impl Menu {
    pub fn new(
        gh: &mut graphics::GraphicsHandle,
        roman: &crate::resource::ResourceManager,
        tr: Rc<text::TextRenderer>,
    ) -> Self {
        let font = roman.get_binary("comfortaa-bold.ttf").clone();
        let font = text::Font::new(&tr, font, 100).unwrap();

        let vs = roman.get_text("default.vert");
        let fs = roman.get_text("texture_optional.frag");

        let background = graphics::Texture::from_image(gh, &roman.get_image("tetris.png")).unwrap();

        let shader = Rc::new(RefCell::new(graphics::Shader::new(gh, &vs, &fs).unwrap()));

        let square = graphics::Model::new(
            gh,
            &[
                (0.0, 0.0, 0.0),
                (1.0, 0.0, 0.0),
                (0.0, 1.0, 0.0),
                (1.0, 0.0, 0.0),
                (1.0, 1.0, 0.0),
                (0.0, 1.0, 0.0),
            ],
            &[
                (0.0, 1.0),
                (1.0, 1.0),
                (0.0, 0.0),
                (1.0, 1.0),
                (1.0, 0.0),
                (0.0, 0.0),
            ],
            &[],
        )
        .unwrap();

        Self {
            chosen: None,
            active_item: 0,
            items: vec![
                MenuItem::new(
                    Screen::SingleGame,
                    String::from("single player"),
                    graphics::Texture::from_image(gh, &roman.get_image("single.png")).unwrap(),
                    Vec4::new(0.1, 0.6, 0.9, 1.0),
                ),
                MenuItem::new(
                    Screen::DoubleGame,
                    String::from("split screen"),
                    graphics::Texture::from_image(gh, &roman.get_image("double.png")).unwrap(),
                    Vec4::new(1.0, 0.0, 1.0, 1.0),
                ),
                MenuItem::new(
                    Screen::Exit,
                    String::from("exit to desktop"),
                    graphics::Texture::from_image(gh, &roman.get_image("exit.png")).unwrap(),
                    Vec4::new(1.0, 0.15, 0.1, 1.0),
                ),
            ],
            spring_position: 0.0,
            tr,
            font,
            shader,
            square,
            background,
            scroll: 0.0,
            color: Vec4::new(0.0, 0.0, 0.0, 1.0),
        }
    }
}

impl Playable for Menu {
    fn update(&mut self) {
        let target_position = self.active_item as f32;
        self.spring_position += (target_position - self.spring_position) * 0.5;
        self.scroll += 0.0001;
        if self.scroll >= 1.0 {
            self.scroll = 0.0;
        }

        for i in 0..self.items.len() {
            let target_zoom = if self.active_item == i { 2.0 } else { 1.0 };
            self.items[i].zoom += (target_zoom - self.items[i].zoom) * 0.2;
        }

        self.color += (self.items[self.active_item].color - self.color) * 0.1;
    }

    fn draw(&mut self, gh: &mut graphics::GraphicsHandle, screen_width: i32, screen_height: i32) {
        let aspect = screen_width as f32 / screen_height as f32;

        // draw the background
        let mat = Mat4::from_scale(Vec3::new(1.05 / aspect, 1.05, 1.0))
            * Mat4::from_translation(Vec3::new(-8.0, -1.0, 0.0))
            * Mat4::from_scale(Vec3::new(8.0, 2.0, 1.0));
        gh.bind(self.shader.clone());
        gh.set_uniform("enable_texture", true);
        gh.set_uniform("color", self.color);
        self.background.bind(gh);
        gh.set_uniform(
            "view",
            mat * Mat4::from_translation(Vec3::new(self.scroll - 1.0, 0.0, 0.0)),
        );
        self.square.render(gh);
        gh.set_uniform(
            "view",
            mat * Mat4::from_translation(Vec3::new(self.scroll, 0.0, 0.0)),
        );
        self.square.render(gh);
        gh.set_uniform(
            "view",
            mat * Mat4::from_translation(Vec3::new(self.scroll + 1.0, 0.0, 0.0)),
        );
        self.square.render(gh);
        gh.unbind();

        let mut mat = Mat4::from_scale(Vec3::new(1.0, aspect, 1.0))
            * Mat4::from_translation(Vec3::new(-1.0, self.spring_position * 0.2, 0.0));
        for i in 0..self.items.len() {
            {
                let mat = mat
                    * Mat4::from_translation(Vec3::new(0.0, -0.1 * self.items[i].zoom, 0.0))
                    * Mat4::from_scale(Vec3::new(
                        self.items[i].zoom,
                        self.items[i].zoom,
                        self.items[i].zoom,
                    ));

                gh.bind(self.shader.clone());

                // draw the background
                let w = self.tr.get_width(gh, &mut self.font, &self.items[i].text) / 2000.0;
                gh.set_uniform("view", mat * Mat4::from_scale(Vec3::new(w + 0.2, 0.1, 1.0)));
                gh.set_uniform("enable_texture", false);
                gh.set_uniform("color", self.items[i].color);
                self.square.render(gh);

                // draw the icon
                gh.set_uniform(
                    "view",
                    mat * Mat4::from_translation(Vec3::new(0.025, 0.0, 0.0))
                        * Mat4::from_scale(Vec3::new(0.1, 0.1, 0.1)),
                );
                gh.set_uniform("enable_texture", true);
                gh.set_uniform("color", Vec4::new(1.0, 1.0, 1.0, 1.0));
                self.items[i].icon.bind(gh);
                self.square.render(gh);

                gh.unbind();

                // draw the text
                self.tr.draw(
                    gh,
                    &mut self.font,
                    mat * Mat4::from_translation(Vec3::new(0.15, 0.025, 0.0))
                        * Mat4::from_scale(Vec3::new(1.0 / 2000.0, 1.0 / 2000.0, 1.0 / 2000.0)),
                    Vec4::new(1.0, 1.0, 1.0, 1.0),
                    &self.items[i].text,
                );
            }
            mat *= Mat4::from_translation(Vec3::new(0.0, -0.1 * self.items[i].zoom, 0.0));
        }
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
