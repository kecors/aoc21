use std::collections::HashSet;
use std::io::{stdin, Read};

#[derive(Debug)]
struct State {
    octopuses: Vec<Vec<u32>>,
    flash_count: u32,
}

impl State {
    fn new(input: &str) -> State {
        let octopuses: Vec<Vec<u32>> = input
            .lines()
            .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
            .collect();
        let flash_count = 0;

        State {
            octopuses,
            flash_count,
        }
    }

    #[allow(dead_code)]
    fn display_octopuses(&self) {
        for y in 0..self.octopuses.len() {
            for x in 0..self.octopuses[0].len() {
                print!("{}", self.octopuses[y][x]);
            }
            println!();
        }
        println!();
    }

    fn neighbors(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();

        if y > 0 && x > 0 {
            neighbors.push((x - 1, y - 1));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y > 0 && x + 1 < self.octopuses[0].len() {
            neighbors.push((x + 1, y - 1));
        }

        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x + 1 < self.octopuses[0].len() {
            neighbors.push((x + 1, y));
        }

        if y + 1 < self.octopuses.len() && x > 0 {
            neighbors.push((x - 1, y + 1));
        }
        if y + 1 < self.octopuses.len() {
            neighbors.push((x, y + 1));
        }
        if y + 1 < self.octopuses.len() && x + 1 < self.octopuses[0].len() {
            neighbors.push((x + 1, y + 1));
        }

        neighbors
    }

    fn step(&mut self) {
        for y in 0..self.octopuses.len() {
            for x in 0..self.octopuses[0].len() {
                self.octopuses[y][x] += 1;
            }
        }

        let mut flasheds = HashSet::new();

        loop {
            let mut new_flash = false;

            for y in 0..self.octopuses.len() {
                for x in 0..self.octopuses[0].len() {
                    if self.octopuses[y][x] <= 9 {
                        continue;
                    }

                    if flasheds.contains(&(x, y)) {
                        continue;
                    }

                    flasheds.insert((x, y));
                    new_flash = true;

                    for (nx, ny) in self.neighbors((x, y)) {
                        self.octopuses[ny][nx] += 1;
                    }
                }
            }

            if !new_flash {
                break;
            }
        }

        for &(x, y) in flasheds.iter() {
            self.octopuses[y][x] = 0;
            self.flash_count += 1;
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut state = State::new(&input);

    for _ in 0..100 {
        state.step();
    }

    println!(
        "Part 1: there are {} total flashes after 100 steps",
        state.flash_count
    );
}
