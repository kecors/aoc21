use std::io::{stdin, Read};

fn part_1(input: &str) -> u64 {
    let lines: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let mut score = 0;

    for line in lines.iter() {
        let mut stack = Vec::new();

        for &ch in line.iter() {
            match ch {
                '(' | '[' | '{' | '<' => {
                    stack.push(ch);
                }
                ')' => {
                    if let Some(prev) = stack.pop() {
                        if prev != '(' {
                            score += 3;
                            break;
                        }
                    }
                }
                ']' => {
                    if let Some(prev) = stack.pop() {
                        if prev != '[' {
                            score += 57;
                            break;
                        }
                    }
                }
                '}' => {
                    if let Some(prev) = stack.pop() {
                        if prev != '{' {
                            score += 1197;
                            break;
                        }
                    }
                }
                '>' => {
                    if let Some(prev) = stack.pop() {
                        if prev != '<' {
                            score += 25137;
                            break;
                        }
                    }
                }
                _ => {
                    panic!("Unknown character {}", ch);
                }
            }
        }
    }

    score
}

fn part_2(input: &str) -> u64 {
    let lines: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let mut scores = Vec::new();

    for line in lines.iter() {
        let mut stack = Vec::new();
        let mut corrupted = false;

        for &ch in line.iter() {
            match ch {
                '(' | '[' | '{' | '<' => {
                    stack.push(ch);
                }
                ')' => {
                    if let Some(prev) = stack.pop() {
                        if prev != '(' {
                            corrupted = true;
                            break;
                        }
                    }
                }
                ']' => {
                    if let Some(prev) = stack.pop() {
                        if prev != '[' {
                            corrupted = true;
                            break;
                        }
                    }
                }
                '}' => {
                    if let Some(prev) = stack.pop() {
                        if prev != '{' {
                            corrupted = true;
                            break;
                        }
                    }
                }
                '>' => {
                    if let Some(prev) = stack.pop() {
                        if prev != '<' {
                            corrupted = true;
                            break;
                        }
                    }
                }
                _ => {
                    panic!("Unknown character {}", ch);
                }
            }
        }

        if corrupted {
            continue;
        }

        let mut score = 0;

        while let Some(ch) = stack.pop() {
            score *= 5;
            score += match ch {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!("Unknown character {}", ch),
            };
        }
        scores.push(score);
    }

    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    println!("Part 1: the total syntax error score is {}", part_1(&input));

    println!("Part 2: the middle score is {}", part_2(&input));
}
