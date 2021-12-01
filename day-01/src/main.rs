use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let depths : Vec<u32> = input
        .lines()
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect();

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

    println!("Part 1: There are {} measurements larger than the previous measurement", increases);
}
