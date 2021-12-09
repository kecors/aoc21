use std::io::{stdin, Read};

#[derive(Debug)]
struct State {
    day: u32,
    fishes: Vec<u32>,
}

impl State {
    fn new(input: &str) -> State {
        let day = 0;
        let fishes: Vec<u32> = input
            .trim()
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        State { day, fishes }
    }

    fn day(&mut self) {
        let mut updated_fishes = Vec::new();
        let mut baby_count = 0;

        for fish in self.fishes.iter() {
            match fish {
                0 => {
                    updated_fishes.push(6);
                    baby_count += 1;
                }
                _ => {
                    updated_fishes.push(fish - 1);
                }
            }
        }

        for _ in 0..baby_count {
            updated_fishes.push(8);
        }

        self.day += 1;
        self.fishes = updated_fishes;
    }

    #[allow(dead_code)]
    fn display(&self) {
        if self.day == 0 {
            print!("Initial state: ");
        } else {
            print!("After {} days: ", self.day);
        }

        for (j, &fish) in self.fishes.iter().enumerate() {
            if j == 0 {
                print!("{}", fish);
            } else {
                print!(",{}", fish);
            }
        }
        println!();
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut state = State::new(&input);
    //state.display();

    for _ in 0..80 {
        state.day();
        //state.display();
    }

    println!(
        "Part 1: there would be {} lanternfish after 80 days",
        state.fishes.len()
    );
}
