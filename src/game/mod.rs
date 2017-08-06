mod orientation;
mod map;
mod snake;
mod autopilot;

pub mod renderable;

use piston::input::keyboard::Key;

use self::snake::Snake;
use self::map::Map;
use self::orientation::{Direction, State};
use self::autopilot::Autopilot;

pub struct Game {
    snake: Snake,
    map: Map,
    delay: f64,
    time: f64,
    round: u64,
    pub dirty: bool,
    pub score: i64,
    autopilot: Autopilot,
    game_over: bool,
    paused: bool,
    help_texts: Vec<&'static str>,
}

impl Game {
    pub fn new(size: (u32, u32)) -> Game {
        let mut map = Map::new(size);
        let snake = Snake::new(size);
        for p in snake.get_tail().iter() {
            map.occupy(*p);
        }

        Game {
            snake,
            map,
            delay: 0.2,
            time: 0.,
            round: 0,
            dirty: true,
            score: 0,
            autopilot: Autopilot::None,
            game_over: false,
            paused: false,
            help_texts: vec![
                "E: Speed up",
                "Q: Slow down",
                "WASD: Steer",
                "F: Stupid Autopilot",
                "T: Smart Autopilot",
                "M: Manual Control",
                "P: Pause and Help",
                "Esc: Exit"
            ],
        }
    }

    fn peek(&mut self) -> State {
        let p = self.snake.peek();
        self.map.at(&p)
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn resume(&mut self) {
        self.paused = false;
    }

    pub fn update(&mut self, dt: f64) {
        if self.paused {
            return
        }

        self.time += dt;

        if self.round as f64 * self.delay > self.time {
            return
        }
        self.dirty = true;

        match self.autopilot {
            Autopilot::Stupid => self.stupid_autopilot(),
            Autopilot::Smart => self.smart_autopilot(),
            _ => (),
        }

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
            Autopilot(Autopilot),
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
            F => Command::Autopilot(Autopilot::Stupid),
            T => Command::Autopilot(Autopilot::Smart),
            M => Command::Autopilot(Autopilot::None),
            H | P => Command::Help,
            _ => Command::None
        };

        match cmd {
            Command::Turn(t) if !self.snake.reverse(t) => self.snake.turn(t),
            Command::ChangeSpeed(f) => {
                self.delay *= f;
                self.time = self.round as f64 * self.delay;
            },
            Command::Autopilot(a) => self.autopilot = a,
            Command::Help => {
                if self.paused {
                    self.resume()
                } else {
                    self.pause();
                }
            }
            _ => self.resume(),
        }

    }

    fn game_over(&mut self) {
        use std::f64;

        println!("Game Over!");
        self.delay = f64::INFINITY;
        self.game_over = true;
    }

    pub fn print_help(&self) {
        for i in &self.help_texts {
            println!("{}", i);
        }
    }
}
