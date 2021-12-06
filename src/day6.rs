
use std::str::FromStr;

pub fn solve() {
    let input = std::fs::read_to_string(format!(
        "resources/{}.txt",
        module_path!().split_once("::").unwrap().1
    ))
    .unwrap();
    let mut school: School = input.parse().unwrap();
    school.nth(79);
    println!("Day 6 part 1 (slow): {}", school.len());
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lanternfish(u8);

impl Lanternfish {
    fn new() -> Lanternfish {
        Lanternfish(8)
    }

    fn with_timer(value: u8) -> Lanternfish {
        Lanternfish(value)
    }
}

impl Iterator for Lanternfish {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            self.0 = 7;
        }
        self.0 -= 1;
        Some(self.0)
    }
}

impl PartialEq<u8> for Lanternfish {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

#[derive(Debug, PartialEq)]
pub struct School(Vec<Lanternfish>);

impl FromStr for School {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(School(s.trim().split(',').map(|s| Lanternfish::with_timer(s.parse().unwrap())).collect()))
    }
}

impl Iterator for School {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let mut new_fish = Vec::new();
        for fish in self.0.iter_mut() {
            if *fish == 0 {
                new_fish.push(Lanternfish::new());
            }
            fish.next();
        }
        self.0.append(&mut new_fish);
        Some(())
    }
}

impl School {
    fn len(&self) -> usize {
        self.0.len()
    }
}



#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn it_parses_school() {
        let input = "3,4,3,1,2";
        let school: Result<School, _> = input.parse();
        assert!(school.is_ok());
        let school = school.unwrap();
        assert_eq!(5, school.0.len());
        assert_eq!(Lanternfish::with_timer(3), school.0[0]);
    }

    #[test]
    fn it_passes_days() {
        let mut school: School = "3,4,3,1,2".parse().unwrap();
        school.next();
        assert_eq!(School::from_str("2,3,2,0,1").unwrap(), school);
        school.next();
        assert_eq!(School::from_str("1,2,1,6,0,8").unwrap(), school);
        school.nth(15);
        assert_eq!(School::from_str("6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8").unwrap(), school);
    }

    #[test]
    fn it_solves_example1() {
        let mut school: School = "3,4,3,1,2".parse().unwrap();
        school.nth(79);
        assert_eq!(5934, school.len());
    }

    #[ignore = "expensive"]
    #[test]
    fn it_solves_example2() {
        let mut school: School = "3,4,3,1,2".parse().unwrap();
        school.nth(255);
        assert_eq!(26984457539, school.len());
    }
}