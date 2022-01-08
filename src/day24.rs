use crate::day24_interpreted;

pub fn solve() {
    let input = std::fs::read_to_string(format!(
        "resources/{}.txt",
        module_path!().split_once("::").unwrap().1
    ))
    .unwrap();
    let code = generate_code(&input);
    std::fs::write("src/day24_generated.rs", code).unwrap();
    println!("Day 24 part 1: {}", 98998519596997u64);
    println!("Day 24 part 2: {}", 31521119151421u64);
    println!("Day 24 solved by manually looking at the summarised code.");
}

#[allow(unused)]
fn _solve_part1() -> i64 {
    let mut max = i64::MIN;
    for i in (11_111_111_111_111u64..=99_999_999_999_999).rev() {
        if i % 10_000_000 == 0 {
            println!("{}", i);
        }
        if let Some(v) = day24_interpreted::solve(ModelNumber::new(i)) {
            if v.3 == 0 {
                println!("{}", i);
                max = max.max(i.try_into().unwrap());
            }
        };
    }
    max
}

fn generate_code(input: &str) -> String {
    let mut out = String::from(
        "
#[allow(unused)]
pub fn solve(mut input: impl Iterator<Item = i64>) -> Option<(i64, i64, i64, i64)> {
    let mut w: i64 = 0;
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut z: i64 = 0;\n",
    );
    for line in input.lines() {
        let mut segments = line.trim().split_ascii_whitespace();
        let (op, arg1, arg2) = (segments.next(), segments.next(), segments.next());
        if let Some(op) = op {
            let arg1 = arg1.unwrap();
            match op {
                "inp" => {
                    out += &format!("    {} = input.next()?;\n", arg1);
                }
                "add" => {
                    let arg2 = arg2.unwrap();
                    out += &format!("    {} += {};\n", arg1, arg2);
                }
                "mul" => {
                    let arg2 = arg2.unwrap();
                    out += &format!("    {} *= {};\n", arg1, arg2);
                }
                "div" => {
                    let arg2 = arg2.unwrap();
                    out += &format!("    {} /= {};\n", arg1, arg2);
                }
                "mod" => {
                    let arg2 = arg2.unwrap();
                    out += &format!("    {} %= {};\n", arg1, arg2);
                }
                "eql" => {
                    let arg2 = arg2.unwrap();
                    out += &format!("    {} = if {0} == {} {{ 1 }} else {{ 0 }};\n", arg1, arg2);
                }
                _ => {}
            }
        }
    }
    out += "    Some((w,x,y,z))\n}\n";
    out
}

#[derive(Debug, Clone, Copy)]
pub struct ModelNumber {
    value: u64,
    index: u32,
}

impl Iterator for ModelNumber {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let digit = self.value / 10u64.pow(self.index);
        if digit == 0 {
            None
        } else {
            self.value -= digit * 10u64.pow(self.index);
            if self.index > 0 {
                self.index -= 1;
            }
            digit.try_into().ok()
        }
    }
}

impl ModelNumber {
    pub fn new(value: u64) -> ModelNumber {
        ModelNumber { value, index: 13 }
    }
}

#[cfg(test)]
mod tests {
    fn get_example1() -> String {
        "inp z
inp x
mul z 3
eql z x"
            .to_owned()
    }

    fn get_example2() -> String {
        "inp w
        add z w
        mod z 2
        div w 2
        add y w
        mod y 2
        div w 2
        add x w
        mod x 2
        div w 2
        mod w 2"
            .to_owned()
    }

    use crate::{day24_generated, day24_interpreted};

    use super::*;
    #[test]
    fn it_iterates_digits() {
        let mut model_number = ModelNumber::new(13579246899999);
        assert_eq!(Some(1), model_number.next());
        assert_eq!(Some(3), model_number.next());
        assert_eq!(Some(5), model_number.next());
        assert_eq!(Some(7), model_number.next());
        assert_eq!(Some(9), model_number.next());
        assert_eq!(Some(2), model_number.next());
        assert_eq!(Some(4), model_number.next());
        assert_eq!(Some(6), model_number.next());
        assert_eq!(Some(8), model_number.next());
        assert_eq!(Some(9), model_number.next());
        assert_eq!(Some(9), model_number.next());
        assert_eq!(Some(9), model_number.next());
        assert_eq!(Some(9), model_number.next());
        assert_eq!(Some(9), model_number.next());
        assert_eq!(None, model_number.next());
    }

    #[test]
    fn it_generates_code() {
        let input = get_example1();
        let code = generate_code(&input);
        println!("{}", code);
        println!();
        println!("{}", generate_code(&get_example2()));
    }

    #[allow(unused)]
    pub fn generated_example1(
        mut input: impl Iterator<Item = i64>,
    ) -> Option<(i64, i64, i64, i64)> {
        let mut w: i64 = 0;
        let mut x: i64 = 0;
        let mut y: i64 = 0;
        let mut z: i64 = 0;
        z = input.next()?;
        x = input.next()?;
        z *= 3;
        z = if z == x { 1 } else { 0 };
        Some((w, x, y, z))
    }

    #[allow(unused)]
    pub fn generated_example2(
        mut input: impl Iterator<Item = i64>,
    ) -> Option<(i64, i64, i64, i64)> {
        let mut w: i64 = 0;
        let mut x: i64 = 0;
        let mut y: i64 = 0;
        let mut z: i64 = 0;
        w = input.next()?;
        z += w;
        z %= 2;
        w /= 2;
        y += w;
        y %= 2;
        w /= 2;
        x += w;
        x %= 2;
        w /= 2;
        w %= 2;
        Some((w, x, y, z))
    }

    #[test]
    fn it_generated_example1_correctly() {
        assert_eq!(1, generated_example1(vec![3, 9].into_iter()).unwrap().3);
        assert_eq!(0, generated_example1(vec![3, 10].into_iter()).unwrap().3);
        assert_eq!(None, generated_example1(vec![3].into_iter()));
    }

    #[test]
    fn it_generated_example2_correctly() {
        assert_eq!(Some((0, 1, 1, 1)), generated_example2(vec![7].into_iter()));
        assert_eq!(Some((1, 1, 0, 1)), generated_example2(vec![13].into_iter()));
    }

    #[test]
    fn interpreted_equals_real() {
        for i in (11_111_111_111_111u64..=11_111_111_999_999).rev() {
            match (
                day24_generated::solve(ModelNumber::new(i)),
                day24_interpreted::solve(ModelNumber::new(i)),
            ) {
                (Some(generated), Some(interpreted)) => {
                    assert_eq!(generated.3, interpreted.3, "{}", i);
                }
                (None, None) => {}
                _ => assert!(false),
            };
        }
    }

    #[test]
    fn it_solves_the_largest_monad_example() {
        assert_eq!(
            0,
            day24_interpreted::solve(ModelNumber::new(98998519596997))
                .unwrap()
                .3
        );
    }

    #[test]
    fn it_solves_the_smallest_monad_example() {
        assert_eq!(
            0,
            day24_interpreted::solve(ModelNumber::new(31521119151421))
                .unwrap()
                .3
        );
    }
}
