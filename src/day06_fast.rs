use std::str::FromStr;

pub fn solve() {
    let input = std::fs::read_to_string("resources/day06.txt").unwrap();
    let mut school: School = input.parse().unwrap();
    school.nth(79);
    println!("Day 6 part 1: {}", school.len());
    let mut school: School = input.parse().unwrap();
    school.nth(255);
    println!("Day 6 part 2: {}", school.len());
}

pub struct School {
    fishes: [usize; 9],
}

impl Iterator for School {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        let mut next_fishes = [0usize; 9];
        next_fishes[6] = self.fishes[0];
        next_fishes[8] = self.fishes[0];
        for i in 1..=8 {
            next_fishes[i - 1] += self.fishes[i];
        }
        self.fishes = next_fishes;
        Some(())
    }
}

impl FromStr for School {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fishes: Vec<usize> = s.trim().split(',').map(|s| s.parse().unwrap()).collect();
        let mut school = [0usize; 9];
        for fish in fishes {
            school[fish] += 1;
        }
        Ok(School { fishes: school })
    }
}

impl School {
    fn len(&self) -> usize {
        self.fishes.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_parses_input() {
        let school: Result<School, _> = "3,4,3,1,2".parse();
        assert!(school.is_ok());
        assert_eq!([0, 1, 1, 2, 1, 0, 0, 0, 0], school.unwrap().fishes);
    }

    #[test]
    fn it_counts_fishes_after_time() {
        let mut school: School = "3,4,3,1,2".parse().unwrap();
        school.nth(17);
        assert_eq!(26, school.len());
    }

    #[test]
    fn it_solves_example1() {
        let mut school: School = "3,4,3,1,2".parse().unwrap();
        school.nth(79);
        assert_eq!(5934, school.len());
    }

    #[test]
    fn it_solves_example2() {
        let mut school: School = "3,4,3,1,2".parse().unwrap();
        school.nth(255);
        assert_eq!(26984457539, school.len());
    }
}
