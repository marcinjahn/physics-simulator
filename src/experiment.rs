use crate::ball::Ball;
use crate::constraints::Constraint;
use crate::point_2d::Point2D;

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