pub struct Grid {
    pub cells: Vec<Vec<Cell>>
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Grid { cells: vec![vec![Cell {}; width]; height] }
    }
}

pub struct Cell {

}