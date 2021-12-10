pub fn solve() {
    let input = std::fs::read_to_string("resources/day03.txt").unwrap();
    let input: Input = input.as_str().into();
    println!(
        "Day 3 part 1: {}",
        input.get_epsilon_rate() * input.get_gamma_rate()
    );
    println!(
        "Day 3 part 2: {}",
        input.get_co2_rating() * input.get_oxygen_rating()
    );
}

#[derive(Clone)]
pub struct Input {
    values: Vec<usize>,
    bit_length: usize,
}

impl Input {
    pub fn get_gamma_rate(&self) -> usize {
        let mut result = 0;
        for i in 0..self.bit_length {
            if self.get_most_common_bit(i) {
                result += 1 << i;
            }
        }
        result
    }

    fn get_most_common_bit(&self, position: usize) -> bool {
        2 * self
            .values
            .iter()
            .map(|v| v & 1 << position != 0)
            .filter(|i| *i)
            .count()
            >= self.values.len()
    }

    pub fn get_oxygen_rating(&self) -> usize {
        let cloned_uboat = self.clone();
        let cloned_uboat = cloned_uboat.filter_oxygen_rating();
        cloned_uboat.values[0]
    }

    fn filter_oxygen_rating(mut self) -> Self {
        for i in (0..self.bit_length).rev() {
            let most_common_bit = self.get_most_common_bit(i);
            self.values = self
                .values
                .into_iter()
                .filter(|v| {
                    if most_common_bit {
                        v & 1 << i != 0
                    } else {
                        v & 1 << i == 0
                    }
                })
                .collect();
        }
        self
    }

    pub fn get_co2_rating(&self) -> usize {
        let cloned_uboat = self.clone();
        let cloned_uboat = cloned_uboat.filter_co2_rating();
        cloned_uboat.values[0]
    }

    fn filter_co2_rating(mut self) -> Self {
        for i in (0..self.bit_length).rev() {
            let least_common_bit = !self.get_most_common_bit(i);
            self.values = self
                .values
                .into_iter()
                .filter(|v| {
                    if least_common_bit {
                        v & 1 << i != 0
                    } else {
                        v & 1 << i == 0
                    }
                })
                .collect();
            if self.values.len() == 1 {
                return self;
            }
        }
        self
    }

    pub fn get_epsilon_rate(&self) -> usize {
        let mut result = 0;
        for i in 0..self.bit_length {
            if !self.get_most_common_bit(i) {
                result += 1 << i;
            }
        }
        result
    }
}

impl From<&str> for Input {
    fn from(input: &str) -> Self {
        Input {
            values: input
                .trim()
                .lines()
                .map(|line| usize::from_str_radix(line.trim(), 2).unwrap())
                .collect(),
            bit_length: input.lines().next().unwrap().len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_parses_input() {
        let input = r"00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010";
        let input: Input = input.into();
        assert_eq!(5, input.bit_length);
        assert_eq!(4, input.values[0])
    }

    #[test]
    fn it_calculates_gamma() {
        let input = r"00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010";
        let input: Input = input.into();
        assert_eq!(22, input.get_gamma_rate());
    }

    #[test]
    fn it_calculates_epsilon() {
        let input = r"00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010";
        let input: Input = input.into();
        assert_eq!(9, input.get_epsilon_rate());
    }

    #[test]
    fn it_gets_most_common_bit() {
        let input = r"01111
        00111
        11100
        10000
        11001
        00010";
        let input: Input = input.into();
        assert_eq!(true, input.get_most_common_bit(0));
    }

    #[test]
    fn it_calculates_oxygen_rating() {
        let input = r"00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010";
        let input: Input = input.into();
        assert_eq!(23, input.get_oxygen_rating())
    }

    #[test]
    fn it_calculates_co2_rating() {
        let input = r"00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010";
        let input: Input = input.into();
        assert_eq!(10, input.get_co2_rating())
    }
}
