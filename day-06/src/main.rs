use std::collections::HashMap;
use std::io::{stdin, Read};

#[derive(Debug)]
struct State {
    day: u64,
    fishes: HashMap<u64, u64>,
}

impl State {
    fn new(input: &str) -> State {
        let day = 0;
        let mut fishes = HashMap::new();
        input
            .trim()
            .split(',')
            .map(|x| x.parse::<u64>().unwrap())
            .for_each(|x| {
                let f = fishes.entry(x).or_insert(0);
                *f += 1;
            });

        State { day, fishes }
    }

    fn day(&mut self) {
        let mut updated_fishes = HashMap::new();

        for (&days, &quantity) in self.fishes.iter() {
            match days {
                0 => {
                    let uf = updated_fishes.entry(6).or_insert(0);
                    *uf += quantity;
                    let uf = updated_fishes.entry(8).or_insert(0);
                    *uf += quantity;
                }
                _ => {
                    let uf = updated_fishes.entry(days - 1).or_insert(0);
                    *uf += quantity;
                }
            }
        }

        self.day += 1;
        self.fishes = updated_fishes;
    }

    fn quantity(&self) -> u64 {
        let mut total_quantity = 0;

        for (_, &quantity) in self.fishes.iter() {
            total_quantity += quantity;
        }

        total_quantity
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    // Part 1

    let mut state = State::new(&input);

    for _ in 0..80 {
        state.day();
    }

    println!(
        "Part 1: there would be {} lanternfish after 80 days",
        state.quantity()
    );

    // Part 2

    let mut state = State::new(&input);

    for _ in 0..256 {
        state.day();
    }

    println!(
        "Part 2: there would be {} lanternfish after 256 days",
        state.quantity()
    );
}
