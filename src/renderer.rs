use macroquad::color::{BLACK, WHITE};
use macroquad::prelude::{clear_background, draw_circle};
use macroquad::text::draw_text;
use crate::experiment::Experiment;

pub struct Renderer<'a> {
    pub experiment: &'a Experiment,
    pub render_ball_ids: bool
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

            if !self.render_ball_ids {
                return;
            }

            let id_x = ball.verlet_object.position_current.x - ball.radius / 4.0;
            let id_y = ball.verlet_object.position_current.y + ball.radius / 4.0;
            draw_text(ball.id.to_string().as_str(), id_x, id_y, ball.radius, WHITE)
        });
    }
}