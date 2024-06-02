use macroquad::color::{BLACK, BLUE};
use macroquad::prelude::{clear_background, draw_circle};
use crate::physics::Experiment;

pub struct Renderer<'a> {
    pub experiment: &'a Experiment
}

impl Renderer<'_> {
    pub fn render(&self) {
        clear_background(BLACK);

        self.experiment.constraint.as_ref().map(|constraint| constraint.render());

        self.experiment.balls.iter().for_each(|ball| {
            draw_circle(
                ball.verlet_object.position_current.x,
                ball.verlet_object.position_current.y,
                ball.radius,
                ball.color,
            );
        });
    }
}