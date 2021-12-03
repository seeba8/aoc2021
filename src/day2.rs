pub fn solve() {
    let input = std::fs::read_to_string("./resources/day2.txt").unwrap();
    let instructions = parse_input(&input);
    let position = instructions.as_slice().follow();
    println!("Day 2 part 1: {}", position.0 * position.1);
    let instructions: Vec<AimInstruction> = instructions.into_iter().map(|i| i.into()).collect();
    let position = instructions.as_slice().follow();
    println!("Day 2 part 2: {}", position.0 * position.1);
}

#[derive(PartialEq, Debug)]
enum Direction {
    Down,
    Forward,
    Up,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Direction {
        match value {
            "down" => Direction::Down,
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            _ => panic!(),
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.trim().lines().map(|line| line.into()).collect()
}

#[derive(PartialEq, Debug)]
struct Instruction(Direction, usize);

#[derive(PartialEq, Debug)]
struct AimInstruction(Direction, usize);

impl From<Instruction> for AimInstruction {
    fn from(i: Instruction) -> Self {
        AimInstruction(i.0, i.1)
    }
}

pub trait Follow {
    fn follow(&self) -> (isize, isize);
}

impl Follow for &[AimInstruction] {
    fn follow(&self) -> (isize, isize) {
        let mut aim = 0isize;
        let mut x = 0isize;
        let mut y = 0isize;
        for AimInstruction(direction, distance) in self.iter() {
            match direction {
                Direction::Down => aim += *distance as isize,
                Direction::Up => aim -= *distance as isize,
                Direction::Forward => {
                    x += *distance as isize;
                    y += aim * (*distance as isize);
                }
            };
        }
        (x, y)
    }
}

impl Follow for &[Instruction] {
    fn follow(&self) -> (isize, isize) {
        self.iter()
            .map(|Instruction(direction, distance)| match direction {
                Direction::Down => (0isize, *distance as isize),
                Direction::Forward => (*distance as isize, 0isize),
                Direction::Up => (0isize, -(*distance as isize)),
            })
            .fold((0, 0), |(xa, ya), (xb, yb)| (xa + xb, ya + yb))
    }
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        if let Some((direction, distance)) = value.trim().split_once(" ") {
            Instruction(direction.into(), distance.parse().unwrap())
        } else {
            panic!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direction_parsing() {
        assert_eq!(Direction::Down, "down".try_into().unwrap());
    }

    #[test]
    fn instruction_parsing() {
        let instruction: Instruction = "forward 5".try_into().unwrap();
        assert_eq!(Instruction(Direction::Forward, 5), instruction);
    }

    #[test]
    fn parse_list() {
        let input = "forward 5\ndown 7";
        assert_eq!(
            vec![
                Instruction(Direction::Forward, 5),
                Instruction(Direction::Down, 7)
            ],
            parse_input(input)
        );
    }

    #[test]
    fn test_example1() {
        let input = r"forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2";
        assert_eq!((15, 10), parse_input(input).as_slice().follow());
    }

    #[test]
    fn test_example2() {
        let input = r"forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2";
        let instructions: Vec<AimInstruction> =
            parse_input(input).into_iter().map(|i| i.into()).collect();
        assert_eq!((15, 60), instructions.as_slice().follow());
    }
}
