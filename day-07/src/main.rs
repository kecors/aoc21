use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let subs: Vec<u32> = input
        .trim()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let &min_sub = subs.iter().min().unwrap();
    let &max_sub = subs.iter().max().unwrap();

    let mut fuel_totals: Vec<(u32, u32)> = Vec::new();

    for position in min_sub..=max_sub {
        let mut fuel_total = 0;
        for &sub in subs.iter() {
            fuel_total += (sub as i32 - position as i32).abs() as u32;
        }
        fuel_totals.push((position, fuel_total));
    }

    let cheapest_fuel_total =
        fuel_totals
            .iter()
            .fold((0, subs.iter().sum()), |acc, &(position, fuel_total)| {
                if fuel_total < acc.1 {
                    (position, fuel_total)
                } else {
                    acc
                }
            });

    println!(
        "Part 1: {} fuel is needed to align at position {}",
        cheapest_fuel_total.1, cheapest_fuel_total.0
    );
}
