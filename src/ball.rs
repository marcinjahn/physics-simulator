use macroquad::color::Color;
use crate::physics::VerletObject;
use crate::point_2d::Point2D;

pub struct Ball {
    pub verlet_object: VerletObject,
    pub radius: f32,
    pub color: Color
}

impl Ball {
    pub fn new(position: Point2D, radius: f32, color: Color) -> Self {
        Self {
            verlet_object: VerletObject::new(position),
            radius,
            color
        }
    }
}

//impl Ball {
//    fn update(nanoseconds_passed: u32) {
//
//    }
//}
