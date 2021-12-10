use std::str::FromStr;

use itertools::Itertools;

pub fn solve() {
    let input = std::fs::read_to_string(format!(
        "resources/{}.txt",
        module_path!().split_once("::").unwrap().1
    ))
    .unwrap();
    let cave: Cave = input.parse().unwrap();
    println!("Day 9 part 1: {}", cave.get_total_risk_level());
    println!("Day 9 part 2: {}", cave.get_largest_basins_product(3));
}

pub struct Cave {
    width: usize,
    height: usize,
    heightmap: Vec<u8>,
}

impl Cave {
    fn get(&self, x: isize, y: isize) -> Option<u8> {
        if x >= self.width as isize || y >= self.height as isize || y < 0 || x < 0 {
            None
        } else {
            Some(self.heightmap[y as usize * self.width + x as usize])
        }
    }

    fn get_risk_levels(&self) -> Vec<u8> {
        self.get_low_points().iter().map(|v| v + 1).collect()
    }

    fn get_low_point_indices(&self) -> Vec<usize> {
        let mut low_points = Vec::new();
        for x in 0..(self.width as isize) {
            for y in 0..(self.height as isize) {
                let height = self.get(x, y).unwrap();
                if self
                    .get_neighbours(x, y)
                    .iter()
                    .map(|v| v.unwrap_or(u8::MAX))
                    .all(|v| v > height)
                {
                    low_points.push(y as usize * self.width + x as usize);
                }
            }
        }
        low_points
    }

    fn get_low_points(&self) -> Vec<u8> {
        self.get_low_point_indices()
            .into_iter()
            .map(|v| self.heightmap[v])
            .collect()
    }

    fn get_total_risk_level(&self) -> usize {
        self.get_risk_levels().iter().map(|&v| v as usize).sum()
    }

    fn get_neighbours(&self, x: isize, y: isize) -> Vec<Option<u8>> {
        vec![
            self.get(x - 1, y),
            self.get(x, y - 1),
            self.get(x + 1, y),
            self.get(x, y + 1),
        ]
    }

    fn get_basins_recursive(&self) -> Vec<Option<usize>> {
        let mut basins = vec![None; self.heightmap.len()];
        let low_points = self.get_low_point_indices();
        for (basin_id, start) in low_points.iter().enumerate() {
            self._get_basins_recursive(&mut basins, *start, basin_id);
        }
        basins
    }

    fn _get_basins_recursive(
        &self,
        basins: &mut Vec<Option<usize>>,
        position: usize,
        basin_id: usize,
    ) {
        if self.heightmap[position] == 9 || basins[position].is_some() {
            return;
        }

        basins[position] = Some(basin_id);
        let x = position % self.width;
        let y = position / self.width;
        if x > 0 {
            self._get_basins_recursive(basins, position - 1, basin_id);
        }
        if x + 1 < self.width {
            self._get_basins_recursive(basins, position + 1, basin_id);
        }
        if y > 0 {
            self._get_basins_recursive(basins, position - self.width, basin_id);
        }
        if y + 1 < self.height {
            self._get_basins_recursive(basins, position + self.width, basin_id);
        }
    }

    fn get_largest_basins_product(&self, count: usize) -> usize {
        let basins = self.get_basins_recursive();
        let basin_sizes: Vec<usize> = basins
            .iter()
            .filter(|v| v.is_some())
            .counts()
            .iter()
            .map(|v| *v.1)
            .collect();
        basin_sizes.iter().sorted().rev().take(count).product()
    }
}

impl FromStr for Cave {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s
            .lines()
            .next()
            .ok_or_else(|| "no line break".to_owned())?
            .trim()
            .len();
        let height = s.trim().lines().count();
        let heightmap: Vec<u8> = s
            .chars()
            .filter_map(|c| {
                if c.is_numeric() {
                    c.to_digit(10).map(|c| c as u8)
                } else {
                    None
                }
            })
            .collect();
        Ok(Cave {
            width,
            height,
            heightmap,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_parses_input() {
        let input = r"2199943210
3987894921
9856789892
8767896789
9899965678";
        let cave: Result<Cave, _> = input.parse();
        assert!(cave.is_ok());
        let cave: Cave = cave.unwrap();
        assert_eq!(5, cave.height);
        assert_eq!(10, cave.width);
    }

    #[test]
    fn it_parses_coordinates() {
        let input = r"2199943210
3987894921
9856789892
8767896789
9899965678";
        let cave: Cave = input.parse().unwrap();
        assert_eq!(Some(2), cave.get(0, 0));
        assert_eq!(Some(3), cave.get(0, 1));
        assert_eq!(Some(1), cave.get(1, 0));
    }

    #[test]
    fn it_finds_risk_levels() {
        let input = r"2199943210
3987894921
9856789892
8767896789
9899965678";
        let cave: Cave = input.parse().unwrap();
        let mut risk_levels = cave.get_risk_levels();
        risk_levels.sort_unstable();
        assert_eq!(4, risk_levels.len());
        assert_eq!(vec![1, 2, 6, 6], risk_levels);
    }

    #[test]
    fn it_solves_example1() {
        let input = r"2199943210
3987894921
9856789892
8767896789
9899965678";
        let cave: Cave = input.parse().unwrap();
        assert_eq!(15, cave.get_total_risk_level());
    }

    #[test]
    fn it_gets_basins() {
        let input = r"2199943210
3987894921
9856789892
8767896789
9899965678";
        let cave: Cave = input.parse().unwrap();
        assert_eq!(1134usize, cave.get_largest_basins_product(3));
    }
}
