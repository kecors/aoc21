use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let points: Vec<Vec<u32>> = input
        .lines()
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut low_points = Vec::new();
    for y in 0..points.len() {
        for x in 0..points[0].len() {
            let mut neighbors = Vec::new();
            if y > 0 {
                neighbors.push(points[y - 1][x]);
            }
            if y + 1 < points.len() {
                neighbors.push(points[y + 1][x]);
            }
            if x > 0 {
                neighbors.push(points[y][x - 1]);
            }
            if x + 1 < points[0].len() {
                neighbors.push(points[y][x + 1]);
            }
            if neighbors.iter().all(|n| points[y][x] < *n) {
                low_points.push((x, y));
            }
        }
    }

    let sum: u32 = low_points.iter().map(|&(x, y)| points[y][x] + 1).sum();
    println!(
        "Part 1: the sum of the risk levels of all low points is {}",
        sum
    );
}
