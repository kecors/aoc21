use std::collections::VecDeque;
use std::fmt;
use std::io::{stdin, Read};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Begin,
    Value(u32),
    Comma,
    End,
}

#[derive(Debug, PartialEq, Eq)]
enum ExplodeState {
    SeekExplosion,
    SeekLeftValue,
    SeekRightValue,
    SeekRightAddend,
    ExplosionDone,
}

#[derive(Debug, PartialEq, Eq)]
enum SplitState {
    SeekSplit,
    SplitDone,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Number {
    tokens: VecDeque<Token>,
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        for token in self.tokens.iter() {
            match token {
                Token::Begin => result.push('['),
                Token::Value(value) => result.push_str(&format!("{}", value)),
                Token::Comma => result.push(','),
                Token::End => result.push(']'),
            }
        }

        write!(f, "{}", result)
    }
}

impl Number {
    fn new(line: &str) -> Number {
        let tokens = line
            .chars()
            .map(|ch| match ch {
                '[' => Token::Begin,
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    Token::Value(ch.to_digit(10).unwrap())
                }
                ',' => Token::Comma,
                ']' => Token::End,
                _ => panic!("Unknown ch {}", ch),
            })
            .collect();

        Number { tokens }
    }

    fn add(mut left_number: Number, mut right_number: Number) -> Number {
        // Form the pair
        let mut tokens = VecDeque::new();
        tokens.push_back(Token::Begin);
        tokens.append(&mut left_number.tokens);
        tokens.push_back(Token::Comma);
        tokens.append(&mut right_number.tokens);
        tokens.push_back(Token::End);
        let mut number = Number { tokens };

        // Reduce
        let mut reducible = true;
        while reducible {
            reducible = number.explode();
            if !reducible {
                reducible = number.split();
            }
        }

        number
    }

    fn explode(&mut self) -> bool {
        let mut explode_state = ExplodeState::SeekExplosion;
        let mut processed_tokens = VecDeque::new();
        let mut tokens_cache = VecDeque::new();
        let mut left_addend_option = None;
        let mut begin_count = 0;

        while let Some(token) = self.tokens.pop_front() {
            match explode_state {
                ExplodeState::SeekExplosion => match token {
                    Token::Begin => {
                        begin_count += 1;
                        if begin_count == 5 {
                            if None == left_addend_option {
                                processed_tokens.append(&mut tokens_cache);
                            }
                            explode_state = ExplodeState::SeekLeftValue;
                        } else {
                            tokens_cache.push_back(token);
                        }
                    }
                    Token::Value(value) => {
                        if let Some(left_addend) = left_addend_option {
                            processed_tokens.push_back(Token::Value(left_addend));
                        }
                        left_addend_option = Some(value);
                        processed_tokens.append(&mut tokens_cache);
                        tokens_cache = VecDeque::new();
                    }
                    Token::Comma => {
                        tokens_cache.push_back(token);
                    }
                    Token::End => {
                        begin_count -= 1;
                        tokens_cache.push_back(token);
                    }
                },
                ExplodeState::SeekLeftValue => match token {
                    Token::Value(value) => {
                        if let Some(left_addend) = left_addend_option {
                            processed_tokens.push_back(Token::Value(left_addend + value));
                            processed_tokens.append(&mut tokens_cache);
                        }
                        processed_tokens.push_back(Token::Value(0));
                        explode_state = ExplodeState::SeekRightValue;
                    }
                    _ => {
                        panic!(
                            "Unexpected token {:?} in explosion state {:?}",
                            token, explode_state
                        );
                    }
                },
                ExplodeState::SeekRightValue => match token {
                    Token::Begin => {
                        panic!(
                            "Unexpected token {:?} in explosion state {:?}",
                            token, explode_state
                        );
                    }
                    Token::Value(value) => {
                        left_addend_option = Some(value);
                        tokens_cache = VecDeque::new();
                    }
                    Token::Comma => {}
                    Token::End => {
                        explode_state = ExplodeState::SeekRightAddend;
                    }
                },
                ExplodeState::SeekRightAddend => match token {
                    Token::Value(value) => {
                        if let Some(left_addend) = left_addend_option {
                            processed_tokens.push_back(Token::Value(left_addend + value));
                            explode_state = ExplodeState::ExplosionDone;
                        }
                    }
                    _ => {
                        processed_tokens.push_back(token);
                    }
                },
                ExplodeState::ExplosionDone => {
                    processed_tokens.push_back(token);
                }
            }
        }

        if explode_state == ExplodeState::SeekExplosion {
            if let Some(left_addend) = left_addend_option {
                processed_tokens.push_back(Token::Value(left_addend));
            }
            processed_tokens.append(&mut tokens_cache);
        }

        self.tokens = processed_tokens;

        explode_state == ExplodeState::ExplosionDone
    }

    fn split(&mut self) -> bool {
        let mut split_state = SplitState::SeekSplit;
        let mut processed_tokens = VecDeque::new();

        while let Some(token) = self.tokens.pop_front() {
            match split_state {
                SplitState::SeekSplit => match token {
                    Token::Value(value) => {
                        if value >= 10 {
                            processed_tokens.push_back(Token::Begin);
                            processed_tokens.push_back(Token::Value(value / 2));
                            processed_tokens.push_back(Token::Comma);
                            processed_tokens.push_back(Token::Value((value + 1) / 2));
                            processed_tokens.push_back(Token::End);
                            split_state = SplitState::SplitDone;
                        } else {
                            processed_tokens.push_back(token);
                        }
                    }
                    _ => {
                        processed_tokens.push_back(token);
                    }
                },
                SplitState::SplitDone => {
                    processed_tokens.push_back(token);
                }
            }
        }

        self.tokens = processed_tokens;

        split_state == SplitState::SplitDone
    }
}

