use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
    str::FromStr,
};

use itertools::Itertools;

pub fn solve() {
    let input = std::fs::read_to_string(format!(
        "resources/{}.txt",
        module_path!().split_once("::").unwrap().1
    ))
    .unwrap();
    let scanners = input
        .trim()
        .split("\n\n")
        .map(|scanner| scanner.parse().unwrap())
        .collect();
    let mut scanners = Scanners {
        scanners,
        beacons: HashSet::new(),
    };
    scanners.get_relative_positions(12);
    println!("Day 19 part 1: {}", scanners.beacons.len());
    println!("Day 19 part 2: {}", scanners.get_largest_distance());
    
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct RelativePosition {
    point: Point,
    variant: usize,
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>5}, {:>5}, {:>5}", self.x, self.y, self.z)
    }
}

impl Point {
    pub fn new(x: isize, y: isize, z: isize) -> Point {
        Point { x, y, z }
    }

    fn get_distance(&self, other: &Point) -> Point {
        self - other
    }
    fn get_manhattan_distance(&self, other: &Point) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) as usize
    }

    fn get_variant(&self, variant: usize) -> Point {
        let &Point { x, y, z } = self;
        match variant {
            1 => Point::new(x, y, z),
            2 => Point::new(x, -z, y),
            3 => Point::new(x, z, -y),
            4 => Point::new(x, -y, -z),

            5 => Point::new(-x, -y, z),
            6 => Point::new(-x, -z, -y),
            7 => Point::new(-x, z, y),
            8 => Point::new(-x, y, -z),

            9 => Point::new(y, -x, z),
            10 => Point::new(y, z, x),
            11 => Point::new(y, x, -z),
            12 => Point::new(y, -z, -x),

            13 => Point::new(-y, x, z),
            14 => Point::new(-y, -z, x),
            15 => Point::new(-y, z, -x),
            16 => Point::new(-y, -x, -z),

            17 => Point::new(z, y, -x),
            18 => Point::new(z, x, y),
            19 => Point::new(z, -y, x),
            20 => Point::new(z, -x, -y),

            21 => Point::new(-z, y, x),
            22 => Point::new(-z, -x, y),
            23 => Point::new(-z, -y, -x),
            24 => Point::new(-z, x, -y),
            _ => unreachable!(),
        }
    }
}

impl FromStr for Point {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coordinates = s
            .trim()
            .split(',')
            .map(|s| s.parse().map_err(|_| "Cannot parse coordinate"))
            .collect::<Result<Vec<isize>, _>>()?;
        if let (Some(x), Some(y), Some(z)) =
            (coordinates.get(0), coordinates.get(1), coordinates.get(2))
        {
            Ok(Point::new(*x, *y, *z))
        } else {
            Err("Did not get three coordinates")
        }
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scanner {
    name: String,
    other_scanners: HashMap<String, (Vec<usize>, RelativePosition)>,
    beacons: Vec<Point>,
}

impl FromStr for Scanner {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.trim().lines();
        let name = lines
            .next()
            .ok_or("cannot get first line")?
            .replace("---", "")
            .trim()
            .to_owned();
        let beacons: Vec<Point> = lines
            .map(|line| line.parse())
            .collect::<Result<Vec<Point>, _>>()?;
        Ok(Scanner {
            name,
            other_scanners: HashMap::new(),
            beacons,
        })
    }
}

