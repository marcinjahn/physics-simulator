mod physics;
mod vector_2d;
mod ball;

use std::future::Future;
use std::time::{Duration, Instant};
use macroquad::prelude::*;
use crate::ball::Ball;
use crate::physics::Experiment;
use crate::vector_2d::Vector2D;

#[macroquad::main("2D Simulation")]
async fn main() {
    let frames_controller = FramesLimiter::new(60);
    let mut experiment = Experiment::new();
    experiment.balls.push(Ball::new(Vector2D { x: 500.0, y: 200.0 }, 10.0));

    loop {
        frames_controller.control_frame(|| {
            clear_background(BLACK);

            experiment.update(get_frame_time());

            experiment.balls.iter().for_each(|ball| {
                draw_circle(ball.verlet_object.position_current.x, ball.verlet_object.position_current.y, ball.radius, BLUE);
            });
        });

        next_frame().await
    }
}

pub struct FramesLimiter {
    pub max_frames_per_second: u32,
    min_frame_time_in_nanoseconds: u128
}

impl FramesLimiter {
    pub fn new(max_frames_per_second: u32) -> Self {
       Self {
           max_frames_per_second,
           min_frame_time_in_nanoseconds: 1000_000_000 / (max_frames_per_second as u128)
       }
    }

    pub fn control_frame(&self, mut func: impl FnMut() -> ()) {
        let start = Instant::now();

        func();

        let elapsed = start.elapsed();

        if elapsed.as_nanos() >= self.min_frame_time_in_nanoseconds {
            return;
        }

        let nanos_left = self.min_frame_time_in_nanoseconds - elapsed.as_nanos();
        std::thread::sleep(Duration::from_nanos(nanos_left as u64));

        println!("elapsed: {}", start.elapsed().as_nanos());
    }
}