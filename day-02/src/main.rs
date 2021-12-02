extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::io::{stdin, Read};

#[derive(Parser)]
#[grammar = "command.pest"]
struct CommandParser;

#[derive(Debug)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[derive(Debug)]
struct State {
    commands: Vec<Command>,
    position: u32,
    depth: u32,
    aim: u32,
}

impl State {
    fn new() -> State {
        let commands = Vec::new();

        State {
            commands,
            position: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn parse_line(&mut self, line: &str) {
        let pairs = CommandParser::parse(Rule::command, line).unwrap_or_else(|e| panic!("{}", e));

        for pair in pairs {
            let rule = pair.as_rule();
            let units = pair.into_inner().next().unwrap().as_str().parse().unwrap();

            match rule {
                Rule::forward => {
                    self.commands.push(Command::Forward(units));
                }
                Rule::down => {
                    self.commands.push(Command::Down(units));
                }
                Rule::up => {
                    self.commands.push(Command::Up(units));
                }
                _ => {
                    dbg!(&rule);
                }
            }
        }
    }

    fn process_commands_part1(&mut self) {
        for command in self.commands.iter() {
            match command {
                Command::Forward(units) => self.position += units,
                Command::Down(units) => self.depth += units,
                Command::Up(units) => self.depth -= units,
            }
        }
    }

    fn process_commands_part2(&mut self) {
        // Reinitialize position and depth
        self.position = 0;
        self.depth = 0;

        for command in self.commands.iter() {
            match command {
                Command::Forward(units) => {
                    self.position += units;
                    self.depth += self.aim * units;
                }
                Command::Down(units) => self.aim += units,
                Command::Up(units) => self.aim -= units,
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut state = State::new();

    for line in input.lines() {
        state.parse_line(&line);
    }

    // Part 1

    state.process_commands_part1();

    println!(
        "Part 1: the product of the final position and final depth is {}",
        state.position * state.depth
    );

    // Part 2

    state.process_commands_part2();

    println!(
        "Part 2: the product of the final position and final depth is {}",
        state.position * state.depth
    );
}
