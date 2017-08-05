use std::collections::HashMap;
use rand::{thread_rng, Rng};

use super::orientation::{ Point, Direction, State };
use super::snake::Snake;

pub struct Map {
    pub size: (u32, u32),
    map: HashMap<Point, State>,
    walls: Vec<Point>,
    food: Point,
}

impl Map {
    pub fn new(size: (u32, u32), s: &Snake) -> Map {
        let mut map = HashMap::new();
        for p in s.get_tail() {
            map.insert(*p, State::Snake);
        }

        let mut m = Map {
            size,
            map,
            walls: vec![],
            food: Point::new(0, 0),
        };

        m.generate_food();
        m
    }

    pub fn at(&self, p: &Point) -> State {
        *self.map.get(p).unwrap_or(&State::Empty)
    }

    pub fn generate_food(&mut self) -> Point {
        let mut rng = thread_rng();
        let mut p;
        while {
            let x = rng.gen_range::<i32>(0, self.size.0 as i32 - 1);
            let y = rng.gen_range::<i32>(0, self.size.1 as i32 - 1);
            p = Point::new(x, y);
            self.at(&p) != State::Empty
        } {}
        self.food = p;
        self.map.insert(p, State::Food);
        self.food
    }

    pub fn consumed_food(&mut self) {
        self.generate_food();
    }

    pub fn get_walls(&self) -> &Vec<Point> {
        &self.walls
    }

    pub fn get_food(&self) -> &Point {
        &self.food
    }

    pub fn free(&mut self, p: Point) {
        self.map.remove(&p);
    }
    
    pub fn occupy(&mut self, p: Point) {
        self.map.insert(p, State::Snake);
    }
}
