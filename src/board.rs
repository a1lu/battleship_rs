use crate::turn;

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Empty,
    Miss,
    Hit,
    Destroyed,
    Ship
}

struct Board{
    board: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(size:u8) -> Board {
        let size = size as usize;
        let board = vec![vec![Cell::Empty; size]; size];
        Board {board}
    }

    fn insert(&mut self, point: &turn::Point, c: &Cell) {
        self.board[point.x as usize][point.y as usize] = *c;
    }

    pub fn hit(&mut self, point: &turn::Point) {
        self.insert(point, &Cell::Hit)
    }

    pub fn miss(&mut self, point: &turn::Point) {
        self.insert(point, &Cell::Miss)
    }

    pub fn destroyed(&mut self, points: &[turn::Point]) {
        for p in points {
            self.insert(p, &Cell::Destroyed)
        }
    }

    pub fn place_ship(&mut self, point: &turn::Point) {
        self.insert(point, &Cell::Ship)
    }

    pub fn board(&self) -> &Vec<Vec<u8>> {
        &self.board()
    }
}

