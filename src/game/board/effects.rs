use glam::Vec2;

pub struct BoardEffects {
    pub position: Vec2,
    pub velocity: Vec2,
    pub scale: f32,
    pub beat: f32,

    spring: f32,
    friction: f32,

    scale_spring: f32,
    scale_friction: f32,
}

impl BoardEffects {
    pub fn new(spring: f32, friction: f32, scale_spring: f32, scale_friction: f32) -> Self {
        Self {
            position: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(0.0, 0.0),
            scale: 1.0,
            beat: 0.0,
            spring,
            friction,
            scale_spring,
            scale_friction,
        }
    }

    pub fn update(&mut self) {
        self.position += self.velocity;
        self.scale += self.beat;

        self.velocity -= self.position * self.spring;
        self.velocity *= 1.0 / (1.0 + self.friction);

        self.beat -= (self.scale - 1.0) * self.scale_spring;
        self.beat *= 1.0 / (1.0 + self.scale_friction);
    }
}
