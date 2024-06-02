use crate::point_2d::Point2D;

pub struct VerletObject {
    pub position_current: Point2D,
    position_old: Point2D,
    acceleration: Point2D,
}

impl VerletObject {
    pub fn new(position: Point2D) -> Self {
        Self {
            position_current: position,
            position_old: position,
            acceleration: Point2D { x: 0.0, y: 0.0 },
        }
    }

    pub(crate) fn update_position(&mut self, dt: f32) {
        let translation= self.position_current - self.position_old; // should be a Vector2D
        self.position_old = self.position_current;
        self.position_current = self.position_current + translation + self.acceleration * (dt * dt);

        self.acceleration = Point2D { x: 0.0, y: 0.0 }
    }

    pub(crate) fn accelerate(&mut self, acceleration: Point2D) {
        self.acceleration = self.acceleration + acceleration;
    }
}