impl Scanner {
    fn get_relative_position_of(
        &mut self,
        other: &Scanner,
        min_overlap: usize,
    ) -> Option<RelativePosition> {
        for i in 1..=24 {
            let mut offsets: HashMap<Point, usize> = HashMap::new();
            for self_beacon in &self.beacons {
                for other_beacon in &other.beacons {
                    offsets
                        .entry(self_beacon.get_distance(&other_beacon.get_variant(i)))
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }
            if let Some((relative_position, _)) = offsets
                .into_iter()
                .find(|(_, count)| *count >= min_overlap)
            {
                self.other_scanners.insert(
                    other.name.clone(),
                    (
                        vec![1],
                        RelativePosition {
                            point: relative_position.clone(),
                            variant: i,
                        },
                    ),
                );
                return Some(RelativePosition {
                    point: relative_position,
                    variant: i,
                });
            }
        }
        None
    }

    fn translated(&self, relative_position: &RelativePosition) -> Scanner {
        Scanner {
            name: self.name.clone(),
            other_scanners: HashMap::new(),
            beacons: self
                .beacons
                .iter()
                .cloned()
                .map(|p| p.get_variant(relative_position.variant) + relative_position.point.clone())
                .collect(),
        }
    }
}

pub struct Scanners {
    scanners: Vec<Scanner>,
    beacons: HashSet<Point>,
}

impl Scanners {
    fn get_relative_positions(&mut self, min_overlap: usize) {
        let others = self.scanners.clone();
        for scanner in self.scanners.iter_mut() {
            for other in &others {
                if scanner.name != other.name {
                    scanner.get_relative_position_of(other, min_overlap);
                }
            }
        }
        let others = self.scanners.clone();
        let first = &mut self.scanners[0];
        while first.other_scanners.len() < others.len() - 1 {
            for other in &others {
                if first.other_scanners.contains_key(&other.name) {
                    let other_relative = first.other_scanners.get(&other.name).unwrap().clone();
                    for (other_other_name, other_other) in &other.other_scanners {
                        if first.other_scanners.contains_key(other_other_name)
                            || &first.name == other_other_name
                        {
                            continue;
                        }
                        let mut p = other_other.1.point.get_variant(other_relative.1.variant);
                        for var in other_relative.0.iter().rev() {
                            p = p.get_variant(*var);
                        }
                        p = other_relative.1.point.clone() + p;
                        let rel_pos = RelativePosition {
                            point: p,
                            variant: other_other.1.variant,
                        };
                        let mut path = other_relative.0.clone();
                        path.push(other_relative.1.variant);
                        first
                            .other_scanners
                            .insert(other_other_name.clone(), (path, rel_pos));
                    }
                }
            }
        }
        let default = (
            vec![1],
            RelativePosition {
                point: Point::new(0, 0, 0),
                variant: 1,
            },
        );
        for scanner in &self.scanners {
            for beacon in &scanner.beacons {
                let path = self.scanners[0]
                    .other_scanners
                    .get(&scanner.name)
                    .unwrap_or(&default);
                let mut p = beacon.get_variant(path.1.variant);
                for var in path.0.iter().rev() {
                    p = p.get_variant(*var);
                }
                self.beacons.insert(path.1.point.clone() + p);
            }
        }
    }

    fn get_largest_distance(&self) -> usize {
        let mut positions: Vec<Point> = self.scanners[0].other_scanners.values().map(|v| v.1.point.clone()).collect();
        positions.push(Point::new(0,0,0));
        //println!("{:#?}", positions);
        positions.iter().permutations(2).map(|v| v[0].get_manhattan_distance(v[1])).max().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_scanner0() -> Scanner {
        "--- scanner 0 ---
        404,-588,-901
        528,-643,409
        -838,591,734
        390,-675,-793
        -537,-823,-458
        -485,-357,347
        -345,-311,381
        -661,-816,-575
        -876,649,763
        -618,-824,-621
        553,345,-567
        474,580,667
        -447,-329,318
        -584,868,-557
        544,-627,-890
        564,392,-477
        455,729,728
        -892,524,684
        -689,845,-530
        423,-701,434
        7,-33,-71
        630,319,-379
        443,580,662
        -789,900,-551
        459,-707,401"
            .parse()
            .unwrap()
    }
    fn get_scanner1() -> Scanner {
        "--- scanner 1 ---
        686,422,578
        605,423,415
        515,917,-361
        -336,658,858
        95,138,22
        -476,619,847
        -340,-569,-846
        567,-361,727
        -460,603,-452
        669,-402,600
        729,430,532
        -500,-761,534
        -322,571,750
        -466,-666,-811
        -429,-592,574
        -355,545,-477
        703,-491,-529
        -328,-685,520
        413,935,-424
        -391,539,-444
        586,-435,557
        -364,-763,-893
        807,-499,-711
        755,-354,-619
        553,889,-390"
            .parse()
            .unwrap()
    }
    fn get_scanner2() -> Scanner {
        "--- scanner 2 ---
        649,640,665
        682,-795,504
        -784,533,-524
        -644,584,-595
        -588,-843,648
        -30,6,44
        -674,560,763
        500,723,-460
        609,671,-379
        -555,-800,653
        -675,-892,-343
        697,-426,-610
        578,704,681
        493,664,-388
        -671,-858,530
        -667,343,800
        571,-461,-707
        -138,-166,112
        -889,563,-600
        646,-828,498
        640,759,510
        -630,509,768
        -681,-892,-333
        673,-379,-804
        -742,-814,-386
        577,-820,562"
            .parse()
            .unwrap()
    }
    fn get_scanner3() -> Scanner {
        "--- scanner 3 ---
        -589,542,597
        605,-692,669
        -500,565,-823
        -660,373,557
        -458,-679,-417
        -488,449,543
        -626,468,-788
        338,-750,-386
        528,-832,-391
        562,-778,733
        -938,-730,414
        543,643,-506
        -524,371,-870
        407,773,750
        -104,29,83
        378,-903,-323
        -778,-728,485
        426,699,580
        -438,-605,-362
        -469,-447,-387
        509,732,623
        647,635,-688
        -868,-804,481
        614,-800,639
        595,780,-596
        "
        .parse()
        .unwrap()
    }
    fn get_scanner4() -> Scanner {
        "--- scanner 4 ---
        727,592,562
        -293,-554,779
        441,611,-461
        -714,465,-776
        -743,427,-804
        -660,-479,-426
        832,-632,460
        927,-485,-438
        408,393,-506
        466,436,-512
        110,16,151
        -258,-428,682
        -393,719,612
        -211,-452,876
        808,-476,-593
        -575,615,604
        -485,667,467
        -680,325,-822
        -627,-443,-432
        872,-547,-609
        833,512,582
        807,604,487
        839,-516,451
        891,-625,532
        -652,-548,-490
        30,-46,-14"
            .parse()
            .unwrap()
    }

    #[test]
    fn it_parses_scanners() -> Result<(), &'static str> {
        let input = "--- scanner 0 ---
        404,-588,-901
        528,-643,409
        -838,591,734
        390,-675,-793
        -537,-823,-458
        -485,-357,347
        -345,-311,381
        -661,-816,-575
        -876,649,763
        -618,-824,-621
        553,345,-567
        474,580,667
        -447,-329,318
        -584,868,-557
        544,-627,-890
        564,392,-477
        455,729,728
        -892,524,684
        -689,845,-530
        423,-701,434
        7,-33,-71
        630,319,-379
        443,580,662
        -789,900,-551
        459,-707,401";
        let scanner: Result<Scanner, _> = input.parse();
        assert!(scanner.is_ok());
        let scanner = scanner.unwrap();
        assert_eq!("scanner 0", scanner.name);
        assert_eq!(Point::new(404, -588, -901), scanner.beacons[0]);
        Ok(())
    }

