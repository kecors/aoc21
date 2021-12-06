use std::cmp::Ordering;
use std::io::{stdin, Read};
use std::iter;

fn part_1_calculate_gamma_and_epsilon(numbers: &[u32]) -> (u32, u32) {
    let mut max_place = 0;
    let mut one_counts: Vec<u32> = iter::repeat(0).take(32).collect();

    for &number in numbers.iter() {
        let mut j = 0;
        loop {
            if 1 << j > number {
                break;
            }
            if 0 != number & 1 << j {
                one_counts[j] += 1;
            }
            j += 1;
        }
        if j > max_place {
            max_place = j;
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for (j, &count) in one_counts.iter().enumerate().take(max_place) {
        if count > numbers.len() as u32 / 2 {
            gamma += 1 << j;
        } else {
            epsilon += 1 << j;
        }
    }

    (gamma, epsilon)
}

fn part_2_calculate_oxgen_and_scrub(numbers: Vec<u32>) -> (u32, u32) {
    let mut max_place = 0;
    for j in 0..32 {
        for &number in numbers.iter() {
            if 0 != number & 1 << j {
                max_place = j;
            }
        }
    }

    let mut o2_numbers = numbers.clone();
    let mut one_counts: Vec<u32> = iter::repeat(0).take(32).collect();

    for j in (0..=max_place).rev() {
        for &number in o2_numbers.iter() {
            if 0 != number & 1 << j {
                one_counts[j] += 1;
            }
        }

        let mcv = match one_counts[j].cmp(&(o2_numbers.len() as u32 - one_counts[j])) {
            Ordering::Greater => 1,
            Ordering::Equal => 1,
            Ordering::Less => 0,
        };

        o2_numbers = o2_numbers
            .into_iter()
            .filter(|&n| (n & 1 << j) == mcv << j)
            .collect();

        if o2_numbers.len() == 1 {
            break;
        }
    }

    let mut co2_numbers = numbers.clone();
    let mut one_counts: Vec<u32> = iter::repeat(0).take(32).collect();

    for j in (0..=max_place).rev() {
        for &number in co2_numbers.iter() {
            if 0 != number & 1 << j {
                one_counts[j] += 1;
            }
        }

        let mcv = match one_counts[j].cmp(&(co2_numbers.len() as u32 - one_counts[j])) {
            Ordering::Greater => 0,
            Ordering::Equal => 0,
            Ordering::Less => 1,
        };

        co2_numbers = co2_numbers
            .into_iter()
            .filter(|&n| (n & 1 << j) == mcv << j)
            .collect();

        if co2_numbers.len() == 1 {
            break;
        }
    }

    (o2_numbers[0], co2_numbers[0])
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let numbers: Vec<u32> = input
        .lines()
        .map(|x| u32::from_str_radix(x, 2).unwrap())
        .collect();

    let (gamma, epsilon) = part_1_calculate_gamma_and_epsilon(&numbers);

    println!(
        "Part 1: the power consumption of the submarine is {}",
        gamma * epsilon
    );

    let (oxgen, scrub) = part_2_calculate_oxgen_and_scrub(numbers.clone());

    println!(
        "Part 2: the life support rating of the submarine is {}",
        oxgen * scrub
    );
}
