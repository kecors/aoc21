extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::io::{stdin, Read};

#[derive(Parser)]
#[grammar = "notes.pest"]
struct NotesParser;

#[derive(Debug)]
enum Signal {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Debug)]
struct Pattern {
    signals: Vec<Signal>,
}

impl Pattern {
    fn new(text: &str) -> Pattern {
        let mut signals = Vec::new();

        for ch in text.chars() {
            match ch {
                'a' => signals.push(Signal::A),
                'b' => signals.push(Signal::B),
                'c' => signals.push(Signal::C),
                'd' => signals.push(Signal::D),
                'e' => signals.push(Signal::E),
                'f' => signals.push(Signal::F),
                'g' => signals.push(Signal::G),
                _ => (),
            }
        }

        Pattern { signals }
    }
}

#[derive(Debug)]
struct Display {
    inputs: Vec<Pattern>,
    outputs: Vec<Pattern>,
}

impl Display {
    fn new() -> Display {
        let inputs = Vec::new();
        let outputs = Vec::new();

        Display { inputs, outputs }
    }
}

#[derive(Debug)]
struct State {
    displays: Vec<Display>,
}

impl State {
    fn new(input: &str) -> State {
        let mut displays = Vec::new();

        for line in input.lines() {
            displays.push(State::parse_line(line));
        }

        State { displays }
    }

    fn parse_line(input: &str) -> Display {
        let pairs = NotesParser::parse(Rule::main, input).unwrap_or_else(|e| panic!("{}", e));

        let mut display = Display::new();

        for pair in pairs {
            let rule = pair.as_rule();
            let text = pair.clone().as_span().as_str().to_string();

            match rule {
                Rule::input => {
                    display.inputs.push(Pattern::new(&text));
                }
                Rule::output => {
                    display.outputs.push(Pattern::new(&text));
                }
                _ => {
                    panic!("Unknown rule {:?}", rule);
                }
            }
        }

        display
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let state = State::new(&input);

    // Part 1

    let mut count = 0;
    for display in state.displays.iter() {
        for pattern in display.outputs.iter() {
            match pattern.signals.len() {
                2 | 3 | 4 | 7 => count += 1,
                _ => (),
            }
        }
    }

    println!("Part 1: the digits 1, 4, 7, and 8 appear {} times", count);
}
