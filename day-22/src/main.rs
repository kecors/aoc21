extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fmt;
use std::io::{stdin, Read};

#[derive(Parser)]
#[grammar = "step.pest"]
struct StepParser;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Range {
    low: i64,
    high: i64,
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        result.push_str(&format!("{}..{}", self.low, self.high));

        write!(f, "{}", result)
    }
}

impl Range {
    fn new(low: i64, high: i64) -> Range {
        Range { low, high }
    }

    fn section(&self, other: &Range) -> Vec<Range> {
        let mut sections = Vec::new();

        if self.high >= other.low && self.low <= other.high {
            if self.low < other.low {
                sections.push(Range::new(self.low, other.low - 1));
                if self.high <= other.high {
                    sections.push(Range::new(other.low, self.high));
                } else {
                    sections.push(Range::new(other.low, other.high));
                    sections.push(Range::new(other.high + 1, self.high));
                }
            } else if self.high <= other.high {
                sections.push(Range::new(self.low, self.high));
            } else {
                sections.push(Range::new(self.low, other.high));
                sections.push(Range::new(other.high + 1, self.high));
            }
        }

        sections
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Cuboid {
    x: Range,
    y: Range,
    z: Range,
}

impl fmt::Display for Cuboid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        result.push_str(&format!("x={},y={},z={}", self.x, self.y, self.z));

        write!(f, "{}", result)
    }
}

impl Cuboid {
    fn new(x_low: i64, x_high: i64, y_low: i64, y_high: i64, z_low: i64, z_high: i64) -> Cuboid {
        Cuboid {
            x: Range::new(x_low, x_high),
            y: Range::new(y_low, y_high),
            z: Range::new(z_low, z_high),
        }
    }

    fn intersects(&self, other: &Cuboid) -> bool {
        if other.x.high < self.x.low || other.x.low > self.x.high {
            return false;
        }
        if other.y.high < self.y.low || other.y.low > self.y.high {
            return false;
        }
        if other.z.high < self.z.low || other.z.low > self.z.high {
            return false;
        }

        true
    }

    fn fragment(&self, other: &Cuboid) -> Vec<Cuboid> {
        let mut cuboids = Vec::new();

        for x_range in self.x.section(&other.x).iter() {
            for y_range in self.y.section(&other.y).iter() {
                for z_range in self.z.section(&other.z).iter() {
                    cuboids.push(Cuboid::new(
                        x_range.low,
                        x_range.high,
                        y_range.low,
                        y_range.high,
                        z_range.low,
                        z_range.high,
                    ));
                }
            }
        }

        cuboids
    }

    fn count(&self) -> i64 {
        (self.x.high - self.x.low + 1)
            * (self.y.high - self.y.low + 1)
            * (self.z.high - self.z.low + 1)
    }
}

#[derive(Debug)]
struct Step {
    set_on: bool,
    cuboid: Cuboid,
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
        let mut z_high: i64;

        for pair in pairs {
            let rule = pair.as_rule();
            let text = pair.clone().as_span().as_str().to_string();

            match rule {
                Rule::setting => {
                    set_on = text.contains("on");
                }
                Rule::x_low => {
                    x_low = text.parse::<i64>().unwrap();
                }
                Rule::x_high => {
                    x_high = text.parse::<i64>().unwrap();
                }
                Rule::y_low => {
                    y_low = text.parse::<i64>().unwrap();
                }
                Rule::y_high => {
                    y_high = text.parse::<i64>().unwrap();
                }
                Rule::z_low => {
                    z_low = text.parse::<i64>().unwrap();
                }
                Rule::z_high => {
                    z_high = text.parse::<i64>().unwrap();
                    let cuboid = Cuboid::new(x_low, x_high, y_low, y_high, z_low, z_high);
                    steps.push(Step { set_on, cuboid });
                }
                _ => {
                    panic!("Unknown rule {:?} with {:?}", rule, text);
                }
            }
        }

        State { steps }
    }

    fn reboot(&self, initialization: bool) -> i64 {
        let mut cuboids: Vec<Cuboid> = Vec::new();

        for step in self.steps.iter() {
            if initialization && (step.cuboid.x.high < -50 || step.cuboid.x.low > 50) {
                continue;
            }
            if initialization && (step.cuboid.y.high < -50 || step.cuboid.y.low > 50) {
                continue;
            }
            if initialization && (step.cuboid.z.high < -50 || step.cuboid.z.low > 50) {
                continue;
            }

            let mut new_cuboids = Vec::new();

            for cuboid in cuboids.drain(..) {
                if cuboid.intersects(&step.cuboid) {
                    new_cuboids.append(
                        &mut cuboid
                            .fragment(&step.cuboid)
                            .into_iter()
                            .filter(|cuboid| !cuboid.intersects(&step.cuboid))
                            .collect(),
                    );
                } else {
                    new_cuboids.push(cuboid);
                }
            }

            if step.set_on {
                new_cuboids.push(step.cuboid);
            }

            cuboids = new_cuboids;
        }

        cuboids.iter().map(|cuboid| cuboid.count()).sum()
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let state = State::new(&input);

    println!("Part 1: {} cubes are on", state.reboot(true));

    println!("Part 2: {} cubes are on", state.reboot(false));
}
