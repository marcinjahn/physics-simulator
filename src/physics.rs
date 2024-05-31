use crate::ball::Ball;
use crate::vector_2d::Vector2D;

pub struct VerletObject {
    pub position_current: Vector2D,
    position_old: Vector2D,
    acceleration: Vector2D
}

impl VerletObject {
    pub fn new(position: Vector2D) -> Self {
        Self {
            position_current: position,
            position_old: position,
            acceleration: Vector2D { x: 0.0, y: 0.0 }
        }
    }

    fn update_position(&mut self, dt: f32) {
        let velocity = self.position_current - self.position_old;
        self.position_old = self.position_current;
        self.position_current = self.position_current + velocity + self.acceleration * dt * dt;

        self.acceleration = Vector2D { x: 0.0, y: 0.0 }
    }

    fn accelerate(&mut self, acceleration: Vector2D) {
        self.acceleration = self.acceleration + acceleration;
    }
}

pub struct Experiment {
    pub balls: Vec<Ball>,
    gravity: Vector2D,
}

impl Experiment {
    pub fn new() -> Self {
        Self {
            gravity: Vector2D { x: 0.0, y: 1000.0 },
            balls: vec![]
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.apply_gravity();
        self.update_positions(dt);
    }

    fn update_positions(&mut self, dt: f32) {
       self.balls.iter_mut().for_each(|ball| {
          ball.verlet_object.update_position(dt);
       })
    }

    fn apply_gravity(&mut self) {
        self.balls.iter_mut().for_each(|ball| {
            ball.verlet_object.accelerate(self.gravity);
        })
    }

}