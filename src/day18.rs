use std::{
    fmt,
    iter::Sum,
    ops::{Add, AddAssign},
    str::FromStr,
};

pub fn solve() {
    let input = std::fs::read_to_string(format!(
        "resources/{}.txt",
        module_path!().split_once("::").unwrap().1
    ))
    .unwrap();
    let pairs =  input.trim()
    .lines()
    .map(|line| line.trim().parse())
    .collect::<Result<Vec<Pair>, _>>().unwrap();
    println!("Day 18 part 1: {}", pairs.iter().cloned().sum::<Pair>().magnitude());
    println!("Day 18 part 2: {}", Pair::get_largest_magnitude(&pairs));
}

#[derive(Clone, PartialEq, Eq)]
pub struct Pair {
    left: Box<Element>,
    right: Box<Element>,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Element {
    Number(u16),
    Pair(Pair),
}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(arg0) => write!(f, "{}", arg0),
            Self::Pair(arg0) => write!(f, "[{:?},{:?}]", arg0.left, arg0.right),
        }
    }
}

impl fmt::Debug for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?},{:?}]", self.left, self.right)
    }
}

impl Element {
    fn explode(&mut self, level: u16) -> Option<(u16, u16)> {
        if level == 4 {
            match self {
                Element::Number(_) => None,
                Element::Pair(pair) => {
                    let cloned = pair.clone();
                    *self = Element::Number(0);
                    match (cloned.left.as_ref(), cloned.right.as_ref()) {
                        (Element::Number(l), Element::Number(r)) => Some((*l, *r)),
                        _ => panic!(),
                    }
                }
            }
        } else {
            match self {
                Element::Number(_) => None,
                Element::Pair(pair) => {
                    if let Some(result_left) = pair.left.explode(level + 1) {
                        // handle the right value
                        pair.right.handle_rightgoing_value(result_left.1);
                        return Some((result_left.0, 0));
                    }
                    if let Some(result_right) = pair.right.explode(level + 1) {
                        pair.left.handle_leftgoing_value(result_right.0);
                        return Some((0, result_right.1));
                    }
                    None
                }
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Element::Number(v) => {
                if *v >= 10 {
                    let left = *v / 2; // rounded down by virtue of integer division
                    let right = *v - left; // rounded up
                    *self = Element::Pair(Pair {
                        left: Box::new(Element::Number(left)),
                        right: Box::new(Element::Number(right)),
                    });
                    true
                } else {
                    false
                }
            }
            Element::Pair(Pair { left, right }) => left.split() || right.split(),
        }
    }

    fn handle_leftgoing_value(&mut self, value: u16) -> bool {
        match self {
            Element::Number(v) => {
                *v += value;
                true
            }
            Element::Pair(pair) => {
                // We're going left, so right is first
                if pair.right.handle_leftgoing_value(value) {
                    return true;
                }
                if pair.left.handle_leftgoing_value(value) {
                    return true;
                }
                false
            }
        }
    }

    fn handle_rightgoing_value(&mut self, value: u16) -> bool {
        match self {
            Element::Number(v) => {
                *v += value;
                true
            }
            Element::Pair(pair) => {
                if pair.left.handle_rightgoing_value(value) {
                    return true;
                }
                if pair.right.handle_rightgoing_value(value) {
                    return true;
                }
                false
            }
        }
    }

    fn parse(chars: &[char], index: &mut usize) -> Element {
        *index += 1;
        let left = if chars[*index - 1] == '[' {
            Element::parse(chars, index)
        } else {
            Element::Number(chars[*index - 1].to_digit(10).unwrap() as u16)
        };
        *index += 2; // because of the comma
        let right = if chars[*index - 1] == '[' {
            Element::parse(chars, index)
        } else {
            Element::Number(chars[*index - 1].to_digit(10).unwrap() as u16)
        };
        *index += 1;
        Element::Pair(Pair {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    pub fn magnitude(&self) -> usize {
        match self {
            Element::Number(n) => *n as usize,
            Element::Pair(pair) => 3 * pair.left.magnitude() + 2 * pair.right.magnitude(),
        }
    }
}

impl Pair {
    pub fn new(left: Pair, right: Pair) -> Pair {
        Pair {
            left: Box::new(Element::Pair(left)),
            right: Box::new(Element::Pair(right)),
        }
    }

    fn reduced(mut self) -> Self {
        //println!("Before reducing: {:?}", self);
        loop {
            if let Some(result) = self.left.explode(1) {
                self.right.handle_rightgoing_value(result.1);
                //println!("Exploded left: {:?}", self);
                continue;
            }
            if let Some(result) = self.right.explode(1) {
                self.left.handle_leftgoing_value(result.0);
                //println!("Exploded right: {:?}", self);
                continue;
            }
            if self.left.split() {
                //println!("Split left: {:?}", self);
                continue;
            }
            if self.right.split() {
                //println!("Split right: {:?}", self);
                continue;
            }
            break;
        }

        self
    }

    pub fn magnitude(&self) -> usize {
        3 * self.left.magnitude() + 2 * self.right.magnitude()
    }

    pub fn get_largest_magnitude(input: &[Pair]) -> usize {
        let mut max_magnitude: usize = 0;
        for a in input.iter() {
            for b in input.iter(){
                if a != b {
                    max_magnitude = max_magnitude.max((a.clone() + b.clone()).magnitude());
                }
            }
        }
        max_magnitude
    }
}

impl Add for Pair {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Pair::new(self, rhs).reduced()
    }
}

impl AddAssign for Pair {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

impl Sum for Pair {
    fn sum<I: Iterator<Item = Self>>(mut iter: I) -> Self {
        let mut sum = iter.next().unwrap();
        for v in iter {
            sum += v;
        }
        sum
    }
}

impl FromStr for Pair {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let element = Element::parse(&s.chars().collect::<Vec<char>>(), &mut 1);
        match element {
            Element::Number(_) => Err("Somehow, a number was the result"),
            Element::Pair(p) => Ok(p),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_parses_pairs() {
        let pair: Pair = "[1,2]".parse().unwrap();
        assert_eq!(
            Pair {
                left: Box::new(Element::Number(1)),
                right: Box::new(Element::Number(2))
            },
            pair
        );
        let pair: Pair = "[[1,2],3]".parse().unwrap();
        assert_eq!(
            Pair {
                left: Box::new(Element::Pair(Pair {
                    left: Box::new(Element::Number(1)),
                    right: Box::new(Element::Number(2))
                })),
                right: Box::new(Element::Number(3))
            },
            pair
        );
        let pair: Pair = "[9,[8,7]]".parse().unwrap();
        assert_eq!(
            Pair {
                left: Box::new(Element::Number(9)),
                right: Box::new(Element::Pair(Pair {
                    left: Box::new(Element::Number(8)),
                    right: Box::new(Element::Number(7))
                })),
            },
            pair
        );
    }

    #[test]
    fn it_adds_simple() {
        let a: Pair = "[1,2]".parse().unwrap();
        let b: Pair = "[[3,4],5]".parse().unwrap();
        assert_eq!(Pair::from_str("[[1,2],[[3,4],5]]").unwrap(), a + b);
    }

    #[test]
    fn it_explodes_left_side() -> Result<(), String> {
        let pair: Pair = "[[[[[9,8],1],2],3],4]".parse()?;
        let reduced = pair.reduced();
        assert_eq!(Pair::from_str("[[[[0,9],2],3],4]")?, reduced);
        Ok(())
    }

    #[test]
    fn it_explodes_right_side() -> Result<(), String> {
        let pair: Pair = "[7,[6,[5,[4,[3,2]]]]]".parse::<Pair>()?;
        let reduced = pair.reduced();
        assert_eq!(Pair::from_str("[7,[6,[5,[7,0]]]]")?, reduced);
        Ok(())
    }
    #[test]
    fn it_explodes_examples() -> Result<(), String> {
        assert_eq!(
            Pair::from_str("[[6,[5,[7,0]]],3]")?,
            Pair::from_str("[[6,[5,[4,[3,2]]]],1]")?.reduced()
        );
        assert_eq!(
            Pair::from_str("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")?,
            Pair::from_str("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")?.reduced()
        );
        Ok(())
    }

    #[test]
    fn it_splits() -> Result<(), String> {
        let mut element: Element = Element::Number(10);
        element.split();
        assert_eq!(Element::Pair(Pair::from_str("[5,5]")?), element);
        let mut element: Element = Element::Number(11);
        element.split();
        assert_eq!(Element::Pair(Pair::from_str("[5,6]")?), element);
        Ok(())
    }

    #[test]
    fn it_adds_complex() -> Result<(), String> {
        let a: Pair = "[[[[4,3],4],4],[7,[[8,4],9]]]".parse()?;
        let b: Pair = "[1,1]".parse()?;
        assert_eq!(Pair::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")?, a + b);

        Ok(())
    }

    #[test]
    fn it_adds_lists() -> Result<(), String> {
        let input = "[1,1]
        [2,2]
        [3,3]
        [4,4]";
        let list: Vec<Pair> = input
            .trim()
            .lines()
            .map(|line| line.trim().parse())
            .collect::<Result<Vec<Pair>, _>>()?;
        let sum: Pair = list.into_iter().sum();
        assert_eq!(Pair::from_str("[[[[1,1],[2,2]],[3,3]],[4,4]]")?, sum);

        let input = "[1,1]
        [2,2]
        [3,3]
        [4,4]
        [5,5]";
        let list: Vec<Pair> = input
            .trim()
            .lines()
            .map(|line| line.trim().parse())
            .collect::<Result<Vec<Pair>, _>>()?;
        let sum: Pair = list.into_iter().sum();
        assert_eq!(Pair::from_str("[[[[3,0],[5,3]],[4,4]],[5,5]]")?, sum);

        let input = "[1,1]
        [2,2]
        [3,3]
        [4,4]
        [5,5]
        [6,6]";
        let list: Vec<Pair> = input
            .trim()
            .lines()
            .map(|line| line.trim().parse())
            .collect::<Result<Vec<Pair>, _>>()?;
        let sum: Pair = list.into_iter().sum();
        assert_eq!(Pair::from_str("[[[[5,0],[7,4]],[5,5]],[6,6]]")?, sum);
        Ok(())
    }

    #[test]
    fn it_sums_slightly_larger_example() -> Result<(), String> {
        let list: Vec<Pair> = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
        [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
        [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
        [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
        [7,[5,[[3,8],[1,4]]]]
        [[2,[2,2]],[8,[8,1]]]
        [2,9]
        [1,[[[9,3],9],[[9,0],[0,7]]]]
        [[[5,[7,4]],7],1]
        [[[[4,2],2],6],[8,7]]"
            .trim()
            .lines()
            .map(|line| line.trim().parse())
            .collect::<Result<Vec<Pair>, _>>()?;
        let sum = list[0].clone() + list[1].clone();
        assert_eq!(
            Pair::from_str("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]")?,
            sum
        );
        let sum = list.into_iter().sum();
        assert_eq!(
            Pair::from_str("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")?,
            sum
        );
        Ok(())
    }

    #[test]
    fn it_calculates_magnitude() -> Result<(), String> {
        assert_eq!(29, Pair::from_str("[9,1]")?.magnitude());
        assert_eq!(21, Pair::from_str("[1,9]")?.magnitude());
        assert_eq!(129, Pair::from_str("[[9,1],[1,9]]")?.magnitude());
        assert_eq!(143, Pair::from_str("[[1,2],[[3,4],5]]")?.magnitude());
        assert_eq!(1384, Pair::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")?.magnitude());
        assert_eq!(445, Pair::from_str("[[[[1,1],[2,2]],[3,3]],[4,4]]")?.magnitude());
        assert_eq!(791, Pair::from_str("[[[[3,0],[5,3]],[4,4]],[5,5]]")?.magnitude());
        assert_eq!(1137, Pair::from_str("[[[[5,0],[7,4]],[5,5]],[6,6]]")?.magnitude());
        assert_eq!(3488, Pair::from_str("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")?.magnitude());
        Ok(())
    }

    #[test]
    fn it_solves_example1() -> Result<(), String> {
        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        let pairs =  input.trim()
        .lines()
        .map(|line| line.trim().parse())
        .collect::<Result<Vec<Pair>, _>>()?;
        assert_eq!(4140, pairs.into_iter().sum::<Pair>().magnitude());
        Ok(())
    }

    #[test]
    fn it_solves_example2() -> Result<(), String> {
        let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        let pairs =  input.trim()
        .lines()
        .map(|line| line.trim().parse())
        .collect::<Result<Vec<Pair>, _>>()?;
        assert_eq!(3993, Pair::get_largest_magnitude(&pairs));
        Ok(())
    }
}
