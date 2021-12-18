use std::io::{stdin, Read};

#[derive(Debug)]
struct State {
    positions: Vec<Vec<u32>>,
    costs: Vec<Vec<Option<u32>>>,
}

impl State {
    fn new(input: &str) -> State {
        let positions: Vec<Vec<u32>> = input
            .lines()
            .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
            .collect();

        let costs: Vec<Vec<Option<u32>>> = vec![vec![None; positions[0].len()]; positions.len()];

        State { positions, costs }
    }

    fn neighbors(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();

        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y + 1 < self.positions.len() {
            neighbors.push((x, y + 1));
        }
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x + 1 < self.positions[0].len() {
            neighbors.push((x + 1, y));
        }

        neighbors
    }

    fn search(&mut self) {
        // This is a little inelegant, but since we never return to 0,0,
        // we never need to add its risk value
        self.costs[0][0] = Some(0);

        let mut stack = vec![(0, 0)];

        while let Some((x, y)) = stack.pop() {
            if let Some(cost) = self.costs[y][x] {
                for &(nx, ny) in self.neighbors((x, y)).iter() {
                    if let Some(neighbor_cost) = self.costs[ny][nx] {
                        if neighbor_cost <= cost + self.positions[ny][nx] {
                            continue;
                        }
                    }

                    self.costs[ny][nx] = Some(cost + self.positions[ny][nx]);
                    stack.push((nx, ny));
                }
            } else {
                panic!("Cost for ({}, {}) should be known", x, y);
            }
        }
    }

    fn lowest_total_risk(&self) -> u32 {
        if let Some(cost) = self.costs[self.costs[0].len() - 1][self.costs.len() - 1] {
            cost
        } else {
            panic!("Lowest total risk not determined");
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    // Part 1

    let mut state = State::new(&input);

    state.search();

    println!(
        "Part 1: the lowest total cost of any path is {}",
        state.lowest_total_risk()
    );
}
