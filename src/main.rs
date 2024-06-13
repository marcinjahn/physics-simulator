mod ball;
mod collisions;
mod constraints;
mod experiment;
mod frame_limiter;
mod point_2d;
mod renderer;
mod vector_2d;
mod verlet_object;
mod grid;

use crate::ball::BallCharacteristics;
use crate::constraints::{CircularConstraint, RectangularConstraint};
use crate::experiment::Experiment;
use crate::frame_limiter::FramesLimiter;
use crate::point_2d::Point2D;
use crate::renderer::Renderer;
use crate::vector_2d::Vector2D;
use crate::verlet_object::StartConditions;
use ::rand;
use macroquad::prelude::*;
use rand::{thread_rng, Rng};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// SETTINGS
const FRAME_RATE: u32 = 60;
const BALL_RADIUS: f32 = 10.;
const MAX_BALLS_COUNT: usize = 500;
const BALL_START_X: f32 = 501.;
const BALL_START_Y: f32 = 400.;
const BALL_SPAWN_DELAY_MS: u64 = 50;
const BALL_RADIUS_RANGE: (f32, f32) = (5., 20.);

#[macroquad::main(window_conf)]
async fn main() {
    let frames_controller = FramesLimiter::new(FRAME_RATE);
    // let mut experiment = Arc::new(Mutex::new(Experiment::new(
    //     Duration::from_secs_f32(1. / FRAME_RATE as f32),
    //     Some(Box::new(CircularConstraint::new(
    //         Point2D { x: 500.0, y: 500.0 },
    //         300.0,
    //     ))),
    //     4,
    // )));


    let mut experiment = Arc::new(Mutex::new(Experiment::new(
        Duration::from_secs_f32(1. / FRAME_RATE as f32),
        Some(Box::new(RectangularConstraint::new(
            Point2D { x: 100.0, y: 100.0 },
            700.0,
            500.0
        ))),
        4,
    )));

    start_spawning_balls(&mut experiment);

    let mut frame_counter = 0;
    loop {
        frames_controller.control_frame(|| {
            let mut experiment = experiment.lock().unwrap();

            experiment.update();

            let renderer = Renderer {
                experiment: &experiment,
                render_ball_ids: false,
            };
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
        let ball_start_position = Point2D {
            x: BALL_START_X,
            y: BALL_START_Y,
        };

        loop {
            {
                let mut experiment = experiment_clone.lock().unwrap();
                let id = experiment.balls.len();
                let experiment_time = experiment.experiment_time;

                experiment.add_ball(
                    BallCharacteristics {
                        radius: get_radius(),
                        color: get_random_color(),
                        id: id as u32,
                    },
                    StartConditions {
                        position: ball_start_position,
                        velocity: get_velocity(experiment_time),
                    },
                );

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

fn get_random_color() -> Color {
    Color {
        r: thread_rng().gen_range(0.0..1.0),
        g: thread_rng().gen_range(0.0..1.0),
        b: thread_rng().gen_range(0.0..1.0),
        a: 1.0,
    }
}

fn get_velocity(elapsed_time: Duration) -> Vector2D {
    let mut asin_arg = elapsed_time.as_secs_f32() % 1.;
    if elapsed_time.as_secs() % 2 == 0 {
        asin_arg = -1. + asin_arg
    };

    // println!("{}", asin_arg);

    let angle = asin_arg.asin() * 2.;

    println!("{}; {}", asin_arg, angle);

    Vector2D::new(angle.cos(), angle.sin()) * 1000.
}

fn get_radius() -> f32 {
    let random: f32 = rand::random::<u8>() as f32 / u8::MAX as f32;
    let range = BALL_RADIUS_RANGE.1 - BALL_RADIUS_RANGE.0;

    BALL_RADIUS_RANGE.0 + (random as f32 * range)
}
