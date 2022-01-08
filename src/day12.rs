use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
    hash::Hash,
    str::FromStr,
};

pub fn solve() {
    let input = std::fs::read_to_string(format!(
        "resources/{}.txt",
        module_path!().split_once("::").unwrap().1
    ))
    .unwrap();
    let mut caves: Caves = input.parse().unwrap();
    let neighbours = Caves::get_neighbours(&input);
    println!("Day 12 part 1: {}", caves.find_paths_start_to_end(&neighbours));
    let mut caves: Caves = input.parse().unwrap();
    println!(
        "Day 12 part 2: {}",
        caves.find_paths_start_to_end_with_extra_time(&neighbours)
    );
}

#[derive(Clone)]
pub struct Cave {
    name: String,
    //is_small: bool,
    remember_visit: bool,
    can_visit_again: bool,
    //visited: bool,
}

impl Hash for Cave {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Eq for Cave {}

impl PartialEq for Cave {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
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
            //is_small,
            remember_visit: is_small,
            can_visit_again,
            //visited: false,
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
    visited: HashSet< Cave>,
}

impl Caves {

    fn get_neighbours(s: &str) ->  HashMap<Cave, HashSet<Cave>> {
        let mut edges: Vec<Edge> = s
            .trim()
            .lines()
            .map(Edge::from_str)
            .collect::<Result<Vec<Edge>, String>>().unwrap();
        // we want bi-directional edges
        for edge in edges.clone() {
            edges.push(Edge(edge.1, edge.0));
        }
        let mut neighbours: HashMap<Cave, HashSet<Cave>> = HashMap::new();
        for edge in edges {
            neighbours
                .entry(edge.0.clone())
                .and_modify(|entry| {
                    entry.insert(edge.1.clone());
                })
                .or_insert_with(HashSet::new)
                .insert(edge.1);
        }

        neighbours
    }
    fn find_paths_start_to_end(&mut self, neighbours: &HashMap<Cave, HashSet<Cave>>) -> usize {
        self.find_paths(&Cave::new("start"), &Cave::new("end"), false, neighbours)
    }

    fn find_paths_start_to_end_with_extra_time(&mut self, neighbours: &HashMap<Cave, HashSet<Cave>>) -> usize {
        self.find_paths(&Cave::new("start"), &Cave::new("end"), true, neighbours)
    }

    fn find_paths(&mut self, from: &Cave, to: &Cave, extra_time: bool, neighbours: &HashMap<Cave, HashSet<Cave>>) -> usize {
        if from == to {
            return 1;
        }
        if from.remember_visit {
           self.visited.insert(from.clone());
        }
        let mut paths = 0;
        for next_cave in neighbours[from]
            .iter()
            .filter(|cave| (!self.visited.contains(cave) || (extra_time && cave.can_visit_again)))
        {
            let mut cloned_self = self.clone();
            paths += if self.visited.contains(next_cave) {
                cloned_self.find_paths( next_cave, to, false, neighbours)
            } else {
                cloned_self.find_paths( next_cave, to, extra_time, neighbours)
            };
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
            .map(Edge::from_str)
            .collect::<Result<Vec<Edge>, String>>()?;
        // we want bi-directional edges
        for edge in edges.clone() {
            edges.push(Edge(edge.1, edge.0));
        }
        let mut neighbours: HashMap<Cave, HashSet<Cave>> = HashMap::new();
        for edge in edges {
            neighbours
                .entry(edge.0.clone())
                .and_modify(|entry| {
                    entry.insert(edge.1.clone());
                })
                .or_insert_with(HashSet::new)
                .insert(edge.1);
        }
        Ok(Caves {
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
    fn it_finds_paths() {
        let input = "start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end";
        let mut caves: Caves = input.parse().unwrap();
        let neighbours = Caves::get_neighbours(&input);
        let paths = caves.find_paths_start_to_end(&neighbours);
        assert_eq!(10, paths);

        let input = "dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc";
        let mut caves: Caves = input.parse().unwrap();
        let neighbours = Caves::get_neighbours(&input);
        assert_eq!(19, caves.find_paths_start_to_end(&neighbours));

        let input = "fs-end
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
        start-RW";
        let mut caves: Caves = input
            .parse()
            .unwrap();
        let neighbours = Caves::get_neighbours(&input);
        assert_eq!(226, caves.find_paths_start_to_end(&neighbours));
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
        let neighbours = Caves::get_neighbours(&input);
        let paths = caves.find_paths_start_to_end_with_extra_time(&neighbours);
        assert_eq!(36, paths);

        let input = "dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc";
        let mut caves: Caves = input.parse().unwrap();
        let neighbours = Caves::get_neighbours(&input);
        assert_eq!(103, caves.find_paths_start_to_end_with_extra_time(&neighbours));

        let input = "fs-end
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
        start-RW";
        let mut caves: Caves = input
            .parse()
            .unwrap();
        let neighbours = Caves::get_neighbours(&input);
        assert_eq!(3509, caves.find_paths_start_to_end_with_extra_time(&neighbours));
    }
}
