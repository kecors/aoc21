use std::collections::HashMap;
use std::io::{stdin, Read};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Cave {
    Start,
    End,
    Big(String),
    Small(String),
}

impl From<&str> for Cave {
    fn from(string: &str) -> Self {
        match string {
            "start" => Cave::Start,
            "end" => Cave::End,
            _ => {
                if string.chars().next().unwrap().is_uppercase() {
                    Cave::Big(String::from(string))
                } else {
                    Cave::Small(String::from(string))
                }
            }
        }
    }
}

#[derive(Debug)]
struct State {
    connections: HashMap<Cave, Vec<Cave>>,
}

impl State {
    fn new(input: String) -> State {
        let mut connections: HashMap<Cave, Vec<Cave>> = HashMap::new();

        let pairs: Vec<(Cave, Cave)> = input
            .lines()
            .map(|line| {
                let strs: Vec<&str> = line.split('-').collect();
                (Cave::from(strs[0]), Cave::from(strs[1]))
            })
            .collect();

        for (cave_a, cave_b) in pairs {
            let vec_a = connections.entry(cave_a.clone()).or_insert_with(Vec::new);
            vec_a.push(cave_b.clone());
            let vec_b = connections.entry(cave_b).or_insert_with(Vec::new);
            vec_b.push(cave_a);
        }

        State { connections }
    }

    fn find_paths(&self) -> Vec<Vec<Cave>> {
        let mut complete_paths: Vec<Vec<Cave>> = Vec::new();
        let mut partial_paths: Vec<Vec<Cave>> = Vec::new();

        for cave in self.connections[&Cave::Start].iter() {
            partial_paths.push(vec![Cave::Start, cave.clone()]);
        }

        while let Some(mut partial_path) = partial_paths.pop() {
            if let Some(previous_cave) = partial_path.pop() {
                for cave in self.connections[&previous_cave].iter() {
                    match cave {
                        Cave::Start => {}
                        Cave::Big(_) => {
                            let mut new_partial_path = partial_path.clone();
                            new_partial_path.push(previous_cave.clone());
                            new_partial_path.push(cave.clone());
                            partial_paths.push(new_partial_path);
                        }
                        Cave::Small(_) => {
                            if partial_path.contains(cave) {
                                continue;
                            }
                            let mut new_partial_path = partial_path.clone();
                            new_partial_path.push(previous_cave.clone());
                            new_partial_path.push(cave.clone());
                            partial_paths.push(new_partial_path);
                        }
                        Cave::End => {
                            let mut complete_path = partial_path.clone();
                            complete_path.push(previous_cave.clone());
                            complete_path.push(cave.clone());
                            complete_paths.push(complete_path);
                        }
                    }
                }
            }
        }

        complete_paths
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let state = State::new(input);

    let complete_paths = state.find_paths();

    println!(
        "Part 1: there are {} paths through the cave system",
        complete_paths.len()
    );
}
