use std::fmt;
use std::io::{stdin, Read};

#[derive(Debug)]
enum Location {
    East,
    South,
    Empty,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Location::East => '>',
                Location::South => 'v',
                Location::Empty => '.',
            }
        )
    }
}

#[derive(Debug)]
struct Region {
    location_rows: Vec<Vec<Location>>,
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        for location_row in self.location_rows.iter() {
            for location in location_row.iter() {
                result.push_str(&format!("{}", location));
            }
            result.push('\n');
        }

        write!(f, "{}", result)
    }
}

impl Region {
    fn new(input: &str) -> Region {
        let location_rows = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| match ch {
                        '>' => Location::East,
                        'v' => Location::South,
                        '.' => Location::Empty,
                        _ => panic!("Unknown location {}", ch),
                    })
                    .collect()
            })
            .collect();

        Region { location_rows }
    }

    fn step(&mut self) -> bool {
        let mut updated = false;
        let mut post_east_location_rows = Vec::new();
        for location_row in self.location_rows.iter() {
            let mut post_east_location_row = Vec::new();
            for (x, location) in location_row.iter().enumerate() {
                match location {
                    Location::South => post_east_location_row.push(Location::South),
                    Location::East => {
                        let next_x = if x == location_row.len() - 1 {
                            0
                        } else {
                            x + 1
                        };
                        if let Location::Empty = location_row[next_x] {
                            post_east_location_row.push(Location::Empty);
                            updated = true;
                        } else {
                            post_east_location_row.push(Location::East);
                        }
                    }
                    Location::Empty => {
                        let previous_x = if x == 0 {
                            location_row.len() - 1
                        } else {
                            x - 1
                        };
                        if let Location::East = location_row[previous_x] {
                            post_east_location_row.push(Location::East);
                            updated = true;
                        } else {
                            post_east_location_row.push(Location::Empty);
                        }
                    }
                }
            }
            post_east_location_rows.push(post_east_location_row);
        }

        let mut post_south_location_rows = Vec::new();
        for (y, location_row) in post_east_location_rows.iter().enumerate() {
            let mut post_south_location_row = Vec::new();
            for (x, location) in location_row.iter().enumerate() {
                match location {
                    Location::East => post_south_location_row.push(Location::East),
                    Location::South => {
                        let next_y = if y == post_east_location_rows.len() - 1 {
                            0
                        } else {
                            y + 1
                        };
                        if let Location::Empty = post_east_location_rows[next_y][x] {
                            post_south_location_row.push(Location::Empty);
                            updated = true;
                        } else {
                            post_south_location_row.push(Location::South);
                        }
                    }
                    Location::Empty => {
                        let previous_y = if y == 0 {
                            post_east_location_rows.len() - 1
                        } else {
                            y - 1
                        };
                        if let Location::South = post_east_location_rows[previous_y][x] {
                            post_south_location_row.push(Location::South);
                            updated = true;
                        } else {
                            post_south_location_row.push(Location::Empty);
                        }
                    }
                }
            }
            post_south_location_rows.push(post_south_location_row);
        }

        self.location_rows = post_south_location_rows;

        updated
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut region = Region::new(&input);

    let mut step = 0;
    loop {
        //println!("After {} steps:\n{}", step, region);
        step += 1;
        if !region.step() {
            break;
        }
    }

    println!("Part 1: the first step without movement is {}", step);
}
