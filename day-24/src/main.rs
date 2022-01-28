// The solutions for both parts of the puzzle were obtained through
// observation and deduction, rather than direct calculation. The
// code below was useful in finding the solutions.
//
// Reading about other people's solutions on reddit helped me solve this puzzle.
// I found especially useful comments by JulienTT, aexl, pedantic_git and
// relativistic-turtle.

use std::fmt;
use std::io::{stdin, Read};
use std::iter::Cycle;
use std::ops::RangeInclusive;

#[derive(Debug, Copy, Clone)]
enum Verb {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

impl fmt::Display for Verb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Verb::Inp => "inp",
                Verb::Add => "add",
                Verb::Mul => "mul",
                Verb::Div => "div",
                Verb::Mod => "mod",
                Verb::Eql => "eql",
            }
        )
    }
}

#[derive(Debug, Copy, Clone)]
enum Variable {
    W,
    X,
    Y,
    Z,
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Variable::W => "w",
                Variable::X => "x",
                Variable::Y => "y",
                Variable::Z => "z",
            }
        )
    }
}

#[derive(Debug, Copy, Clone)]
enum Value {
    Variable(Variable),
    Number(i64),
    Unused,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:>3}",
            match self {
                Value::Variable(variable) => format!("{}", variable),
                Value::Number(number) => format!("{}", number),
                Value::Unused => String::from(""),
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Alu {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl fmt::Display for Alu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            &format!(
                "Alu <w: {:>3}, x: {:>3}, y: {:>3}, z: {:>3}>",
                self.w, self.x, self.y, self.z
            )
        )
    }
}

impl Alu {
    fn new(w: i64, x: i64, y: i64, z: i64) -> Alu {
        Alu { w, x, y, z }
    }

    fn variable(&mut self, variable: &Variable) -> &mut i64 {
        match variable {
            Variable::W => &mut self.w,
            Variable::X => &mut self.x,
            Variable::Y => &mut self.y,
            Variable::Z => &mut self.z,
        }
    }

    fn value(&self, value: &Value) -> i64 {
        match value {
            Value::Variable(variable) => match variable {
                Variable::W => self.w,
                Variable::X => self.x,
                Variable::Y => self.y,
                Variable::Z => self.z,
            },
            Value::Number(number) => *number,
            Value::Unused => panic!("Alu should not examine value when unused"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    verb: Verb,
    variable: Variable,
    value: Value,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.verb, self.variable, self.value)
    }
}

impl Instruction {
    fn new(line: &str) -> Instruction {
        let parts: Vec<&str> = line.split(' ').collect();

        let verb = match parts[0] {
            "inp" => Verb::Inp,
            "add" => Verb::Add,
            "mul" => Verb::Mul,
            "div" => Verb::Div,
            "mod" => Verb::Mod,
            "eql" => Verb::Eql,
            _ => panic!("Unknown verb {}", parts[0]),
        };

        let variable = match parts[1] {
            "w" => Variable::W,
            "x" => Variable::X,
            "y" => Variable::Y,
            "z" => Variable::Z,
            _ => panic!("Unknown variable {}", parts[1]),
        };

        let value = if parts.len() == 3 {
            match parts[2] {
                "w" => Value::Variable(Variable::W),
                "x" => Value::Variable(Variable::X),
                "y" => Value::Variable(Variable::Y),
                "z" => Value::Variable(Variable::Z),
                _ => Value::Number(parts[2].parse::<i64>().unwrap()),
            }
        } else {
            Value::Unused
        };

        Instruction {
            verb,
            variable,
            value,
        }
    }
}

trait Input {
    fn next_digit(&mut self) -> i64;
}

#[derive(Debug)]
struct Executor<'a> {
    alu: Alu,
    instructions: &'a [Instruction],
}

impl<'a> Executor<'a> {
    fn new(alu: Alu, instructions: &[Instruction]) -> Executor {
        Executor { alu, instructions }
    }

