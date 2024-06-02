use crate::ball::Ball;
use crate::point_2d::Point2D;
use macroquad::color::WHITE;
use macroquad::prelude::draw_circle;
use macroquad::shapes::{draw_circle_lines, draw_poly};

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

    fn update_position(&mut self, dt: f32) {
        let velocity = self.position_current - self.position_old;
        self.position_old = self.position_current;
        self.position_current = self.position_current + velocity + self.acceleration * dt * dt;

        self.acceleration = Point2D { x: 0.0, y: 0.0 }
    }

    fn accelerate(&mut self, acceleration: Point2D) {
        self.acceleration = self.acceleration + acceleration;
    }
}

pub struct Experiment {
    pub balls: Vec<Ball>,
    gravity: Point2D,
    pub(crate) constraint: Option<Box<dyn Constraint + Send + Sync>>,
}

impl Experiment {
    pub fn new(constraint: Option<Box<dyn Constraint + Send + Sync>>) -> Self {
        Self {
            gravity: Point2D { x: 0.0, y: 1000.0 },
            balls: vec![],
            constraint,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.apply_gravity();
        self.apply_constraint();
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

    fn apply_constraint(&mut self) {
        if self.constraint.is_none() {
            return;
        }

        let constraint = self.constraint.as_ref().unwrap();

        self.balls.iter_mut().for_each(|ball| {
            let new_position = constraint.calculate_new_position(ball);

            if new_position.is_none() {
                return;
            }

            ball.verlet_object.position_current = new_position.unwrap();
        })
    }
}

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
