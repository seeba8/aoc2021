use std::{
    collections::HashSet,
    fmt::Display,
    hash::{Hash, Hasher},
    str::FromStr,
    sync::Mutex,
};
lazy_static! {
    static ref VISITED: Mutex<HashSet<u64>> = Mutex::new(HashSet::new());
}

pub fn solve() {
    let input = std::fs::read_to_string("resources/day23.txt").unwrap();
    let mut burrow: Burrow = input.parse().unwrap();
    println!("Day 23 part 1: {}", burrow.solve());
    let input = std::fs::read_to_string("resources/day23_part2.txt").unwrap();
    let mut burrow: Burrow = input.parse().unwrap();
    println!("Day 23 part 2: {}", burrow.solve());
}
/**
This is the burrow:
```
   0123456789a -> x
################
 0#...........#
 1###B#C#B#D###
 2  #D#C#B#A#
 3  #D#B#A#C#
 4  #A#D#C#A#
 5  #########
```
*/
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
struct Burrow {
    /// Vec: A A B B C C D D
    amphipods: Vec<u8>,
    total_cost: u16,
    amphipods_per_colour: u8,
}

impl Burrow {
    fn get_coordinate(position: u8) -> (u8, u8) {
        (position / 0xf, position % 0xf)
    }

    fn get_position(x: u8, y: u8) -> u8 {
        x * 0xf + y
    }

    fn is_finished(&self) -> bool {
        (0..self.amphipods.len()).all(|i| self.is_in_own_room(i))
    }

    fn is_in_hallway(&self, amphipod_id: usize) -> bool {
        Burrow::get_coordinate(self.amphipods[amphipod_id]).1 == 0
    }

    fn is_in_own_room(&self, amphipod_id: usize) -> bool {
        let (x, y) = Burrow::get_coordinate(self.amphipods[amphipod_id]);
        y > 0 && x == self.get_own_room_x(amphipod_id)
    }

    fn get_own_room_x(&self, amphipod_id: usize) -> u8 {
        2 * (amphipod_id as u8 / self.amphipods_per_colour) + 2
    }

    #[allow(clippy::identity_op)]
    fn get_eligible_hallway_positions() -> [u8; 7] {
        [0, 1 * 0xf, 3 * 0xf, 5 * 0xf, 7 * 0xf, 9 * 0xf, 0xa * 0xf]
    }

