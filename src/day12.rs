use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    str::FromStr,
};

pub fn solve() {
    let input = std::fs::read_to_string(format!(
        "resources/{}.txt",
        module_path!().split_once("::").unwrap().1
    ))
    .unwrap();
    let mut caves: Caves = input.parse().unwrap();
    println!("Day 12 part 1: {}", caves.find_paths_start_to_end().len());
    let mut caves: Caves = input.parse().unwrap();
    println!(
        "Day 12 part 2: {}",
        caves.find_paths_start_to_end_with_extra_time().len()
    );
}

#[derive(PartialEq, Hash, Eq, Clone)]
pub struct Cave {
    name: String,
    is_small: bool,
    remember_visit: bool,
    can_visit_again: bool,
}
impl Cave {
    fn new(name: &str) -> Cave {
        let is_small = name
            .chars()
            .next()
            .map(|c| c.is_lowercase())
            .unwrap_or(false);
        let can_visit_again = !vec!["start", "end"].contains(&name);
        Cave {
            name: name.to_owned(),
            is_small,
            remember_visit: is_small,
            can_visit_again,
        }
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Debug for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cave").field("name", &self.name).finish()
    }
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct Edge(Cave, Cave);

impl Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.0, self.1)
    }
}

impl FromStr for Edge {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s
            .trim()
            .split_once('-')
            .ok_or_else(|| "No minus in line".to_owned())?;
        Ok(Edge(Cave::new(from), Cave::new(to)))
    }
}

#[derive(Clone)]
pub struct Caves {
    edges: Vec<Edge>,
    visited: HashSet<Cave>,
}

impl Caves {
    fn find_paths_start_to_end(&mut self) -> Vec<Vec<Cave>> {
        self.find_paths(&Cave::new("start"), &Cave::new("end"), false)
    }

    fn find_paths_start_to_end_with_extra_time(&mut self) -> Vec<Vec<Cave>> {
        self.find_paths(&Cave::new("start"), &Cave::new("end"), true)
    }

    fn find_paths(&mut self, from: &Cave, to: &Cave, extra_time: bool) -> Vec<Vec<Cave>> {
        if from == to {
            let mut res = Vec::new();
            res.push(vec![from.clone()]);
            return res;
        }
        if from.remember_visit {
            self.visited.insert(from.clone());
        }
        let mut paths = Vec::new();
        let next_caves: Vec<Cave> = self
            .edges
            .iter()
            .filter(|edge| {
                edge.0 == *from
                    && (!self.visited.contains(&edge.1) || (extra_time && edge.1.can_visit_again))
            })
            .map(|edge| edge.1.clone())
            .collect();
        for next_cave in next_caves {
            let mut cloned_self = self.clone();
            let next_cave_paths = if self.visited.contains(&next_cave) {
                cloned_self.find_paths(&next_cave, to, false)
            } else {
                cloned_self.find_paths(&next_cave, to, extra_time)
            };
            for mut path in next_cave_paths {
                let mut new_path = vec![from.clone()];
                new_path.append(&mut path);
                paths.push(new_path);
            }
        }
        paths
    }
}

impl FromStr for Caves {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut edges: Vec<Edge> = s
            .trim()
            .lines()
            .map(|line| Edge::from_str(line))
            .collect::<Result<Vec<Edge>, String>>()?;
        // we want bi-directional edges
        for edge in edges.clone() {
            edges.push(Edge(edge.1, edge.0));
        }
        Ok(Caves {
            edges,
            visited: HashSet::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_equals() {
        let cave1 = Cave::new("a");
        let cave2 = Cave::new("a");
        let cave3 = Cave::new("A");
        assert_eq!(cave1, cave2);
        assert_ne!(cave1, cave3);
    }

    #[test]
    fn it_parses_caves() {
        let input = "start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end";
        let caves: Result<Caves, _> = input.parse();
        assert!(caves.is_ok());
        let caves = caves.unwrap();
        assert_eq!(14, caves.edges.len());
    }

    #[test]
    fn it_finds_paths() {
        let input = "start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end";
        let mut caves: Caves = input.parse().unwrap();
        let paths = caves.find_paths_start_to_end();
        assert_eq!(10, paths.len());

        let mut caves: Caves = "dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc"
            .parse()
            .unwrap();
        assert_eq!(19, caves.find_paths_start_to_end().len());

        let mut caves: Caves = "fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW"
            .parse()
            .unwrap();
        assert_eq!(226, caves.find_paths_start_to_end().len());
    }

    #[test]
    fn it_finds_paths_with_extra_time() {
        let input = "start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end";
        let mut caves: Caves = input.parse().unwrap();
        let paths = caves.find_paths_start_to_end_with_extra_time();
        assert_eq!(36, paths.len());

        let mut caves: Caves = "dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc"
            .parse()
            .unwrap();
        assert_eq!(103, caves.find_paths_start_to_end_with_extra_time().len());

        let mut caves: Caves = "fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW"
            .parse()
            .unwrap();
        assert_eq!(3509, caves.find_paths_start_to_end_with_extra_time().len());
    }
}
