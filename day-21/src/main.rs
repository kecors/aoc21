#[derive(Debug)]
struct Die {
    number: u32,
}

impl Die {
    fn new() -> Die {
        let number = 100;

        Die { number }
    }

    fn roll(&mut self) -> u32 {
        if self.number == 100 {
            self.number = 1;
        } else {
            self.number += 1;
        }

        self.number
    }
}

#[derive(Debug)]
struct Player {
    space: u32,
    score: u32,
}

impl Player {
    fn new(space: u32) -> Player {
        let score = 0;

        Player { space, score }
    }
}

#[derive(Debug)]
struct State {
    die: Die,
    players: Vec<Player>,
    rolls: u32,
}

impl State {
    fn new(player_0_starting_space: u32, player_1_starting_space: u32) -> State {
        let die = Die::new();
        let players = vec![
            Player::new(player_0_starting_space),
            Player::new(player_1_starting_space),
        ];
        let rolls = 0;

        State {
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

    fn play(&mut self) -> u32 {
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

fn main() {
    let mut state = State::new(4, 8);
    let result = state.play();
    println!("Sample input (4, 8): the product is {}", result);

    let mut state = State::new(10, 7);
    let result = state.play();
    println!("Part 1 (10, 7): the product is {}", result);
}
