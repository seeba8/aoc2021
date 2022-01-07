use std::{collections::HashSet, str::FromStr};

use bitvec::prelude::*;

pub fn solve() {
    let input = std::fs::read_to_string(format!(
        "resources/{}.txt",
        module_path!().split_once("::").unwrap().1
    ))
    .unwrap();
    let instructions: Vec<Instruction> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut reactor = Reactor::new();
    for instruction in instructions.clone() {
        reactor.set(instruction);
    }
    println!("Day 22 part 1: {}", reactor.count_enabled_cubes());
    let mut reactor = FastReactor::new(instructions);
    reactor.apply_instructions();
    println!("Day 22 part 2: {}", reactor.count_enabled_cubes());

}
struct Reactor {
    grid: BitVec,
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
}

#[derive(PartialEq, Debug, Clone)]
struct Instruction {
    status: bool,
    x1: isize,
    x2: isize,
    y1: isize,
    y2: isize,
    z1: isize,
    z2: isize,
}


impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (status, range) = s
            .trim()
            .split_once(' ')
            .ok_or("Cannot split between status and range")?;
        let status = status == "on";
        let ranges: Vec<(isize, isize)> = range
            .trim()
            .replace("x=", "")
            .replace("y=", "")
            .replace("z=", "")
            .split(',')
            .map(|range| range.trim().split_once("..").unwrap())
            .map(|(from, to)| (from.parse().unwrap(), to.parse().unwrap()))
            .collect();
        Ok(Instruction {
            status,
            x1: ranges[0].0,
            x2: ranges[0].1,
            y1: ranges[1].0,
            y2: ranges[1].1,
            z1: ranges[2].0,
            z2: ranges[2].1,
        })
    }
}

impl Reactor {
    fn new() -> Reactor {
        Reactor {
            grid: bitvec![0; 101*101*101],
            x: (-50, 50),
            y: (-50, 50),
            z: (-50, 50),
        }
    }
    fn set(
        &mut self,
        Instruction {
            status,
            x1,
            x2,
            y1,
            y2,
            z1,
            z2,
        }: Instruction,
    ) {
        for x in x1.max(self.x.0)..=x2.min(self.x.1) {
            for y in y1.max(self.y.0)..=y2.min(self.y.1) {
                for z in z1.max(self.z.0)..=z2.min(self.z.1) {
                    self.grid.set(
                        (z - self.z.0) as usize * 101 * 101
                            + (y - self.y.0) as usize * 101
                            + (x - self.x.0) as usize,
                        status,
                    );
                }
            }
        }
    }

    fn count_enabled_cubes(&self) -> usize {
        self.grid.count_ones()
    }
}

/// in the grid, a cell being on means the range of all numbers excluding the next number.
/// For example, turning on 1..=5 and 5..=10, then turning off 1..=5 means that only 6..=10 are on.
/// Turning on 5..=10 again means that now, 5..=10 are on.
/// Thus, we need the following values in the grid:
/// 1, 5, 6, 11
/// We need the following number for each end, since we interpret it exclusive.
/// We need the number itself for the start, since we interpret it inclusive.
struct FastReactor {
    grid: BitVec,
    x_values: Vec<isize>,
    y_values: Vec<isize>,
    z_values: Vec<isize>,
    instructions: Vec<Instruction>,
}

impl FastReactor {
    fn new(instructions: Vec<Instruction>) -> FastReactor {
        let mut x_values: HashSet<isize> = HashSet::new();
        let mut y_values: HashSet<isize> = HashSet::new();
        let mut z_values: HashSet<isize> = HashSet::new();

        for instruction in &instructions {
            x_values.insert(instruction.x1);
            x_values.insert(instruction.x2 + 1);
            y_values.insert(instruction.y1);
            y_values.insert(instruction.y2 + 1);
            z_values.insert(instruction.z1);
            z_values.insert(instruction.z2 + 1);
        }

        // If we sort them, we can use binary search later.
        let mut x_values: Vec<isize> = Vec::from_iter(x_values);
        x_values.sort_unstable();
        let mut y_values: Vec<isize> = Vec::from_iter(y_values);
        y_values.sort_unstable();
        let mut z_values: Vec<isize> = Vec::from_iter(z_values);
        z_values.sort_unstable();
        FastReactor {
            grid: bitvec![0; x_values.len() * y_values.len() * z_values.len()],
            x_values,
            y_values,
            z_values,
            instructions,
        }
    }

