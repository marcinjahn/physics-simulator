pub struct Vector2D {
    pub x: f32,
    pub y: f32,
    pub length: f32,
}

impl Vector2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            length: Vector2D::length(x, y),
        }
    }

    fn length(x: f32, y: f32) -> f32 {
        (x.powi(2) + y.powi(2)).sqrt()
    }

    pub fn to_normalized(&self) -> Self {
        Self {
            x: self.x / self.length,
            y: self.y / self.length,
            length: 1.0,
        }
    }
}
