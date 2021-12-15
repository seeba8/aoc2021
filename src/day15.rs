pub fn solve() {
    let input = std::fs::read_to_string(format!(
        "resources/{}.txt",
        module_path!().split_once("::").unwrap().1
    ))
    .unwrap();
    let cave: Cave = input.parse().unwrap();
    let end = cave.risk.len() - 1;
    println!("Day 15 part 1: {}", cave.find_path(0, end));
    let cave = Cave::from_tile(&cave, 5);
    let end = cave.risk.len() - 1;
    println!("Day 15 part 2: {}", cave.find_path(0, end));
}

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

pub struct Cave {
    risk: Vec<usize>,
    neighbours: HashMap<usize, Vec<usize>>,
}
impl Cave {
    fn find_path(&self, start: usize, end: usize) -> usize {
        let mut distances: HashMap<usize, usize> = HashMap::new();
        let mut visited: HashSet<usize> = HashSet::new();
        distances.insert(start, 0);
        while !distances.is_empty() {
            let (&u, &dist) = distances.iter().min_by_key(|(_, v)| **v).unwrap();
            visited.insert(u);
            distances.remove(&u);
            if u == end {
                return dist;
            }
            if let Some(neighbours) = self.neighbours.get(&u) {
                for neighbour in neighbours {
                    if !visited.contains(neighbour) {
                        let alternative = dist + self.risk[*neighbour];
                        if alternative < *distances.get(neighbour).unwrap_or(&(usize::max as usize))
                        {
                            distances.insert(*neighbour, alternative);
                        }
                    }
                }
            }
        }
        panic!("Didn't arrive at end");
    }

    fn set_neighbours(&mut self) {
        let width = (self.risk.len() as f64).sqrt() as usize;
        for i in 0..self.risk.len() {
            let x = i % width;
            let y = i / width;
            let mut n = Vec::new();
            if x > 0 {
                n.push(i - 1);
            }
            if x + 1 < width {
                n.push(i + 1);
            }
            if y > 0 {
                n.push(i - width);
            }
            if y + 1 < width {
                n.push(i + width);
            }
            self.neighbours.insert(i, n);
        }
    }

    fn from_tile(tile: &Cave, factor: usize) -> Cave {
        let width = (tile.risk.len() as f64).sqrt() as usize;
        let mut risk: Vec<usize> = Vec::with_capacity(tile.risk.len() * factor * factor);
        for f_y in 0..factor {
            for y in 0..width {
                for f_x in 0..factor {
                    for x in 0..width {
                        let new_risk = tile.risk[y * width + x] + f_y + f_x;
                        let new_risk = ((new_risk - 1) % 9) + 1;
                        risk.push(new_risk);
                    }
                }
            }
        }
        let neighbours = HashMap::with_capacity(risk.len());
        let mut cave = Cave { risk, neighbours };
        cave.set_neighbours();
        cave
    }
}

impl FromStr for Cave {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let risk: Vec<usize> = s
            .trim()
            .chars()
            .filter_map(|c| if c.is_numeric() { c.to_digit(10) } else { None })
            .map(|v| v as usize)
            .collect();
        let neighbours = HashMap::with_capacity(risk.len());
        let mut cave = Cave { risk, neighbours };
        cave.set_neighbours();
        Ok(cave)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn it_parses_input() {
        let input = "1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581";
        let cave: Result<Cave, _> = input.parse();
        assert!(cave.is_ok());
        let cave = cave.unwrap();
        assert_eq!(100, cave.risk.len());
        assert_eq!(2, cave.neighbours[&0].len());
        assert_eq!(3, cave.neighbours[&1].len());
        assert_eq!(4, cave.neighbours[&11].len());
    }

    #[test]
    fn it_finds_path() {
        let input = "1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581";
        let cave: Cave = input.parse().unwrap();
        assert_eq!(40, cave.find_path(0, 99));
    }

    #[test]
    fn it_tiles() {
        let cave = Cave {
            risk: vec![8],
            neighbours: HashMap::new(),
        };
        let cave2 = Cave::from_tile(&cave, 5);
        let expected = vec![
            8, 9, 1, 2, 3, 9, 1, 2, 3, 4, 1, 2, 3, 4, 5, 2, 3, 4, 5, 6, 3, 4, 5, 6, 7,
        ];
        assert_eq!(expected, cave2.risk);
    }

    #[test]
    fn it_finds_long_path() {
        let input = "1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581";
        let cave: Cave = input.parse().unwrap();
        let cave = Cave::from_tile(&cave, 5);
        assert_eq!(315, cave.find_path(0, cave.risk.len() - 1));
    }
}
