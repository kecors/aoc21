extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::cmp::Ordering;
use std::io::{stdin, Read};

#[derive(Parser)]
#[grammar = "target.pest"]
struct TargetParser;

#[derive(Debug, Clone)]
struct Solution {
    velocity_x: i32,
    velocity_y: i32,
    steps: Vec<(i32, i32)>,
    max_y_option: Option<i32>,
}

#[derive(Debug)]
struct State {
    target_x_min: i32,
    target_x_max: i32,
    target_y_min: i32,
    target_y_max: i32,
    solutions: Vec<Solution>,
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

        let solutions = Vec::new();

        State {
            target_x_min,
            target_x_max,
            target_y_min,
            target_y_max,
            solutions,
        }
    }

    fn fire_probe(&mut self, velocity_x: i32, velocity_y: i32) {
        let mut adjusted_velocity_x = velocity_x;
        let mut adjusted_velocity_y = velocity_y;
        let mut steps = Vec::new();
        let mut max_y_option = None;
        let mut max_y = self.target_y_min;

        let mut x = 0;
        let mut y = 0;
        steps.push((x, y));

        while x <= self.target_x_max && y >= self.target_y_min {
            x += adjusted_velocity_x;
            y += adjusted_velocity_y;
            match adjusted_velocity_x.cmp(&0) {
                Ordering::Less => adjusted_velocity_x += 1,
                Ordering::Equal => (),
                Ordering::Greater => adjusted_velocity_x -= 1,
            }
            adjusted_velocity_y -= 1;

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

        self.solutions.push(Solution {
            velocity_x,
            velocity_y,
            steps,
            max_y_option,
        });
    }

    #[allow(dead_code)]
    fn display_firing(&self, solution: Solution) {
        println!("Velocity: {}, {}", solution.velocity_x, solution.velocity_y);
        if let Some(max_y) = solution.max_y_option {
            println!("Success, maximum y is {}", max_y);
        } else {
            println!("Failure");
        }

        let steps_min_x = solution.steps.iter().map(|(x, _)| *x).min().unwrap();
        let steps_max_y = solution.steps.iter().map(|(_, y)| *y).max().unwrap();

        let mut y = steps_max_y;
        loop {
            for x in steps_min_x..=self.target_x_max {
                if x == 0 && y == 0 {
                    print!("S");
                    continue;
                }
                if solution.steps.contains(&(x, y)) {
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
        println!();
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut state = State::new(&input);

    // Fire the probe for many different velocities and store the results
    // COMMENT: I selected the end points of these ranges somewhat
    // arbitrarily, through experimentation. It is essential that these ranges
    // be inclusive enough. I did not come up with a method to determine
    // exactly what ranges are appropriate.
    for velocity_x in 0..=state.target_x_max {
        for velocity_y in -800..800 {
            state.fire_probe(velocity_x, velocity_y);
        }
    }

    // Part 1

    // Best solution
    let best_solution = state.solutions.iter().cloned().fold(
        Solution {
            velocity_x: 0,
            velocity_y: 0,
            steps: vec![],
            max_y_option: None,
        },
        |acc, item| {
            if let Some(item_max_y) = item.max_y_option {
                if let Some(acc_max_y) = acc.max_y_option {
                    if item_max_y > acc_max_y {
                        item
                    } else {
                        acc
                    }
                } else {
                    item
                }
            } else {
                acc
            }
        },
    );

    println!(
        "Part 1: the highest y position reached is {} (for velocity ({}, {}))",
        best_solution.max_y_option.unwrap(),
        best_solution.velocity_x,
        best_solution.velocity_y
    );

    // Part 2

    // Number of successful solutions
    println!(
        "Part 2: {} initial velocities reach the target",
        state
            .solutions
            .iter()
            .filter(|solution| solution.max_y_option != None)
            .count()
    );
}
