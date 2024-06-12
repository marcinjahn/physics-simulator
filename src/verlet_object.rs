use crate::point_2d::Point2D;
use crate::vector_2d::Vector2D;
use std::time::Duration;

pub struct VerletObject {
    pub position_current: Point2D,
    position_old: Point2D,
    acceleration: Point2D,
}

impl VerletObject {
    pub fn new(start_conditions: StartConditions, dt: Duration) -> Self {
        Self {
            position_current: start_conditions.position,
            position_old: start_conditions.position - start_conditions.velocity * dt.as_secs_f32(),
            acceleration: Point2D { x: 0.0, y: 0.0 },
        }
    }

    pub(crate) fn update_position(&mut self, dt: Duration) {
        let translation = (self.position_current - self.position_old).as_vector();
        self.position_old = self.position_current;

        let dt_secs = dt.as_secs_f32();

        self.position_current =
            self.position_current + translation + self.acceleration * (dt_secs * dt_secs);

        self.acceleration = Point2D { x: 0.0, y: 0.0 }
    }

    pub(crate) fn accelerate(&mut self, acceleration: Point2D) {
        self.acceleration = self.acceleration + acceleration;
    }
}

pub struct StartConditions {
    pub position: Point2D,
    pub velocity: Vector2D,
}
