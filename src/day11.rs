use std::str::FromStr;

pub fn solve() {
    let input = std::fs::read_to_string(format!(
        "resources/{}.txt",
        module_path!().split_once("::").unwrap().1
    ))
    .unwrap();
    let octopuses: Octopuses = input.parse().unwrap();
    println!("Day 11 part 1: {}", octopuses.take(100).sum::<usize>());
    let mut octopuses: Octopuses = input.parse().unwrap();
    println!("Day 11 part 2: {}", octopuses.get_simultaneous_flash());
}

#[derive(Debug, PartialEq)]
pub struct Octopuses {
    width: usize,
    height: usize,
    octopuses: Vec<u8>,
}

impl Octopuses {
    fn get_simultaneous_flash(&mut self) -> usize {
        self
            .enumerate()
            .take_while(|(_, flashes)| *flashes < 100)
            .map(|(i, _)| i)
            .max()
            .unwrap_or(0)
            + 2
    }

    fn tick(&mut self) -> usize {
        let mut total_flashes = 0;
        self.increase_all();
        while self.octopuses.iter().any(|&octopus| octopus > 9) {
            let flashes: Vec<usize> = self
                .octopuses
                .iter_mut()
                .enumerate()
                .filter(|(_, o)| **o > 9)
                .map(|(i, v)| {
                    *v = 0;
                    i
                })
                .collect();
            total_flashes += flashes.len();
            for flash_index in flashes {
                for neighbour in self.get_neighbour_indices(flash_index) {
                    if self.octopuses[neighbour] != 0 {
                        self.octopuses[neighbour] += 1;
                    }
                }
            }
        }
        total_flashes
    }

    fn get_neighbour_indices(&self, i: usize) -> Vec<usize> {
        let x = i % self.width;
        let y = i / self.width;
        let mut neighbours = Vec::new();
        for x_n in x.checked_sub(1).unwrap_or(x)..=(x+1).min(self.width-1) {
            for y_n in y.checked_sub(1).unwrap_or(y)..=(y+1).min(self.height-1) {
                if x_n == x && y_n == y {
                    continue;
                }
                neighbours.push(y_n * self.width + x_n);
            }
        }
        neighbours
    }

    fn increase_all(&mut self) {
        for octopus in &mut self.octopuses {
            *octopus += 1;
        }
    }
}

impl Iterator for Octopuses {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.tick())
    }
}

impl FromStr for Octopuses {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        Ok(Octopuses {
            width: s
                .lines()
                .next()
                .ok_or_else(|| "No line breaks".to_owned())?
                .len(),
            height: s.lines().count(),
            octopuses: s
                .chars()
                .filter_map(|c| c.to_digit(10).map(|v| v as u8))
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Octopuses;

    #[test]
    fn it_parses_input() {
        let input = r"11111
19991
19191
19991
11111";
        let octopuses: Result<Octopuses, _> = input.parse();
        assert!(octopuses.is_ok());
        let octopuses = octopuses.unwrap();
        assert_eq!(5, octopuses.width);
        assert_eq!(5, octopuses.height);
    }

    #[test]
    fn it_processes_tick() {
        let input = r"11111
19991
19191
19991
11111";
        let mut octopuses: Octopuses = input.parse().unwrap();
        let flashes = octopuses.next();
        assert!(flashes.is_some());
        let flashes = flashes.unwrap();
        assert_eq!(9, flashes);
        let expected: Octopuses = "34543
40004
50005
40004
34543"
            .parse()
            .unwrap();
        assert_eq!(expected, octopuses);
        assert_eq!(Some(0), octopuses.next());
    }

    #[test]
    fn it_runs_example1() {
        let example = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let octopuses: Octopuses = example.parse().unwrap();
        assert_eq!(1656_usize, octopuses.take(100).sum());
    }

    #[test]
    fn it_runs_example2() {
        let example = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let mut octopuses: Octopuses = example.parse().unwrap();
        assert_eq!(195, octopuses.get_simultaneous_flash());
    }
}
