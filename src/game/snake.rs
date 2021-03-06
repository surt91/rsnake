use std::collections::VecDeque;

use super::orientation::{Point, Direction};
use super::map::Map;

pub struct Snake {
    tail: VecDeque<Point>,
    pub direction: Direction,
    pub last_direction: Direction,
    pub length: usize,
    size: (u32, u32),
}

impl Snake {
    pub fn new(size: (u32, u32)) -> Snake {
        let mut init = VecDeque::new();
        init.push_back(Point::new(7, 5));
        init.push_back(Point::new(6, 5));
        init.push_back(Point::new(5, 5));

        let length = init.len();

        Snake {
            tail: init,
            direction: Direction::E,
            last_direction: Direction::E,
            length,
            size,
        }
    }

    pub fn head(&self) -> &Point {
        self.tail.front().expect("Zero Length Snake! Panic!")
    }

    pub fn end(&self) -> &Point {
        self.tail.back().expect("Zero Length Snake! Panic!")
    }

    pub fn step(&mut self, map: &mut Map) {
        let new_head = self.peek();
        self.tail.push_front(new_head);
        map.occupy(new_head);

        while self.tail.len() > self.length {
            let to_free = self.tail.pop_back().unwrap();
            map.free(to_free);
        }

        self.last_direction = self.direction;
    }

    pub fn peek(&self) -> Point {
        let mut p = *self.head() + self.direction;

        // periodic boundaries
        if p.x == self.size.0 as i32 {
            p.x = 0;
        } else if p.x == -1 {
            p.x = self.size.0 as i32 - 1;
        }

        if p.y == self.size.1 as i32 {
            p.y = 0;
        } else if p.y == -1 {
            p.y = self.size.1 as i32 - 1;
        }

        p
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

        self.last_direction == r
    }

    pub fn left(&self) -> Direction {
        match self.direction {
            Direction::N => Direction::W,
            Direction::S => Direction::E,
            Direction::E => Direction::N,
            Direction::W => Direction::S,
        }
    }

    pub fn right(&self) -> Direction {
        match self.direction {
            Direction::N => Direction::E,
            Direction::S => Direction::W,
            Direction::E => Direction::S,
            Direction::W => Direction::N,
        }
    }

    pub fn turn_left(&mut self) {
        let d = self.left();
        self.turn(d);
    }

    pub fn turn_right(&mut self) {
        let d = self.right();
        self.turn(d);
    }

    pub fn feed(&mut self) {
        self.length += 1;
    }

    pub fn get_tail(&self) -> &VecDeque<Point> {
        &self.tail
    }
}
