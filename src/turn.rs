#[derive(Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
pub enum Move {
    Shot(Point),
    X(Point),
}
