use super::Game;
use super::orientation::{State, Direction, Reachable, best_first_search};
use rand::{thread_rng, Rng};

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
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

        let mut dx = self.map.get_food().x - self.snake.head().x;
        let mut dy = self.map.get_food().y - self.snake.head().y;

        // get shortest way over periodic boundaries
        if dx.abs() as f64 > self.map.size.0 as f64 / 2. {
            dx -= self.map.size.0 as i32 * dx.signum();
        }
        if dy.abs() as f64 > self.map.size.1 as f64 / 2. {
            dy -= self.map.size.1 as i32 * dx.signum();
        }

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
        if self.detect_hazard() {
            self.snake.turn(original)
        }
    }

    fn detect_hazard(&self) -> bool {
        match self.map.at(&self.snake.peek()) {
            State::Empty | State::Food => false,
            _ => true
        }
    }

    fn avoid_hazard(&mut self) -> bool {
        let mut decision = false;
        if self.detect_hazard() {
            decision = true;
            // decide to not collide in the next step
            let mut rng = thread_rng();
            let left = rng.gen::<f64>() > 0.5;
            if left {
                self.snake.turn_left();
            } else {
                self.snake.turn_right();
            }

            if self.detect_hazard() {
                self.snake.turn_right();
                self.snake.turn_right();
            }
        }

        decision
    }

    fn occupied_neighbors(&self) -> usize {
        let p = self.snake.head();
        p.neighbors2()
         .filter(|x| self.map.at(x) != State::Empty && self.map.at(x) != State::Food)
         .count()
    }

    fn avoid_trapping(&mut self) -> bool {
        let mut decision = false;
        if self.occupied_neighbors() <= 1 {
            // no trapping possible with only 1 neighbor
            return decision
        }

        let original = self.snake.direction;

        // test if we can reach the end of our tail and the food
        // if we can, we can go on forever
        if self.detect_hazard()
            || best_first_search(&self.snake.peek(), self.snake.end(), &self.map) == Reachable::No
        {
            self.snake.turn_left();
            if self.detect_hazard()
                || best_first_search(&self.snake.peek(), self.snake.end(), &self.map) == Reachable::No
            {
                self.snake.turn_left();
                self.snake.turn_left();
            }
            decision = true;
        }

        // maybe we are trapped, the at least do not die immediately
        if self.detect_hazard() {
            self.snake.turn(original);

            self.avoid_hazard();
            decision = true;
        }

        decision
    }

    pub fn stupid_autopilot(&mut self) {
        if !self.avoid_hazard() {
            self.find_food()
        }
    }

    pub fn smart_autopilot(&mut self) {
        if !self.avoid_trapping() {
            self.find_food();
        }
    }
}
