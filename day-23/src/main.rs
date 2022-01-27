use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt;
use std::io::{stdin, Read};

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn from(letter: char) -> Amphipod {
        match letter {
            'A' => Amphipod::Amber,
            'B' => Amphipod::Bronze,
            'C' => Amphipod::Copper,
            'D' => Amphipod::Desert,
            _ => panic!("Unexpected letter {}", letter),
        }
    }

    fn as_char(&self) -> char {
        match self {
            Amphipod::Amber => 'A',
            Amphipod::Bronze => 'B',
            Amphipod::Copper => 'C',
            Amphipod::Desert => 'D',
        }
    }

    fn energy_per_step(&self) -> u32 {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone)]
enum Place {
    Unmapped,
    Wall,
    Hall(Option<Amphipod>),
    Doorway,
    Room((Amphipod, Option<Amphipod>)),
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Place::Unmapped => ' ',
                Place::Wall => '#',
                Place::Hall(resident) => {
                    if let Some(amphipod) = resident {
                        amphipod.as_char()
                    } else {
                        '.'
                    }
                }
                Place::Doorway => 'o',
                Place::Room((_, resident)) => {
                    if let Some(amphipod) = resident {
                        amphipod.as_char()
                    } else {
                        '-'
                    }
                }
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
struct Map {
    energy_spent: u32,
    place_rows: Vec<Vec<Place>>,
    part_2_flag: bool,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        result.push_str(&format!("Energy spent: {}\n", self.energy_spent));
        for place_row in self.place_rows.iter() {
            for place in place_row.iter() {
                result.push_str(&format!("{}", place));
            }
            result.push('\n');
        }

        write!(f, "{}", result)
    }
}

impl Map {
    fn new(input: &str, part_2_flag: bool) -> Map {
        let energy_spent = 0;

        let mut lines: Vec<&str> = input.lines().collect();

        if part_2_flag {
            let mut remaining_lines = lines.split_off(3);
            lines.append(&mut vec!["  #D#C#B#A#", "  #D#B#A#C#"]);
            lines.append(&mut remaining_lines);
        }

        let mut place_rows: Vec<Vec<Place>> = lines
            .iter()
            .map(|line| {
                let mut room_columns: HashSet<usize> = HashSet::new();
                line.chars()
                    .enumerate()
                    .map(|(column, ch)| match ch {
                        ' ' => Place::Unmapped,
                        '#' => Place::Wall,
                        '.' => Place::Hall(None),
                        'A' | 'B' | 'C' | 'D' => {
                            room_columns.insert(column);
                            let target = match room_columns.len() {
                                1 => Amphipod::Amber,
                                2 => Amphipod::Bronze,
                                3 => Amphipod::Copper,
                                4 => Amphipod::Desert,
                                _ => panic!("Too many room columns"),
                            };
                            let resident = Amphipod::from(ch);
                            Place::Room((target, Some(resident)))
                        }
                        _ => panic!("Unknown place {:?}", ch),
                    })
                    .collect()
            })
            .collect();

        // Update place_rows with Doorway locations
        let mut room_columns = Vec::new();
        for (column, place) in place_rows[2].iter().enumerate() {
            if let &Place::Room(_) = place {
                room_columns.push(column);
            }
        }
        for &room_column in room_columns.iter() {
            place_rows[1][room_column] = Place::Doorway;
        }

        Map {
            energy_spent,
            place_rows,
            part_2_flag,
        }
    }