    #[test]
    fn it_calculates_offsets() {
        let mut scanner0 = get_scanner0();
        let mut scanner1 = get_scanner1();
        let relative_position = scanner0.get_relative_position_of(&scanner1, 12);
        assert!(relative_position.is_some());
        let RelativePosition {
            point: relative_position,
            variant: p1variant,
        } = relative_position.unwrap();
        assert_eq!(Point::new(68, -1246, -43), relative_position);
        let position_scanner1 = scanner0.get_relative_position_of(&scanner1, 12).unwrap();
        let scanner4 = get_scanner4();
        let position_scanner4 = scanner1.get_relative_position_of(&scanner4, 12);
        assert!(position_scanner4.is_some());
        //println!("{:?}", position_scanner4);
        let RelativePosition {
            point: p4,
            variant: _p4variant,
        } = position_scanner4.unwrap();
        let position_scanner4 = position_scanner1.point + p4.get_variant(p1variant);
        assert_eq!(Point::new(-20, -1133, 1061), position_scanner4);
    }

    #[test]
    fn it_translates() {
        let mut scanner0 = get_scanner0();
        let scanner1 = get_scanner1();
        let relative_position = scanner0.get_relative_position_of(&scanner1, 12).unwrap();
        let translated_scanner1 = scanner1.translated(&relative_position);
        println!("{:#?}", translated_scanner1);
    }

    #[test]
    fn it_calculates_relative_positions() {
        let scanners = vec![
            get_scanner0(),
            get_scanner1(),
            get_scanner2(),
            get_scanner3(),
            get_scanner4(),
        ];
        let mut scanners = Scanners {
            scanners,
            beacons: HashSet::new(),
        };
        scanners.get_relative_positions(12);
        assert_eq!(
            Point::new(68, -1246, -43),
            scanners.scanners[0]
                .other_scanners
                .get("scanner 1")
                .unwrap()
                .1
                .point
        );
        assert_eq!(
            Point::new(1105, -1205, 1229),
            scanners.scanners[0]
                .other_scanners
                .get("scanner 2")
                .unwrap()
                .1
                .point
        );
        assert_eq!(
            Point::new(-92, -2380, -20),
            scanners.scanners[0]
                .other_scanners
                .get("scanner 3")
                .unwrap()
                .1
                .point
        );
        assert_eq!(
            Point::new(-20, -1133, 1061),
            scanners.scanners[0]
                .other_scanners
                .get("scanner 4")
                .unwrap()
                .1
                .point
        );
        let mut beacons: Vec<Point> = scanners.beacons.iter().cloned().collect();
        beacons.sort_unstable();
        println!("{:#?}", beacons);
        assert_eq!(79, scanners.beacons.len());
    }

    #[test]
    fn it_gets_manhattan_distance() {
        let scanners = vec![
            get_scanner0(),
            get_scanner1(),
            get_scanner2(),
            get_scanner3(),
            get_scanner4(),
        ];
        let mut scanners = Scanners {
            scanners,
            beacons: HashSet::new(),
        };
        scanners.get_relative_positions(12);
        assert_eq!(3621, scanners.get_largest_distance());
    }
}
