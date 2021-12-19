use std::cmp::Ordering;

pub fn solve() {
    let target = Rectangle(Point(143, -71), Point(177, -106));
    let best_flightpath = get_best_flightpath(&target);
    println!(
        "Day 17 part 1: {}",
        (best_flightpath.1.pow(2) + best_flightpath.1) / 2
    );
    println!("Day 17 part 2: {}", get_possible_flightpaths(&target).len());
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct Point(isize, isize);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct Rectangle(Point, Point);

impl Rectangle {
    fn contains(&self, point: &Point) -> bool {
        self.get_min_x() <= point.0
            && self.get_max_x() >= point.0
            && self.get_min_y() <= point.1
            && self.get_max_y() >= point.1
    }

    fn get_min_y(&self) -> isize {
        self.0 .1.min(self.1 .1)
    }
    fn get_min_x(&self) -> isize {
        self.0 .0.min(self.1 .0)
    }
    fn get_max_y(&self) -> isize {
        self.0 .1.max(self.1 .1)
    }
    fn get_max_x(&self) -> isize {
        self.0 .0.max(self.1 .0)
    }
}

fn get_best_flightpath(target: &Rectangle) -> Point {
    get_possible_flightpaths(target)
        .iter()
        .max_by(|a, b| {
            if a.1.cmp(&b.1) == Ordering::Equal {
                (-a.0).cmp(&-&b.0)
            } else {
                a.1.cmp(&b.1)
            }
        })
        .unwrap()
        .clone()
}

fn get_possible_flightpaths(target: &Rectangle) -> Vec<Point> {
    // We need the x velocity to be at least high enough so that (1+2+..+vel_x) = min_x of the rectangle
    // We need the x velocity to be at most max_x of the rectangle
    // the former can be solved via a quadratic equation since the sum of integers is gauss.. sum = (n²+n)/2 = 0.5n² + 0.5n.
    // Thus: 0.5n² + 0.5n - min_x = 0
    // (-0.5 +- sqrt(0.5² -4*0.5*(-min_x)))/(2*0.5)
    // (-0.5 +- sqrt(0.25 + 2*min_x))
    //
    // For example, for a rectangle with x=20..30:
    // -0.5 +- sqrt(0.25 + 2*20) = ~ 6, rounded up
    let min_x = (-0.5 + (0.25 + 2.0 * target.get_min_x() as f64).sqrt().ceil()) as isize;
    let max_x: isize = target.get_max_x();

    // For y, the lower bound is the lowest corner of the rectangle
    // The upper bound can be calculated since the y points of the flight path are symmetric as can be seen in the example.
    // This means that max vel_y is the distance from start to the lower end of the target area
    let mut possible_velocities: Vec<Point> = Vec::new();
    for x in min_x..=max_x {
        for y in target.get_min_y()..(-target.get_min_y()) {
            if can_hit(target, Point(x, y)) {
                possible_velocities.push(Point(x, y));
            }
        }
    }
    possible_velocities
}

fn can_hit(target: &Rectangle, initial_velocity: Point) -> bool {
    let mut velocity = initial_velocity;
    let mut probe = Point(0, 0);
    loop {
        probe.0 += velocity.0;
        probe.1 += velocity.1;
        velocity.0 += match velocity.0.cmp(&0) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };
        velocity.1 -= 1;
        if target.contains(&probe) {
            return true;
        }
        if probe.0 > target.get_max_x() || probe.1 < target.get_min_y() {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_possible_flightpaths() {
        let target = Rectangle(Point(20, -5), Point(30, -10));
        let possible_velocities = get_possible_flightpaths(&target);
        assert!(!possible_velocities.contains(&Point(17, -4)));
        assert_eq!(112, possible_velocities.len());
    }

    #[test]
    fn it_gets_best_flightpath() {
        let target = Rectangle(Point(20, -5), Point(30, -10));
        let best_flightpath = get_best_flightpath(&target);
        assert_eq!(Point(6, 9), best_flightpath);
        assert_eq!(45, (best_flightpath.1.pow(2) + best_flightpath.1) / 2);
    }
}
