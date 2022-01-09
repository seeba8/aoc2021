use std::{str::FromStr, fmt::Display};

pub fn solve() {
    let input = std::fs::read_to_string(format!(
        "resources/{}.txt",
        module_path!().split_once("::").unwrap().1
    ))
    .unwrap();
    let mut region: Region = input.parse().unwrap();
    println!("Day 25 part 1: {}", region.count_steps_until_no_movement());
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cucumber {
    Right,
    Down,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Region {
    grid: Vec<Option<Cucumber>>,
    width: usize,
    height: usize,
}

impl Region {
    fn tick(&mut self) -> bool {
        let mut changed = false;
        for y in 0..self.height {
            let first_slot = self.grid[y * self.width].clone();
            let mut x = 0;
            while x < self.width {
                if let Some(Cucumber::Right) = self.grid[y * self.width + x] {
                    if x + 1 == self.width {
                        if first_slot.is_none() {
                            self.grid[y * self.width + x] = None;
                            self.grid[y * self.width] = Some(Cucumber::Right);
                            changed = true;
                            x += 1;
                        }
                    } else if self.grid[y * self.width + x + 1].is_none() {
                        self.grid[y * self.width + x] = None;
                        self.grid[y * self.width + x + 1] = Some(Cucumber::Right);
                        changed = true;
                        x += 1;
                    }
                }
                x += 1;
            }
        }

        for x in 0..self.width {
            let first_slot = self.grid[x].clone();
            let mut y = 0;
            while y < self.height {
                if let Some(Cucumber::Down) = self.grid[y * self.width + x] {
                    if y + 1 >= self.height {
                        if first_slot.is_none() {
                            self.grid[y * self.width + x] = None;
                            self.grid[x] = Some(Cucumber::Down);
                            changed = true;
                            y += 1;
                        }
                    } else if self.grid[(y + 1) * self.width + x].is_none() {
                        self.grid[y * self.width + x] = None;
                        self.grid[(y + 1) * self.width + x] = Some(Cucumber::Down);
                        changed = true;
                        y += 1;
                    }
                }
                y += 1;
            }
        }
        changed
    }

    fn count_steps_until_no_movement(&mut self) -> usize {
        let mut count = 1;
        while self.tick() {
            count += 1;
        }
        count
    }

    fn get_display(&self) -> String {
        let mut out = String::with_capacity((self.width + 1) * self.height);
        for (index, cucumber) in self.grid.iter().enumerate() {
            if index > 0 && index % self.width == 0 {
                out += "\n";
            }
            out += match cucumber {
                Some(Cucumber::Down) => "v",
                Some(Cucumber::Right) => ">",
                None => "."
            };
        }
        out
    }
}

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.get_display())
    }
}

impl FromStr for Region {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().ok_or("Cannot split by line")?.trim().len();
        let height = s.trim().lines().count();
        let mut grid = Vec::new();
        for c in s.chars() {
            match c {
                'v' => grid.push(Some(Cucumber::Down)),
                '>' => grid.push(Some(Cucumber::Right)),
                '.' => grid.push(None),
                _ => {}
            };
        }
        Ok(Region {
            grid,
            width,
            height,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_parses_input() {
        let input = "...\n.>v";
        let region: Result<Region, _> = input.parse();
        assert!(region.is_ok());
        let region = region.unwrap();
        assert_eq!(vec![None, None, None, None, Some(Cucumber::Right), Some(Cucumber::Down)], region.grid);
        assert_eq!(3, region.width);
        assert_eq!(2, region.height);
        assert_eq!(input, region.get_display());
    }

    #[test]
    fn it_runs_1d_simulation() {
        let input = "...>>>>>...";
        let mut region: Region = input.parse().unwrap();
        assert_eq!(true, region.tick());
        assert_eq!("...>>>>.>..", region.get_display());
        assert_eq!(true, region.tick());
        assert_eq!("...>>>.>.>.", region.get_display());
    }

    #[test]
    fn it_runs_2d_simulation() {
        let input = "..........
        .>v....v..
        .......>..
        ..........
        ";
        let mut region: Region = input.parse().unwrap();
        region.tick();
        assert_eq!(Region::from_str("..........
        .>........
        ..v....v>.
        ..........
        ").unwrap(), region);
    }
    
    #[test]
    fn it_wraps() {
        let input = "...>...
        .......
        ......>
        v.....>
        ......>
        .......
        ..vvv..";
        let mut region: Region = input.parse().unwrap();
        region.tick();
        region.tick();
        region.tick();
        region.tick();
        assert_eq!(Region::from_str(">......
        ..v....
        ..>.v..
        .>.v...
        ...>...
        .......
        v......").unwrap(), region);
    }

    #[test]
    fn it_calculates_until_standstill() {
        let mut region: Region = "v...>>.vv>
        .vv>>.vv..
        >>.>v>...v
        >>v>>.>.v.
        v>v.vv.v..
        >.>>..v...
        .vv..>.>v.
        v.v..>>v.v
        ....v..v.>".parse().unwrap();
        assert_eq!(58, region.count_steps_until_no_movement());
        let expected = Region::from_str("..>>v>vv..
        ..v.>>vv..
        ..>>v>>vv.
        ..>>>>>vv.
        v......>vv
        v>v....>>v
        vvv.....>>
        >vv......>
        .>v.vv.v..").unwrap();
        assert_eq!(expected, region);
    }
}