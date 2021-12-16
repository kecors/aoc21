extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::collections::HashMap;
use std::io::{stdin, Read};

#[derive(Parser)]
#[grammar = "manual.pest"]
struct ManualParser;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Pair {
    left: char,
    right: char,
}

#[derive(Debug)]
struct Ruler {
    rules: HashMap<Pair, char>, // Pair insertion rules
}

impl Ruler {
    fn expand(&self, pair: &Pair) -> (Pair, Pair) {
        let insertion_element = self.rules.get(pair).unwrap();

        let pair_1 = Pair {
            left: pair.left,
            right: *insertion_element,
        };
        let pair_2 = Pair {
            left: *insertion_element,
            right: pair.right,
        };

        (pair_1, pair_2)
    }
}

#[derive(Debug)]
struct State {
    ruler: Ruler,
    leftmost_element: char,
    rightmost_element: char,
    pair_counts: HashMap<Pair, u32>,
}

impl State {
    fn new(input: &str) -> State {
        let mut polymer_template = Vec::new();
        let mut rules = HashMap::new();

        let pairs = ManualParser::parse(Rule::main, input).unwrap_or_else(|e| panic!("{}", e));

        let mut element_1: char = '?';
        let mut element_2: char = '?';

        for pair in pairs {
            let rule = pair.as_rule();
            let text = pair.clone().as_span().as_str().to_string();

            match rule {
                Rule::pt_element => {
                    polymer_template.push(text.chars().next().unwrap());
                }
                Rule::pir_e_1 => {
                    element_1 = text.chars().next().unwrap();
                }
                Rule::pir_e_2 => {
                    element_2 = text.chars().next().unwrap();
                }
                Rule::pir_e_3 => {
                    let pair = Pair {
                        left: element_1,
                        right: element_2,
                    };
                    let element_3 = text.chars().next().unwrap();
                    rules.insert(pair, element_3);
                }
                _ => {
                    panic!("Unknown rule {:?} with {:?}", rule, text);
                }
            }
        }

        let ruler = Ruler { rules };

        let leftmost_element = polymer_template[0];
        let rightmost_element = polymer_template[polymer_template.len() - 1];

        let pairs: Vec<Pair> = polymer_template
            .iter()
            .cloned()
            .zip(polymer_template[1..].iter().cloned())
            .map(|(l, r)| Pair { left: l, right: r })
            .collect();

        let mut pair_counts: HashMap<Pair, u32> = HashMap::new();

        for pair in pairs {
            let o = pair_counts.entry(pair).or_insert(0);
            *o += 1;
        }

        State {
            ruler,
            leftmost_element,
            rightmost_element,
            pair_counts,
        }
    }

    fn step(&mut self) {
        let mut new_pair_counts: HashMap<Pair, u32> = HashMap::new();

        for (pair, count) in self.pair_counts.iter() {
            let (new_pair_1, new_pair_2) = self.ruler.expand(pair);
            let o = new_pair_counts.entry(new_pair_1).or_insert(0);
            *o += count;
            let o = new_pair_counts.entry(new_pair_2).or_insert(0);
            *o += count;
        }

        self.pair_counts = new_pair_counts;
    }

    fn element_counts(&self) -> HashMap<char, u32> {
        let mut element_counts = HashMap::new();

        element_counts.insert(self.leftmost_element, 1);
        element_counts.insert(self.rightmost_element, 1);

        for (pair, count) in self.pair_counts.iter() {
            let o = element_counts.entry(pair.left).or_insert(0);
            *o += count;
            let o = element_counts.entry(pair.right).or_insert(0);
            *o += count;
        }

        for (_, count) in element_counts.iter_mut() {
            *count /= 2;
        }

        element_counts
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut state = State::new(&input);

    // Part 1

    for _ in 1..=10 {
        state.step();
    }
    let ec = state.element_counts();
    let counts: Vec<u32> = ec.values().cloned().collect();
    let &max = counts.iter().max().unwrap();
    let &min = counts.iter().min().unwrap();
    println!("Part 1: the difference of most and least is {}", max - min);
}
