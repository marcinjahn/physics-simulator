use crate::ball::Ball;
use crate::point_2d::Point2D;
use macroquad::color::{BLACK, WHITE};
use macroquad::prelude::draw_poly;
use macroquad::shapes::{draw_poly_lines, draw_rectangle, draw_rectangle_lines};

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
        draw_poly(self.center.x, self.center.y, 50, self.radius, 0., BLACK);
        draw_poly_lines(
            self.center.x,
            self.center.y,
            50,
            self.radius + 5.,
            0.,
            10.,
            WHITE,
        );
    }

    fn calculate_new_position(&self, ball: &Ball) -> Option<Point2D> {
        let vector_from_constraint_to_ball =
            self.center.vector_to(&ball.verlet_object.position_current);

        let max_distance = self.radius - ball.characteristics.radius;

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

pub struct RectangularConstraint {
    top_left: Point2D,
    width: f32,
    height: f32,
}

impl RectangularConstraint {
    pub fn new(top_left: Point2D, width: f32, height: f32) -> Self {
        Self {
            top_left,
            width,
            height,
        }
    }
}

impl Constraint for RectangularConstraint {
    fn render(&self) {
        draw_rectangle(
            self.top_left.x,
            self.top_left.y,
            self.width,
            self.height,
            BLACK,
        );
        draw_rectangle_lines(
            self.top_left.x - 5.,
            self.top_left.y - 5.,
            self.width + 10.,
            self.height + 10.,
            10.,
            WHITE,
        );
    }

    fn calculate_new_position(&self, ball: &Ball) -> Option<Point2D> {
       let new_x = {
           if ball.verlet_object.position_current.x - ball.characteristics.radius < self.top_left.x {
               Some(self.top_left.x + ball.characteristics.radius)
           } else if ball.verlet_object.position_current.x + ball.characteristics.radius > self.top_left.x + self.width {
               Some(self.top_left.x + self.width - ball.characteristics.radius)
           } else {
               None
           }
       };

       let new_y = {
           if ball.verlet_object.position_current.y - ball.characteristics.radius < self.top_left.y {
               Some(self.top_left.y + ball.characteristics.radius)
           } else if ball.verlet_object.position_current.y + ball.characteristics.radius > self.top_left.y + self.height {
               Some(self.top_left.y + self.height - ball.characteristics.radius)
           } else {
               None
           }
       };

        if new_x.is_none() && new_y.is_none() {
            return None;
        }

        return Some(Point2D {
            x: new_x.unwrap_or(ball.verlet_object.position_current.x),
            y: new_y.unwrap_or(ball.verlet_object.position_current.y)
        });
    }
}
