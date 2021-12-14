use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

pub fn solve() {
    let input = std::fs::read_to_string(format!(
        "resources/{}.txt",
        module_path!().split_once("::").unwrap().1
    ))
    .unwrap();
    let mut polymers: Polymers = input.parse().unwrap();
    polymers.nth(9);
    println!("Day 14 part 1: {}", polymers.get_max_minus_min());
    let mut polymers: Polymers = input.parse().unwrap();
    polymers.nth(39);
    println!("Day 14 part 2: {}", polymers.get_max_minus_min());
}

pub struct Polymers {
    elements: Vec<char>,
    pair_insertions: HashMap<(char, char), char>
}

impl Polymers {
    fn get_max_minus_min(&self) -> usize {
        let counts = self.elements.iter().counts();
        counts.values().max().unwrap() - counts.values().min().unwrap()
    }
}

impl Iterator for Polymers {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let mut new_elements = Vec::new();
        for i in 0..self.elements.len()-1 {
            let pair = (self.elements[i], self.elements[i+1]);
            new_elements.push(self.elements[i]);
            if let Some(inserted) = self.pair_insertions.get(&pair) {
                new_elements.push(*inserted);
            }
        }
        new_elements.push(*self.elements.last().unwrap());
        self.elements = new_elements;
        Some(())
    }
}

impl FromStr for Polymers {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (template, rules) = s.trim().split_once("\n\n").ok_or("cannot split input into 2")?;
        let elements: Vec<char> = template.trim().chars().collect();
        let mut pair_insertions = HashMap::new();
        for rule in rules.trim().lines() {
            let (input, c) = rule.trim().split_once(" -> ").ok_or("cannot parse pair_insertion")?;
            let mut chars = input.chars();
            let (c1, c2) = (chars.next().ok_or("cannot parse char 1")?, chars.next().ok_or("cannot parse char 2")?);
            let inserted = c.chars().next().ok_or("cannot parse inserted char")?;
            pair_insertions.insert((c1, c2), inserted);
        }
        Ok(Polymers {elements, pair_insertions})

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses() {
        let input = "NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C";

        let polymers: Result<Polymers, _> = input.parse();
        assert!(polymers.is_ok());
        let polymers = polymers.unwrap();
        assert_eq!("NNCB".chars().collect::<Vec<char>>(), polymers.elements);
        assert_eq!(16, polymers.pair_insertions.len());
    }

    #[test]
    fn it_steps() {
        let input = "NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C";

        let mut polymers: Polymers = input.parse().unwrap();
        polymers.next();
        assert_eq!("NCNBCHB".chars().collect::<Vec<char>>(), polymers.elements);
        polymers.next();
        assert_eq!("NBCCNBBBCBHCB".chars().collect::<Vec<char>>(), polymers.elements);
        polymers.next();
        assert_eq!("NBBBCNCCNBBNBNBBCHBHHBCHB".chars().collect::<Vec<char>>(), polymers.elements);
        polymers.next();
        assert_eq!("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB".chars().collect::<Vec<char>>(), polymers.elements);
    }

    #[test]
    fn it_solves_example1() {
        let input = "NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C";

        let mut polymers: Polymers = input.parse().unwrap();
        polymers.nth(9);
        assert_eq!(1588, polymers.get_max_minus_min());
    }

    #[ignore = "too slow"]
    #[test]
    fn it_solves_example2() {
        let input = "NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C";

        let mut polymers: Polymers = input.parse().unwrap();
        polymers.nth(39);
        assert_eq!(2188189693529, polymers.get_max_minus_min());
    }
}