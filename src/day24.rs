use std::collections::{HashSet, HashMap};
use std::fs::read_to_string;
use std::io::{self, Write};

#[allow(unused_imports)]
use itertools::Itertools;

pub struct Solution {
    lines: Vec<String>,
    black_tiles: HashSet<(i32, i32)>,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day24.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        Self {
            lines,
            black_tiles: HashSet::new(),
        }
    }

    fn part1(&mut self) -> usize {
        for line in &self.lines {
            let directions = parse_directions(line);
            let (x, y) = locate(&directions);
            if self.black_tiles.contains(&(x, y)) {
                self.black_tiles.remove(&(x, y));
            } else {
                self.black_tiles.insert((x, y));
            }
        }

        self.black_tiles.len()
    }

    fn part2(&mut self) -> usize{
        for _ in 0..100 {
            // Count neighboring tiles
            let tiles = self.black_tiles.clone();
            let mut neighbors_counts: HashMap<(i32, i32), i32> = HashMap::from_iter(tiles.iter().map(|&tile| (tile, 0)));
            for (x, y) in &tiles {
                for direction in &[
                    Direction::East,
                    Direction::SouthEast,
                    Direction::SouthWest,
                    Direction::West,
                    Direction::NorthWest,
                    Direction::NorthEast,
                ] {
                    let (dx, dy) = direction.to_delta(*y);
                    let neighbor = (x + dx, y + dy);
                    *neighbors_counts.entry(neighbor).or_insert(0) += 1;
                }
            }

            // Update black tiles
            for (tile, count) in neighbors_counts {
                let black = self.black_tiles.contains(&tile);
                if black && (count == 0 || count > 2) {
                    self.black_tiles.remove(&tile);
                } else if !black && count == 2 {
                    self.black_tiles.insert(tile);
                }
            }
        }
        
        self.black_tiles.len()
    }

    pub fn solve(&mut self) {
        println!("========= DAY 24 ========");
        print!("Solving part 1: ");
        io::stdout().flush().unwrap();

        let start = std::time::Instant::now();
        let part1 = self.part1();
        let part1_time = start.elapsed();
        println!("{:?} (took {:?})", part1, part1_time);

        print!("Solving part 2: ");
        io::stdout().flush().unwrap();
        let start = std::time::Instant::now();
        let part2 = self.part2();
        let part2_time = start.elapsed();
        println!("{:?} (took {:?})", part2, part2_time);
        println!();
    }
}


#[derive(Debug, Clone)]
enum Direction {
    East, 
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl TryFrom<&str> for Direction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "e" => Ok(Direction::East),
            "se" => Ok(Direction::SouthEast),
            "sw" => Ok(Direction::SouthWest),
            "w" => Ok(Direction::West),
            "nw" => Ok(Direction::NorthWest),
            "ne" => Ok(Direction::NorthEast),
            _ => Err(()),
        }
    }
}

impl Direction {
    fn to_delta(&self, cur_y: i32) -> (i32, i32) {
        let a = (cur_y % 2).abs(); // 1 if cur_y is odd, otherwise 0
        let b = 1 - a;
        match self {
            Direction::East => (1, 0),
            Direction::SouthEast => (a, 1),
            Direction::NorthEast => (a, -1),
            Direction::West => (-1, 0),
            Direction::SouthWest => (-b, 1),
            Direction::NorthWest => (-b, -1),
        }
    }
}

fn parse_directions(line: &str) -> Vec<Direction> {
    let mut directions = vec![];
    let mut chars = line.chars();
    let mut current_str = String::new();

    while let Some(c) = chars.next() {
        current_str.push(c);
        if let Some(direction) = Direction::try_from(current_str.as_str()).ok() {
            directions.push(direction);
            current_str.clear();
        }
    }

    directions
}

fn locate(directions: &[Direction]) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;

    for direction in directions {
        let (dx, dy) = direction.to_delta(y);
        x += dx;
        y += dy;
    }

    (x, y)
}