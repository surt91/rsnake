use std::ops::Add;
use std::collections::{HashSet, BinaryHeap};
use std::cmp::Ordering;

use super::map::Map;

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

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Point {
    pub fn neighbors(&self) -> Neighbor {
        Neighbor::direct(*self)
    }

    pub fn neighbors2(&self) -> Neighbor {
        Neighbor::diagonal(*self)
    }
}

pub struct Neighbor {
    pos: Point,
    offsets: Vec<Point>,
}

impl Neighbor {
    fn direct(pos: Point) -> Neighbor {
        let offsets = vec![
            Point::new(0, 1),
            Point::new(1, 0),
            Point::new(0, -1),
            Point::new(-1, 0),
        ];

        Neighbor {
            pos,
            offsets
        }
    }

    fn diagonal(pos: Point) -> Neighbor {
        let offsets = vec![
            Point::new(0, 1),
            Point::new(1, 1),
            Point::new(1, 0),
            Point::new(1, -1),
            Point::new(0, -1),
            Point::new(-1, -1),
            Point::new(-1, 0),
            Point::new(-1, 1),
        ];

        Neighbor {
            pos,
            offsets
        }
    }
}

impl Iterator for Neighbor {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        match self.offsets.pop() {
            Some(o) => Some(self.pos + o),
            None => None
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

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Empty,
    Wall,
    Snake,
    Food,
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reachable {
    Yes,
    No,
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
struct Thingy {
    distance: i32,
    pos: Point,
}

impl Thingy {
    fn new(point: &Point, target: &Point, map: &Map) -> Thingy {
        Thingy {
            // - because the heap is a max-heap, but we want a min heap
            distance: -map.manhattan(point, target),
            pos: *point
        }
    }
}

impl PartialOrd for Thingy {
    fn partial_cmp(&self, other: &Thingy) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Thingy {
    fn cmp(&self, other: &Thingy) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

pub fn best_first_search(start: &Point, target: &Point, map: &Map) -> Reachable {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut tmp_visited: Vec<Point> = Vec::new();
    let mut q: BinaryHeap<Thingy> = BinaryHeap::new();

    visited.insert(*start);
    q.push(Thingy::new(start, target, map));

    loop {
        let nearest = match q.pop() {
            None => {
                println!("trapped, space: {}", visited.len());
                map.print();
                return Reachable::No
            }
            Some(x) => x,
        };

        if nearest.distance == -1 {
            // distance is 1 -> found
            // println!("reachable");
            return Reachable::Yes
        } else if nearest.distance.abs() as u32 > map.size.0 + map.size.1 {
            // distance larger than the board -> something is wrong!
            println!("too long");
            panic!();
            return Reachable::No
        }

        // print!("{:?}: ", nearest);

        for n in nearest.pos
                        .neighbors()
                        .filter(|x| !visited.contains(&map.normalize(x))
                                 && match map.at(x) {
                                        State::Wall | State::Snake => false,
                                        State::Food | State::Empty => true
                                    }
                               )
        {
            let n = map.normalize(&n);
            // print!(" {:?}", n);
            tmp_visited.push(n);
            q.push(Thingy::new(&n, target, map))
        }
        // print!("\n");

        for i in &tmp_visited {
            visited.insert(*i);
        }
        tmp_visited.clear();
    }
}
