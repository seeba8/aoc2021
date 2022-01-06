use std::{fmt::Display, str::FromStr};

pub fn solve() {
    let input = std::fs::read_to_string(format!(
        "resources/{}.txt",
        module_path!().split_once("::").unwrap().1
    ))
    .unwrap();
    let mut burrow: Burrow = input.parse().unwrap();
    println!("Day 23 part 1: {}", burrow.solve());
}
/**
This is the burrow:
```
#############
#abcdefghijk#
###l#n#p#r###
  #m#o#q#s#
  #########
```
*/
#[derive(Clone, Debug, PartialEq)]
struct Burrow {
    /// Vec: A A B B C C D D
    amphipods: Vec<u8>,
    cost: Vec<usize>,
    total_cost: usize,
}

impl Burrow {
    fn is_finished(&self) -> bool {
        (0..8).all(|i| self.is_in_own_room(i))
    }

    fn is_in_hallway(&self, amphipod_id: usize) -> bool {
        self.amphipods[amphipod_id] < b'l'
    }

    fn is_in_own_room(&self, amphipod_id: usize) -> bool {
        self.get_own_room(amphipod_id)
            .contains(&self.amphipods[amphipod_id])
        /*match amphipod_id {
            0 => (self.amphipods[amphipod_id] == 'l' || self.amphipods[amphipod_id] == 'm'),
            1 => (self.amphipods[amphipod_id] == 'l' || self.amphipods[amphipod_id] == 'm'),
            2 => (self.amphipods[amphipod_id] == 'n' || self.amphipods[amphipod_id] == 'o'),
            3 => (self.amphipods[amphipod_id] == 'n' || self.amphipods[amphipod_id] == 'o'),
            4 => (self.amphipods[amphipod_id] == 'p' || self.amphipods[amphipod_id] == 'q'),
            5 => (self.amphipods[amphipod_id] == 'p' || self.amphipods[amphipod_id] == 'q'),
            6 => (self.amphipods[amphipod_id] == 'r' || self.amphipods[amphipod_id] == 's'),
            7 => (self.amphipods[amphipod_id] == 'r' || self.amphipods[amphipod_id] == 's'),
            _ => panic!()
        }*/
    }

    fn get_own_room(&self, amphipod_id: usize) -> [u8; 2] {
        match amphipod_id {
            0..=1 => [b'l', b'm'],
            2..=3 => [b'n', b'o'],
            4..=5 => [b'p', b'q'],
            6..=7 => [b'r', b's'],
            _ => panic!(),
        }
    }

    fn can_move_to_own_room(&self, amphipod_id: usize) -> bool {
        let own_room = self.get_own_room(amphipod_id);
        if self
            .amphipods
            .iter()
            .enumerate()
            .any(|(amphipod, position)| {
                own_room.contains(position) && !self.is_in_own_room(amphipod)
            })
        {
            return false;
        }
        let path = self.get_path(self.amphipods[amphipod_id], own_room[0]);
        if path
            .iter()
            .any(|p| self.amphipods.contains(p) && self.amphipods[amphipod_id] != *p)
        {
            return false;
        }
        true
    }

    fn can_move_to(&self, amphipod_id: usize, target: u8) -> bool {
        let path = self.get_path(self.amphipods[amphipod_id], target);
        if path
            .iter()
            .any(|p| self.amphipods.contains(p) && self.amphipods[amphipod_id] != *p)
        {
            return false;
        }
        true
    }

