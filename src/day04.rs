use std::{num::ParseIntError, str::FromStr};

pub fn solve() {
    let input = std::fs::read_to_string(format!(
        "resources/{}.txt",
        module_path!().split_once("::").unwrap().1
    ))
    .unwrap();
    let mut game: BingoGame = input.parse().unwrap();
    println!("Day 4 part 1: {}", game.get_winning_score());
    let mut game: BingoGame = input.parse().unwrap();
    println!("Day 4 part 2: {}", game.get_losing_score());
}

const BINGO_SIZE: usize = 5;

#[derive(Debug)]
pub struct BingoError(String);

#[derive(Debug, PartialEq)]
pub struct BingoResult {
    after_number: usize,
    score: usize,
}

pub struct BingoBoard {
    grid: [Option<u8>; BINGO_SIZE * BINGO_SIZE],
}

impl BingoBoard {
    pub fn play_game(&mut self, sequence: &[u8]) -> BingoResult {
        for (i, number) in sequence.iter().enumerate() {
            self.play_card(*number);
            if self.is_won() {
                let sum: usize = self.grid.iter().map(|v| v.unwrap_or(0) as usize).sum();
                return BingoResult {
                    after_number: i,
                    score: sum * (*number as usize),
                };
            }
        }
        BingoResult {
            after_number: BINGO_SIZE * BINGO_SIZE,
            score: 0,
        }
    }

    fn play_card(&mut self, number: u8) {
        for v in self.grid.iter_mut() {
            if Some(number) == *v {
                *v = None
            }
        }
    }

    fn is_won(&self) -> bool {
        for i in 0..BINGO_SIZE {
            // column
            if self
                .grid
                .iter()
                .skip(i)
                .step_by(BINGO_SIZE)
                .all(|p| p.is_none())
            {
                return true;
            }
            // row
            if self.grid[i * BINGO_SIZE..(i + 1) * BINGO_SIZE]
                .iter()
                .all(|p| p.is_none())
            {
                return true;
            }
        }
        false
    }
}

impl FromStr for BingoBoard {
    type Err = BingoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .trim()
            .split_ascii_whitespace()
            .map(|v| {
                v.trim().parse().map(Some).map_err(|e: ParseIntError| {
                    BingoError(format!("Cannot parse number {}", e.to_string()))
                })
            })
            .collect::<Result<Vec<Option<u8>>, BingoError>>()?;
        Ok(BingoBoard {
            grid: grid.try_into().unwrap(),
        })
    }
}

pub struct BingoGame {
    boards: Vec<BingoBoard>,
    sequence: Vec<u8>,
}

impl BingoGame {
    pub fn get_winning_score(&mut self) -> usize {
        let results: Vec<BingoResult> = self
            .boards
            .iter_mut()
            .map(|board| board.play_game(&self.sequence))
            .collect();
        results
            .iter()
            .min_by_key(|res| res.after_number)
            .map(|v| v.score)
            .unwrap_or(0)
    }

    pub fn get_losing_score(&mut self) -> usize {
        let results: Vec<BingoResult> = self
            .boards
            .iter_mut()
            .map(|board| board.play_game(&self.sequence))
            .collect();
        results
            .iter()
            .max_by_key(|res| res.after_number)
            .map(|v| v.score)
            .unwrap_or(0)
    }
}

impl FromStr for BingoGame {
    type Err = BingoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let sequence: Vec<u8> = s
            .lines()
            .next()
            .ok_or_else(|| BingoError("Empty input".to_owned()))?
            .split(',')
            .map(|v| {
                v.parse().map_err(|e: ParseIntError| {
                    BingoError(format!("Cannot parse number {}", e.to_string()))
                })
            })
            .collect::<Result<Vec<u8>, BingoError>>()?;
        let boards: Vec<BingoBoard> = s
            .split("\n\n")
            .skip(1)
            .map(|s| {
                s.parse()
                    .map_err(|_| BingoError("Cannot parse bingo board".to_owned()))
            })
            .collect::<Result<Vec<BingoBoard>, BingoError>>()?;
        Ok(BingoGame { boards, sequence })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_marks_played_card() {
        let mut board = BingoBoard {
            grid: [Some(0); 25],
        };
        board.grid[2] = Some(2);
        board.play_card(2);
        assert_eq!(board.grid[2], None);
        assert_eq!(board.grid[1].is_some(), true);
    }

    #[test]
    fn it_parses_board() {
        let input = r"22 13 17 11  0
        8  2 23  4 24
       21  9 14 16  7
        6 10  3 18  5
        1 12 20 15 19";
        let grid: BingoBoard = input.parse().unwrap();
        assert_eq!(grid.grid[0], Some(22));
        assert_eq!(grid.grid[24], Some(19));
        assert_eq!(grid.grid[5], Some(8));
    }

    #[test]
    fn it_detects_winning() {
        let input = r"14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
        2  0 12  3  7";
        let sequence: Vec<u8> =
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1"
                .split(',')
                .map(|v| v.parse().unwrap())
                .collect();
        let mut grid: BingoBoard = input.parse().unwrap();
        assert_eq!(
            BingoResult {
                after_number: 11,
                score: 4512
            },
            grid.play_game(&sequence)
        );
    }

    #[test]
    fn it_parses_input() {
        let input = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        let game: Result<BingoGame, BingoError> = input.parse();
        assert_eq!(true, game.is_ok());
        let game = game.unwrap();
        assert_eq!(vec![7, 4, 9, 5], game.sequence[0..4]);
        assert_eq!(3, game.boards.len());
    }

    #[test]
    fn it_finds_winner() {
        let input = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        let mut game: BingoGame = input.parse().unwrap();
        assert_eq!(4512, game.get_winning_score());
    }

    #[test]
    fn it_finds_loser() {
        let input = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        let mut game: BingoGame = input.parse().unwrap();
        assert_eq!(1924, game.get_losing_score());
    }
}
