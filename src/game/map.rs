use std::collections::HashMap;
use rand::{thread_rng, Rng};

use super::orientation::{ Point, Direction, State };
use super::snake::Snake;

pub struct Map {
    pub size: (u32, u32),
    pub map: HashMap<Point, State>,
    walls: Vec<Point>,
    pub food: Point,
}

impl Map {
    pub fn new(size: (u32, u32)) -> Map {
        let map = HashMap::new();

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
        let periodic = self.normalize(p);
        *self.map.get(&periodic).unwrap_or(&State::Empty)
    }

    pub fn manhattan(&self, p1: &Point, p2: &Point) -> i32 {
        let mut dx = p1.x - p2.x;
        let mut dy = p1.y - p2.y;

        // get shortest way over periodic boundaries
        if dx as f64 > self.size.0 as f64 / 2. {
            dx -= self.size.0 as i32;
        }
        if dy as f64 > self.size.1 as f64 / 2. {
            dy -= self.size.1 as i32;
        }

        dx.abs() + dy.abs()
    }

    pub fn print(&self) {
        for j in 0..self.size.1 {
            for i in 0..self.size.0 {
                match self.at(&Point::new(i as i32, j as i32)) {
                    State::Snake => print!("s"),
                    State::Food => print!("f"),
                    State::Empty => print!(" "),
                    State::Wall => print!("#"),
                }
            }
            println!("");
        }
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
        let p = self.normalize(&p);
        self.map.remove(&p);
    }

    pub fn occupy(&mut self, p: Point) {
        let p = self.normalize(&p);
        self.map.insert(p, State::Snake);
    }

    /// apply periodic boundaries
    pub fn normalize(&self, p: &Point) -> Point {
        let mut periodic = *p;

        if periodic.x >= self.size.0 as i32 {
            periodic.x -= self.size.0 as i32
        } else if periodic.x < 0 {
            periodic.x += self.size.0 as i32
        }
        if periodic.y >= self.size.1 as i32 {
            periodic.y -= self.size.1 as i32
        } else if periodic.y < 0 {
            periodic.y += self.size.1 as i32
        }

        periodic
    }
}

#[test]
fn test_normalize() {
    let map = Map::new((3, 3));
    assert_eq!(map.normalize(&Point::new(3, 3)), Point::new(0, 0));
    assert_eq!(map.normalize(&Point::new(2, 2)), Point::new(2, 2));
    assert_eq!(map.normalize(&Point::new(-1, 2)), Point::new(2, 2));
    assert_eq!(map.normalize(&Point::new(-1, -1)), Point::new(2, 2));
    assert_eq!(map.normalize(&Point::new(-1, 3)), Point::new(2, 0));
    assert_eq!(map.normalize(&Point::new(3, -1)), Point::new(0, 2));
}