    fn run(&mut self, input: &mut impl Input, debug_flag: bool) {
        if debug_flag {
            println!("{}", self.alu);
        }
        for instruction in self.instructions.iter() {
            if debug_flag {
                print!("{}", instruction);
            }
            match instruction.verb {
                Verb::Inp => {
                    let a = self.alu.variable(&instruction.variable);
                    let value = input.next_digit();
                    *a = value;
                }
                Verb::Add => {
                    let b = self.alu.value(&instruction.value);
                    let a = self.alu.variable(&instruction.variable);
                    *a += b;
                }
                Verb::Mul => {
                    let b = self.alu.value(&instruction.value);
                    let a = self.alu.variable(&instruction.variable);
                    *a *= b;
                }
                Verb::Div => {
                    let b = self.alu.value(&instruction.value);
                    let a = self.alu.variable(&instruction.variable);
                    *a /= b;
                }
                Verb::Mod => {
                    let b = self.alu.value(&instruction.value);
                    let a = self.alu.variable(&instruction.variable);
                    *a %= b;
                }
                Verb::Eql => {
                    let b = self.alu.value(&instruction.value);
                    let a = self.alu.variable(&instruction.variable);
                    if *a == b {
                        *a = 1;
                    } else {
                        *a = 0;
                    }
                }
            }
            if debug_flag {
                println!(" => {}", self.alu);
            }
        }
    }
}

#[derive(Debug)]
struct ModelNumber {
    digits: Vec<i64>,
    index: usize,
}

impl Input for ModelNumber {
    fn next_digit(&mut self) -> i64 {
        let digit = self.digits[self.index];
        self.index += 1;

        digit
    }
}

impl fmt::Display for ModelNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        for digit in self.digits.iter() {
            result.push_str(&format!("{}", digit));
        }

        write!(f, "{}", result)
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let instructions: Vec<Instruction> = input.lines().map(Instruction::new).collect();

    // The puzzle input repeats, almost verbatim, 18 instructions
    // 14 times. The only differences are the values for the
    // fifth, sixth, and sixteen instructions in each section.
    // This code displays those values for each section.
    let mut section_values = Vec::new();
    let offsets = vec![4, 5, 15];
    for section in 0..14 {
        let mut values = Vec::new();
        for offset in offsets.iter() {
            let value = instructions[section * 18 + offset].value;
            if let Value::Number(number) = value {
                values.push(number);
            } else {
                panic!(
                    "Non-number value {:?} for section {} at offset {}",
                    value, section, offset
                );
            }
        }
        section_values.push(values);
    }
    for (section, values) in section_values.iter().enumerate() {
        println!("section {}", section);
        for value in values.iter() {
            print!("{} ", value);
        }
        println!();
    }

    // Sections whose fifth value is 26 "push" and sections whose fifth value
    // is 1 "pop". Each push is reversed by a pop. I identified pairs of sections
    // based on ordering. The difference between the pushing section's input and
    // the popping section's input is determined by the pushing section's
    // sixteen value and the popping section's sixth value. The code below
    // enabled me to work out the balance between inputs by considering the paired
    // sections in isolation.
    let data = vec![
        (0, 4, 1, 5, 8),  // So section 4 input must equal section 5 input - 7
        (0, 6, 6, 7, 1),  // So section 6 input must equal section 7 input + 5
        (0, 3, 9, 8, 1),  // So section 3 input must equal section 8 input + 8
        (0, 9, 5, 10, 1), // So section 9 input must equal section 10 input + 4
        (0, 2, 7, 11, 7), // So section 2 input must equal section 11 input
        (0, 1, 1, 12, 6), // So section 1 input must equal section 12 input - 5
        (0, 0, 2, 13, 1), // So section 0 input must equal section 13 input + 1
    ];
    for &(initial_z, section_m, input_m, section_n, input_n) in data.iter() {
        let alu = Alu::new(0, 0, 0, initial_z);
        let range = (section_m * 18)..(section_m * 18 + 18);
        let mut executor = Executor::new(alu, &instructions[range]);
        let mut model_number = ModelNumber {
            digits: vec![input_m],
            index: 0,
        };
        println!("\nsection {}", section_m);
        executor.run(&mut model_number, true);

        let alu = Alu::new(0, 0, 0, executor.alu.z);
        let range = (section_n * 18)..(section_n * 18 + 18);
        let mut executor = Executor::new(alu, &instructions[range]);
        let mut model_number = ModelNumber {
            digits: vec![input_n],
            index: 0,
        };
        println!("section {}", section_n);
        executor.run(&mut model_number, true);
    }

    // Part 1 requires the largest model number. I deduced the model number here
    // by making the input of the earlier section of each pair as large as possible.
    println!("\nPart 1:");
    let alu = Alu::new(0, 0, 0, 0);
    let mut executor = Executor::new(alu, &instructions);
    let mut model_number = ModelNumber {
        digits: vec![9, 4, 9, 9, 2, 9, 9, 4, 1, 9, 5, 9, 9, 8],
        index: 0,
    };
    executor.run(&mut model_number, true);
    println!("Part 1 result: {}", executor.alu);

    // Part 2 requires the smallest model number. Similar reasoning as above.
    println!("\nPart 2:");
    let alu = Alu::new(0, 0, 0, 0);
    let mut executor = Executor::new(alu, &instructions);
    let mut model_number = ModelNumber {
        digits: vec![2, 1, 1, 9, 1, 8, 6, 1, 1, 5, 1, 1, 6, 1],
        index: 0,
    };
    executor.run(&mut model_number, true);
    println!("Part 2 result: {}", executor.alu);
}

