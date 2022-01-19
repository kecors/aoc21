use std::fmt;
use std::io::{stdin, Read};

// Image enhancement algorithm
#[derive(Debug)]
struct Iea {
    pixels: Vec<bool>,
}

impl fmt::Display for Iea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        for x in 0..self.pixels.len() {
            result.push(if self.pixels[x] { '#' } else { '.' });
        }
        result.push('\n');

        write!(f, "{}", result)
    }
}

impl Iea {
    fn new(lines: Vec<&str>) -> Iea {
        let pixels: Vec<bool> = lines
            .iter()
            .flat_map(|line| line.chars().map(|ch| ch == '#').collect::<Vec<bool>>())
            .collect();

        Iea { pixels }
    }

    fn apply(&self, number: usize) -> bool {
        self.pixels[number]
    }

    // Account for the "fun" part of this puzzle
    fn field_flips(&self) -> bool {
        self.pixels[0]
    }
}

#[derive(Debug)]
struct Image {
    pixel_rows: Vec<Vec<bool>>,
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        for y in 0..self.pixel_rows.len() {
            for x in 0..self.pixel_rows[0].len() {
                result.push(if self.pixel_rows[y][x] { '#' } else { '.' });
            }
            result.push('\n');
        }

        write!(f, "{}", result)
    }
}

impl Image {
    fn new(lines: Vec<&str>) -> Image {
        let pixel_rows: Vec<Vec<bool>> = lines
            .iter()
            .map(|line| line.chars().map(|ch| ch == '#').collect())
            .collect();

        Image { pixel_rows }
    }

    fn generate_output_image(&self, field: bool, iea: &Iea) -> Image {
        let mut input_pixel_rows =
            vec![vec![field; self.pixel_rows[0].len() + 4]; self.pixel_rows.len() + 4];
        for y in 0..self.pixel_rows.len() {
            for x in 0..self.pixel_rows[0].len() {
                input_pixel_rows[y + 2][x + 2] = self.pixel_rows[y][x];
            }
        }

        let mut output_pixel_rows = Vec::new();

        for y in 1..input_pixel_rows.len() - 1 {
            let mut output_pixel_row = Vec::new();
            for x in 1..input_pixel_rows[0].len() - 1 {
                let pixels = vec![
                    input_pixel_rows[y - 1][x - 1],
                    input_pixel_rows[y - 1][x],
                    input_pixel_rows[y - 1][x + 1],
                    input_pixel_rows[y][x - 1],
                    input_pixel_rows[y][x],
                    input_pixel_rows[y][x + 1],
                    input_pixel_rows[y + 1][x - 1],
                    input_pixel_rows[y + 1][x],
                    input_pixel_rows[y + 1][x + 1],
                ];
                let mut number = 0;
                let mut digit = 1;
                for &pixel in pixels.iter().rev() {
                    if pixel {
                        number += digit;
                    }
                    digit *= 2;
                }
                output_pixel_row.push(iea.apply(number));
            }
            output_pixel_rows.push(output_pixel_row);
        }

        Image {
            pixel_rows: output_pixel_rows,
        }
    }

    fn pixel_count(&self) -> u32 {
        let mut count = 0;

        for y in 0..self.pixel_rows.len() {
            for x in 0..self.pixel_rows[0].len() {
                if self.pixel_rows[y][x] {
                    count += 1;
                }
            }
        }

        count
    }
}

#[derive(Debug)]
struct State {
    iea: Iea,
    image: Image,
    field: bool,
}

impl State {
    fn new(input: &str) -> State {
        let iea = Iea::new(input.lines().take_while(|line| !line.is_empty()).collect());
        let image = Image::new(
            input
                .lines()
                .skip_while(|line| !line.is_empty())
                .skip(1)
                .collect(),
        );

        let field = false;

        State { iea, image, field }
    }

    fn apply_algorithm(&mut self) {
        self.image = self.image.generate_output_image(self.field, &self.iea);
        if self.iea.field_flips() {
            self.field = !self.field;
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    // Part 1

    let mut state = State::new(&input);
    for _ in 0..2 {
        state.apply_algorithm();
    }
    println!("Part 1: {} pixels are lit", state.image.pixel_count());

    // Part 2

    let mut state = State::new(&input);
    for _ in 0..50 {
        state.apply_algorithm();
    }
    println!("Part 2: {} pixels are lit", state.image.pixel_count());
}