    fn amphipods_are_organized(&self) -> bool {
        for place_row in self.place_rows.iter() {
            for place in place_row.iter() {
                if let Place::Room((target, resident_option)) = place {
                    if let Some(resident) = resident_option {
                        if resident != target {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn visitors_have_departed(&self, x: usize) -> bool {
        let mut y = 1;
        loop {
            y += 1;
            match self.place_rows[y][x] {
                Place::Room((_target, None)) => {}
                Place::Room((target, Some(amphipod))) => {
                    if amphipod != target {
                        return false;
                    }
                }
                Place::Wall => {
                    break;
                }
                _ => {
                    panic!("Unexpected {:?} at ({},{})", self.place_rows[y][x], x, y);
                }
            }
        }

        true
    }

    fn amphipod_locations(&self) -> Vec<(usize, usize)> {
        let mut locations: Vec<(usize, usize)> = Vec::new();

        for (y, place_row) in self.place_rows.iter().enumerate() {
            for (x, place) in place_row.iter().enumerate() {
                match place {
                    Place::Hall(Some(_)) => locations.push((x, y)),
                    Place::Room((_, Some(_))) => locations.push((x, y)),
                    _ => (),
                }
            }
        }

        locations
    }

    fn room_x(&self, amphipod: Amphipod) -> usize {
        for (x, place) in self.place_rows[2].iter().enumerate() {
            if let Place::Room((target, _)) = place {
                if amphipod == *target {
                    return x;
                }
            }
        }

        panic!("Target column not found for {:?}", amphipod);
    }

    fn hall_path_is_clear(&self, start_x: usize, end_x: usize) -> bool {
        for x in start_x..=end_x {
            match self.place_rows[1][x] {
                Place::Hall(None) => (),
                Place::Doorway => (),
                _ => return false,
            }
        }

        true
    }

    fn available_room_place(&self, x: usize) -> Option<usize> {
        let mut room_y_option = None;
        let mut y = 1;

        loop {
            y += 1;
            match self.place_rows[y][x] {
                Place::Room((_, None)) => {
                    room_y_option = Some(y);
                }
                Place::Room((target, Some(amphipod))) => {
                    if amphipod != target {
                        return None;
                    }
                }
                Place::Wall => {
                    break;
                }
                _ => {
                    panic!(
                        "Unexpected place {:?} at ({},{}) instead of room",
                        self.place_rows[y][x], x, y
                    );
                }
            }
        }

        room_y_option
    }

    fn steps_to_leave_room(&self, amphipod_x: usize, amphipod_y: usize) -> Option<u32> {
        for y in 2..amphipod_y {
            match self.place_rows[y][amphipod_x] {
                Place::Room((_, None)) => (),
                Place::Room((_, Some(_))) => return None,
                _ => panic!(
                    "{:?} at ({},{}) unexpectedly blocks leaving room",
                    self.place_rows[y][amphipod_x], amphipod_x, y
                ),
            }
        }

        Some(amphipod_y as u32 - 1)
    }

    fn hall_to_room(&self, x: usize, amphipod: Amphipod) -> Option<(u32, usize, usize)> {
        let room_x = self.room_x(amphipod);

        if room_x == x {
            return None;
        }

        if x < room_x {
            if !self.hall_path_is_clear(x + 1, room_x) {
                return None;
            }
        } else if !self.hall_path_is_clear(room_x, x - 1) {
            return None;
        }

        if let Some(room_y) = self.available_room_place(room_x) {
            let steps = (x as isize - room_x as isize).abs() as usize + room_y - 1;
            Some((steps as u32, room_x, room_y))
        } else {
            None
        }
    }

    fn doorway_to_hall(&self, doorway_x: usize) -> Vec<(u32, usize)> {
        let mut moves = Vec::new();

        let mut x = doorway_x;
        loop {
            x -= 1;
            match self.place_rows[1][x] {
                Place::Doorway => (),
                Place::Hall(None) => moves.push(((doorway_x - x) as u32, x)),
                Place::Hall(Some(_)) => break,
                Place::Wall => break,
                _ => panic!("Unexpected place {:?}", self.place_rows[1][x]),
            }
        }

        let mut x = doorway_x;
        loop {
            x += 1;
            match self.place_rows[1][x] {
                Place::Doorway => (),
                Place::Hall(None) => moves.push(((x - doorway_x) as u32, x)),
                Place::Hall(Some(_)) => break,
                Place::Wall => break,
                _ => panic!("Unexpected place {:?}", self.place_rows[1][x]),
            }
        }

        moves
    }

    fn move_amphipod(
        &mut self,
        from_x: usize,
        from_y: usize,
        to_x: usize,
        to_y: usize,
        caller: u8,
    ) {
        let amphipod = match self.place_rows[from_y][from_x] {
            Place::Hall(Some(amphipod)) => {
                self.place_rows[from_y][from_x] = Place::Hall(None);
                amphipod
            }
            Place::Room((target, Some(amphipod))) => {
                self.place_rows[from_y][from_x] = Place::Room((target, None));
                amphipod
            }
            _ => {
                panic!(
                    "caller {}: Attempt to move {:?} from ({},{})",
                    caller, self.place_rows[from_y][from_x], from_x, from_y
                );
            }
        };

        match self.place_rows[to_y][to_x] {
            Place::Hall(None) => {
                self.place_rows[to_y][to_x] = Place::Hall(Some(amphipod));
            }
            Place::Room((target, None)) => {
                if amphipod == target {
                    self.place_rows[to_y][to_x] = Place::Room((target, Some(amphipod)));
                } else {
                    panic!(
                        "caller {}: Attempt to move {:?} to room {:?} at ({},{})",
                        caller, amphipod, target, to_x, to_y
                    );
                }
            }
            _ => {
                panic!(
                    "caller {}: Attempt to move {:?} to ({},{}) containing {:?}",
                    caller, amphipod, to_x, to_y, self.place_rows[to_y][to_x]
                );
            }
        }
    }

    fn estimate_remaining_energy_cost(&self) -> u32 {
        let mut total_estimate = 0;

        let mut completed_d2r_steps: HashMap<Amphipod, usize> = HashMap::new();
        completed_d2r_steps.insert(Amphipod::Amber, 0);
        completed_d2r_steps.insert(Amphipod::Bronze, 0);
        completed_d2r_steps.insert(Amphipod::Copper, 0);
        completed_d2r_steps.insert(Amphipod::Desert, 0);

        for &(x, y) in self.amphipod_locations().iter() {
            match self.place_rows[y][x] {
                Place::Hall(Some(amphipod)) => {
                    let steps = (x as isize - self.room_x(amphipod) as isize).abs() as usize;
                    total_estimate += steps as u32 * amphipod.energy_per_step();
                }
                Place::Room((target, Some(amphipod))) => {
                    if amphipod == target && self.visitors_have_departed(x) {
                        *completed_d2r_steps.entry(amphipod).or_insert(0) += y - 1;
                        continue;
                    }
                    let r2d_steps = y - 1;
                    let d2d_steps = (x as isize - self.room_x(amphipod) as isize).abs() as usize;
                    let steps = r2d_steps + d2d_steps;
                    total_estimate += steps as u32 * amphipod.energy_per_step();
                }
                _ => {
                    panic!(
                        "Cannot estimate remaining energy cost for {:?} at ({},{})",
                        self.place_rows[y][x], x, y
                    );
                }
            }
        }

        let max_d2r_steps = if self.part_2_flag { 10 } else { 3 };

        // Calculate the cost of entering rooms now that it is known
        // how many amphipods have already entered
        for (&amphipod, &steps) in completed_d2r_steps.iter() {
            total_estimate += (max_d2r_steps - steps as u32) * amphipod.energy_per_step();
        }

        total_estimate
    }

    fn all_valid_moves(&self) -> Vec<Map> {
        let mut moves = Vec::new();

        for &(x, y) in self.amphipod_locations().iter() {
            match self.place_rows[y][x] {
                Place::Hall(Some(amphipod)) => {
                    if let Some((steps, room_x, room_y)) = self.hall_to_room(x, amphipod) {
                        let mut new_map = self.clone();
                        new_map.move_amphipod(x, y, room_x, room_y, 1);
                        new_map.energy_spent += steps * amphipod.energy_per_step();
                        moves.push(new_map);
                    }
                }
                Place::Room((target, Some(amphipod))) => {
                    if amphipod == target && self.visitors_have_departed(x) {
                        continue;
                    }

                    let r2d_steps = if let Some(room_steps) = self.steps_to_leave_room(x, y) {
                        room_steps
                    } else {
                        continue;
                    };

                    if let Some((h2r_steps, room_x, room_y)) = self.hall_to_room(x, amphipod) {
                        let mut new_map = self.clone();
                        new_map.move_amphipod(x, y, room_x, room_y, 2);
                        let steps = r2d_steps + h2r_steps;
                        new_map.energy_spent += steps * amphipod.energy_per_step();
                        moves.push(new_map);
                    }

                    for (d2h_steps, hall_x) in self.doorway_to_hall(x) {
                        let mut new_map = self.clone();
                        new_map.move_amphipod(x, y, hall_x, 1, 3);
                        let steps = r2d_steps + d2h_steps;
                        new_map.energy_spent += steps * amphipod.energy_per_step();
                        moves.push(new_map);
                    }
                }
                _ => {}
            }
        }

        moves
    }
}

#[derive(Debug)]
struct State {
    map: Map,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        result.push_str(&format!("{}", self.map));

        write!(f, "{}", result)
    }
}

impl State {
    fn new(input: &str, part_2_flag: bool) -> State {
        let map = Map::new(input, part_2_flag);

        State { map }
    }

    fn solve(&self) -> u32 {
        let mut candidates: BinaryHeap<Reverse<(u32, Map)>> = BinaryHeap::new();
        let mut parents: HashMap<Map, Map> = HashMap::new();

        candidates.push(Reverse((0, self.map.clone())));

        while let Some(Reverse((_estimate, map))) = candidates.pop() {
            if map.amphipods_are_organized() {
                return map.energy_spent;
            }

            for new_map in map.all_valid_moves().drain(..) {
                if !parents.contains_key(&new_map) {
                    parents.insert(new_map.clone(), map.clone());
                    let new_estimate = map.energy_spent + map.estimate_remaining_energy_cost();
                    candidates.push(Reverse((new_estimate, new_map)));
                }
            }
        }

        panic!("No solution found");
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    // Part 1

    let state = State::new(&input, false);
    println!(
        "Part 1: the least energy required to organize the amphipods is {}",
        state.solve()
    );

    // Part 2

    let state = State::new(&input, true);
    println!(
        "Part 2: the least energy required to organize the amphipods is {}",
        state.solve()
    );
}
