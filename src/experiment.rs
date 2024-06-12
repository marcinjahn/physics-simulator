use crate::ball::Ball;
use crate::collisions::calculate_postcollision_positions;
use crate::constraints::Constraint;
use crate::point_2d::Point2D;
use std::time::Duration;

pub struct Experiment {
    pub balls: Vec<Ball>,
    pub(crate) constraint: Option<Box<dyn Constraint + Send + Sync>>,
    sub_steps: u8,
    step_dt: Duration,
    gravity: Point2D,
}

impl Experiment {
    pub fn new(
        frame_time: Duration,
        constraint: Option<Box<dyn Constraint + Send + Sync>>,
        sub_steps: u8
    ) -> Self {
        Self {
            gravity: Point2D { x: 0.0, y: 1000.0 },
            balls: vec![],
            constraint,
            sub_steps,
            step_dt: frame_time / sub_steps as u32
        }
    }

    pub fn update(&mut self) {
        for _ in 0..self.sub_steps {
            self.apply_gravity();
            self.apply_constraint();
            self.handle_collisions();
            self.update_positions();
        }
    }

    fn update_positions(&mut self) {
        self.balls.iter_mut().for_each(|ball| {
            ball.verlet_object.update_position(self.step_dt);
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

    fn handle_collisions(&mut self) {
        let len = self.balls.len();

        for i in 0..len {
            for j in i + 1..len {
                let (ball_1, ball_2) = get_two_mut(&mut self.balls, i, j);
                let postcollision_positions = calculate_postcollision_positions(ball_1, ball_2);

                if postcollision_positions.is_none() {
                    continue;
                }

                ball_1.verlet_object.position_current = postcollision_positions.unwrap().0;
                ball_2.verlet_object.position_current = postcollision_positions.unwrap().1;
            }
        }
    }
}

fn get_two_mut<T>(slice: &mut [T], i: usize, j: usize) -> (&mut T, &mut T) {
    let (left, right) = slice.split_at_mut(j);
    (&mut left[i], &mut right[0])
}
