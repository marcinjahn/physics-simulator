use crate::physics::VerletObject;
use crate::vector_2d::Vector2D;

pub struct Ball {
   pub verlet_object: VerletObject,
   pub radius: f32,
}

impl Ball {
   pub fn new(position: Vector2D, radius: f32) -> Self {
      Self {
         verlet_object: VerletObject::new(position),
         radius
      }
   }
}

//impl Ball {
//    fn update(nanoseconds_passed: u32) {
//                
//    }
//}