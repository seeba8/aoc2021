use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    str::FromStr,
};

pub fn solve() {
    let input = std::fs::read_to_string(format!(
        "resources/{}.txt",
        module_path!().split_once("::").unwrap().1
    ))
    .unwrap();
    let ocean: Ocean = input.parse().unwrap();
    println!(
        "Day 5 part 1: {}",
        ocean.get_overlapping_points(false).len()
    );
    println!("Day 5 part 2: {}", ocean.get_overlapping_points(true).len());
}

#[derive(Debug, Clone)]
pub struct Vent {
    start: Point,
    end: Point,
}

impl Vent {
    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn get_covering_points(&self) -> HashSet<Point> {
        let mut covering_points = HashSet::new();
        if !self.is_horizontal() && !self.is_vertical() {
            match (self.end.x.cmp(&self.start.x), self.end.y.cmp(&self.start.y)) {
                (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => {
                    for (i, x) in (self.end.x..=self.start.x).enumerate() {
                        covering_points.insert((x, self.end.y + i).into());
                    }
                }
                (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => {
                    for (i, x) in (self.end.x..=self.start.x).enumerate() {
                        covering_points.insert((x, self.end.y - i).into());
                    }
                }
                (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => {
                    for (i, x) in (self.start.x..=self.end.x).enumerate() {
                        covering_points.insert((x, self.start.y - i).into());
                    }
                }
                (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => {
                    for (i, x) in (self.start.x..=self.end.x).enumerate() {
                        covering_points.insert((x, self.start.y + i).into());
                    }
                }
                _ => panic!("Somehow, a vertical or horizontal vent is in the diagonals"),
            }
        } else {
            for x in self.start.x.min(self.end.x)..=self.start.x.max(self.end.x) {
                for y in self.start.y.min(self.end.y)..=self.start.y.max(self.end.y) {
                    covering_points.insert((x, y).into());
                }
            }
        }

        covering_points
    }
}

impl FromStr for Vent {
    type Err = VentParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(" -> ").ok_or(VentParseError::Vent)?;
        Ok(Vent {
            start: start.parse()?,
            end: end.parse()?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Point { x, y }
    }
}

impl FromStr for Point {
    type Err = VentParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(VentParseError::Point)?;
        let x = x.parse().map_err(|_| VentParseError::Int)?;
        let y = y.parse().map_err(|_| VentParseError::Point)?;
        Ok((x, y).into())
    }
}

#[derive(Debug, Clone)]
pub enum VentParseError {
    Point,
    Int,
    Vent,
}

pub struct Ocean {
    vents: Vec<Vent>,
}

impl FromStr for Ocean {
    type Err = VentParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vents: Vec<Vent> = s
            .trim()
            .lines()
            .map(|l| l.trim().parse())
            .collect::<Result<Vec<Vent>, VentParseError>>()?;
        Ok(Ocean { vents })
    }
}

impl Ocean {
    fn get_overlapping_points(&self, with_diagonals: bool) -> HashSet<Point> {
        let mut overlaps: HashMap<Point, usize> = HashMap::new();
        for vent in self
            .vents
            .iter()
            .filter(|vent| with_diagonals || vent.is_horizontal() || vent.is_vertical())
        {
            for point in vent.get_covering_points() {
                overlaps
                    .entry(point)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
        overlaps
            .into_iter()
            .filter(|(_, count)| *count > 1)
            .map(|(point, _)| point)
            .collect()
    }
}

impl Display for Ocean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_x = self
            .vents
            .iter()
            .map(|v| v.start.x.min(v.end.x))
            .min()
            .ok_or(std::fmt::Error)?;
        let min_y = self
            .vents
            .iter()
            .map(|v| v.start.y.min(v.end.y))
            .min()
            .ok_or(std::fmt::Error)?;
        let max_x = self
            .vents
            .iter()
            .map(|v| v.start.x.max(v.end.x))
            .max()
            .ok_or(std::fmt::Error)?;
        let max_y = self
            .vents
            .iter()
            .map(|v| v.start.y.max(v.end.y))
            .max()
            .ok_or(std::fmt::Error)?;
        let width = max_x - min_x + 1;
        let height = max_y - min_y + 1;
        let mut grid = vec![0usize; width * height];
        for vent in self.vents.iter()
        //.filter(|v| v.is_horizontal() || v.is_vertical())
        {
            for point in vent.get_covering_points() {
                grid[(point.y - min_y) * width + (point.x - min_x)] += 1;
            }
        }
        for (i, p) in grid.iter().enumerate() {
            if i % width == 0 {
                writeln!(f)?;
            }
            if *p == 0 {
                write!(f, ".")?;
            } else {
                write!(f, "{}", p)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_parses_vent() {
        let vent: Result<Vent, VentParseError> = "1,1 -> 1,3".parse();
        assert!(vent.is_ok());
        let vent = vent.unwrap();
        assert_eq!(Point::from((1, 1)), vent.start);
        assert_eq!(Point::from((1, 3)), vent.end);
    }
    #[test]
    fn it_calculates_covering_points() {
        let vent = Vent {
            start: (1, 1).into(),
            end: (1, 3).into(),
        };
        let covering_points = vent.get_covering_points();
        assert_eq!(3, covering_points.len());
    }

    #[test]
    fn it_parses_ocean() {
        let ocean = r"0,9 -> 5,9
0,9 -> 2,9";
        let ocean: Result<Ocean, VentParseError> = ocean.parse();
        assert!(ocean.is_ok());
        let ocean = ocean.unwrap();
        assert_eq!(2, ocean.vents.len());
    }

    #[test]
    fn it_calculates_overlaps() {
        let ocean = r"0,9 -> 5,9
0,9 -> 2,9";
        let ocean: Ocean = ocean.parse().unwrap();
        let overlaps = ocean.get_overlapping_points(false);
        assert_eq!(3, overlaps.len());
        assert!(overlaps.contains(&Point::from((0, 9))));
        assert!(overlaps.contains(&Point::from((1, 9))));
        assert!(overlaps.contains(&Point::from((2, 9))));
        assert_eq!(false, overlaps.contains(&Point::from((3, 9))));
    }

    #[test]
    fn it_solves_example1() {
        let ocean = r"0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2";
        let ocean: Ocean = ocean.parse().unwrap();
        //println!("{}", ocean);
        assert_eq!(5, ocean.get_overlapping_points(false).len());
    }

    #[test]
    fn it_handles_diagonals() {
        let vent: Vent = "1,1 -> 3,3".parse().unwrap();
        let coverings = vent.get_covering_points();
        assert_eq!(3, coverings.len());
        assert!(coverings.contains(&Point::from((1, 1))));
        assert!(coverings.contains(&Point::from((2, 2))));
        assert!(coverings.contains(&Point::from((3, 3))));

        let vent: Vent = "3,3 -> 1,1".parse().unwrap();
        let coverings = vent.get_covering_points();
        assert_eq!(3, coverings.len());
        assert!(coverings.contains(&Point::from((1, 1))));
        assert!(coverings.contains(&Point::from((2, 2))));
        assert!(coverings.contains(&Point::from((3, 3))));

        let vent: Vent = "9,7 -> 7,9".parse().unwrap();
        let coverings = vent.get_covering_points();
        assert_eq!(3, coverings.len());
        assert!(coverings.contains(&Point::from((9, 7))));
        assert!(coverings.contains(&Point::from((8, 8))));
        assert!(coverings.contains(&Point::from((7, 9))));

        let vent: Vent = "7,9 -> 9,7".parse().unwrap();
        let coverings = vent.get_covering_points();
        assert_eq!(3, coverings.len());
        assert!(coverings.contains(&Point::from((9, 7))));
        assert!(coverings.contains(&Point::from((8, 8))));
        assert!(coverings.contains(&Point::from((7, 9))));
    }

    #[test]
    fn it_solves_example2() {
        let ocean = r"0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2";
        let ocean: Ocean = ocean.parse().unwrap();
        //println!("{}", ocean);
        assert_eq!(12, ocean.get_overlapping_points(true).len());
    }
}
