extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::collections::HashMap;
use std::io::{stdin, Read};

#[derive(Parser)]
#[grammar = "lines.pest"]
struct LineParser;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Line {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl Line {
    fn points(&self, part_2_flag: bool) -> Vec<Point> {
        let mut points = Vec::new();

        // Horizontal lines
        if self.y1 == self.y2 {
            if self.x1 > self.x2 {
                for x in self.x2..=self.x1 {
                    points.push(Point { x, y: self.y1 });
                }
            } else {
                for x in self.x1..=self.x2 {
                    points.push(Point { x, y: self.y1 });
                }
            }
        }

        // Vertical lines
        if self.x1 == self.x2 {
            if self.y1 > self.y2 {
                for y in self.y2..=self.y1 {
                    points.push(Point { x: self.x1, y });
                }
            } else {
                for y in self.y1..=self.y2 {
                    points.push(Point { x: self.x1, y });
                }
            }
        }

        // Ignore diagonal lines for part 1
        if !part_2_flag {
            return points;
        }

        // Diagonal lines
        if self.x1 > self.x2 && self.y1 != self.y2 {
            if self.y1 > self.y2 {
                for delta in 0..=(self.x1 - self.x2) {
                    points.push(Point {
                        x: self.x1 - delta,
                        y: self.y1 - delta,
                    });
                }
            } else {
                for delta in 0..=(self.x1 - self.x2) {
                    points.push(Point {
                        x: self.x1 - delta,
                        y: self.y1 + delta,
                    });
                }
            }
        }
        if self.x2 > self.x1 && self.y1 != self.y2 {
            if self.y1 > self.y2 {
                for delta in 0..=(self.x2 - self.x1) {
                    points.push(Point {
                        x: self.x1 + delta,
                        y: self.y1 - delta,
                    });
                }
            } else {
                for delta in 0..=(self.x2 - self.x1) {
                    points.push(Point {
                        x: self.x1 + delta,
                        y: self.y1 + delta,
                    });
                }
            }
        }

        points
    }
}

#[derive(Debug)]
struct State {
    lines: Vec<Line>,
    coverages: HashMap<Point, u32>,
}

impl State {
    fn new() -> State {
        let lines = Vec::new();
        let coverages = HashMap::new();

        State { lines, coverages }
    }

    fn parse_line(&mut self, input: &str) {
        let pairs = LineParser::parse(Rule::main, input).unwrap_or_else(|e| panic!("{}", e));

        let mut x1: u32 = 0;
        let mut y1: u32 = 0;
        let mut x2: u32 = 0;
        let mut y2: u32;

        for pair in pairs {
            let rule = pair.as_rule();
            let text = pair.clone().as_span().as_str().to_string();

            match rule {
                Rule::x1 => {
                    x1 = text.parse::<u32>().unwrap();
                }
                Rule::y1 => {
                    y1 = text.parse::<u32>().unwrap();
                }
                Rule::x2 => {
                    x2 = text.parse::<u32>().unwrap();
                }
                Rule::y2 => {
                    y2 = text.parse::<u32>().unwrap();
                    self.lines.push(Line { x1, y1, x2, y2 });
                }
                _ => {
                    panic!("Unknown rule {:?}", rule);
                }
            }
        }
    }

    fn consider_lines(&mut self, part_2_flag: bool) -> u32 {
        for line in self.lines.iter() {
            for point in line.points(part_2_flag) {
                let coverage = self.coverages.entry(point).or_insert(0);
                *coverage += 1;
            }
        }

        let mut overlaps = 0;
        for (_, &coverage) in self.coverages.iter() {
            if coverage >= 2 {
                overlaps += 1;
            }
        }

        overlaps
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    // Part 1

    let mut state = State::new();
    for line in input.lines() {
        state.parse_line(line);
    }

    let overlaps = state.consider_lines(false);
    println!("Part 1: at {} points at least two lines overlap", overlaps);

    // Part 2

    let mut state = State::new();
    for line in input.lines() {
        state.parse_line(line);
    }

    let overlaps = state.consider_lines(true);
    println!("Part 2: at {} points at least two lines overlap", overlaps);
}
