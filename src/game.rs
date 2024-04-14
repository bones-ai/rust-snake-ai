//! Snake Game Logic
//! Snake Actions from a Neural Network

use crate::nn::Net;
use crate::*;

#[derive(Clone)]
pub struct Game {
    pub head: Point,
    pub body: Vec<Point>,
    pub food: Point,
    pub dir: FourDirs,
    pub brain: Net,

    pub is_complete: bool,
    no_food_steps: usize,
    num_steps: usize,
}

impl Game {
    pub fn new() -> Self {
        let mut body = Vec::new();
        let head = Point::new(GRID_W / 2, GRID_H / 2);
        body.push(head.clone());

        Self {
            body,
            head,
            food: Point::rand(),
            dir: FourDirs::get_rand_dir(),
            brain: Net::new(),
            is_complete: false,
            no_food_steps: 0,
            num_steps: 0,
        }
    }

    pub fn update(&mut self) {
        if self.is_complete {
            return;
        }

        self.num_steps += 1;
        self.dir = self.get_brain_output();
        self.handle_food_collision();
        self.update_snake_positions();
        self.handle_step_limit();
        if self.is_wall(self.head) || self.is_snake_body(self.head) {
            self.is_complete = true;
        }
    }

    pub fn get_net_output(&self) -> Vec<Vec<f64>> {
        let vision = self.get_snake_vision();
        self.brain.predict(&vision)
    }

    fn get_brain_output(&self) -> FourDirs {
        let vision = self.get_snake_vision();
        let nn_out = self.brain.predict(&vision).pop().unwrap();
        let max_index = nn_out
            .iter()
            .enumerate()
            .max_by(|(_, &a), (_, &b)| a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap();
        let mut dir = match max_index {
            0 => FourDirs::Left,
            1 => FourDirs::Right,
            2 => FourDirs::Bottom,
            _ => FourDirs::Top,
        };

        if self.dir.is_horizontal() {
            if dir.is_horizontal() && self.dir != dir {
                dir = self.dir;
            }
        }
        if self.dir.is_vertical() {
            if dir.is_vertical() && self.dir != dir {
                dir = self.dir;
            }
        }

        dir
    }

    fn get_snake_vision(&self) -> Vec<f64> {
        // self.get_11_vision()
        // self.get_custom_vision()
        // self.get_eight_dir_vision()
        self.get_four_dir_vision()
    }

    fn get_four_dir_vision(&self) -> Vec<f64> {
        let mut vision = Vec::new();
        let dirs = FourDirs::get_all_dirs();

        for d in dirs {
            let (wall, food, body) = self.look_in_dir(self.head, d);
            vision.push(wall as f64);
            vision.push(if food { 1.0 } else { 0.0 });
            vision.push(body as f64);
        }

        vision
    }

    pub fn fitness(&self) -> f32 {
        let score = self.body.len() as f32;
        if score <= 1.0 {
            return 1.0;
        }

        if score < 5.0 {
            return (self.num_steps as f32 * 0.1) * (2.0 as f32).powf(score) * score;
        }

        let mut fitness = 1.0;
        fitness *= (2.0 as f32).powf(score) * score;
        fitness *= self.num_steps as f32;

        // TODO f32 shouldn't work as it can't hold such a big value
        // This is broken
        fitness
    }

    pub fn score(&self) -> usize {
        self.body.len()
    }

    pub fn is_wall(&self, pt: Point) -> bool {
        pt.x >= GRID_W || pt.x <= 0 || pt.y >= GRID_H || pt.y <= 0
    }

    pub fn is_snake_body(&self, pt: Point) -> bool {
        for p in self.body.iter().skip(1) {
            if pt == *p {
                return true;
            }
        }

        false
    }

    fn update_snake_positions(&mut self) {
        self.head.x += self.dir.value().0;
        self.head.y += self.dir.value().1;

        let mut prev_pos = self.head.clone();
        for p in self.body.iter_mut() {
            let new_pos = *p;
            *p = prev_pos;
            prev_pos = new_pos;
        }
    }

    pub fn with_brain(new_brain: &Net) -> Self {
        let mut new_game = Self::new();
        new_game.brain = new_brain.clone();

        new_game
    }

    fn handle_food_collision(&mut self) {
        if self.head != self.food {
            self.no_food_steps += 1;
            return;
        }

        self.body.push(Point::new(self.head.x, self.head.y));
        self.food = self.get_random_empty_pos();
        self.no_food_steps = 0;
    }

    fn handle_step_limit(&mut self) {
        let limit = match self.score() {
            score if score > 10 => NUM_SIM_STEPS * 2,
            score if score > 20 => NUM_SIM_STEPS * 3,
            score if score > 30 => NUM_SIM_STEPS * 5,
            score if score > 80 => NUM_SIM_STEPS * 8,
            _ => NUM_SIM_STEPS,
        };

        if self.no_food_steps >= limit {
            self.is_complete = true;
        }
    }

    fn get_random_empty_pos(&self) -> Point {
        let mut pt = Point::rand();

        let mut num_tries = 0;
        while num_tries < 5 {
            num_tries += 1;
            pt = Point::rand();

            if !self.body.contains(&pt) {
                break;
            }
        }

        pt
    }

    fn look_in_dir(&self, st: Point, dir: (i32, i32)) -> (f32, bool, f32) {
        let mut food = false;
        // let mut body = false;
        let mut temp_pt: Point = st;
        let mut dist = 0;

        loop {
            if self.is_wall(temp_pt) {
                break;
            }

            if self.food == temp_pt {
                food = true;
            }

            if self.is_snake_body(temp_pt) {
                // body = true;
                break;
            }

            temp_pt = Point::new(temp_pt.x + dir.0, temp_pt.y + dir.1);

            dist += 1;
            if dist > 1000 {
                break;
            }
        }

        (1.0 / dist as f32, food, 1.0 / dist as f32)
    }

    pub fn render(&self) {
        for x in 0..=GRID_W {
            for y in 0..=GRID_H {
                let pt = (x, y).into();
                if self.is_wall(pt) {
                    print!("□");
                    continue;
                }
                if self.is_snake_body(pt) {
                    print!("■");
                    continue;
                }
                if self.head == pt {
                    print!("■");
                }
                if self.food == pt {
                    print!("●");
                }
                print!(".");
            }
            println!();
        }
        println!();
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.fitness() == other.fitness()
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.fitness().partial_cmp(&other.fitness())
    }
}
