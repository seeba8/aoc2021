use std::str::FromStr;

pub fn solve() {
    let input = std::fs::read_to_string(format!(
        "resources/{}.txt",
        module_path!().split_once("::").unwrap().1
    ))
    .unwrap();
    let crabs = Crabs::from_str(&input).unwrap();
    println!("Day 7 part 1: {}", crabs.get_cheapest_position());
    println!(
        "Day 7 part 2: {}",
        crabs.get_cheapest_position_increasing_cost()
    );
}
pub struct Crabs(Vec<usize>);

impl Crabs {
    fn get_cheapest_position(&self) -> usize {
        let mut best = usize::MAX;
        for i in *self.0.iter().min().unwrap()..*self.0.iter().max().unwrap() {
            let sum: usize = self
                .0
                .iter()
                .map(|&v| (v as isize - i as isize).abs() as usize)
                .sum();
            if sum < best {
                best = sum;
            }
        }
        best
    }

    fn get_cheapest_position_increasing_cost(&self) -> usize {
        let mut best = usize::MAX;
        for i in *self.0.iter().min().unwrap()..*self.0.iter().max().unwrap() {
            let sum: usize = self
                .0
                .iter()
                .map(|&v| Crabs::get_cost((v as isize - i as isize).abs() as usize))
                .sum();
            if sum < best {
                best = sum;
            }
        }
        best
    }

    fn get_cost(distance: usize) -> usize {
        //(1..=distance).sum()
        (distance * (distance + 1)) / 2 // gauss
                                        
    }
}

impl FromStr for Crabs {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Crabs(
            s.trim()
                .split(',')
                .map(|crab| crab.parse().map_err(|_| "Cannot parse crab".to_owned()))
                .collect::<Result<Vec<usize>, String>>()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_parses() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let crabs = Crabs::from_str(input);
        assert!(crabs.is_ok());
        let crabs = crabs.unwrap();
        assert_eq!(10, crabs.0.len());
    }

    #[test]
    fn it_calculates_example1() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let crabs = Crabs::from_str(input).unwrap();
        assert_eq!(37, crabs.get_cheapest_position());
    }

    #[test]
    fn it_calculates_example2() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let crabs = Crabs::from_str(input).unwrap();
        assert_eq!(168, crabs.get_cheapest_position_increasing_cost());
    }
}
