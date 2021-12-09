use std::io::{stdin, Read};

#[derive(Debug)]
struct State {
    subs: Vec<u32>,
}

impl State {
    fn new(input: &str) -> State {
        let subs: Vec<u32> = input
            .trim()
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        State { subs }
    }

    fn calculate(&self, part_2_flag: bool) -> (u32, u32) {
        let &min_sub = self.subs.iter().min().unwrap();
        let &max_sub = self.subs.iter().max().unwrap();

        let mut fuel_totals: Vec<(u32, u32)> = Vec::new();

        for position in min_sub..=max_sub {
            let mut fuel_total = 0;
            for &sub in self.subs.iter() {
                let distance = (sub as i32 - position as i32).abs() as u32;
                let fuel = self.compute_fuel(distance, part_2_flag);
                fuel_total += fuel;
            }
            fuel_totals.push((position, fuel_total));
        }

        let (best_position, cheapest_fuel_total) = fuel_totals.iter().fold(
            (0, self.compute_fuel(self.subs.iter().sum(), part_2_flag)),
            |acc, &(position, fuel_total)| {
                if fuel_total < acc.1 {
                    (position, fuel_total)
                } else {
                    acc
                }
            },
        );

        (best_position, cheapest_fuel_total)
    }

    fn compute_fuel(&self, distance: u32, part_2_flag: bool) -> u32 {
        if part_2_flag {
            (1..=distance).sum()
        } else {
            distance
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let state = State::new(&input);

    // Part 1

    let (best_position, cheapest_fuel_total) = state.calculate(false);
    println!(
        "Part 1: {} fuel is needed to align at position {}",
        cheapest_fuel_total, best_position
    );

    // Part 2

    let (best_position, cheapest_fuel_total) = state.calculate(true);
    println!(
        "Part 2: {} fuel is needed to align at position {}",
        cheapest_fuel_total, best_position
    );
}
