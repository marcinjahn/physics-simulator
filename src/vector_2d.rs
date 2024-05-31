use std::ops;

#[derive(Copy, Clone)]
pub struct Vector2D{
   pub x: f32,
   pub y: f32,
}

impl ops::Add<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Vector2D) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl ops::Sub<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Vector2D) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl ops::Mul<f32> for Vector2D {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
} 