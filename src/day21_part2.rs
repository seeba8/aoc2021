use std::ops::{AddAssign, Mul};

pub fn solve() {
    let wins = roll(3, 0, 5, 0, Player::Player1);
    println!("Day 21 part 2: {}", wins.0.max(wins.1));
}

enum Player {
    Player1,
    Player2,
}
#[derive(Clone, Copy, Default, Debug, PartialEq)]
struct Wins(usize, usize);

impl AddAssign for Wins {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Mul<u8> for Wins {
    type Output = Wins;

    fn mul(self, rhs: u8) -> Self::Output {
        Wins(self.0 * rhs as usize, self.1 * rhs as usize)
    }
}

#[inline]
fn move_score(old_position: u8, old_score: u8, roll: u8) -> (u8, u8) {
    let new_position = ((old_position + roll - 1) % 10) + 1;
    let new_score = old_score + new_position;
    (new_position, new_score)
}

fn roll(
    player1_position: u8,
    player1_score: u8,
    player2_position: u8,
    player2_score: u8,
    next_turn: Player,
) -> Wins {
    if player1_score > 20 {
        return Wins(1, 0);
    }
    if player2_score > 20 {
        return Wins(0, 1);
    }
    let mut wins = Wins::default();
    // rolling 3 dice can give the following 27 results:
    // 1 x 3
    // 3 x 4
    // 6 x 5
    // 7 x 6
    // 6 x 7
    // 3 x 8
    // 1 x 9
    match next_turn {
        Player::Player1 => {
            for (frequency, dice_throw) in [(1, 3), (3, 4), (6, 5), (7, 6), (6, 7), (3, 8), (1, 9)]
            {
                let (new_position, new_score) =
                    move_score(player1_position, player1_score, dice_throw);
                wins += roll(
                    new_position,
                    new_score,
                    player2_position,
                    player2_score,
                    Player::Player2,
                ) * frequency;
            }
        }
        Player::Player2 => {
            for (frequency, dice_throw) in [(1, 3), (3, 4), (6, 5), (7, 6), (6, 7), (3, 8), (1, 9)]
            {
                let (new_position, new_score) =
                    move_score(player2_position, player2_score, dice_throw);
                wins += roll(
                    player1_position,
                    player1_score,
                    new_position,
                    new_score,
                    Player::Player1,
                ) * frequency;
            }
        }
    }
    wins
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_diracs() {
        let result = roll(4, 0, 8, 0, Player::Player1);
        assert_eq!(Wins(444356092776315, 341960390180808), result);
    }
}
