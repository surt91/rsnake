mod orientation;
mod map;
mod snake;
mod autopilot;

pub mod renderable;

use piston::input::keyboard::Key;

use self::snake::Snake;
use self::map::Map;
use self::orientation::{Direction, State, Point};
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
    game_won: bool,
    paused: bool,
    help_texts: Vec<&'static str>,
}

impl Game {
    pub fn new(size: (u32, u32)) -> Game {
        let mut map = Map::new(size);
        let snake = Snake::new(size);
        map.init_snake(snake.get_tail().iter());

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
            game_won: false,
            paused: false,
            help_texts: vec![
                "R: Restart",
                "E: Speed up",
                "Q: Slow down",
                "WASD: Steer",
                "F: Stupid Autopilot",
                "T: Smart Autopilot",
                "G: Boring Autopilot",
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

        if self.game_over {
            // if the autopilot is activated, start again after 2 seconds
            if self.autopilot != Autopilot::None {
                self.time += dt;
                if self.time > 2. {
                    self.restart();
                }
            }
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
            Autopilot::Boring => self.boring_autopilot(),
            _ => (),
        }

        self.round += 1;
        match self.peek() {
            State::Snake | State::Wall => self.game_over(),
            State::Food => {
                self.snake.feed();
                // test if we filled the whole map
                if self.snake.length as u32 >= self.map.size.1 * self.map.size.0 - 1 {
                    self.game_won();
                    // remove food from sight
                    self.map.food = Point::new(-1, -1);
                } else {
                    self.map.consumed_food();
                }
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
            Restart,
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
            G => Command::Autopilot(Autopilot::Boring),
            M => Command::Autopilot(Autopilot::None),
            H | P => Command::Help,
            R => Command::Restart,
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
            Command::Restart => self.restart(),
            _ => self.resume(),
        }

    }

    fn game_over(&mut self) {
        println!("Game Over!");
        self.game_over = true;
        self.time = 0.;
    }

    fn game_won(&mut self) {
        println!("Game Won!");
        self.game_won = true;
    }

    fn restart(&mut self) {
        self.map = Map::new(self.map.size);
        self.snake = Snake::new(self.map.size);
        self.map.init_snake(self.snake.get_tail().iter());

        self.time = 0.;
        self.round = 0;
        self.dirty = true;
        self.score = 0;
        self.game_over = false;
        self.game_won = false;
        self.paused = false;
    }

    pub fn print_help(&self) {
        for i in &self.help_texts {
            println!("{}", i);
        }
    }
}
