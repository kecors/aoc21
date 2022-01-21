extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::collections::HashSet;
use std::io::{stdin, Read};

#[derive(Parser)]
#[grammar = "step.pest"]
struct StepParser;

#[derive(Debug)]
struct Step {
    set_on: bool,
    x_low: i32,
    x_high: i32,
    y_low: i32,
    y_high: i32,
    z_low: i32,
    z_high: i32,
}

#[derive(Debug)]
struct State {
    steps: Vec<Step>,
}

impl State {
    fn new(input: &str) -> State {
        let pairs = StepParser::parse(Rule::main, input).unwrap_or_else(|e| panic!("{}", e));
        let mut steps = Vec::new();
        let mut set_on = false;
        let mut x_low = 0;
        let mut x_high = 0;
        let mut y_low = 0;
        let mut y_high = 0;
        let mut z_low = 0;
        let mut z_high: i32;

        for pair in pairs {
            let rule = pair.as_rule();
            let text = pair.clone().as_span().as_str().to_string();

            match rule {
                Rule::setting => {
                    set_on = text.contains("on");
                }
                Rule::x_low => {
                    x_low = text.parse::<i32>().unwrap();
                }
                Rule::x_high => {
                    x_high = text.parse::<i32>().unwrap();
                }
                Rule::y_low => {
                    y_low = text.parse::<i32>().unwrap();
                }
                Rule::y_high => {
                    y_high = text.parse::<i32>().unwrap();
                }
                Rule::z_low => {
                    z_low = text.parse::<i32>().unwrap();
                }
                Rule::z_high => {
                    z_high = text.parse::<i32>().unwrap();
                    steps.push(Step {
                        set_on,
                        x_low,
                        x_high,
                        y_low,
                        y_high,
                        z_low,
                        z_high,
                    });
                }
                _ => {
                    panic!("Unknown rule {:?} with {:?}", rule, text);
                }
            }
        }

        State { steps }
    }

    fn reboot(&self) -> usize {
        let mut cubes: HashSet<(i32, i32, i32)> = HashSet::new();

        for step in self.steps.iter() {
            if step.z_high < -50 || step.z_low > 50 {
                continue;
            }
            if step.y_high < -50 || step.y_low > 50 {
                continue;
            }
            if step.x_high < -50 || step.x_low > 50 {
                continue;
            }
            for z in step.z_low..=step.z_high {
                for y in step.y_low..=step.y_high {
                    for x in step.x_low..=step.x_high {
                        if step.set_on {
                            cubes.insert((x, y, z));
                        } else {
                            cubes.remove(&(x, y, z));
                        }
                    }
                }
            }
        }

        cubes.len()
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let state = State::new(&input);
    println!("Part 1: {} cubes are on", state.reboot());
}
