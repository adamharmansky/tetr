use glam::{Vec2, Vec4};

pub struct InfoText {
    pub text: String,
    pub time: std::time::Instant,
}

pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    gravity: Vec2,
    pub model: ParticleModel,
    pub size: f32,
    shrink_speed: f32,
}

pub enum ParticleModel {
    Colorful(Vec4),
    Star,
}

impl ParticleModel {
    pub fn random_color() -> Self {
        Self::Colorful(match rand::random::<u32>() % 6 {
            0 => Vec4::new(1.0, 0.5, 0.5, rand::random::<f32>()),
            1 => Vec4::new(0.5, 1.0, 0.5, rand::random::<f32>()),
            2 => Vec4::new(1.0, 1.0, 0.5, rand::random::<f32>()),
            3 => Vec4::new(0.5, 0.5, 1.0, rand::random::<f32>()),
            4 => Vec4::new(1.0, 0.5, 1.0, rand::random::<f32>()),
            5 => Vec4::new(0.5, 1.0, 1.0, rand::random::<f32>()),
            _ => Vec4::new(0.5, 0.5, 0.5, rand::random::<f32>()),
        })
    }
}

impl Particle {
    pub fn new(
        position: Vec2,
        velocity: Vec2,
        gravity: Vec2,
        size: f32,
        shrink_speed: f32,
        model: ParticleModel,
    ) -> Self {
        Self {
            position,
            velocity,
            gravity,
            model,
            size,
            shrink_speed,
        }
    }
}

pub struct BoardEffects {
    pub position: Vec2,
    pub velocity: Vec2,
    pub scale: f32,
    pub beat: f32,

    spring: f32,
    friction: f32,

    scale_spring: f32,
    scale_friction: f32,

    pub particles: Vec<Particle>,

    pub info: Option<InfoText>,
}

impl BoardEffects {
    pub fn new(spring: f32, friction: f32, scale_spring: f32, scale_friction: f32) -> Self {
        Self {
            position: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(0.0, 0.0),
            scale: 0.5,
            beat: 0.0,
            spring,
            friction,
            scale_spring,
            scale_friction,
            particles: Vec::new(),
            info: None,
        }
    }

    pub fn update(&mut self) {
        self.position += self.velocity;
        self.scale += self.beat;

        self.velocity -= self.position * self.spring;
        self.velocity *= 1.0 / (1.0 + self.friction);

        self.beat -= (self.scale - 1.0) * self.scale_spring;
        self.beat *= 1.0 / (1.0 + self.scale_friction);

        // O(1) removal
        for i in (0..self.particles.len()).rev() {
            if {
                let p = &mut self.particles[i];
                p.position += p.velocity;
                p.velocity += p.gravity;
                p.size -= p.shrink_speed;
                p.size < 0.0
            } {
                self.particles.swap_remove(i);
            }
        }

        let now = std::time::Instant::now();

        if let Some(x) = &mut self.info {
            if now.duration_since(x.time) > std::time::Duration::from_millis(1000) {
                self.info = None;
            }
        }
    }
}