// Below is some more code I wrote while working on this problem.
// This code was not needed for finding the solutions.

#[allow(dead_code)]
#[derive(Debug)]
struct ModelNumberSource {
    internals: Vec<(i64, Cycle<RangeInclusive<i64>>)>,
    model_number: ModelNumber,
}

impl ModelNumberSource {
    #[allow(dead_code)]
    fn new() -> ModelNumberSource {
        let mut internals = vec![(0, (1..=9).cycle()); 14];
        let mut digits = Vec::new();

        for (digit, cycle) in internals.iter_mut() {
            *digit = cycle.next().unwrap();
            digits.push(10 - *digit);
        }

        let model_number = ModelNumber { digits, index: 0 };

        ModelNumberSource {
            internals,
            model_number,
        }
    }

    #[allow(dead_code)]
    fn advance(&mut self) {
        let mut cycle_flag = true;
        let mut digits = Vec::new();

        for (digit, cycle) in self.internals.iter_mut().rev() {
            if cycle_flag {
                if *digit != 9 {
                    cycle_flag = false;
                }
                *digit = cycle.next().unwrap();
            }
            digits.push(10 - *digit);
        }
        digits = digits.into_iter().rev().collect();

        self.model_number = ModelNumber { digits, index: 0 };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_1() {
        use crate::*;

        let input = "inp x\nmul x -1";
        let instructions: Vec<Instruction> = input.lines().map(Instruction::new).collect();
        let alu = Alu::new(0, 0, 0, 0);
        let mut executor = Executor::new(alu, &instructions);
        let mut model_number = ModelNumber {
            digits: vec![7],
            index: 0,
        };
        executor.run(&mut model_number, false);
        assert_eq!(
            executor.alu,
            Alu {
                w: 0,
                x: -7,
                y: 0,
                z: 0
            }
        );
    }

    #[test]
    fn sample_2() {
        use crate::*;

        let input = "inp z\ninp x\nmul z 3\neql z x";
        let instructions: Vec<Instruction> = input.lines().map(Instruction::new).collect();
        let alu = Alu::new(0, 0, 0, 0);
        let mut executor = Executor::new(alu, &instructions);
        let mut model_number = ModelNumber {
            digits: vec![3, 9],
            index: 0,
        };
        executor.run(&mut model_number, false);
        assert_eq!(
            executor.alu,
            Alu {
                w: 0,
                x: 9,
                y: 0,
                z: 1
            }
        );
    }

    #[test]
    fn sample_3() {
        use crate::*;

        let input = "inp w\nadd z w\nmod z 2\ndiv w 2\nadd y w\nmod y 2\ndiv w 2\nadd x w\nmod x 2\ndiv w 2\nmod w 2";
        let instructions: Vec<Instruction> = input.lines().map(Instruction::new).collect();
        let alu = Alu::new(0, 0, 0, 0);
        let mut executor = Executor::new(alu, &instructions);
        let mut model_number = ModelNumber {
            digits: vec![6],
            index: 0,
        };
        executor.run(&mut model_number, false);
        assert_eq!(
            executor.alu,
            Alu {
                w: 0,
                x: 1,
                y: 1,
                z: 0
            }
        );
    }
}