    fn apply_instructions(&mut self) {
        for i in 0..self.instructions.len() {
            self.apply_instruction(i);
        }
    }

    fn apply_instruction(&mut self, index: usize) {
        let instruction = &self.instructions[index];
        let x1 = self.x_values.binary_search(&instruction.x1).unwrap();
        let x2 = self.x_values.binary_search(&(instruction.x2 + 1)).unwrap();
        let y1 = self.y_values.binary_search(&instruction.y1).unwrap();
        let y2 = self.y_values.binary_search(&(instruction.y2 + 1)).unwrap();
        let z1 = self.z_values.binary_search(&instruction.z1).unwrap();
        let z2 = self.z_values.binary_search(&(instruction.z2 + 1)).unwrap();
        for x in x1..x2 {
            for y in y1..y2 {
                for z in z1..z2 {
                    self.grid.set(
                        z * self.y_values.len() * self.x_values.len() + y * self.x_values.len() + x,
                        instruction.status,
                    );
                }
            }
        }
    }
    
    #[allow(clippy::many_single_char_names)]
    fn get_coordinates(&self, index: usize) -> (usize, usize, usize) {
        let a = self.x_values.len() * self.y_values.len();
        let z = index / a;
        let b = index - a * z;
        let y = b / self.x_values.len();
        let x = b % self.x_values.len();
        (x, y, z)
    }

