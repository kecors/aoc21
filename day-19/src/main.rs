extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::io::{stdin, Read};

#[derive(Parser)]
#[grammar = "scanner.pest"]
struct ScannerParser;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Beacon {
    x: i32,
    y: i32,
    z: i32,
}

impl fmt::Display for Beacon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        result.push_str(&format!("{},{},{}", self.x, self.y, self.z));

        write!(f, "{}", result)
    }
}

impl Beacon {
    fn new(x: i32, y: i32, z: i32) -> Beacon {
        Beacon { x, y, z }
    }

    fn offset(&self, other: &Beacon) -> Offset {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;

        Offset::new(x, y, z)
    }

    fn reorient(&self, code: u8) -> Beacon {
        match code {
            0 => Beacon::new(self.x, self.y, self.z),
            1 => Beacon::new(self.x, -self.y, -self.z),
            2 => Beacon::new(self.x, self.z, -self.y),
            3 => Beacon::new(self.x, -self.z, self.y),

            4 => Beacon::new(-self.x, self.y, -self.z),
            5 => Beacon::new(-self.x, -self.y, self.z),
            6 => Beacon::new(-self.x, self.z, self.y),
            7 => Beacon::new(-self.x, -self.z, -self.y),

            8 => Beacon::new(self.y, self.x, -self.z),
            9 => Beacon::new(self.y, -self.x, self.z),
            10 => Beacon::new(self.y, self.z, self.x),
            11 => Beacon::new(self.y, -self.z, -self.x),

            12 => Beacon::new(-self.y, self.x, self.z),
            13 => Beacon::new(-self.y, -self.x, -self.z),
            14 => Beacon::new(-self.y, self.z, -self.x),
            15 => Beacon::new(-self.y, -self.z, self.x),

            16 => Beacon::new(self.z, self.x, self.y),
            17 => Beacon::new(self.z, -self.x, -self.y),
            18 => Beacon::new(self.z, self.y, -self.x),
            19 => Beacon::new(self.z, -self.y, self.x),

            20 => Beacon::new(-self.z, self.x, -self.y),
            21 => Beacon::new(-self.z, -self.x, self.y),
            22 => Beacon::new(-self.z, self.y, self.x),
            23 => Beacon::new(-self.z, -self.y, -self.x),

            _ => panic!("Unsupported reorient code {}", code),
        }
    }

    fn translate(&self, offset: Offset) -> Beacon {
        let x = offset.x + self.x;
        let y = offset.y + self.y;
        let z = offset.z + self.z;

        Beacon::new(x, y, z)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Offset {
    x: i32,
    y: i32,
    z: i32,
}

impl fmt::Display for Offset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        result.push_str(&format!("{},{},{}", self.x, self.y, self.z));

        write!(f, "{}", result)
    }
}

impl Offset {
    fn new(x: i32, y: i32, z: i32) -> Offset {
        Offset { x, y, z }
    }

    fn manhattan(&self) -> (i32, i32, i32) {
        let mut units = vec![self.x.abs(), self.y.abs(), self.z.abs()];
        units.sort_unstable();

        (units[0], units[1], units[2])
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Pair {
    beacon_1: Beacon,
    beacon_2: Beacon,
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        result.push_str(&format!("({}) and ({})", self.beacon_1, self.beacon_2));

        write!(f, "{}", result)
    }
}

impl Pair {
    fn new(beacon_1: Beacon, beacon_2: Beacon) -> Pair {
        Pair { beacon_1, beacon_2 }
    }
}

#[derive(Debug)]
struct Scanner {
    id: u32,
    offset_option: Option<Offset>,
    beacons: HashSet<Beacon>,
}

impl fmt::Display for Scanner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        result.push_str(&format!("Scanner {}", self.id));
        result.push_str(&format!("\nOffset {:?}", self.offset_option));

        result.push_str("\nBeacons");
        for beacon in self.beacons.iter() {
            result.push_str(&format!("\n({})", beacon));
        }
        result.push('\n');

        write!(f, "{}", result)
    }
}

impl Scanner {
    fn new(id: u32, beacons: HashSet<Beacon>) -> Scanner {
        let offset_option = None;
        Scanner {
            id,
            offset_option,
            beacons,
        }
    }

    fn pair_offsets(&self) -> HashMap<Offset, Pair> {
        let mut pair_offsets = HashMap::new();

        for beacon_1 in self.beacons.iter() {
            for beacon_2 in self.beacons.iter() {
                if beacon_1 != beacon_2 {
                    pair_offsets.insert(beacon_1.offset(beacon_2), Pair::new(*beacon_1, *beacon_2));
                }
            }
        }

        pair_offsets
    }

    fn reorient(&self, code: u8) -> Scanner {
        let mut beacons = HashSet::new();

        for beacon in self.beacons.iter() {
            beacons.insert(beacon.reorient(code));
        }

        Scanner::new(self.id, beacons)
    }

