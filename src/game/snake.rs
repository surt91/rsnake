use std::collections::VecDeque;

use super::orientation::{Point, Direction, State};
use super::map::Map;

pub struct Snake {
    tail: VecDeque<Point>,
    direction: Direction,
    length: usize,
}

impl Snake {
    pub fn new() -> Snake {
        let mut init = VecDeque::new();
        init.push_back(Point::new(7, 5));
        init.push_back(Point::new(6, 5));
        init.push_back(Point::new(5, 5));

        let length = init.len();

        Snake {
            tail: init,
            direction: Direction::E,
            length,
        }
    }

    fn head(&self) -> &Point {
        self.tail.front().expect("Zero Length Snake! Panic!")
    }

    pub fn step(&mut self, map: &mut Map) {
        let new_head = self.peek();
        self.tail.push_front(new_head);
        map.occupy(new_head);

        while self.tail.len() > self.length {
            let to_free = self.tail.pop_back().unwrap();
            map.free(to_free);
        }
    }

    pub fn peek(&mut self) -> Point {
        *self.head() + self.direction
    }

    pub fn turn(&mut self, dir: Direction) {
        self.direction = dir;
    }

    pub fn reverse(&self, dir: Direction) -> bool {
        let r = match dir {
            Direction::N => Direction::S,
            Direction::S => Direction::N,
            Direction::E => Direction::W,
            Direction::W => Direction::E,
        };

        self.direction == r
    }

    pub fn feed(&mut self) {
        self.length += 1;
    }

    pub fn get_tail(&self) -> &VecDeque<Point> {
        &self.tail
    }
}