    fn count_enabled_cubes(&self) -> usize {
        let mut sum = 0;
        for i in self.grid.iter_ones() {
            let (x, y, z) = self.get_coordinates(i);
            sum += ((self.x_values[x + 1] - self.x_values[x])
                * (self.y_values[y + 1] - self.y_values[y])
                * (self.z_values[z + 1] - self.z_values[z])) as usize;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_parses_instruction() {
        let test: isize = "-7".parse().unwrap();
        assert_eq!(-7, test);
        let instruction: Result<Instruction, _> = "on x=-20..26,y=-36..17,z=-47..7".parse();
        assert!(instruction.is_ok());
        let instruction = instruction.unwrap();
        assert_eq!(
            Instruction {
                status: true,
                x1: -20,
                x2: 26,
                y1: -36,
                y2: 17,
                z1: -47,
                z2: 7
            },
            instruction
        );
    }

    #[test]
    fn it_turns_on_cubes() {
        let instruction: Instruction = "on x=10..12,y=10..12,z=10..12".parse().unwrap();
        let mut reactor = Reactor::new();
        reactor.set(instruction);
        assert_eq!(27, reactor.count_enabled_cubes())
    }

    #[test]
    fn it_manages_overlapping_cubes() {
        let instruction: Instruction = "on x=10..12,y=10..12,z=10..12".parse().unwrap();
        let mut reactor = Reactor::new();
        reactor.set(instruction);
        assert_eq!(27, reactor.count_enabled_cubes());
        let instruction: Instruction = "on x=11..13,y=11..13,z=11..13".parse().unwrap();
        reactor.set(instruction);
        assert_eq!(27 + 19, reactor.count_enabled_cubes());
        let instruction: Instruction = "off x=9..11,y=9..11,z=9..11".parse().unwrap();
        reactor.set(instruction);
        assert_eq!(27 + 19 - 8, reactor.count_enabled_cubes());
        let instruction: Instruction = "on x=10..10,y=10..10,z=10..10".parse().unwrap();
        reactor.set(instruction);
        assert_eq!(27 + 19 - 8 + 1, reactor.count_enabled_cubes());
    }

    #[test]
    fn it_handles_larger_example() {
        let instructions: Vec<Instruction> = "on x=-20..26,y=-36..17,z=-47..7
        on x=-20..33,y=-21..23,z=-26..28
        on x=-22..28,y=-29..23,z=-38..16
        on x=-46..7,y=-6..46,z=-50..-1
        on x=-49..1,y=-3..46,z=-24..28
        on x=2..47,y=-22..22,z=-23..27
        on x=-27..23,y=-28..26,z=-21..29
        on x=-39..5,y=-6..47,z=-3..44
        on x=-30..21,y=-8..43,z=-13..34
        on x=-22..26,y=-27..20,z=-29..19
        off x=-48..-32,y=26..41,z=-47..-37
        on x=-12..35,y=6..50,z=-50..-2
        off x=-48..-32,y=-32..-16,z=-15..-5
        on x=-18..26,y=-33..15,z=-7..46
        off x=-40..-22,y=-38..-28,z=23..41
        on x=-16..35,y=-41..10,z=-47..6
        off x=-32..-23,y=11..30,z=-14..3
        on x=-49..-5,y=-3..45,z=-29..18
        off x=18..30,y=-20..-8,z=-3..13
        on x=-41..9,y=-7..43,z=-33..15
        on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
        on x=967..23432,y=45373..81175,z=27513..53682"
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();
        let mut reactor: Reactor = Reactor::new();
        for instruction in instructions {
            reactor.set(instruction);
        }
        assert_eq!(590784, reactor.count_enabled_cubes());
    }

    #[test]
    fn it_turns_on_cubes_fast() {
        let instruction: Instruction = "on x=10..12,y=10..12,z=10..12".parse().unwrap();
        let mut reactor = FastReactor::new(vec![instruction]);
        reactor.apply_instructions();
        assert_eq!(27, reactor.count_enabled_cubes())
    }

    #[test]
    fn it_manages_overlapping_cubes_fast() {
        let instructions: Vec<Instruction> = vec![
            "on x=10..12,y=10..12,z=10..12".parse().unwrap(),
            "on x=11..13,y=11..13,z=11..13".parse().unwrap(),
            "off x=9..11,y=9..11,z=9..11".parse().unwrap(),
            "on x=10..10,y=10..10,z=10..10".parse().unwrap(),
        ];
        let mut reactor = FastReactor::new(instructions);
        reactor.apply_instruction(0);
        assert_eq!(27, reactor.count_enabled_cubes());
        reactor.apply_instruction(1);
        assert_eq!(27 + 19, reactor.count_enabled_cubes());
        reactor.apply_instruction(2);
        assert_eq!(27 + 19 - 8, reactor.count_enabled_cubes());
        reactor.apply_instruction(3);
        assert_eq!(27 + 19 - 8 + 1, reactor.count_enabled_cubes());
    }

    #[test]
    fn it_handles_larger_example_fast() {
        let instructions: Vec<Instruction> = "on x=-20..26,y=-36..17,z=-47..7
        on x=-20..33,y=-21..23,z=-26..28
        on x=-22..28,y=-29..23,z=-38..16
        on x=-46..7,y=-6..46,z=-50..-1
        on x=-49..1,y=-3..46,z=-24..28
        on x=2..47,y=-22..22,z=-23..27
        on x=-27..23,y=-28..26,z=-21..29
        on x=-39..5,y=-6..47,z=-3..44
        on x=-30..21,y=-8..43,z=-13..34
        on x=-22..26,y=-27..20,z=-29..19
        off x=-48..-32,y=26..41,z=-47..-37
        on x=-12..35,y=6..50,z=-50..-2
        off x=-48..-32,y=-32..-16,z=-15..-5
        on x=-18..26,y=-33..15,z=-7..46
        off x=-40..-22,y=-38..-28,z=23..41
        on x=-16..35,y=-41..10,z=-47..6
        off x=-32..-23,y=11..30,z=-14..3
        on x=-49..-5,y=-3..45,z=-29..18
        off x=18..30,y=-20..-8,z=-3..13
        on x=-41..9,y=-7..43,z=-33..15"
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();
        let mut reactor = FastReactor::new(instructions);
        reactor.apply_instructions();
        assert_eq!(590784, reactor.count_enabled_cubes());
    }

    #[test]
    fn it_handles_example_2_fast() {
        let input = std::fs::read_to_string("resources/day22_example.txt").unwrap();
        let instructions: Vec<Instruction> = input
            .trim()
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();
        let mut reactor = FastReactor::new(instructions);
        reactor.apply_instructions();
        assert_eq!(2758514936282235, reactor.count_enabled_cubes());
    }
}
