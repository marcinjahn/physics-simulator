use crate::ball::Ball;
use crate::point_2d::Point2D;
use macroquad::color::WHITE;
use macroquad::prelude::draw_poly;

pub trait Constraint {
    fn render(&self);

    fn calculate_new_position(&self, ball: &Ball) -> Option<Point2D>;
}

pub struct CircularConstraint {
    radius: f32,
    center: Point2D,
}

impl CircularConstraint {
    pub fn new(center: Point2D, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Constraint for CircularConstraint {
    fn render(&self) {
        draw_poly(self.center.x, self.center.y, 50, self.radius, 0., WHITE);
    }

    fn calculate_new_position(&self, ball: &Ball) -> Option<Point2D> {
        let vector_from_constraint_to_ball =
            self.center.vector_to(&ball.verlet_object.position_current);

        let max_distance = self.radius - ball.radius;

        if vector_from_constraint_to_ball.length <= max_distance {
            return None;
        }

        let normalized_vector = vector_from_constraint_to_ball.to_normalized();

        Some(Point2D {
            x: self.center.x + normalized_vector.x * max_distance,
            y: self.center.y + normalized_vector.y * max_distance,
        })
    }
}
