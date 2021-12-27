use std::{fmt::Display, str::FromStr};

use bitvec::prelude::*;

pub fn solve() {
    let input = std::fs::read_to_string(format!(
        "resources/{}.txt",
        module_path!().split_once("::").unwrap().1
    ))
    .unwrap();
    let mut img_enhancer = ImageEnhancer::from_str(&input).unwrap().with_iterations(2);
    println!("Day 20 part 1: {}", img_enhancer.do_ticks());
    let mut img_enhancer = ImageEnhancer::from_str(&input).unwrap().with_iterations(50);
    println!("Day 20 part 2: {}", img_enhancer.do_ticks());
}

#[derive(Debug)]
struct ImageEnhancer {
    algorithm: BitVec,
    iterations: usize,
    image: BitVec,
    buf: BitVec,
    width: usize,
    height: usize,
    iteration: usize,
}

impl ImageEnhancer {
    fn new(
        algorithm: &BitSlice,
        iterations: usize,
        image: &BitSlice,
        width: usize,
    ) -> ImageEnhancer {
        let new_width = width + 2 * iterations;
        let new_height = image.len() / width + 2 * iterations;
        let mut new_image = BitVec::with_capacity(new_width * new_height);
        new_image.extend(bitvec![0; iterations * new_width]);
        for (i, bit) in image.iter().enumerate() {
            if i % width == 0 {
                for _ in 0..iterations {
                    new_image.push(false);
                }
            }
            new_image.push(*bit);
            if i % width == width - 1 {
                for _ in 0..iterations {
                    new_image.push(false);
                }
            }
        }
        new_image.extend(bitvec![0; iterations * new_width]);
        let height = new_image.len() / new_width;
        ImageEnhancer {
            algorithm: algorithm.to_bitvec(),
            iterations,
            image: new_image.clone(),
            buf: new_image,
            width: new_width,
            height,
            iteration: 0,
        }
    }

    fn with_iterations(self, iterations: usize) -> ImageEnhancer {
        ImageEnhancer::new(&self.algorithm, iterations, &self.image, self.width)
    }

    fn do_ticks(&mut self) -> usize {
        for _ in 0..self.iterations {
            self.tick();
        }
        self.image.count_ones()
    }

    fn tick(&mut self) {
        for i in 0..self.image.len() {
            let v = self.get_hash(i);
            self.buf.set(i, self.algorithm[v]);
        }
        //self.image = self.buf.clone();
        std::mem::swap(&mut self.image, &mut self.buf);
        self.iteration += 1;
    }

    fn get_hash(&self, i: usize) -> usize {
        let x = (i % self.width) as isize;
        let y = (i / self.width) as isize;
        let mut hash: BitVec<Msb0> = BitVec::with_capacity(9);
        for y_offset in 0..3 {
            for x_offset in 0..3 {
                hash.push(self.get_or(x + x_offset - 1, y + y_offset - 1));
            }
        }
        hash.load::<usize>()
    }

    ///
    /// The fallback value will fluctuate between true and false IIF algorithm[0] = 1 and algorithm[511] = 0
    fn get_or(&self, x: isize, y: isize) -> bool {
        if x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
            self.algorithm[0] && self.iteration % 2 != 0
            //false
        } else {
            self.image[(y as usize) * self.width + (x as usize)]
        }
    }
}

impl Display for ImageEnhancer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, bit) in self.image.iter().enumerate() {
            if i > 0 && i % self.width == 0 {
                writeln!(f)?;
            }
            match *bit {
                true => write!(f, "#")?,
                false => write!(f, ".")?,
            };
        }
        writeln!(f)
    }
}

impl FromStr for ImageEnhancer {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (algorithm, image) = s
            .trim()
            .split_once("\n\n")
            .ok_or("cannot parse input: no empty line between algorithm and image")?;
        let algorithm: BitVec = algorithm
            .trim()
            .chars()
            .filter_map(|c| match c {
                '.' => Some(false),
                '#' => Some(true),
                _ => None,
            })
            .collect();
        let width = image
            .lines()
            .next()
            .ok_or("cannot parse first line of image")?
            .trim()
            .len();
        let image: BitVec = image
            .trim()
            .chars()
            .filter_map(|c| match c {
                '.' => Some(false),
                '#' => Some(true),
                _ => None,
            })
            .collect();
        Ok(ImageEnhancer::new(&algorithm, 0, &image, width))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example() -> &'static str {
        "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"
    }

    #[test]
    fn it_gets_image_size_correct() {
        let img_enhancer = ImageEnhancer::from_str(get_example());
        assert!(img_enhancer.is_ok(), "{}", img_enhancer.unwrap_err());
        let img_enhancer = img_enhancer.unwrap();
        assert_eq!(512, img_enhancer.algorithm.len());
        assert_eq!(25, img_enhancer.image.len());
    }

    #[test]
    fn it_prints_image() {
        let img_enhancer = ImageEnhancer::from_str(get_example()).unwrap();
        println!("{}", img_enhancer);
        assert_eq!(
            "#..#.
#....
##..#
..#..
..###
",
            format!("{}", img_enhancer)
        );
    }

    #[test]
    fn it_resizes() {
        let img_enhancer = ImageEnhancer::from_str(get_example());
        let img_enhancer = img_enhancer.unwrap().with_iterations(2);
        assert_eq!(9, img_enhancer.width);
        println!("{}", img_enhancer);
    }

    #[test]
    fn it_gets_hash() {
        let img_enhancer = ImageEnhancer::from_str(get_example());
        let img_enhancer = img_enhancer.unwrap().with_iterations(2);
        assert_eq!(34, img_enhancer.get_hash(img_enhancer.image.len() / 2));
    }

    #[test]
    fn it_ticks() {
        let img_enhancer = ImageEnhancer::from_str(get_example());
        let mut img_enhancer = img_enhancer.unwrap().with_iterations(2);
        img_enhancer.tick();
        assert_eq!(
            ".........
..##.##..
.#..#.#..
.##.#..#.
.####..#.
..#..##..
...##..#.
....#.#..
.........
",
            format!("{}", img_enhancer)
        );
    }

    #[test]
    fn it_ticks_twice() {
        let img_enhancer = ImageEnhancer::from_str(get_example());
        let mut img_enhancer = img_enhancer.unwrap().with_iterations(2);
        img_enhancer.do_ticks();
        println!("{}", img_enhancer);
        assert_eq!(
            ".......#.
.#..#.#..
#.#...###
#...##.#.
#.....#.#
.#.#####.
..#.#####
...##.##.
....###..
",
            format!("{}", img_enhancer)
        );
    }

    #[test]
    fn it_counts_ones() {
        let img_enhancer = ImageEnhancer::from_str(get_example());
        let mut img_enhancer = img_enhancer.unwrap().with_iterations(2);
        assert_eq!(35, img_enhancer.do_ticks());
    }

    #[test]
    fn it_converts_bitvec_to_number() {
        let hash: BitVec<Msb0> = vec![true; 9].iter().collect();
        assert_eq!(511, hash.load::<usize>());
        let hash: BitVec<Msb0> = vec![true, false, true, true, true, true, true, true, true]
            .iter()
            .collect();
        assert_eq!(0b101111111, hash.load::<usize>());
    }

    #[test]
    fn it_ticks_50_times() {
        let img_enhancer = ImageEnhancer::from_str(get_example());
        let mut img_enhancer = img_enhancer.unwrap().with_iterations(50);
        //println!("{}", img_enhancer);
        assert_eq!(3351, img_enhancer.do_ticks());
        //println!("{}", img_enhancer);
    }
}
