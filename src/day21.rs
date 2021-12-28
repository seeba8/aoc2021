pub fn solve() {
    let mut game = DiracDice::new(3, 5);
    println!("Day 21 part 1: {}", game.play());
}

struct Player {
    score: usize,
    position: usize,
}

impl Player {
    fn new(starting_position: usize) -> Player {
        Player {
            score: 0,
            position: starting_position,
        }
    }

    fn move_forward(&mut self, amount: usize) {
        self.position = ((self.position + amount - 1) % 10) + 1;
        self.score += self.position as usize;
    }
}

enum WhoseTurnIsIt {
    Player1,
    Player2,
}

struct DiracDice {
    player1: Player,
    player2: Player,
    dice: DeterministicDice,
    whose_turn_is_it: WhoseTurnIsIt,
    die_rolls: usize,
}

impl DiracDice {
    fn new(player1: usize, player2: usize) -> DiracDice {
        DiracDice {
            player1: Player::new(player1),
            player2: Player::new(player2),
            dice: DeterministicDice::new(),
            whose_turn_is_it: WhoseTurnIsIt::Player1,
            die_rolls: 0,
        }
    }

    fn roll(&mut self) -> usize {
        let player = match self.whose_turn_is_it {
            WhoseTurnIsIt::Player1 => {
                self.whose_turn_is_it = WhoseTurnIsIt::Player2;
                &mut self.player1
            }
            WhoseTurnIsIt::Player2 => {
                self.whose_turn_is_it = WhoseTurnIsIt::Player1;
                &mut self.player2
            }
        };
        let roll = self.dice.roll_3_times().unwrap();
        self.die_rolls += 3;
        player.move_forward(roll);
        player.score
    }

    fn play(&mut self) -> usize {
        while self.player1.score < 1000 && self.player2.score < 1000 {
            self.roll();
        }
        self.player1.score.min(self.player2.score) * self.die_rolls
    }
}

#[derive(Debug, Clone)]
struct DeterministicDice {
    value: usize,
}

impl Iterator for DeterministicDice {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.value += 1;
        if self.value == 101 {
            self.value = 1;
        }
        Some(self.value)
    }
}

impl DeterministicDice {
    fn new() -> DeterministicDice {
        DeterministicDice { value: 0 }
    }

    fn roll_3_times(&mut self) -> Option<usize> {
        Some(self.next()? + self.next()? + self.next()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_cycles_the_dice() {
        let mut dice = DeterministicDice::new();
        assert_eq!(Some(1), dice.next());
        assert_eq!(Some(100), dice.nth(98));
        assert_eq!(Some(1), dice.next());
    }

    #[test]
    fn it_rolls() {
        let mut game = DiracDice::new(4, 8);
        assert_eq!(10, game.roll());
        assert_eq!(3, game.roll());
        assert_eq!(14, game.roll());
        assert_eq!(9, game.roll());
    }

    #[test]
    fn it_finishes_game() {
        let mut game = DiracDice::new(4, 8);
        assert_eq!(739785, game.play());
    }
}