    fn get_own_room(&self, amphipod_id: usize) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.amphipods_per_colour.into());
        for i in 0..self.amphipods_per_colour {
            result.push(Burrow::get_position(
                self.get_own_room_x(amphipod_id),
                i + 1,
            ));
        }
        result
    }

    fn can_move_to_own_room(&self, amphipod_id: usize) -> bool {
        if self.is_foreign_amphipod_in_own_room(amphipod_id) {
            return false;
        }
        for step in self.get_path(
            self.amphipods[amphipod_id],
            Burrow::get_position(self.get_own_room_x(amphipod_id), 1),
        ) {
            // let _s = Burrow::get_coordinate(step);
            if self.amphipods.contains(&step) && self.amphipods[amphipod_id] != step {
                return false;
            }
        }
        true
    }

    fn is_foreign_amphipod_in_own_room(&self, amphipod_id: usize) -> bool {
        let amphipods_per_colour = self.amphipods_per_colour as usize;
        //let (x, y) = Burrow::get_coordinate(self.amphipods[amphipod_id]);
        for amphipod in 0..self.amphipods.len() {
            if amphipod / amphipods_per_colour == amphipod_id / amphipods_per_colour {
                // same type
                continue;
            }
            let p = Burrow::get_coordinate(self.amphipods[amphipod]);
            if p.0 == self.get_own_room_x(amphipod_id) && p.1 > 0 {
                return true;
            }
        }
        false
    }

    fn can_move_to(&self, amphipod_id: usize, target: u8) -> bool {
        let path = self.get_path(self.amphipods[amphipod_id], target);
        for step in path {
            if self.amphipods.contains(&step) && self.amphipods[amphipod_id] != step {
                return false;
            }
        }
        true
    }

    fn solve(&mut self) -> u16 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.hash(&mut hasher);
        let hash = hasher.finish();

        //println!("{}", self);
        let mut v = VISITED.lock().unwrap();
        if v.contains(&hash) {
            return u16::MAX;
        }
        v.insert(hash);
        drop(v);

        let mut min_cost = u16::MAX;
        if self.is_finished() {
            // println!("{}", self.total_cost);
            // println!("{}", self);
            return self.total_cost;
        }
        for amphipod_id in 0..self.amphipods.len() {
            let (amphipod_x, amphipod_y) = Burrow::get_coordinate(self.amphipods[amphipod_id]);
            let is_in_own_room = self.is_in_own_room(amphipod_id);
            if is_in_own_room {
                if amphipod_y == self.amphipods_per_colour {
                    // already in the lowest slot
                    continue;
                }
                if !self.is_foreign_amphipod_in_own_room(amphipod_id) {
                    continue;
                }
            }
            // check if we are in the lower slot and the upper slot is in use
            if !self.can_move_to(amphipod_id, Burrow::get_position(amphipod_x, 0)) {
                continue;
            }
            let can_move_to_own_room = self.can_move_to_own_room(amphipod_id);
            if self.is_in_hallway(amphipod_id) && !can_move_to_own_room {
                continue;
            }
            if !is_in_own_room && can_move_to_own_room {
                let own_room = self.get_own_room(amphipod_id);
                for position_in_room in own_room.iter().rev() {
                    if self.amphipods.contains(position_in_room) {
                        continue;
                    }
                    let mut cloned = self.clone();
                    cloned.amphipods[amphipod_id] = *position_in_room;
                    let step_cost = 10u16
                        .pow(amphipod_id as u32 / self.amphipods_per_colour as u32)
                        * Burrow::get_distance(self.amphipods[amphipod_id], *position_in_room);
                    cloned.total_cost = cloned.total_cost.saturating_add(step_cost);
                    // println!(
                    //     "Walking cost: {}. New total cost: {}",
                    //     step_cost, cloned.total_cost
                    // );
                    min_cost = cloned.solve().min(min_cost);
                    break;
                }
            } else {
                // In foreign room, cannot move to own room. Must move to hallway. Or in own room, but foreign amphipod below
                for target in Burrow::get_eligible_hallway_positions() {
                    if self.amphipods.contains(&target) {
                        continue;
                    }
                    if !self.can_move_to(amphipod_id, target) {
                        continue;
                    }
                    let mut cloned = self.clone();
                    cloned.amphipods[amphipod_id] = target;
                    let step_cost = 10u16
                        .pow(amphipod_id as u32 / self.amphipods_per_colour as u32)
                        * Burrow::get_distance(self.amphipods[amphipod_id], target);
                    cloned.total_cost = cloned.total_cost.saturating_add(step_cost);
                    // println!(
                    //     "Walking cost: {}. New total cost: {}",
                    //     step_cost, cloned.total_cost
                    // );
                    min_cost = cloned.solve().min(min_cost);
                }
            }
        }
        min_cost
    }

    fn get_distance(a: u8, b: u8) -> u16 {
        let a = Burrow::get_coordinate(a);
        let b = Burrow::get_coordinate(b);
        if a.0 == b.0 {
            (a.1 as i16 - b.1 as i16).abs() as u16
        } else if a.1 == b.1 && a.1 == 0 {
            (a.0 as i16 - b.0 as i16).abs() as u16
        } else {
            a.1 as u16 + b.1 as u16 + (a.0 as i16 - b.0 as i16).abs() as u16
        }
    }

    fn get_path(&self, a: u8, b: u8) -> Vec<u8> {
        let mut path = Vec::new();
        let a = Burrow::get_coordinate(a);
        let b = Burrow::get_coordinate(b);
        if a.0 == b.0 {
            for y in a.1.min(b.1)..=a.1.max(b.1) {
                path.push(Burrow::get_position(a.0, y));
            }
        } else if a.1 == b.1 && a.1 == 0 {
            for x in a.0.min(b.0)..=a.0.max(b.0) {
                path.push(Burrow::get_position(x, a.1));
            }
        } else {
            for y in 0..=a.1 {
                path.push(Burrow::get_position(a.0, y));
            }
            for x in a.0.min(b.0)..=a.0.max(b.0) {
                path.push(Burrow::get_position(x, 0));
            }
            for y in 0..=b.1 {
                path.push(Burrow::get_position(b.0, y));
            }
        }
        path
    }
}

