mod orientation;
mod map;
mod snake;
mod food;

pub mod renderable;

use piston::input::keyboard::Key;

use self::snake::Snake;
use self::map::Map;
use self::orientation::{Point, Direction, State};

pub struct Game {
    snake: Snake,
    map: Map,
    delay: f64,
    time: f64,
    round: u64,
    pub dirty: bool,
}

impl Game {
    pub fn new() -> Game {
        let snake = Snake::new();
        let map = Map::new(&snake);
        Game {
            snake,
            map,
            delay: 0.2,
            time: 0.,
            round: 0,
            dirty: true,
        }
    }

    fn peek(&mut self) -> State {
        let p = self.snake.peek();
        self.map.at(&p)
    }

    fn turn(&mut self, dir: Direction) {
        self.snake.turn(dir)
    }

    pub fn update(&mut self, dt: f64) {
        self.time += dt;
        // println!("{:?}", self.time);
        if self.round as f64 * self.delay > self.time {
            return
        }
        self.dirty = true;
        println!("{:?}", self.round);

        self.round += 1;
        match self.peek() {
            State::Snake | State::Wall => self.game_over(),
            State::Food => {
                self.snake.feed();
                self.map.consumed_food();
                self.snake.step(&mut self.map);
            }
            State::Empty => self.snake.step(&mut self.map),
        }
    }


    pub fn key_press(&mut self, key: Key) {
        use piston::input::keyboard::Key::*;

        let turn = match key {
            Up | W => Some(Direction::N),
            Down | S => Some(Direction::S),
            Right | D => Some(Direction::E),
            Left | A => Some(Direction::W),
            _ => None
        };

        match turn {
            Some(t) if !self.snake.reverse(t) => self.snake.turn(t),
            _ => ()
        }

    }

    fn game_over(&mut self) {
        use std::f64;

        println!("Game Over!");
        self.delay = f64::INFINITY;
    }
}
