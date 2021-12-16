extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::io::{stdin, Read};

#[derive(Parser)]
#[grammar = "manual.pest"]
struct ManualParser;

#[derive(Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

#[allow(dead_code)]
#[derive(Debug)]
struct State {
    dots: Vec<(usize, usize)>,
    folds: Vec<Fold>,
    paper: Vec<Vec<bool>>,
    max_x: usize,
    max_y: usize,
}

impl State {
    fn new(input: &str) -> State {
        let mut dots = Vec::new();
        let mut folds = Vec::new();

        let pairs = ManualParser::parse(Rule::main, input).unwrap_or_else(|e| panic!("{}", e));

        let mut dot_x: usize = 0;
        let mut dot_y: usize;

        for pair in pairs {
            let rule = pair.as_rule();
            let text = pair.clone().as_span().as_str().to_string();

            match rule {
                Rule::dot_x => {
                    dot_x = text.parse::<usize>().unwrap();
                }
                Rule::dot_y => {
                    dot_y = text.parse::<usize>().unwrap();
                    dots.push((dot_x, dot_y));
                }
                Rule::fold_x => {
                    folds.push(Fold::X(text.parse::<usize>().unwrap()));
                }
                Rule::fold_y => {
                    folds.push(Fold::Y(text.parse::<usize>().unwrap()));
                }
                _ => {}
            }
        }

        let max_x = dots
            .iter()
            .fold(0, |acc, &(x, _)| if x > acc { x } else { acc })
            + 1;
        let max_y = dots
            .iter()
            .fold(0, |acc, &(_, y)| if y > acc { y } else { acc })
            + 1;

        let mut paper = vec![vec![false; max_x]; max_y];

        for &(x, y) in dots.iter() {
            paper[y][x] = true;
        }

        State {
            dots,
            folds,
            paper,
            max_x,
            max_y,
        }
    }

    #[allow(dead_code)]
    fn display(&self) {
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                print!("{}", if self.paper[y][x] { "#" } else { "." });
            }
            println!();
        }
    }

    fn fold(&mut self) {
        match self.folds[0] {
            Fold::X(along_x) => {
                for y in 0..self.max_y {
                    for x in 1..(self.max_x - along_x) {
                        if self.paper[y][along_x + x] {
                            self.paper[y][along_x - x] = true;
                        }
                    }
                }
                self.max_x = along_x;
            }
            Fold::Y(along_y) => {
                for y in 1..(self.max_y - along_y) {
                    for x in 0..self.max_x {
                        if self.paper[along_y + y][x] {
                            self.paper[along_y - y][x] = true;
                        }
                    }
                }
                self.max_y = along_y;
            }
        }
    }

    fn count_dots(&self) -> u32 {
        let mut count = 0;

        for y in 0..self.max_y {
            for x in 0..self.max_x {
                if self.paper[y][x] {
                    count += 1;
                }
            }
        }

        count
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    // Part 1

    let mut state = State::new(&input);

    state.fold();

    println!(
        "Part 1: after one fold, {} dots are visible",
        state.count_dots()
    );
}