#[derive(Debug, PartialEq, Eq)]
enum MagnitudeState {
    SeekBegin,
    SeekLeftElement,
    SeekLeftElementEnd,
    SeekComma,
    SeekRightElement,
    SeekRightElementEnd,
    SeekEnd,
    Done,
}

fn magnitude(mut tokens: VecDeque<Token>) -> u32 {
    let mut magnitude_state = MagnitudeState::SeekBegin;
    let mut tokens_cache = VecDeque::new();
    let mut begin_count = 0;
    let mut magnitude_value = 0;

    while let Some(token) = tokens.pop_front() {
        match magnitude_state {
            MagnitudeState::SeekBegin => match token {
                Token::Begin => {
                    magnitude_state = MagnitudeState::SeekLeftElement;
                }
                _ => {
                    panic!("Unexpected token {:?} in magnitude_state SeekBegin", token);
                }
            },
            MagnitudeState::SeekLeftElement => match token {
                Token::Begin => {
                    tokens_cache = VecDeque::new();
                    tokens_cache.push_back(token);
                    begin_count = 1;
                    magnitude_state = MagnitudeState::SeekLeftElementEnd;
                }
                Token::Value(value) => {
                    magnitude_value = value * 3;
                    magnitude_state = MagnitudeState::SeekComma;
                }
                _ => {
                    panic!(
                        "Unexpected token {:?} in magnitude_state SeekLeftElement",
                        token
                    );
                }
            },
            MagnitudeState::SeekLeftElementEnd => match token {
                Token::Begin => {
                    tokens_cache.push_back(token);
                    begin_count += 1;
                }
                Token::Value(_) | Token::Comma => {
                    tokens_cache.push_back(token);
                }
                Token::End => {
                    tokens_cache.push_back(token);
                    begin_count -= 1;
                    if begin_count == 0 {
                        let value = magnitude(tokens_cache.clone());
                        magnitude_value = value * 3;
                        magnitude_state = MagnitudeState::SeekComma;
                    }
                }
            },
            MagnitudeState::SeekComma => match token {
                Token::Comma => {
                    magnitude_state = MagnitudeState::SeekRightElement;
                }
                _ => {
                    panic!("Unexpected token {:?} in magnitude_state SeekComma", token);
                }
            },
            MagnitudeState::SeekRightElement => match token {
                Token::Begin => {
                    tokens_cache = VecDeque::new();
                    tokens_cache.push_back(token);
                    begin_count = 1;
                    magnitude_state = MagnitudeState::SeekRightElementEnd;
                }
                Token::Value(value) => {
                    magnitude_value += value * 2;
                    magnitude_state = MagnitudeState::SeekEnd;
                }
                _ => {
                    panic!(
                        "Unexpected token {:?} in magnitude_state SeekRightElement",
                        token
                    );
                }
            },
            MagnitudeState::SeekRightElementEnd => match token {
                Token::Begin => {
                    tokens_cache.push_back(token);
                    begin_count += 1;
                }
                Token::Value(_) | Token::Comma => {
                    tokens_cache.push_back(token);
                }
                Token::End => {
                    tokens_cache.push_back(token);
                    begin_count -= 1;
                    if begin_count == 0 {
                        let value = magnitude(tokens_cache.clone());
                        magnitude_value += value * 2;
                        magnitude_state = MagnitudeState::SeekEnd;
                    }
                }
            },
            MagnitudeState::SeekEnd => match token {
                Token::End => {
                    magnitude_state = MagnitudeState::Done;
                }
                _ => {
                    panic!("Unexpected token {:?} in magnitude_state SeekEnd", token);
                }
            },
            MagnitudeState::Done => {
                panic!("Unexpected token {:?} in magnitude_state Done", token);
            }
        }
    }

    magnitude_value
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    // Part 1

    let sum = input.lines().map(Number::new).reduce(Number::add).unwrap();

    println!(
        "Part 1: the magnitude of the final value is {}",
        magnitude(sum.tokens)
    );

    // Part 2

    let numbers: Vec<Number> = input.lines().map(Number::new).collect();
    let mut max_magnitude_value = 0;

    for number_a in numbers.iter() {
        for number_b in numbers.iter() {
            if number_a == number_b {
                continue;
            }
            let sum = Number::add(number_a.clone(), number_b.clone());
            let magnitude_value = magnitude(sum.tokens);
            if magnitude_value > max_magnitude_value {
                max_magnitude_value = magnitude_value;
            }
        }
    }

    println!(
        "Part 2: the largest magnitude of any sum is {}",
        max_magnitude_value
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn explode_example_1() {
        use crate::Number;

        let input = "[[[[[9,8],1],2],3],4]";
        let mut number = Number::new(&input);
        number.explode();
        assert_eq!(number.to_string(), "[[[[0,9],2],3],4]");
    }

    #[test]
    fn explode_example_2() {
        use crate::Number;

        let input = "[7,[6,[5,[4,[3,2]]]]]";
        let mut number = Number::new(&input);
        number.explode();
        assert_eq!(number.to_string(), "[7,[6,[5,[7,0]]]]");
    }

    #[test]
    fn explode_example_3() {
        use crate::Number;

        let input = "[[6,[5,[4,[3,2]]]],1]";

        let mut number = Number::new(&input);
        number.explode();
        assert_eq!(number.to_string(), "[[6,[5,[7,0]]],3]");
    }

    #[test]
    fn explode_example_4() {
        use crate::Number;

        let input = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]";

        let mut number = Number::new(&input);
        number.explode();
        assert_eq!(number.to_string(), "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
    }

    #[test]
    fn explode_example_5() {
        use crate::Number;

        let input = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";

        let mut number = Number::new(&input);
        number.explode();
        assert_eq!(number.to_string(), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    }

    #[test]
    fn addition_example_1() {
        use crate::Number;

        let addend_1 = "[[[[4,3],4],4],[7,[[8,4],9]]]";
        let addend_2 = "[1,1]";

        let number_1 = Number::new(&addend_1);
        let number_2 = Number::new(&addend_2);
        let result = Number::add(number_1, number_2);
        assert_eq!(result.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn magnitude_example_1() {
        use crate::Number;

        let input = "[[1,2],[[3,4],5]]";

        let number = Number::new(&input);
        assert_eq!(crate::magnitude(number.tokens), 143);
    }

    #[test]
    fn magnitude_example_2() {
        use crate::Number;

        let input = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";

        let number = Number::new(&input);
        assert_eq!(crate::magnitude(number.tokens), 1384);
    }

    #[test]
    fn magnitude_example_3() {
        use crate::Number;

        let input = "[[[[1,1],[2,2]],[3,3]],[4,4]]";

        let number = Number::new(&input);
        assert_eq!(crate::magnitude(number.tokens), 445);
    }

    #[test]
    fn magnitude_example_4() {
        use crate::Number;

        let input = "[[[[3,0],[5,3]],[4,4]],[5,5]]";

        let number = Number::new(&input);
        assert_eq!(crate::magnitude(number.tokens), 791);
    }

    #[test]
    fn magnitude_example_5() {
        use crate::Number;

        let input = "[[[[5,0],[7,4]],[5,5]],[6,6]]";

        let number = Number::new(&input);
        assert_eq!(crate::magnitude(number.tokens), 1137);
    }

    #[test]
    fn magnitude_example_6() {
        use crate::Number;

        let input = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";

        let number = Number::new(&input);
        assert_eq!(crate::magnitude(number.tokens), 3488);
    }
}
