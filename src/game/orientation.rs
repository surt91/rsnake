use std::ops::Add;

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {x, y}
    }
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, other: Direction) -> Point {
        match other {
            Direction::N => Point {x: self.x , y: self.y - 1},
            Direction::S => Point {x: self.x , y: self.y + 1},
            Direction::E => Point {x: self.x + 1 , y: self.y},
            Direction::W => Point {x: self.x - 1 , y: self.y},
        }
    }
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    N,
    S,
    W,
    E,
}

#[derive(Hash, Debug, Clone, Copy)]
pub enum State {
    Empty,
    Wall,
    Snake,
    Food,
}
