use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::{self, Write};

#[allow(unused_imports)]
use itertools::Itertools;

pub struct Solution {
    lines: Vec<String>,
    num: Vec<i32>,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day10.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        Self {
            num: lines.iter().map(|x| x.parse().unwrap()).into_iter().sorted().collect(),
            lines,
        }
    }

    fn part1(&mut self) -> i32 {
        let mut last = 0;
        let mut differences = HashMap::new();
        differences.insert(3, 1);

        for value in &self.num {
            let diff = value - last;
            *differences.entry(diff).or_insert(0) += 1;
            last = *value;
        }

        *differences.get(&1).unwrap_or(&0) * *differences.get(&3).unwrap_or(&0)
    }

    fn part2(&mut self) -> i64 {
        let last = self.num.last().unwrap() + 3;

        self.num.push(last);
        
        let mut dp: HashMap<i32, i64> = HashMap::from_iter(self.num.iter().map(|&x| (x, 0)));
        dp.insert(0, 1);

        for current in &self.num {
            for possible_origin in *current-3..*current {
                *dp.get_mut(current).unwrap() += *dp.get(&possible_origin).unwrap_or(&0);
            }
        }

        *dp.get(&last).unwrap()
    }

    pub fn solve(&mut self) {
        println!("========= DAY 10 ========");
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