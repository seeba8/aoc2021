pub fn solve() {
    let input: Vec<usize> = std::fs::read_to_string("./resources/day01.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();
    println!("Day 1 part 1: {}", get_number_of_increases(&input));
    println!(
        "Day 1 part 2: {}",
        get_number_of_increases_windowed(&input, 3)
    );
}

pub fn get_number_of_increases(depths: &[usize]) -> usize {
    depths
        .iter()
        .zip(depths.iter().skip(1))
        .map(|(&a, &b)| if a < b { 1 } else { 0 })
        .sum()
}

pub fn get_number_of_increases_windowed(depths: &[usize], window_size: usize) -> usize {
    get_number_of_increases(
        &depths
            .windows(window_size)
            .map(|window| window.iter().sum())
            .collect::<Vec<usize>>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_works() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(7, get_number_of_increases(&input))
    }

    #[test]
    fn part2_example_works() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(5, get_number_of_increases_windowed(&input, 3))
    }
}
