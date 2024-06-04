use crate::point_2d::Point2D;
use crate::verlet_object::VerletObject;
use macroquad::color::Color;

pub struct Ball {
    pub verlet_object: VerletObject,
    pub radius: f32,
    pub color: Color,
    pub id: u32,
}

impl Ball {
    pub fn new(id: u32, position: Point2D, radius: f32, color: Color) -> Self {
        Self {
            verlet_object: VerletObject::new(position),
            radius,
            color,
            id,
        }
    }
}
