use crate::verlet_object::{StartConditions, VerletObject};
use macroquad::color::Color;
use std::time::Duration;

pub struct BallCharacteristics {
    pub radius: f32,
    pub color: Color,
    pub id: u32,
}

pub struct Ball {
    pub characteristics: BallCharacteristics,
    pub verlet_object: VerletObject,
}

impl Ball {
    pub fn new(
        ball_characteristics: BallCharacteristics,
        start_conditions: StartConditions,
        dt: Duration,
    ) -> Self {
        Self {
            verlet_object: VerletObject::new(start_conditions, dt),
            characteristics: ball_characteristics,
        }
    }
}
