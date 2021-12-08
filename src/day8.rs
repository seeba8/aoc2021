use std::str::FromStr;

pub fn solve() {
    let input = std::fs::read_to_string(format!(
        "resources/{}.txt",
        module_path!().split_once("::").unwrap().1
    ))
    .unwrap();
    let entries: Vec<Entry> = input.trim().split('\n').map(|e| e.trim().parse().unwrap()).collect();
    println!("Day 8 part 1: {}", entries.iter().map(|entry| entry.get_simple_digits_in_output()).sum::<usize>());
}

#[derive(Debug)]
pub struct Entry {
    //input_digits: [SevenSegmentDisplay; 10],
    //output_digits: [SevenSegmentDisplay; 4],
    digits: [SevenSegmentDisplay; 14],
}

impl Entry {
    pub fn get_input_slice(&self) -> &[SevenSegmentDisplay] {
        &self.digits[0..10]
    }

    pub fn get_output_slice(&self) -> &[SevenSegmentDisplay] {
        &self.digits[10..]
    }

    pub fn get_simple_digits_in_output(&self) -> usize {
        let simple_lengths = [2,4,3,7];
        self.get_output_slice().iter().filter(|entry| simple_lengths.contains(&entry.input.len())).count()
    }

    
}

impl FromStr for Entry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (input, output) = s.split_once(" | ").ok_or("No pipe".to_owned())?;
        let mut input_digits = input
            .trim()
            .split_ascii_whitespace()
            .map(|display| SevenSegmentDisplay::from_str(display).map_err(|_| "Cannot parse display".to_owned()))
            .collect::<Result<Vec<SevenSegmentDisplay>, String>>()?;
        let mut output_digits = output
            .trim()
            .split_ascii_whitespace()
            .map(|display| SevenSegmentDisplay::from_str(display).map_err(|_| "Cannot parse display".to_owned()))
            .collect::<Result<Vec<SevenSegmentDisplay>, String>>()?;
        input_digits.append(&mut output_digits);
        Ok(Entry{digits: input_digits.try_into().map_err(|_| "cannot try into".to_owned())?})
    }
}

#[derive(Debug, PartialEq)]
pub struct SevenSegmentDisplay {
    input: String,
    digit: Option<u8>,
}

impl FromStr for SevenSegmentDisplay {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SevenSegmentDisplay {
            input: s.to_owned(),
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
        assert_eq!("cfbegad", display.input);
    }

    #[test]
    fn it_parses_entry () {
        let entry = Entry::from_str("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe");
        assert!(entry.is_ok());
        let entry = entry.unwrap();
        assert_eq!(SevenSegmentDisplay::from_str("be").unwrap(), entry.digits[0]);
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
        let entries: Vec<Entry> = input.split('\n').map(|e| e.trim().parse().unwrap()).collect();
        assert_eq!(10, entries.len());
        assert_eq!(26usize, entries.iter().map(|entry| entry.get_simple_digits_in_output()).sum());
    }
}
