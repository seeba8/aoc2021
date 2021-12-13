use std::{collections::HashSet, fmt::Display, str::FromStr};

use itertools::Itertools;

pub fn solve() {
    let input = std::fs::read_to_string(format!(
        "resources/{}.txt",
        module_path!().split_once("::").unwrap().1
    ))
    .unwrap();
    let mut paper: Paper = input.parse().unwrap();
    paper.apply_fold(paper.folds[0].clone());
    println!("Day 13 part 1: {}", paper.points.len());
    let mut paper: Paper = input.parse().unwrap();
    paper.fold();
    println!("Day 13 part 2: \n{}", paper);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point(usize, usize);

impl FromStr for Point {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .trim()
            .split_once(',')
            .ok_or("cannot split Point by comma")?;
        Ok(Point(
            x.parse().map_err(|_| "cannot parse x coordinate")?,
            y.parse().map_err(|_| "cannot parse y coordinate")?,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fold(Option<usize>, Option<usize>);

impl FromStr for Fold {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .trim()
            .split_ascii_whitespace()
            .nth(2)
            .ok_or("cannot parse fold instruction")?;
        let (axis, value) = s
            .trim()
            .split_once('=')
            .ok_or("cannot parse fold instruction value")?;
        if axis == "x" {
            Ok(Fold(
                Some(value.parse().map_err(|_| "Cannot parse fold value")?),
                None,
            ))
        } else {
            Ok(Fold(
                None,
                Some(value.parse().map_err(|_| "Cannot parse fold value")?),
            ))
        }
    }
}

#[derive(Debug)]
pub struct Paper {
    points: HashSet<Point>,
    folds: Vec<Fold>,
}

impl Paper {
    fn fold(&mut self) {
        let folds: Vec<Fold> = self.folds.to_vec();
        for fold in folds {
            self.apply_fold(fold);
        }
    }

    fn apply_fold(&mut self, fold: Fold) {
        match fold {
            Fold(Some(x), None) => {
                let mut points = HashSet::new();
                for Point(px, py) in self.points.drain() {
                    if px > x {
                        points.insert(Point(x - (px - x), py));
                    } else {
                        points.insert(Point(px, py));
                    }
                }
                self.points = points;
            }
            Fold(None, Some(y)) => {
                let mut points = HashSet::new();
                for Point(px, py) in self.points.drain() {
                    if py > y {
                        points.insert(Point(px, y - (py - y)));
                    } else {
                        points.insert(Point(px, py));
                    }
                }
                self.points = points;
            }
            _ => panic!(),
        }
    }
}

impl FromStr for Paper {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (points, folds) = s
            .trim()
            .split_once("\n\n")
            .ok_or("Error splitting input in points and folds")?;
        let points = points
            .lines()
            .map(|point| point.parse())
            .collect::<Result<HashSet<Point>, &'static str>>()?;
        let folds = folds
            .lines()
            .map(|fold| fold.parse())
            .collect::<Result<Vec<Fold>, &'static str>>()?;
        Ok(Paper { points, folds })
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = self.points.iter().map(|p| p.0).max().unwrap() + 1;
        let max_y = self.points.iter().map(|p| p.1).max().unwrap() + 1;
        let mut grid: Vec<Vec<bool>> = vec![vec![false; max_y]; max_x];
        for point in &self.points {
            grid[point.0][point.1] = true;
        }
        for y in 0..max_y {
            writeln!(
                f,
                "{}",
                (0..max_x)
                    .map(|x| if grid[x][y] { "#" } else { "." })
                    .join("")
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_point() {
        let point: Result<Point, _> = "6,10".parse();
        assert!(point.is_ok());
        let point = point.unwrap();
        assert_eq!(Point(6, 10), point);
    }

    #[test]
    fn it_parses_fold() {
        let fold: Result<Fold, _> = "fold along y=7".parse();
        assert!(fold.is_ok());
        let fold = fold.unwrap();
        assert_eq!(Fold(None, Some(7)), fold);
    }

    #[test]
    fn it_folds() {
        let fold: Fold = "fold along y=5".parse().unwrap();
        let point = Point(2, 7);
        let mut paper: Paper = Paper {
            points: [point].iter().cloned().collect(),
            folds: vec![fold.clone()],
        };
        paper.apply_fold(fold);
        assert_eq!(Point(2, 3), paper.points.iter().next().unwrap().clone());
    }

    #[test]
    fn it_folds_multiple_points() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7";
        let mut paper: Paper = input.parse().unwrap();
        paper.fold();
        assert_eq!(17, paper.points.len());
        println!("{}", paper);
    }
}
