use std::str::FromStr;

use itertools::Itertools;

pub fn solve() {
    let input = std::fs::read_to_string(format!(
        "resources/{}.txt",
        module_path!().split_once("::").unwrap().1
    ))
    .unwrap();
    let mut entries: Vec<Entry> = input
        .trim()
        .split('\n')
        .map(|e| e.trim().parse().unwrap())
        .collect();
    println!(
        "Day 8 part 1: {}",
        entries
            .iter()
            .map(|entry| entry.get_simple_digits_in_output())
            .sum::<usize>()
    );
    println!(
        "Day 8 part 2: {}",
        entries
            .iter_mut()
            .map(|entry| entry.determine_digits())
            .sum::<usize>()
    );
}

#[derive(Debug)]
pub struct Entry {
    //input_digits: [SevenSegmentDisplay; 10],
    //output_digits: [SevenSegmentDisplay; 4],
    digits: [SevenSegmentDisplay; 14],
}

impl Entry {
    pub fn get_output_slice(&self) -> &[SevenSegmentDisplay] {
        &self.digits[10..]
    }

    pub fn get_simple_digits_in_output(&self) -> usize {
        let simple_lengths = [2, 4, 3, 7];
        self.get_output_slice()
            .iter()
            .filter(|entry| simple_lengths.contains(&entry.input.len()))
            .count()
    }

    pub fn determine_digits(&mut self) -> usize {
        let mut sorted: Vec<Option<SevenSegmentDisplay>> = vec![None; 10];

        for digit in self.digits.iter_mut() {
            match digit.input.len() {
                2 => {
                    digit.digit = Some(1);
                    sorted[1] = Some(digit.clone());
                }
                3 => {
                    digit.digit = Some(7);
                    sorted[7] = Some(digit.clone());
                }
                4 => {
                    digit.digit = Some(4);
                    sorted[4] = Some(digit.clone());
                }
                7 => {
                    digit.digit = Some(8);
                    sorted[8] = Some(digit.clone());
                }
                _ => {}
            }
        }
        let four_minus_one = String::from_iter(
            sorted[4]
                .as_ref()
                .unwrap()
                .input
                .chars()
                .filter(|c| !sorted[1].as_ref().unwrap().input.contains(*c)),
        );
        for digit in self.digits.iter_mut() {
            if digit.input.len() == 5 {
                // could be 2,3,5
                if four_minus_one.chars().all(|c| digit.input.contains(c)) {
                    // it's a 5
                    sorted[5] = Some(digit.clone());
                    digit.digit = Some(5);
                } else {
                    // it's a 2 or a 3. if it contains all of 1, it's a 3
                    if sorted[1]
                        .as_ref()
                        .unwrap()
                        .input
                        .chars()
                        .all(|c| digit.input.contains(c))
                    {
                        sorted[3] = Some(digit.clone());
                        digit.digit = Some(3);
                    } else {
                        sorted[2] = Some(digit.clone());
                        digit.digit = Some(2);
                    }
                }
            }
        }

        let mut five_plus_one = sorted[5].as_ref().unwrap().input.clone();
        five_plus_one.push_str(&sorted[1].as_ref().unwrap().input);
        let five_plus_one = String::from_iter(five_plus_one.chars().sorted_unstable().unique());
        for digit in self.digits.iter_mut() {
            if digit.input.len() == 6 {
                if five_plus_one == digit.input {
                    // it's a 9
                    sorted[9] = Some(digit.clone());
                    digit.digit = Some(9);
                } else {
                    // it's a 0 or 6. if it contains all segments of 7, it's a 0
                    if sorted[7]
                        .as_ref()
                        .unwrap()
                        .input
                        .chars()
                        .all(|c| digit.input.contains(c))
                    {
                        sorted[0] = Some(digit.clone());
                        digit.digit = Some(0);
                    } else {
                        sorted[6] = Some(digit.clone());
                        digit.digit = Some(6);
                    }
                }
            }
        }
        for digit in self.digits.iter_mut() {
            if digit.digit.is_none() {
                digit.digit = Some(
                    sorted
                        .iter()
                        .find(|elem| elem.as_ref().unwrap().input == digit.input)
                        .unwrap()
                        .as_ref()
                        .unwrap()
                        .digit
                        .unwrap(),
                );
            }
        }
        let mut result = 0;
        for (i, out_digit) in self.get_output_slice().iter().enumerate() {
            result += 10usize.pow(3 - i as u32) * out_digit.digit.unwrap() as usize;
        }
        result
    }
}

impl FromStr for Entry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (input, output) = s.split_once(" | ").ok_or_else(|| "No pipe".to_owned())?;
        let mut input_digits = input
            .trim()
            .split_ascii_whitespace()
            .map(|display| {
                SevenSegmentDisplay::from_str(display)
                    .map_err(|_| "Cannot parse display".to_owned())
            })
            .collect::<Result<Vec<SevenSegmentDisplay>, String>>()?;
        let mut output_digits = output
            .trim()
            .split_ascii_whitespace()
            .map(|display| {
                SevenSegmentDisplay::from_str(display)
                    .map_err(|_| "Cannot parse display".to_owned())
            })
            .collect::<Result<Vec<SevenSegmentDisplay>, String>>()?;
        input_digits.append(&mut output_digits);
        Ok(Entry {
            digits: input_digits
                .try_into()
                .map_err(|_| "cannot try into".to_owned())?,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SevenSegmentDisplay {
    input: String,
    digit: Option<u8>,
}

impl FromStr for SevenSegmentDisplay {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_unstable();
        Ok(SevenSegmentDisplay {
            input: String::from_iter(chars),
            digit: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_display() {
        let input = "cfbegad";
        let display = SevenSegmentDisplay::from_str(input);
        assert!(display.is_ok());
        let display = display.unwrap();
        assert_eq!(None, display.digit);
        assert_eq!("abcdefg", display.input);
    }

    #[test]
    fn it_parses_entry() {
        let entry = Entry::from_str("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe");
        assert!(entry.is_ok());
        let entry = entry.unwrap();
        assert_eq!(
            SevenSegmentDisplay::from_str("be").unwrap(),
            entry.digits[0]
        );
    }

    #[test]
    fn it_counts_simple_digits() {
        let input = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        let entries: Vec<Entry> = input
            .split('\n')
            .map(|e| e.trim().parse().unwrap())
            .collect();
        assert_eq!(10, entries.len());
        assert_eq!(
            26usize,
            entries
                .iter()
                .map(|entry| entry.get_simple_digits_in_output())
                .sum()
        );
    }

    #[test]
    fn it_determines_digits() {
        let input =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let mut entry: Entry = input.parse().unwrap();
        assert_eq!(5353, entry.determine_digits());
    }

    #[test]
    fn it_runs_example2() {
        let input = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        let mut entries: Vec<Entry> = input
            .split('\n')
            .map(|e| e.trim().parse().unwrap())
            .collect();
        assert_eq!(
            61229usize,
            entries
                .iter_mut()
                .map(|entry| entry.determine_digits())
                .sum()
        );
    }
}
