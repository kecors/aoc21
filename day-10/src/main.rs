use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let lines: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let mut score = 0;

    for line in lines.iter() {
        let mut stack = Vec::new();

        for &ch in line.iter() {
            match ch {
                '(' => {
                    stack.push(ch);
                }
                '[' => {
                    stack.push(ch);
                }
                '{' => {
                    stack.push(ch);
                }
                '<' => {
                    stack.push(ch);
                }
                ')' => {
                    if let Some(prev) = stack.pop() {
                        match prev {
                            '(' => {}
                            _ => {
                                score += 3;
                                break;
                            }
                        }
                    }
                }
                ']' => {
                    if let Some(prev) = stack.pop() {
                        match prev {
                            '[' => {}
                            _ => {
                                score += 57;
                                break;
                            }
                        }
                    }
                }
                '}' => {
                    if let Some(prev) = stack.pop() {
                        match prev {
                            '{' => {}
                            _ => {
                                score += 1197;
                                break;
                            }
                        }
                    }
                }
                '>' => {
                    if let Some(prev) = stack.pop() {
                        match prev {
                            '<' => {}
                            _ => {
                                score += 25137;
                                break;
                            }
                        }
                    }
                }
                _ => {
                    panic!("Unknown character {}", ch);
                }
            }
        }
    }

    println!("Part 1: the total syntax error score is {}", score);
}
