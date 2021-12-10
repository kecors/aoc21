extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::collections::HashSet;
use std::io::{stdin, Read};

#[derive(Parser)]
#[grammar = "notes.pest"]
struct NotesParser;

#[derive(Debug, Hash, PartialEq, Eq)]
enum Signal {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

fn parse_signal_pattern(text: &str) -> HashSet<Signal> {
    use self::Signal::*;

    let mut signals = HashSet::new();

    for ch in text.chars() {
        signals.insert(match ch {
            'a' => A,
            'b' => B,
            'c' => C,
            'd' => D,
            'e' => E,
            'f' => F,
            'g' => G,
            _ => panic!("Unknown signal {}", ch),
        });
    }

    signals
}

#[derive(Debug)]
struct Display {
    inputs: Vec<HashSet<Signal>>,
    outputs: Vec<HashSet<Signal>>,
}

impl Display {
    fn new(input: &str) -> Display {
        let mut inputs = Vec::new();
        let mut outputs = Vec::new();

        let pairs = NotesParser::parse(Rule::main, input).unwrap_or_else(|e| panic!("{}", e));

        for pair in pairs {
            let rule = pair.as_rule();
            let text = pair.clone().as_span().as_str().to_string();

            match rule {
                Rule::input => {
                    inputs.push(parse_signal_pattern(&text));
                }
                Rule::output => {
                    outputs.push(parse_signal_pattern(&text));
                }
                _ => {
                    panic!("Unknown rule {:?}", rule);
                }
            }
        }

        Display { inputs, outputs }
    }

    fn deduce_digits(&self) -> Vec<&HashSet<Signal>> {
        let one = self.inputs.iter().find(|x| x.len() == 2).unwrap();
        let seven = self.inputs.iter().find(|x| x.len() == 3).unwrap();
        let four = self.inputs.iter().find(|x| x.len() == 4).unwrap();
        let eight = self.inputs.iter().find(|x| x.len() == 7).unwrap();

        let three = self
            .inputs
            .iter()
            .find(|x| x.len() == 5 && x.intersection(one).count() == 2)
            .unwrap();
        let six = self
            .inputs
            .iter()
            .find(|x| x.len() == 6 && x.intersection(one).count() == 1)
            .unwrap();
        let five = self
            .inputs
            .iter()
            .find(|x| x.len() == 5 && x.intersection(six).count() == 5)
            .unwrap();
        let nine = self
            .inputs
            .iter()
            .find(|x| x.len() == 6 && x.difference(four).count() == 2)
            .unwrap();
        let zero = self
            .inputs
            .iter()
            .find(|x| {
                x.len() == 6
                    && x.intersection(four).count() == 3
                    && x.intersection(seven).count() == 3
            })
            .unwrap();
        let two = self
            .inputs
            .iter()
            .find(|x| x.len() == 5 && x.intersection(four).count() == 2)
            .unwrap();

        vec![zero, one, two, three, four, five, six, seven, eight, nine]
    }

    fn solve(&self) -> u32 {
        let digit_hashsets = self.deduce_digits();

        let mut sum = 0;

        for (j, output) in self.outputs.iter().enumerate() {
            for (k, digit_hashset) in digit_hashsets.iter().enumerate() {
                if output.symmetric_difference(digit_hashset).count() == 0 {
                    sum += k as u32 * (10_u32.pow(3 - (j as u32)));
                    break;
                }
            }
        }

        sum
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
            displays.push(Display::new(line));
        }

        State { displays }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let state = State::new(&input);

    // Part 1

    let mut count = 0;
    for display in state.displays.iter() {
        for output in display.outputs.iter() {
            match output.len() {
                2 | 3 | 4 | 7 => count += 1,
                _ => (),
            }
        }
    }

    println!("Part 1: the digits 1, 4, 7, and 8 appear {} times", count);

    // Part 2

    let mut sum = 0;
    for display in state.displays.iter() {
        sum += display.solve();
    }

    println!("Part 2: the sum of the output values is {}", sum);
}
