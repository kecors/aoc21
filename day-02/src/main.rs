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
}

impl State {
    fn new() -> State {
        let commands = Vec::new();

        State {
            commands,
            position: 0,
            depth: 0,
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

    fn process_commands(&mut self) {
        for command in self.commands.iter() {
            match command {
                Command::Forward(units) => self.position += units,
                Command::Down(units) => self.depth += units,
                Command::Up(units) => self.depth -= units,
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

    state.process_commands();

    println!(
        "Part 1: the product of the final position and final depth is {}",
        state.position * state.depth
    );
}
