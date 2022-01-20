use std::cmp;
use std::collections::HashMap;

#[derive(Debug)]
struct Die {
    number: u64,
}

impl Die {
    fn new() -> Die {
        let number = 100;

        Die { number }
    }

    fn roll(&mut self) -> u64 {
        if self.number == 100 {
            self.number = 1;
        } else {
            self.number += 1;
        }

        self.number
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Player {
    space: u64,
    score: u64,
}

impl Player {
    fn new(space: u64) -> Player {
        let score = 0;

        Player { space, score }
    }
}

struct Part1 {
    die: Die,
    players: Vec<Player>,
    rolls: u64,
}

impl Part1 {
    fn new(player_0_starting_space: u64, player_1_starting_space: u64) -> Part1 {
        let die = Die::new();
        let players = vec![
            Player::new(player_0_starting_space),
            Player::new(player_1_starting_space),
        ];
        let rolls = 0;

        Part1 {
            die,
            players,
            rolls,
        }
    }

    fn player_turn(&mut self, player: usize) {
        let mut space = self.players[player].space;
        for _ in 0..3 {
            space += self.die.roll();
            self.rolls += 1;
        }
        space %= 10;
        if space == 0 {
            space = 10;
        }
        self.players[player].space = space;
        self.players[player].score += space;
    }

    fn play(&mut self) -> u64 {
        loop {
            self.player_turn(0);
            if self.players[0].score >= 1000 {
                return self.players[1].score * self.rolls;
            }
            self.player_turn(1);
            if self.players[1].score >= 1000 {
                return self.players[0].score * self.rolls;
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Game {
    players: Vec<Player>,
    next_player: usize,
}

impl Game {
    fn new(player_0_starting_space: u64, player_1_starting_space: u64) -> Game {
        let players = vec![
            Player::new(player_0_starting_space),
            Player::new(player_1_starting_space),
        ];
        let next_player = 0;

        Game {
            players,
            next_player,
        }
    }
}

struct Part2 {
    advance_counts: HashMap<u64, u64>,
    games: HashMap<Game, u64>,
    wins: Vec<u64>,
}

impl Part2 {
    fn new(player_0_starting_space: u64, player_1_starting_space: u64) -> Part2 {
        let mut advance_counts = HashMap::new();
        for roll_1 in 1..=3 {
            for roll_2 in 1..=3 {
                for roll_3 in 1..=3 {
                    let roll_total = roll_1 + roll_2 + roll_3;
                    *advance_counts.entry(roll_total).or_insert(0) += 1;
                }
            }
        }

        let mut games = HashMap::new();
        games.insert(
            Game::new(player_0_starting_space, player_1_starting_space),
            1,
        );

        let wins = vec![0, 0];

        Part2 {
            advance_counts,
            games,
            wins,
        }
    }

    fn play(&mut self) -> u64 {
        loop {
            let mut new_games = HashMap::new();

            for (game, universe_count) in self.games.drain() {
                for (advance, advance_count) in self.advance_counts.iter() {
                    let mut space = game.players[game.next_player].space + advance;
                    if space > 10 {
                        space -= 10;
                    }
                    let score = game.players[game.next_player].score + space;

                    if score >= 21 {
                        self.wins[game.next_player] += universe_count * advance_count;
                        continue;
                    }

                    let mut new_game = game.clone();
                    new_game.players[new_game.next_player].space = space;
                    new_game.players[new_game.next_player].score = score;
                    new_game.next_player = if new_game.next_player == 0 { 1 } else { 0 };

                    if let Some(count) = new_games.get_mut(&new_game) {
                        *count += universe_count * advance_count;
                    } else {
                        new_games.insert(new_game, universe_count * advance_count);
                    }
                }
            }

            self.games = new_games;

            if self.games.is_empty() {
                break;
            }
        }

        cmp::max(self.wins[0], self.wins[1])
    }
}

fn main() {
    // Part 1
    let mut part1 = Part1::new(4, 8);
    println!("Sample input (4, 8): the product is {}", part1.play());

    let mut part1 = Part1::new(10, 7);
    println!("Part 1 (10, 7): the product is {}", part1.play());

    // Part 2
    let mut part2 = Part2::new(4, 8);
    println!("Sample input (4, 8): the total is {}", part2.play());

    let mut part2 = Part2::new(10, 7);
    println!("Part 2 (10, 7): the total is {}", part2.play());
}
