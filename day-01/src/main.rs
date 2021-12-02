use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let depths: Vec<u32> = input
        .lines()
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect();

    // Part 1

    let mut last_depth_opt = None;
    let mut increases = 0;
    for &depth in depths.iter() {
        if let Some(last_depth) = last_depth_opt {
            if depth > last_depth {
                increases += 1;
            }
        }
        last_depth_opt = Some(depth);
    }

    println!(
        "Part 1: There are {} measurements larger than the previous measurement",
        increases
    );

    // Part 2

    let mut last_sum_opt = None;
    let mut increases = 0;
    for j in 2..depths.len() {
        let sum = depths[j - 2] + depths[j - 1] + depths[j];
        if let Some(last_sum) = last_sum_opt {
            if sum > last_sum {
                increases += 1;
            }
        }
        last_sum_opt = Some(sum);
    }

    println!(
        "Part 2: {} sums are larger than the previous sum",
        increases
    );
}
