use std::io::{stdin, Read};

#[allow(dead_code)]
fn display_positions(positions: &[Vec<u32>]) {
    for row in positions.iter() {
        for position in row.iter() {
            print!("{}", position);
        }
        println!();
    }
}

#[derive(Debug)]
struct State {
    positions: Vec<Vec<u32>>,
}

impl State {
    fn new(input: &str) -> State {
        let positions: Vec<Vec<u32>> = input
            .lines()
            .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
            .collect();

        State { positions }
    }

    fn expand_x25(&mut self) {
        let mut new_positions = Vec::new();

        // Expand vertically
        for _ in 0..=4 {
            new_positions.append(&mut self.positions.clone());
        }

        // Expand horizontally
        let width = self.positions[0].len();
        for y in 0..new_positions.len() {
            for _ in 1..=4 {
                new_positions[y].append(&mut self.positions[y % width].clone());
            }
        }

        // Adjust risk levels
        for area_y in 0..=4 {
            for area_x in 0..=4 {
                for tile_y in 0..self.positions.len() {
                    for tile_x in 0..self.positions[0].len() {
                        let y = area_y * self.positions.len() + tile_y;
                        let x = area_x * self.positions[0].len() + tile_x;
                        let mut value = new_positions[y][x] + area_y as u32 + area_x as u32;
                        if value > 9 {
                            value -= 9
                        }
                        new_positions[y][x] = value;
                    }
                }
            }
        }
        //display_positions(&new_positions);

        self.positions = new_positions;
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

    fn calculate_lowest_total_risk(&mut self) -> u32 {
        let mut costs: Vec<Vec<Option<u32>>> =
            vec![vec![None; self.positions[0].len()]; self.positions.len()];
        // No matter what the puzzle input specifies as the risk level
        // of postion (0, 0), we will never revisit it, so we should
        // treat its risk level as zero
        costs[0][0] = Some(0);

        let mut stack = vec![(0, (0, 0))];

        while let Some((cost, (x, y))) = stack.pop() {
            for &(nx, ny) in self.neighbors((x, y)).iter() {
                let new_neighbor_cost = cost + self.positions[ny][nx];
                if let Some(old_neighbor_cost) = costs[ny][nx] {
                    if old_neighbor_cost <= new_neighbor_cost {
                        continue;
                    }
                }

                costs[ny][nx] = Some(new_neighbor_cost);
                stack.push((new_neighbor_cost, (nx, ny)));
                stack.sort_by(|a, b| b.0.cmp(&a.0)); // big optimization!
            }
        }

        if let Some(cost) = costs[costs[0].len() - 1][costs.len() - 1] {
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

    println!(
        "Part 1: the lowest total cost of any path is {}",
        state.calculate_lowest_total_risk()
    );

    // Part 2

    let mut state = State::new(&input);
    state.expand_x25();

    println!(
        "Part 2: the lowest total cost of any path is {}",
        state.calculate_lowest_total_risk()
    );
}