    fn solve(&mut self) -> usize {
        //println!("{}", self);
        let mut min_cost = usize::MAX;
        if self.is_finished() {
            //println!("{}", self.total_cost);
            return self.total_cost;
        }
        for amphipod_id in 0..8 {
            if self.is_in_own_room(amphipod_id) {
                if [b'm', b'o', b'q', b's'].contains(&self.amphipods[amphipod_id]) {
                    // already in the lower slot
                    continue;
                }
                let lower_slot = self.amphipods[amphipod_id] + 1;
                let amphipod_in_lower_slot = self
                    .amphipods
                    .iter()
                    .position(|c| *c == lower_slot)
                    .unwrap();
                if amphipod_id / 2 == amphipod_in_lower_slot / 2 {
                    // below us is only our own type, so we can stay. Otherwise, we will have to move
                    continue;
                }
            }
            // check if we are in the lower slot and the upper slot is in use
            if [b'm', b'o', b'q', b's'].contains(&self.amphipods[amphipod_id])
                && self
                    .amphipods
                    .contains(&(self.amphipods[amphipod_id] as u8 - 1))
            {
                continue;
            }
            if self.is_in_hallway(amphipod_id) && !self.can_move_to_own_room(amphipod_id) {
                continue;
            }
            if !self.is_in_own_room(amphipod_id) && self.can_move_to_own_room(amphipod_id) {
                let own_room = self.get_own_room(amphipod_id);
                if self.amphipods.contains(&own_room[1]) {
                    // Lower place already taken
                    if !self.can_move_to(amphipod_id, own_room[0]) {
                        continue;
                    }
                    let mut cloned = self.clone();
                    cloned.amphipods[amphipod_id] = own_room[0];
                    cloned.total_cost += cloned.cost[amphipod_id]
                        * Burrow::get_distance(self.amphipods[amphipod_id], own_room[0]);
                    min_cost = cloned.solve().min(min_cost);
                } else {
                    // Lower place not taken, of course we take the lower place
                    if !self.can_move_to(amphipod_id, own_room[1]) {
                        continue;
                    }
                    let mut cloned = self.clone();
                    cloned.amphipods[amphipod_id] = own_room[1];
                    cloned.total_cost += cloned.cost[amphipod_id]
                        * Burrow::get_distance(self.amphipods[amphipod_id], own_room[1]);
                    min_cost = cloned.solve().min(min_cost);
                }
            } else {
                // In foreign room, cannot move to own room. Must move to hallway. Or in own room, but foreign amphipod below
                for target in [b'a', b'b', b'd', b'f', b'h', b'j', b'k'] {
                    if self.amphipods.contains(&target) {
                        continue;
                    }
                    if !self.can_move_to(amphipod_id, target) {
                        continue;
                    }
                    let mut cloned = self.clone();
                    cloned.amphipods[amphipod_id] = target;
                    cloned.total_cost += cloned.cost[amphipod_id]
                        * Burrow::get_distance(self.amphipods[amphipod_id], target);
                    min_cost = cloned.solve().min(min_cost);
                }
            }
        }
        min_cost
    }
    /**
    This is the burrow:
    ```
    #############
    #abcdefghijk#
    ###l#n#p#r###
      #m#o#q#s#
      #########
    ```
    */
    fn get_distance(mut a: u8, mut b: u8) -> usize {
        // pretend we are moving to the hallway, noop if already there
        let mut sum = 0;
        let (a_hallway, a_distance_to_hallway) = match a {
            b'l' | b'm' => (b'c', 1 + (a - b'l')),
            b'n' | b'o' => (b'e', 1 + (a - b'n')),
            b'p' | b'q' => (b'g', 1 + (a - b'p')),
            b'r' | b's' => (b'i', 1 + (a - b'r')),
            _ => (a, 0),
        };
        sum += a_distance_to_hallway as usize;
        a = a_hallway;

        let (b_hallway, b_distance_to_hallway) = match b {
            b'l' | b'm' => (b'c', 1 + (b - b'l')),
            b'n' | b'o' => (b'e', 1 + (b - b'n')),
            b'p' | b'q' => (b'g', 1 + (b - b'p')),
            b'r' | b's' => (b'i', 1 + (b - b'r')),
            _ => (b, 0),
        };
        sum += b_distance_to_hallway as usize;
        b = b_hallway;
        sum += (a as isize - b as isize).abs() as usize;
        sum
    }

    fn get_path(&self, mut from: u8, mut to: u8) -> Vec<u8> {
        let mut path = Vec::new();
        // pretend we are moving to the hallway, noop if already there
        match from {
            b'l' => {
                from = b'c';
            }
            b'm' => {
                path.push(b'l');
                from = b'c';
            }
            b'n' => {
                from = b'e';
            }
            b'o' => {
                path.push(b'n');
                from = b'e';
            }
            b'p' => {
                from = b'g';
            }
            b'q' => {
                path.push(b'p');
                from = b'g';
            }
            b'r' => {
                from = b'i';
            }
            b's' => {
                path.push(b'r');
                from = b'i';
            }
            _ => {}
        };
        match to {
            b'l' => {
                to = b'c';
            }
            b'm' => {
                path.push(b'l');
                to = b'c';
            }
            b'n' => {
                to = b'e';
            }
            b'o' => {
                path.push(b'n');
                to = b'e';
            }
            b'p' => {
                to = b'g';
            }
            b'q' => {
                path.push(b'p');
                to = b'g';
            }
            b'r' => {
                to = b'i';
            }
            b's' => {
                path.push(b'r');
                to = b'i';
            }
            _ => {}
        };
        for i in from.min(to)..=from.max(to) {
            path.push(i);
        }
        path
    }
}

