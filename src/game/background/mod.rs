use crate::graphics;
use glam::{Mat4, Vec2, Vec3, Vec4};
use std::cell::RefCell;
use std::rc::Rc;

struct Triangle {
    position: Vec2,
    scale: f32,
    velocity: Vec2,
    color: Vec4,
}

impl Triangle {
    pub fn new() -> Self {
        Triangle {
            position: Vec2::new(rand::random(), rand::random()),
            scale: (1.0 + rand::random::<f32>()) / 30.0,
            velocity: Vec2::new(0.0, (0.2 + rand::random::<f32>()) / 1600.0),
            color: Vec4::new(1.0, 1.0, 1.0, rand::random::<f32>() * 0.05),
        }
    }
}

pub struct Background {
    shader: Rc<RefCell<graphics::Shader>>,
    triangle: graphics::Model,

    background: graphics::Texture,
    background_shader: Rc<RefCell<graphics::Shader>>,
    background_model: graphics::Model,
    background_aspect: f32,

    triangles: Vec<Triangle>,
}

impl Background {
    pub fn new(
        gh: &mut crate::graphics::GraphicsHandle,
        roman: &crate::resource::ResourceManager,
    ) -> Self {
        let vert = roman.get_text("default.vert");
        let frag = roman.get_text("solid_color.frag");
        let shader = Rc::new(RefCell::new(
            graphics::Shader::new(gh, &vert, &frag).expect("couldn't compile background shader"),
        ));
        let triangle = graphics::Model::new(
            gh,
            &[(-1.0, -1.0, 0.0), (0.0, 0.732, 0.0), (1.0, -1.0, 0.0)],
            &[],
            &[],
        )
        .unwrap();
        let mut triangles = Vec::<Triangle>::new();
        for _ in 0..100 {
            triangles.push(Triangle::new());
        }

        let background = Self::pick_wapllpaper(roman);
        let background_aspect = background.width() as f32 / background.height() as f32;
        let background = graphics::Texture::from_image(gh, &background).unwrap();

        let frag = roman.get_text("texture.frag");
        let background_shader = Rc::new(RefCell::new(
            graphics::Shader::new(gh, &vert, &frag).expect("couldn't compile background shader"),
        ));
        let background_model = graphics::Model::new(
            gh,
            &[
                (0.0, 0.0, 0.0),
                (background_aspect, 0.0, 0.0),
                (0.0, 1.0, 0.0),
                (background_aspect, 0.0, 0.0),
                (background_aspect, 1.0, 0.0),
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
            shader,
            triangle,
            triangles,
            background,
            background_shader,
            background_model,
            background_aspect,
        }
    }

    fn update(&mut self) {
        for i in &mut self.triangles {
            i.position += i.velocity;
            if i.position.x > 1.0 {
                *i = Triangle::new();
                i.position.x = 0.0;
            }
            if i.position.x < 0.0 {
                *i = Triangle::new();
                i.position.x = 1.0;
            }
            if i.position.y > 1.0 {
                *i = Triangle::new();
                i.position.y = 0.0;
            }
            if i.position.y < 0.0 {
                *i = Triangle::new();
                i.position.y = 1.0;
            }
        }
    }

    pub fn draw(
        &mut self,
        gh: &mut crate::graphics::GraphicsHandle,
        screen_width: i32,
        screen_height: i32,
    ) {
        let aspect = screen_width as f32 / screen_height as f32;

        let mat = Mat4::from_scale(if aspect < 1.0 {
            Vec3::new(1.0 / aspect, 1.0, 1.0)
        } else {
            Vec3::new(1.0, aspect, 1.0)
        }) * Mat4::from_translation(Vec3::new(-self.background_aspect, -1.0, 0.0))
            * Mat4::from_scale(Vec3::new(2.0, 2.0, 1.0));

        {
            gh.push_shader(self.background_shader.clone());
            gh.set_uniform("color", Vec4::new(0.5, 0.5, 0.5, 1.0));
            gh.set_uniform("view", mat);
            self.background.bind(gh);
            self.background_model.render(gh);
            gh.pop_shader();
        }

        {
            gh.push_shader(self.shader.clone());
            let mat = Mat4::from_scale(Vec3::new(1.0, aspect * 1.0, 1.0))
                * Mat4::from_translation(Vec3::new(-1.5, -1.5, 0.0))
                * Mat4::from_scale(Vec3::new(3.0, 3.0, 3.0));
            for i in &self.triangles {
                let mat = mat
                    * Mat4::from_translation(Vec3::new(i.position.x, i.position.y, 0.0))
                    * Mat4::from_scale(Vec3::new(i.scale, i.scale, i.scale));
                gh.set_uniform("view", mat);
                gh.set_uniform("color", i.color);
                self.triangle.render(gh);
            }
            gh.pop_shader();
        }

        self.update();
    }

    fn pick_wapllpaper(
        roman: &crate::resource::ResourceManager,
    ) -> Rc<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>> {
        let backgrounds_dir = "backgrounds";
        let nobackgrounds = roman.get_image("no_background.png");
        if let Ok(dir) = std::fs::read_dir(backgrounds_dir) {
            let files = dir
                .map(|x| x.unwrap().file_name().into_string().unwrap())
                .collect::<Vec<String>>();
            if files.len() != 0 {
                let file = format!(
                    "{}/{}",
                    backgrounds_dir,
                    files[rand::random::<usize>() % files.len()]
                );

                Rc::new(
                    image::io::Reader::new(std::io::BufReader::new(
                        std::fs::File::open(file).unwrap(),
                    ))
                    .with_guessed_format()
                    .expect("Unrecognized background file format!")
                    .decode()
                    .expect("unable to load background image")
                    .into_rgba8(),
                )
            } else {
                nobackgrounds
            }
        } else {
            nobackgrounds
        }
    }
}
