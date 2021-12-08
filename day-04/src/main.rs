extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::io::{stdin, Read};
use std::iter;

#[derive(Parser)]
#[grammar = "bingo.pest"]
struct BingoParser;

#[derive(Debug)]
struct Board {
    targets: Vec<u32>,
    marks: Vec<bool>,
}

impl Board {
    fn new(targets: Vec<u32>) -> Board {
        let marks: Vec<bool> = iter::repeat(false).take(targets.len()).collect();

        Board { targets, marks }
    }

    fn mark_draw(&mut self, draw: u32) {
        for j in 0..self.targets.len() {
            if self.targets[j] == draw {
                self.marks[j] = true;
                return;
            }
        }
    }

    fn check_winner(&self) -> Option<u32> {
        let groupings = vec![
            [0, 1, 2, 3, 4],
            [5, 6, 7, 8, 9],
            [10, 11, 12, 13, 14],
            [15, 16, 17, 18, 19],
            [20, 21, 22, 23, 24],
            [0, 5, 10, 15, 20],
            [1, 6, 11, 16, 21],
            [2, 7, 12, 17, 22],
            [3, 8, 13, 18, 23],
            [4, 9, 14, 19, 24],
        ];

        for &grouping in groupings.iter() {
            if grouping.iter().all(|&x| self.marks[x]) {
                let mut unmarked_sum = 0;
                for j in 0..25 {
                    if !self.marks[j] {
                        unmarked_sum += self.targets[j];
                    }
                }
                return Some(unmarked_sum);
            }
        }

        None
    }
}

#[derive(Debug)]
struct State {
    draws: Vec<u32>,
    boards: Vec<Board>,
}

impl State {
    fn new() -> State {
        let draws = Vec::new();
        let boards = Vec::new();

        State { draws, boards }
    }

    fn parse_input(&mut self, input: &str) {
        let pairs = BingoParser::parse(Rule::main, input).unwrap_or_else(|e| panic!("{}", e));

        let mut targets = Vec::new();

        for pair in pairs {
            let rule = pair.as_rule();
            let text = pair.clone().as_span().as_str().to_string();

            match rule {
                Rule::draw => {
                    self.draws.push(text.parse::<u32>().unwrap());
                }
                Rule::new_board => {
                    if !targets.is_empty() {
                        self.boards.push(Board::new(targets));
                        targets = Vec::new();
                    }
                }
                Rule::target => {
                    targets.push(text.parse::<u32>().unwrap());
                }
                _ => {
                    panic!("unknown rule {:?}", rule);
                }
            }
        }

        self.boards.push(Board::new(targets));
    }

    fn play(&mut self) -> (u32, u32) {
        for &draw in self.draws.iter() {
            for board in self.boards.iter_mut() {
                board.mark_draw(draw);
                if let Some(unmarked_sum) = board.check_winner() {
                    return (draw, unmarked_sum);
                }
            }
        }

        panic!("No winner!");
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut state = State::new();
    state.parse_input(&input);

    let (winning_draw, unmarked_sum) = state.play();
    println!(
        "Part 1: the final score for the winning board is {}",
        winning_draw * unmarked_sum
    );
}