impl Display for Burrow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#############")?;
        write!(f, "#")?;
        for i in 0x0..=0xa {
            let position = Burrow::get_position(i, 0);
            match self.amphipods.iter().position(|c| *c == position) {
                Some(amphipod) => write!(
                    f,
                    "{}",
                    (b'A' + (amphipod / self.amphipods_per_colour as usize) as u8) as char
                ),
                None => write!(f, "."),
            }?;
        }
        writeln!(f, "#")?;
        for y in 1..=self.amphipods_per_colour {
            if y == 1 {
                write!(f, "###")?;
            } else {
                write!(f, "  #")?;
            }
            match self
                .amphipods
                .iter()
                .position(|c| *c == Burrow::get_position(2, y))
            {
                Some(amphipod) => write!(
                    f,
                    "{}",
                    (b'A' + (amphipod / self.amphipods_per_colour as usize) as u8) as char
                ),
                None => write!(f, "."),
            }?;
            write!(f, "#")?;
            match self
                .amphipods
                .iter()
                .position(|c| *c == Burrow::get_position(4, y))
            {
                Some(amphipod) => write!(
                    f,
                    "{}",
                    (b'A' + (amphipod / self.amphipods_per_colour as usize) as u8) as char
                ),
                None => write!(f, "."),
            }?;
            write!(f, "#")?;
            match self
                .amphipods
                .iter()
                .position(|c| *c == Burrow::get_position(6, y))
            {
                Some(amphipod) => write!(
                    f,
                    "{}",
                    (b'A' + (amphipod / self.amphipods_per_colour as usize) as u8) as char
                ),
                None => write!(f, "."),
            }?;
            write!(f, "#")?;
            match self
                .amphipods
                .iter()
                .position(|c| *c == Burrow::get_position(8, y))
            {
                Some(amphipod) => write!(
                    f,
                    "{}",
                    (b'A' + (amphipod / self.amphipods_per_colour as usize) as u8) as char
                ),
                None => write!(f, "."),
            }?;
            if y == 1 {
                writeln!(f, "###")?;
            } else {
                writeln!(f, "#  ")?;
            }
        }

        writeln!(f, "  #########")
    }
}

impl FromStr for Burrow {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let number = s.lines().count() - 3;
        let mut amphipods = vec![0; 4 * number];
        for (y, line) in s.lines().skip(1).enumerate() {
            for (position, c) in line.chars().skip(1).enumerate() {
                match c {
                    'A' | 'B' | 'C' | 'D' => {
                        for i in 0..number {
                            if amphipods[number * (c as u8 - b'A') as usize + i] == 0 {
                                amphipods[number * (c as u8 - b'A') as usize + i] =
                                    Burrow::get_position(position as u8, y as u8);
                                break;
                            }
                        }
                    }
                    _ => {}
                };
            }
        }
        let mut cost = vec![0; 4 * number];
        for i in 0..4 {
            for j in 0..number {
                cost[i * number + j] = 10usize.pow(i as u32);
            }
        }
        Ok(Burrow {
            amphipods,
            total_cost: 0,
            amphipods_per_colour: number as u8,
        })
    }
}

/**
This is the burrow:
```
   0123456789a -> x
################
 0#...........#
 1###B#C#B#D###
 2  #D#C#B#A#
 3  #D#B#A#C#
 4  #A#D#C#A#
 5  #########
```
*/
#[cfg(test)]
mod tests {
    use super::*;
    fn get_example1_burrow() -> Burrow {
        Burrow {
            amphipods: vec![
                Burrow::get_position(2, 2),
                Burrow::get_position(8, 2),
                Burrow::get_position(2, 1),
                Burrow::get_position(6, 1),
                Burrow::get_position(4, 1),
                Burrow::get_position(6, 2),
                Burrow::get_position(8, 1),
                Burrow::get_position(4, 2),
            ],
            total_cost: 0,
            amphipods_per_colour: 2,
        }
    }

    #[test]
    fn it_solves_example1() {
        let mut burrow = get_example1_burrow();
        assert_eq!(12521, burrow.solve());
    }

    #[test]
    fn it_parses_input() {
        let input = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";
        let burrow: Result<Burrow, _> = input.parse();
        assert!(burrow.is_ok());
        let burrow = burrow.unwrap();
        let expected = get_example1_burrow();
        assert_eq!(expected, burrow);
    }

    #[test]
    fn it_converts_position_coordinate() {
        for i in 0..255 {
            let (x, y) = Burrow::get_coordinate(i);
            assert_eq!(i, Burrow::get_position(x, y));
        }
    }

    #[test]
    fn it_solves_example2() {
        let input = "#############
#...........#
###B#C#B#D###
  #D#C#B#A#
  #D#B#A#C#
  #A#D#C#A#
  #########";
        let mut burrow: Burrow = input.parse().unwrap();
        assert_eq!(44169, burrow.solve());
    }
}
