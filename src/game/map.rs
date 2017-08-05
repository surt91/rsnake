use std::collections::HashMap;
use rand::{thread_rng, Rng};

use super::orientation::{ Point, Direction, State };
use super::snake::Snake;

pub struct Map {
    map: HashMap<Point, State>
}

impl Map {
    pub fn new(s: &Snake) -> Map {
        let mut map = HashMap::new();
        for p in s.get_tail() {
            map.insert(*p, State::Snake);
        }
        Map {
            map,
        }
    }

    pub fn at(&self, p: &Point) -> State {
        *self.map.get(p).unwrap_or(&State::Empty)
    }

    pub fn consumed_food(&mut self) {
        // TODO
    }

    pub fn free(&mut self, p: Point) {
        self.map.remove(&p);
    }
}