impl Display for Burrow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#############")?;
        write!(f, "#")?;
        for i in b'a'..=b'k' {
            match self.amphipods.iter().position(|c| *c == i) {
                Some(amphipod) => write!(f, "{}", (b'A' + (amphipod / 2) as u8) as char),
                None => write!(f, "."),
            }?;
        }
        writeln!(f, "#")?;
        write!(f, "###")?;
        match self.amphipods.iter().position(|c| *c == b'l') {
            Some(amphipod) => write!(f, "{}", (b'A' + (amphipod / 2) as u8) as char),
            None => write!(f, "."),
        }?;
        write!(f, "#")?;
        match self.amphipods.iter().position(|c| *c == b'n') {
            Some(amphipod) => write!(f, "{}", (b'A'+ (amphipod / 2) as u8) as char),
            None => write!(f, "."),
        }?;
        write!(f, "#")?;
        match self.amphipods.iter().position(|c| *c == b'p') {
            Some(amphipod) => write!(f, "{}", (b'A' + (amphipod / 2) as u8) as char),
            None => write!(f, "."),
        }?;
        write!(f, "#")?;
        match self.amphipods.iter().position(|c| *c == b'r') {
            Some(amphipod) => write!(f, "{}", (b'A' + (amphipod / 2) as u8) as char),
            None => write!(f, "."),
        }?;
        writeln!(f, "###")?;

        write!(f, "  #")?;
        match self.amphipods.iter().position(|c| *c == b'm') {
            Some(amphipod) => write!(f, "{}", (b'A' + (amphipod / 2) as u8) as char),
            None => write!(f, "."),
        }?;
        write!(f, "#")?;
        match self.amphipods.iter().position(|c| *c == b'o') {
            Some(amphipod) => write!(f, "{}", (b'A' + (amphipod / 2) as u8) as char),
            None => write!(f, "."),
        }?;
        write!(f, "#")?;
        match self.amphipods.iter().position(|c| *c == b'q') {
            Some(amphipod) => write!(f, "{}", (b'A' + (amphipod / 2) as u8) as char),
            None => write!(f, "."),
        }?;
        write!(f, "#")?;
        match self.amphipods.iter().position(|c| *c == b's') {
            Some(amphipod) => write!(f, "{}", (b'A' + (amphipod / 2) as u8) as char),
            None => write!(f, "."),
        }?;
        writeln!(f, "#")?;
        writeln!(f, "  #########")
    }
}

impl FromStr for Burrow {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.trim().lines();
        let third_line = lines.nth(2).ok_or("cannot get third line")?;
        let mut amphipods = vec![0; 8];
        for (position, c) in third_line.chars().enumerate() {
            let position = match position {
                3 => b'l',
                5 => b'n',
                7 => b'p',
                9 => b'r',
                _ => 0,
            };
            match c {
                'A' => {
                    if amphipods[0] == 0 {
                        amphipods[0] = position;
                    } else {
                        amphipods[1] = position;
                    }
                }
                'B' => {
                    if amphipods[2] == 0 {
                        amphipods[2] = position;
                    } else {
                        amphipods[3] = position;
                    }
                }
                'C' => {
                    if amphipods[4] == 0 {
                        amphipods[4] = position;
                    } else {
                        amphipods[5] = position;
                    }
                }
                'D' => {
                    if amphipods[6] == 0 {
                        amphipods[6] = position;
                    } else {
                        amphipods[7] = position;
                    }
                }
                _ => {}
            };
        }
        for (position, c) in lines
            .next()
            .ok_or("cannot get fourth line")?
            .chars()
            .enumerate()
        {
            let position = match position {
                3 => b'm',
                5 => b'o',
                7 => b'q',
                9 => b's',
                _ => 0,
            };
            match c {
                'A' => {
                    if amphipods[0] == 0 {
                        amphipods[0] = position;
                    } else {
                        amphipods[1] = position;
                    }
                }
                'B' => {
                    if amphipods[2] == 0 {
                        amphipods[2] = position;
                    } else {
                        amphipods[3] = position;
                    }
                }
                'C' => {
                    if amphipods[4] == 0 {
                        amphipods[4] = position;
                    } else {
                        amphipods[5] = position;
                    }
                }
                'D' => {
                    if amphipods[6] == 0 {
                        amphipods[6] = position;
                    } else {
                        amphipods[7] = position;
                    }
                }
                _ => {}
            };
        }
        Ok(Burrow {
            amphipods,
            cost: vec![1, 1, 10, 10, 100, 100, 1000, 1000],
            total_cost: 0,
        })
    }
}

/**
This is the burrow:
```
#############
#abcdefghijk#
###l#n#p#r###
  #m#o#q#s#
  #########
```
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_example1() {
        let mut burrow = Burrow {
            //amphipods: vec!['m', 's', 'l', 'p', 'n', 'q', 'o', 'r'],
            amphipods: vec![b'm', b's', b'l', b'p', b'n', b'q', b'o', b'r'],
            cost: vec![1, 1, 10, 10, 100, 100, 1000, 1000],
            total_cost: 0,
        };
        assert_eq!(12521, burrow.solve());
    }

    #[test]
    fn it_parses_and_solves_example1() {
        let input = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";
        let mut burrow: Burrow = input.parse().unwrap();
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
        let expected = Burrow {
            //amphipods: vec!['m', 's', 'l', 'p', 'n', 'q', 'o', 'r'],
            amphipods: vec![b'm', b's', b'l', b'p', b'n', b'q', b'r', b'o'],
            cost: vec![1, 1, 10, 10, 100, 100, 1000, 1000],
            total_cost: 0,
        };
        assert_eq!(expected, burrow);
    }
}
