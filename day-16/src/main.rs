use std::io::{stdin, Read};

fn hexchar_to_bools(ch: char) -> Vec<bool> {
    match ch {
        '0' => vec![false, false, false, false],
        '1' => vec![false, false, false, true],
        '2' => vec![false, false, true, false],
        '3' => vec![false, false, true, true],
        '4' => vec![false, true, false, false],
        '5' => vec![false, true, false, true],
        '6' => vec![false, true, true, false],
        '7' => vec![false, true, true, true],
        '8' => vec![true, false, false, false],
        '9' => vec![true, false, false, true],
        'A' => vec![true, false, true, false],
        'B' => vec![true, false, true, true],
        'C' => vec![true, true, false, false],
        'D' => vec![true, true, false, true],
        'E' => vec![true, true, true, false],
        'F' => vec![true, true, true, true],
        _ => panic!("Unknown character {}", ch),
    }
}

fn bools_to_decimal(binary_digits: &[bool]) -> u64 {
    let mut value = 0;
    let mut units = 1;

    for &digit in binary_digits.iter().rev() {
        if digit {
            value += units;
        }
        units *= 2;
    }

    value
}

#[derive(Debug)]
struct State {
    data: Vec<bool>,
    cursor: usize,
    packet_version_sum: u64,
}

impl State {
    fn new(input: &str) -> State {
        let data: Vec<bool> = input.trim().chars().flat_map(hexchar_to_bools).collect();
        let cursor = 0;
        let packet_version_sum = 0;

        State {
            data,
            cursor,
            packet_version_sum,
        }
    }

    fn process_packet(&mut self) {
        let packet_version = bools_to_decimal(&self.data[self.cursor..self.cursor + 3]);
        self.cursor += 3;
        println!("packet version: {}", packet_version);

        let packet_type_id = bools_to_decimal(&self.data[self.cursor..self.cursor + 3]);
        self.cursor += 3;
        println!("packet type id: {}", packet_type_id);

        match packet_type_id {
            4 => {
                self.process_literal_value_packet();
            }
            _ => {
                self.process_operator_packet();
            }
        }

        self.packet_version_sum += packet_version;
    }

    fn process_literal_value_packet(&mut self) {
        let mut literal_value_bools = Vec::new();

        loop {
            let group_flag = self.data[self.cursor];
            self.cursor += 1;
            literal_value_bools.append(&mut self.data[self.cursor..self.cursor + 4].to_vec());
            self.cursor += 4;
            if !group_flag {
                break;
            }
        }
        let literal_value = bools_to_decimal(&literal_value_bools);
        println!("literal value: {}", &literal_value);
    }

    fn process_operator_packet(&mut self) {
        let length_type_id = self.data[self.cursor];
        self.cursor += 1;

        match length_type_id {
            false => {
                let total_length_in_bits =
                    bools_to_decimal(&self.data[self.cursor..self.cursor + 15]);
                self.cursor += 15;

                let end_of_packet_index = self.cursor + total_length_in_bits as usize;

                while self.cursor < end_of_packet_index {
                    self.process_packet();
                }
            }
            true => {
                let subpacket_count = bools_to_decimal(&self.data[self.cursor..self.cursor + 11]);
                self.cursor += 11;

                for _ in 0..subpacket_count {
                    self.process_packet();
                }
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut state = State::new(&input);
    state.process_packet();

    println!(
        "Part 1: the sum of all packet version numbers is {}",
        state.packet_version_sum
    );
}
