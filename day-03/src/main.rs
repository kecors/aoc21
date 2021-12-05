use std::io::{stdin, Read};
use std::iter;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let numbers: Vec<u32> = input
        .lines()
        .map(|x| u32::from_str_radix(x, 2).unwrap())
        .collect();

    let mut max_j = 0;
    let mut counts: Vec<u32> = iter::repeat(0).take(32).collect();
    for &number in numbers.iter() {
        let mut j = 0;
        loop {
            if 1 << j > number {
                break;
            }
            if 0 != number & 1 << j {
                counts[j] += 1;
            }
            j += 1;
        }
        if j > max_j {
            max_j = j;
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for (j, &count) in counts.iter().enumerate().take(max_j) {
        if count > numbers.len() as u32 / 2 {
            gamma += 1 << j;
        } else {
            epsilon += 1 << j;
        }
    }

    println!(
        "Part 1: the power consumption of the submarine is {}",
        gamma * epsilon
    );
}
