use crate::ball::Ball;
use crate::collisions::calculate_postcollision_positions;
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
            gravity: Point2D { x: 0.0, y: 500.0 },
            balls: vec![],
            constraint,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.apply_gravity();
        self.apply_constraint();
        self.handle_collisions();
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

    fn handle_collisions(&mut self) {
        let len = self.balls.len();

        for i in 0..len {
            for j in i+1..len {

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
