use std::io::{stdin, Read};

#[derive(Debug)]
struct State {
    points: Vec<Vec<u32>>,
    low_points: Vec<(usize, usize)>,
}

impl State {
    fn new(input: &str) -> State {
        let points: Vec<Vec<u32>> = input
            .lines()
            .map(|x| x.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        let low_points = Vec::new();

        State { points, low_points }
    }

    fn neighbors(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();

        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y + 1 < self.points.len() {
            neighbors.push((x, y + 1));
        }
        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x + 1 < self.points[0].len() {
            neighbors.push((x + 1, y));
        }

        neighbors
    }

    fn find_low_points(&mut self) {
        for y in 0..self.points.len() {
            for x in 0..self.points[0].len() {
                if self
                    .neighbors((x, y))
                    .iter()
                    .all(|&(nx, ny)| self.points[y][x] < self.points[ny][nx])
                {
                    self.low_points.push((x, y));
                }
            }
        }
    }

    fn risk_level_sum(&self) -> u32 {
        self.low_points
            .iter()
            .map(|&(x, y)| self.points[y][x] + 1)
            .sum()
    }

    fn find_basin_point_count(&self, low_point: (usize, usize)) -> u32 {
        let mut count = 0;
        let mut points_visited: Vec<Vec<bool>> =
            vec![vec![false; self.points[0].len()]; self.points.len()];
        let mut basin_points: Vec<(usize, usize)> = vec![low_point];

        while let Some((bx, by)) = basin_points.pop() {
            if points_visited[by][bx] {
                continue;
            }

            points_visited[by][bx] = true;

            if self.points[by][bx] == 9 {
                continue;
            }

            count += 1;

            for (nx, ny) in self.neighbors((bx, by)) {
                basin_points.push((nx, ny));
            }
        }

        count
    }

    fn calculate_basins_product(&self) -> u32 {
        let mut basin_products = Vec::new();

        for &low_point in self.low_points.iter() {
            basin_products.push(self.find_basin_point_count(low_point));
        }

        basin_products.sort_unstable();

        basin_products.iter().rev().take(3).product()
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut state = State::new(&input);

    state.find_low_points();

    println!(
        "Part 1: the sum of the risk levels of all low points is {}",
        state.risk_level_sum()
    );

    println!(
        "Part 2: the product of the three largest basin sizes is {}",
        state.calculate_basins_product()
    );
}
