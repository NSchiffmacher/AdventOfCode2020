use std::fmt::Debug;
use std::fs::read_to_string;
use std::io::{self, Write};
use std::collections::HashMap;

#[allow(unused_imports)]
use itertools::Itertools;

pub struct Solution {
    lines: Vec<String>,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day23.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        Self {
            lines,
        }
    }

    fn part1(&mut self) -> Cups {
        let values = self.lines[0].chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<_>>();
        let mut cups = Cups::from(values);
        
        for _ in 0..10 {
            cups.play_move();
        }

        cups
    }

    fn part2(&mut self) -> usize {
        let mut values = self.lines[0].chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<_>>();
        values.reserve(1_000_000);
        for i in 10..=1_000_000 {
            values.push(i);
        }
        let mut cups = Cups::from(values);

        for _ in 0..10_000_000 {
            cups.play_move();
        }

        let a = cups.cups[&1];
        let b = cups.cups[&a];
        a * b
    }

    pub fn solve(&mut self) {
        println!("========= DAY 23 ========");
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

type T = usize;
struct Cups {
    cups: HashMap<T, T>, 
    max: T,
    current: T,
}

impl Cups {
    fn from(values: Vec<T>) -> Self {
        let mut cups = HashMap::new();
        for i in 0..values.len() - 1 {
            cups.insert(values[i], values[i + 1]);
        }
        cups.insert(*values.last().unwrap(), *values.first().unwrap());

        Self {
            cups,
            current: values[0],
            max: *values.iter().max().unwrap(),
        }
    }

    fn play_move(&mut self) {
        // 1) Step one, pick up three cups
        let current = self.current;
        let mut picked_up = Vec::new();
        picked_up.push(self.cups[&current]);
        picked_up.push(self.cups[&picked_up[0]]);
        picked_up.push(self.cups[&picked_up[1]]);
        let next_current = self.cups[&picked_up[2]];

        let mut destination = current;
        loop {
            destination = if destination == 1 {
                self.max
            } else {
                destination - 1
            };

            if !picked_up.contains(&destination) {
                break;
            }
        }

        // 3) Step three, place cups
        self.cups.insert(picked_up[2], self.cups[&destination]);
        self.cups.insert(destination, picked_up[0]);

        // 4) Step four, select new current cup
        self.cups.insert(current, next_current);
        
        self.current = next_current;
    }
}

impl Debug for Cups {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Print the first 10 cups
        let mut current = self.cups[&1];
        for _ in 0..8 {
            write!(f, "{}", current)?;
            current = self.cups[&current];
        }

        Ok(())
    }
}
