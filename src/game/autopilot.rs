use super::Game;
use super::orientation::{State, Direction};
use rand::{thread_rng, Rng};

pub enum Autopilot {
    Stupid,
    Smart,
    Boring,
    None,
}

impl Game {
    fn find_food(&mut self) {
        // take shortest way to food (manhattan metric)
        let original = self.snake.direction;

        let dx = self.map.get_food().x - self.snake.head().x;
        let dy = self.map.get_food().y - self.snake.head().y;

        if dy > 0 {
            if self.snake.direction != Direction::N && self.snake.direction != Direction::S {
                self.snake.turn(Direction::S);
            }
        } else if dy < 0 {
            if self.snake.direction != Direction::S && self.snake.direction != Direction::N {
                self.snake.turn(Direction::N);
            }
        } else if dx > 0 {
            if self.snake.direction != Direction::W && self.snake.direction != Direction::E {
                self.snake.turn(Direction::E);
            }
        } else if dx < 0 {
            if self.snake.direction != Direction::E && self.snake.direction != Direction::W {
                self.snake.turn(Direction::W);
            }
        }

        // ensure that we do not walk into a wall
        match self.map.at(&self.snake.peek()) {
            State::Empty | State::Food => (),
            _ => self.snake.turn(original)
        }
    }

    fn avoid_hazard(&mut self) {
        // decide to not collide in the next step
        println!("detected hazard!");
        let mut rng = thread_rng();
        let left = rng.gen::<f64>() > 0.5;
        if left {
            self.snake.turn_left();
            println!("turn left!");
        } else {
            self.snake.turn_right();
            println!("turn right!");
        }

        if self.map.at(&self.snake.peek()) != State::Empty {
            println!("also hazard, turn the other way!");
            self.snake.turn_right();
            self.snake.turn_right();
        }
    }

    pub fn stupid_autopilot(&mut self) {
        match self.map.at(&self.snake.peek()) {
            State::Empty | State::Food => self.find_food(),
            _ => self.avoid_hazard()
        }
    }
}
