use std::fs::read_to_string;
use std::io::{self, Write};
use std::collections::HashMap;

#[allow(unused_imports)]
use itertools::Itertools;

pub struct Solution {
    lines: Vec<String>,
    map: Map,
}

#[derive(PartialEq, Clone)]
enum Tile {
    Empty,
    Seat,
    OccupiedSeat,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'L' => Tile::Seat,
            '#' => Tile::OccupiedSeat,
            '.' => Tile::Empty,
            _ => unreachable!(),
        }
    }
}

impl Tile {
    fn as_char(&self) -> char {
        match self {
            Tile::Seat => 'L',
            Tile::OccupiedSeat => '#',
            Tile::Empty => '.',
        }
    }
}

#[derive(Clone)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    seat_positions: Vec<(i32, i32)>,
    occupied_seat_map: HashMap<((i32, i32), (i32, i32)), (i32, i32)>,

    width: i32,
    height: i32,
}

impl From<&Vec<String>> for Map {
    fn from(value: &Vec<String>) -> Self {
        let mut seat_positions = vec![];
        let mut tiles = vec![];

        for (y, line) in value.iter().enumerate() {
            let mut cur = vec![];
            for (x, c) in line.chars().enumerate() {
                let v = Tile::from(c);
                if v != Tile::Empty {
                    seat_positions.push((x as i32, y as i32));
                }
                cur.push(v);
            }
            tiles.push(cur);
        }

        let height = tiles.len() as i32;
        let width = tiles[0].len() as i32;

        // Part 2 map 
        let mut occupied_seat_map: HashMap<((i32, i32), (i32, i32)), (i32, i32)> = HashMap::new();
        for (x, y) in &seat_positions {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if (dx, dy) == (0, 0) {
                        continue;
                    }

                    let (mut nx, mut ny) = (x + dx, y + dy);
                    while nx >= 0 && nx < width && ny >= 0 && ny < height {
                        if tiles[ny as usize][nx as usize] != Tile::Empty {
                            occupied_seat_map.insert(((*x, *y), (dx, dy)), (nx, ny));
                            break;
                        }

                        nx += dx;
                        ny += dy;
                    }
                }
            }
        }

        Self {
            height,
            width,

            tiles,
            seat_positions,
            occupied_seat_map,
        }
    }
}

impl std::cmp::PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        self.tiles == other.tiles
    }
}

impl Map {
    fn print(&self) {
        for line in &self.tiles {
            for tile in line {
                print!("{}", tile.as_char());
            }
            println!();
        }
    }

    fn count_occupied_adjacent(&self, (x, y): (i32, i32)) -> usize {
        let mut res = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                let (nx, ny) = (x + dx, y + dy);

                if (dx, dy) != (0, 0) && nx >= 0 && nx < self.width && ny >= 0 && ny < self.height && self.get((nx, ny)) == Tile::OccupiedSeat {
                    res += 1;
                }
            }
        }

        res
    }

    fn count_occupied_distant(&self, (x, y): (i32, i32)) -> usize {
        let mut res = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if (dx, dy) == (0, 0) {
                    continue;
                }

                if let Some(distant_seat) = self.occupied_seat_map.get(&((x, y), (dx, dy))) {
                    if self.get(*distant_seat) == Tile::OccupiedSeat {
                        res += 1;
                    }
                }
            }
        }

        res
    }

    fn get(&self, (x, y): (i32, i32)) -> Tile {
        self.tiles[y as usize][x as usize].clone()
    }

    fn set(&mut self, (x, y): (i32, i32), value: Tile) {
        self.tiles[y as usize][x as usize] = value;
    }

    fn evolve(&self, first_part: bool) -> Self {
        let mut new_map = self.clone();
        let occupied_threshold = if first_part { 4 } else { 5 };

        for seat in &self.seat_positions {
            let count = if first_part {
                self.count_occupied_adjacent(*seat)
            } else {
                self.count_occupied_distant(*seat)
            };

            if self.get(*seat) == Tile::Seat && count == 0 {
                new_map.set(*seat, Tile::OccupiedSeat);
            } else if self.get(*seat) == Tile::OccupiedSeat && count >= occupied_threshold {
                new_map.set(*seat, Tile::Seat);
            }
        }

        new_map
    }

    fn count_all_occupied(&self) -> i32 {
        let mut res = 0;
        for seat in &self.seat_positions {
            if self.get(*seat) == Tile::OccupiedSeat {
                res += 1;
            }
        }

        res
    }

}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day11.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        Self {
            map: Map::from(&lines),
            lines,
        }
    }

    fn part1(&mut self) -> i32 {
        let mut map = self.map.clone();
        loop {
            let new_map = map.evolve(true);
            if new_map == map {
                break;
            }
            map = new_map;
        }

        map.count_all_occupied()
    }

    fn part2(&mut self) -> i32 {
        let mut map = self.map.clone();

        loop {
            let new_map = map.evolve(false);
            if new_map == map {
                break;
            }
            map = new_map;
        }

        map.count_all_occupied()
    }

    pub fn solve(&mut self) {
        println!("========= DAY 11 ========");
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