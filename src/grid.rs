use crate::ball::Ball;

pub struct Grid {
    pub cells: Vec<Vec<Cell>>,
    width_pixels: usize,
    height_pixels: usize,
    cell_size: usize
}

impl Grid {
    pub fn new(width_pixels: usize, height_pixels: usize, cell_size: usize) -> Self {
        let grid_width = width_pixels / cell_size + (if width_pixels % cell_size == 0 { 0 } else { 1 });
        let grid_height = height_pixels / cell_size + (if height_pixels % cell_size == 0 { 0 } else { 1 });

        Grid {
            cells: vec![vec![Cell { balls: vec![] }; grid_width]; grid_height],
            width_pixels,
            height_pixels,
            cell_size
        }
    }

    pub fn add_ball(&mut self, ball: Ball) {
        let width_index = ball.verlet_object.position_current.x / self.cell_size as f32;
    }
}

pub struct Cell {
    balls: Vec<Ball>
}