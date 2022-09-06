use super::*;
use glam::{Mat4, Vec2, Vec3, Vec4};

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
    shader: Rc<graphics::Shader>,
    transform: graphics::Transformer,
    color_uniform: graphics::UniformHandle,
    triangle: graphics::Model,
    triangles: Vec<Triangle>,
    gl: Rc<gl33::GlFns>,

    background: graphics::Texture,
    background_shader: Rc<graphics::Shader>,
    background_transform: graphics::Transformer,
    background_model: graphics::Model,
    background_color: graphics::UniformHandle,
    background_aspect: f32,
}

impl Background {
    pub fn new(gl: Rc<gl33::GlFns>) -> Self {
        let shader = Rc::new(
            graphics::Shader::new(
                gl.clone(),
                include_str!("../shaders/default.vert"),
                include_str!("../shaders/solid_color.frag"),
            )
            .expect("couldn't compile background shader"),
        );
        let transform = graphics::Transformer::new(shader.clone());
        let color_uniform = graphics::UniformHandle::new(shader.clone(), "kolor");
        let triangle = graphics::Model::new(
            gl.clone(),
            &[(-1.0, -1.0, 0.0), (0.0, 0.732, 0.0), (1.0, -1.0, 0.0)],
            &[],
            &[],
        )
        .unwrap();
        let mut triangles = Vec::<Triangle>::new();
        for _ in 0..100 {
            triangles.push(Triangle::new());
        }

        let background = Self::pick_wapllpaper();
        let background_aspect = background.width() as f32 / background.height() as f32;
        let background = graphics::Texture::from_image(gl.clone(), background).unwrap();

        let background_shader = Rc::new(
            graphics::Shader::new(
                gl.clone(),
                include_str!("../shaders/default.vert"),
                include_str!("../shaders/texture.frag"),
            )
            .expect("couldn't compile background shader"),
        );
        let background_transform = graphics::Transformer::new(background_shader.clone());
        let background_model = graphics::Model::new(
            gl.clone(),
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
        let background_color = graphics::UniformHandle::new(background_shader.clone(), "kolor");

        Self {
            shader,
            transform,
            color_uniform,
            triangle,
            triangles,
            gl,
            background,
            background_shader,
            background_transform,
            background_model,
            background_color,
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

    pub fn draw(&mut self, screen_width: i32, screen_height: i32) {
        let aspect = screen_width as f32 / screen_height as f32;

        unsafe {
            self.gl.ClearColor(0.0, 0.0, 0.0, 1.0);
            self.gl
                .Clear(gl33::GL_COLOR_BUFFER_BIT | gl33::GL_DEPTH_BUFFER_BIT);
        }

        self.background_shader.bind();
        self.background_transform
            .set(Mat4::from_scale(if aspect < 1.0 {
                Vec3::new(1.0 / aspect, 1.0, 1.0)
            } else {
                Vec3::new(1.0, aspect, 1.0)
            }));
        self.background_transform
            .transform(Mat4::from_translation(Vec3::new(
                -self.background_aspect,
                -1.0,
                0.0,
            )));
        self.background_transform
            .transform(Mat4::from_scale(Vec3::new(2.0, 2.0, 1.0)));

        self.background_color.set(Vec4::new(0.5, 0.5, 0.5, 1.0));
        self.background.bind();
        self.background_model.render();

        self.shader.bind();
        self.transform
            .set(Mat4::from_scale(Vec3::new(1.0, aspect * 1.0, 1.0)));
        self.transform
            .transform(Mat4::from_translation(Vec3::new(-1.5, -1.5, 0.0)));
        self.transform
            .transform(Mat4::from_scale(Vec3::new(3.0, 3.0, 3.0)));
        for i in &self.triangles {
            self.transform.push();
            self.transform.transform(Mat4::from_translation(Vec3::new(
                i.position.x,
                i.position.y,
                0.0,
            )));
            self.transform
                .transform(Mat4::from_scale(Vec3::new(i.scale, i.scale, i.scale)));
            self.color_uniform.set(i.color);
            self.triangle.render();
            self.transform.pop();
        }

        self.update();
    }

    pub fn change_wallpaper(&mut self) {
        let background = Self::pick_wapllpaper();
        self.background_aspect = background.width() as f32 / background.height() as f32;
        self.background = graphics::Texture::from_image(self.gl.clone(), background).unwrap();
    }

    fn pick_wapllpaper() -> image::DynamicImage {
        let backgrounds_dir = "backgrounds";
        let nobackgrounds = include_bytes!("../assets/no_background.png");
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

                image::io::Reader::new(std::io::BufReader::new(std::fs::File::open(file).unwrap()))
                    .with_guessed_format()
                    .expect("Unrecognized background file format!")
                    .decode()
                    .expect("unable to load background image")
            } else {
                image::io::Reader::new(std::io::Cursor::new(nobackgrounds))
                    .with_guessed_format()
                    .expect("Unrecognized background file format!")
                    .decode()
                    .unwrap()
            }
        } else {
            image::io::Reader::new(std::io::Cursor::new(nobackgrounds))
                .with_guessed_format()
                .expect("Unrecognized background file format!")
                .decode()
                .unwrap()
        }
    }
}
