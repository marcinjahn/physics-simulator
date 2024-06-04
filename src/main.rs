mod ball;
mod experiment;
mod point_2d;
mod renderer;
mod constraints;
mod verlet_object;
mod vector_2d;
mod collisions;

use crate::point_2d::Point2D;
use macroquad::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use ::rand;
use rand::{Rng, thread_rng};
use crate::ball::Ball;
use crate::constraints::CircularConstraint;
use crate::experiment::Experiment;
use crate::renderer::Renderer;

// SETTINGS
const FRAME_RATE: u32 = 60;
const BALL_RADIUS: f32 = 10.;
const MAX_BALLS_COUNT: usize = 500;
const BALL_START_X: f32 = 700.;
const BALL_START_Y: f32 = 400.;
const BALL_SPAWN_DELAY_MS: u64 = 50;



#[macroquad::main(window_conf)]
async fn main() {
    let frames_controller = FramesLimiter::new(FRAME_RATE);
    let mut experiment = Arc::new(Mutex::new(Experiment::new(Some(Box::new(
        CircularConstraint::new(Point2D { x: 500.0, y: 500.0 }, 300.0),
    )))));

    start_spawning_balls(&mut experiment);

    let mut frame_counter = 0;
    let frame_time = 1. / FRAME_RATE as f32;
    loop {
        frames_controller.control_frame(|| {
            let mut experiment = experiment.lock().unwrap();

            experiment.update(frame_time);

            let renderer = Renderer { experiment: &experiment, render_ball_ids: false };
            renderer.render();
        });

        render_fps_counter();

        next_frame().await;
        frame_counter += 1;
    }
}

fn start_spawning_balls(experiment: &mut Arc<Mutex<Experiment>>) {
    let experiment_clone = experiment.clone();

    thread::spawn(move || {
        let ball_start_position = Point2D { x: BALL_START_X, y: BALL_START_Y };

        loop {
            {
                let mut experiment = experiment_clone.lock().unwrap();
                let id = experiment.balls.len();
                experiment.balls.push(Ball::new(id as u32, ball_start_position, BALL_RADIUS, get_random_color()));

                if experiment.balls.len() >= MAX_BALLS_COUNT {
                    break;
                }
            }

            thread::sleep(Duration::from_millis(BALL_SPAWN_DELAY_MS));
        }
    });
}

fn render_fps_counter() {
    let fps = get_fps();
    draw_text(fps.to_string().as_str(), 10., 40., 40., WHITE);
}

fn window_conf() -> Conf {
    Conf {
        window_title: "2D Simulation".to_owned(),
        window_height: 1000,
        window_width: 1000,
        ..Default::default()
    }
}

pub struct FramesLimiter {
    pub max_frames_per_second: u32,
    min_frame_time_in_nanoseconds: u128,
}

impl FramesLimiter {
    pub fn new(max_frames_per_second: u32) -> Self {
        Self {
            max_frames_per_second,
            min_frame_time_in_nanoseconds: 1000_000_000 / (max_frames_per_second as u128),
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
        thread::sleep(Duration::from_nanos(nanos_left as u64));

        println!("elapsed: {}", start.elapsed().as_nanos());
    }
}

fn get_random_color() -> Color {
    Color {
        r: thread_rng().gen_range(0.0..1.0),
        g: thread_rng().gen_range(0.0..1.0),
        b: thread_rng().gen_range(0.0..1.0),
        a: 1.0,
    }
}