extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::cmp::Ordering;
use std::io::{stdin, Read};

#[derive(Parser)]
#[grammar = "target.pest"]
struct TargetParser;

#[derive(Debug)]
struct State {
    target_x_min: i32,
    target_x_max: i32,
    target_y_min: i32,
    target_y_max: i32,
}

impl State {
    fn new(input: &str) -> State {
        let pairs = TargetParser::parse(Rule::main, input).unwrap_or_else(|e| panic!("{}", e));

        let mut target_x_min: i32 = 0;
        let mut target_x_max: i32 = 0;
        let mut target_y_min: i32 = 0;
        let mut target_y_max: i32 = 0;

        for pair in pairs {
            let rule = pair.as_rule();
            let text = pair.clone().as_span().as_str().to_string();

            match rule {
                Rule::x1 => {
                    target_x_min = text.parse::<i32>().unwrap();
                }
                Rule::x2 => {
                    target_x_max = text.parse::<i32>().unwrap();
                }
                Rule::y1 => {
                    target_y_min = text.parse::<i32>().unwrap();
                }
                Rule::y2 => {
                    target_y_max = text.parse::<i32>().unwrap();
                }
                _ => {
                    panic!("Unknown rule {:?} with {:?}", rule, text);
                }
            }
        }

        State {
            target_x_min,
            target_x_max,
            target_y_min,
            target_y_max,
        }
    }

    fn fire_probe(
        &mut self,
        mut velocity_x: i32,
        mut velocity_y: i32,
    ) -> (Vec<(i32, i32)>, Option<i32>) {
        let mut steps = Vec::new();
        let mut max_y = self.target_y_min;
        let mut max_y_option = None;

        let mut x = 0;
        let mut y = 0;
        steps.push((x, y));

        while x <= self.target_x_max && y >= self.target_y_min {
            x += velocity_x;
            y += velocity_y;
            match velocity_x.cmp(&0) {
                Ordering::Less => velocity_x += 1,
                Ordering::Equal => (),
                Ordering::Greater => velocity_x -= 1,
            }
            velocity_y -= 1;

            steps.push((x, y));

            if y > max_y {
                max_y = y;
            }

            if x >= self.target_x_min
                && x <= self.target_x_max
                && y >= self.target_y_min
                && y <= self.target_y_max
            {
                max_y_option = Some(max_y);
            }
        }

        (steps, max_y_option)
    }

    #[allow(dead_code)]
    fn display(&self, steps: &[(i32, i32)]) {
        let steps_min_x = steps.iter().map(|(x, _)| *x).min().unwrap();
        let steps_max_y = steps.iter().map(|(_, y)| *y).max().unwrap();

        let mut y = steps_max_y;
        loop {
            for x in steps_min_x..=self.target_x_max {
                if x == 0 && y == 0 {
                    print!("S");
                    continue;
                }
                if steps.contains(&(x, y)) {
                    print!("#");
                    continue;
                }
                if x >= self.target_x_min && y <= self.target_y_max {
                    print!("T");
                    continue;
                }
                print!(".");
            }
            println!();
            y -= 1;
            if y < self.target_y_min {
                break;
            }
        }
    }

    fn experiment(&mut self) {
        let mut best_solution = (0, (0, 0));

        for velocity_x in 0..self.target_x_max {
            for velocity_y in 0..300 {
                let (_steps, max_y_option) = self.fire_probe(velocity_x, velocity_y);
                if let Some(max_y) = max_y_option {
                    //println!("Velocity: {}, {}", velocity_x, velocity_y);
                    //self.display(&steps);
                    ////println!("Success, maximum y is {}", max_y);
                    //println!();
                    if max_y > best_solution.0 {
                        best_solution = (max_y, (velocity_x, velocity_y));
                        println!("Velocity: {}, {}", velocity_x, velocity_y);
                        //self.display(&steps);
                        println!("Success, maximum y is {}", max_y);
                        println!();
                    }
                }
            }
        }

        dbg!(&best_solution);
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut state = State::new(&input);

    state.experiment();
}
