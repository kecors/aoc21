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
    result: Option<(usize, u32, u32)>,
}

impl Board {
    fn new(targets: Vec<u32>) -> Board {
        let marks: Vec<bool> = iter::repeat(false).take(targets.len()).collect();
        let result = None;

        Board {
            targets,
            marks,
            result,
        }
    }

    fn play_round(&mut self, draw_index: usize, draw: u32) {
        // If this board already won, don't play it any more
        if self.result != None {
            return;
        }

        // Mark the draw, if found in targets for this board
        for j in 0..self.targets.len() {
            if self.targets[j] == draw {
                self.marks[j] = true;
                break;
            }
        }

        // Check whether this board is now a winner
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
                for j in 0..self.marks.len() {
                    if !self.marks[j] {
                        unmarked_sum += self.targets[j];
                    }
                }
                self.result = Some((draw_index, draw, unmarked_sum));
            }
        }
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

    fn play(&mut self) {
        for (draw_index, &draw) in self.draws.iter().enumerate() {
            for board in self.boards.iter_mut() {
                board.play_round(draw_index, draw);
            }
        }
    }

    fn part_1_first_winner(&self) -> (u32, u32) {
        let mut winning_round = self.draws.len();
        let mut winning_board_index = 0;

        for (j, board) in self.boards.iter().enumerate() {
            if let Some((round, _, _)) = board.result {
                if round < winning_round {
                    winning_round = round;
                    winning_board_index = j;
                }
            }
        }

        if let Some((_, winning_draw, unmarked_sum)) = self.boards[winning_board_index].result {
            return (winning_draw, unmarked_sum);
        } else {
            panic!("Problem with first winner!");
        }
    }

    fn part_2_last_winner(&self) -> (u32, u32) {
        let mut winning_round = 0;
        let mut winning_board_index = 0;

        for (j, board) in self.boards.iter().enumerate() {
            if let Some((round, _, _)) = board.result {
                if round > winning_round {
                    winning_round = round;
                    winning_board_index = j;
                }
            }
        }

        if let Some((_, winning_draw, unmarked_sum)) = self.boards[winning_board_index].result {
            return (winning_draw, unmarked_sum);
        } else {
            panic!("Problem with last winner!");
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut state = State::new();
    state.parse_input(&input);

    state.play();

    let (winning_draw, unmarked_sum) = state.part_1_first_winner();
    println!(
        "Part 1: the final score for the first winning board is {}",
        winning_draw * unmarked_sum
    );

    let (winning_draw, unmarked_sum) = state.part_2_last_winner();
    println!(
        "Part 2: the final score for the last winning board is {}",
        winning_draw * unmarked_sum
    );
}
