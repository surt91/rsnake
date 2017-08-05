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
    pub score: i64,
}

impl Game {
    pub fn new(size: (u32, u32)) -> Game {
        let mut map = Map::new(size);
        let snake = Snake::new(size);
        for p in snake.get_tail().iter() {
            map.occupy(*p);
        }
        map.generate_food();

        Game {
            snake,
            map,
            delay: 0.2,
            time: 0.,
            round: 0,
            dirty: true,
            score: 0,
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
                self.score += 1;
            }
            State::Empty => self.snake.step(&mut self.map),
        }
    }


    pub fn key_press(&mut self, key: Key) {
        use piston::input::keyboard::Key::*;

        enum Command {
            Turn(Direction),
            ChangeSpeed(f64),
            Help,
            None
        }

        let cmd = match key {
            Up | W => Command::Turn(Direction::N),
            Down | S => Command::Turn(Direction::S),
            Right | D => Command::Turn(Direction::E),
            Left | A => Command::Turn(Direction::W),
            E => Command::ChangeSpeed(0.8),
            Q => Command::ChangeSpeed(1./0.8),
            H => Command::Help,
            _ => Command::None
        };

        match cmd {
            Command::Turn(t) if !self.snake.reverse(t) => self.snake.turn(t),
            Command::ChangeSpeed(f) => {
                self.delay *= f;
                self.time = self.round as f64 * self.delay;
            },
            Command::Help => {
                println!("E: Speed up");
                println!("Q: Slow down");
                println!("WASD: Steer");
            }
            _ => ()
        }

    }

    fn game_over(&mut self) {
        use std::f64;

        println!("Game Over!");
        self.delay = f64::INFINITY;
    }
}