    fn reorient_other(&self, other: &Scanner) -> Option<Scanner> {
        let own_pair_offsets = self.pair_offsets();
        let other_pair_offsets = other.pair_offsets();

        let other_pair_manhattans: HashSet<(i32, i32, i32)> = other_pair_offsets
            .keys()
            .map(|offset| offset.manhattan())
            .collect();

        let mut shared_own_beacons = HashSet::new();

        for (offset, own_pair) in own_pair_offsets.iter() {
            if other_pair_manhattans.contains(&offset.manhattan()) {
                shared_own_beacons.insert(own_pair.beacon_1);
                shared_own_beacons.insert(own_pair.beacon_2);
            }
        }

        if shared_own_beacons.len() < 12 {
            return None;
        }

        for code in 0..24 {
            let mut reoriented_other = other.reorient(code);
            let reoriented_other_pair_offsets = reoriented_other.pair_offsets();

            let mut other_scanner_offsets: HashMap<Offset, u32> = HashMap::new();

            for (other_offset, other_pair) in reoriented_other_pair_offsets.iter() {
                if let Some(own_pair) = own_pair_offsets.get(other_offset) {
                    let o_b1_b1 = own_pair.beacon_1.offset(&other_pair.beacon_1);
                    let o_b2_b2 = own_pair.beacon_2.offset(&other_pair.beacon_2);
                    let o_b1_b2 = own_pair.beacon_1.offset(&other_pair.beacon_2);
                    let o_b2_b1 = own_pair.beacon_2.offset(&other_pair.beacon_1);

                    if o_b1_b1 == o_b2_b2 {
                        let o = other_scanner_offsets.entry(o_b1_b1).or_insert(0);
                        *o += 1;
                        continue;
                    }
                    if o_b1_b2 == o_b2_b1 {
                        let o = other_scanner_offsets.entry(o_b1_b2).or_insert(0);
                        *o += 1;
                    }
                }
            }

            let other_scanner_offset = if let Some((&offset, _)) = other_scanner_offsets
                .iter()
                .find(|(_, &count)| count >= 12 * 11)
            {
                offset
            } else {
                continue;
            };

            let mut translated_beacons = HashSet::new();
            for beacon in reoriented_other.beacons.iter() {
                translated_beacons.insert(beacon.translate(other_scanner_offset));
            }
            reoriented_other.beacons = translated_beacons;
            reoriented_other.offset_option = Some(other_scanner_offset);

            return Some(reoriented_other);
        }

        None
    }
}

#[derive(Debug)]
struct State {
    scanners: Vec<Scanner>,
}

impl State {
    fn new(input: &str) -> State {
        let mut scanners = Vec::new();
        let mut scanner_id_option: Option<u32> = None;
        let mut beacons = HashSet::new();
        let mut x = 0;
        let mut y = 0;
        let mut z: i32;

        let pairs = ScannerParser::parse(Rule::main, input).unwrap_or_else(|e| panic!("{}", e));

        for pair in pairs {
            let rule = pair.as_rule();
            let text = pair.clone().as_span().as_str().to_string();

            match rule {
                Rule::scanner_id => {
                    if let Some(id) = scanner_id_option {
                        scanners.push(Scanner::new(id, beacons));
                        beacons = HashSet::new();
                    }

                    let scanner_id = text.parse::<u32>().unwrap();
                    scanner_id_option = Some(scanner_id);
                }
                Rule::x => {
                    x = text.parse::<i32>().unwrap();
                }
                Rule::y => {
                    y = text.parse::<i32>().unwrap();
                }
                Rule::z => {
                    z = text.parse::<i32>().unwrap();
                    let beacon = Beacon::new(x, y, z);
                    beacons.insert(beacon);
                }
                _ => {
                    panic!("Unknown rule {:?} with {:?}", rule, text);
                }
            }
        }

        if let Some(id) = scanner_id_option {
            scanners.push(Scanner::new(id, beacons));
        }

        State { scanners }
    }

    fn reorient_scanners(&mut self) {
        let mut scanner_pairs = VecDeque::new();
        for j in 0..self.scanners.len() {
            for k in 0..self.scanners.len() {
                if j != k {
                    scanner_pairs.push_back((j, k));
                }
            }
        }

        self.scanners[0].offset_option = Some(Offset::new(0, 0, 0));

        while let Some((j, k)) = scanner_pairs.pop_front() {
            match (
                self.scanners[j].offset_option,
                self.scanners[k].offset_option,
            ) {
                (None, None) => scanner_pairs.push_back((j, k)),
                (Some(_), Some(_)) => {}
                (Some(_), None) => {
                    if let Some(scanner) = self.scanners[j].reorient_other(&self.scanners[k]) {
                        self.scanners[k] = scanner;
                    }
                }
                (None, Some(_)) => {
                    if let Some(scanner) = self.scanners[k].reorient_other(&self.scanners[j]) {
                        self.scanners[j] = scanner;
                    }
                }
            }
        }
    }

    fn beacon_count(&self) -> usize {
        let mut all_beacons = HashSet::new();

        for scanner in self.scanners.iter() {
            for beacon in scanner.beacons.iter() {
                all_beacons.insert(beacon);
            }
        }

        all_beacons.len()
    }

    fn largest_manhattan_distance(&self) -> i32 {
        let mut largest_manhattan_distance = 0;

        for j in 0..self.scanners.len() {
            for k in 0..self.scanners.len() {
                let offset_j = self.scanners[j].offset_option.unwrap();
                let offset_k = self.scanners[k].offset_option.unwrap();
                let x = (offset_j.x - offset_k.x).abs();
                let y = (offset_j.y - offset_k.y).abs();
                let z = (offset_j.z - offset_k.z).abs();
                if x + y + z > largest_manhattan_distance {
                    largest_manhattan_distance = x + y + z;
                }
            }
        }

        largest_manhattan_distance
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        for scanner in self.scanners.iter() {
            result.push_str(&format!("{}\n", scanner));
        }

        write!(f, "{}", result)
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut state = State::new(&input);
    state.reorient_scanners();

    // Part 1
    println!("Part 1: there are {} beacons", state.beacon_count());

    // Part 2
    println!(
        "Part 2: the largest Manhattan distance is {}",
        state.largest_manhattan_distance()
    );
}
