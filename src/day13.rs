use std::fs::read_to_string;
use std::io::{self, Write};

#[allow(unused_imports)]
use itertools::Itertools;

pub struct Solution {
    lines: Vec<String>,

    earliest_bus: i64,
    valid_bus_ids: Vec<i64>,
    valid_bus_ids_with_indices: Vec<(i64, i64)>,
}

fn pos_mod(mut a: i64, b: i64) -> i64 {
    while a < 0 {
        a = a + b;
    }

    a %  b
}

fn inv_mod(a: i64, n: i64) -> i64 {
    let mut t = 0;
    let mut newt = 1;
    let mut r = n;
    let mut newr = a;

    while newr != 0 {
        let quotient = r / newr;
        (t, newt) = (newt, t - quotient * newt);
        (r, newr) = (newr, r - quotient * newr);
    }

    if r > 1 {
        panic!("a = {} is not invertible mod {}", a, n);
    } else if t < 0 {
        t = t + n;
    }

    t
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day13.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        let earliest_bus: i64 = lines[0].parse().unwrap();
        let valid_bus_ids: Vec<i64> = lines[1].split(',').filter_map(|s| s.parse().ok()).collect();
        let valid_bus_ids_with_indices: Vec<(i64, i64)> = lines[1].split(',')
                                                                  .enumerate()
                                                                  .map(|(i, s)| (i as i64, s.parse().unwrap_or(0)))
                                                                  .filter(|(_a, b)| b != &0)
                                                                  .map(|(a, b)| (pos_mod(b-a, b) , b))
                                                                  .sorted_by_key(|(_a, b)| *b)
                                                                  .rev()
                                                                  .collect();
        Self {
            lines,

            earliest_bus,
            valid_bus_ids,
            valid_bus_ids_with_indices,
        }
    }

    fn part1(&mut self) -> i64 {
        let mut closest_bus = 0;
        let mut wait_time = i64::MAX;

        for bus in &self.valid_bus_ids {
            let bus_time = *bus * (self.earliest_bus / *bus + 1);
            // let bus_time = (self.earliest_bus as f64 / *bus as f64).ceil() as i64 * bus;
            if bus_time - self.earliest_bus < wait_time {
                wait_time = bus_time - self.earliest_bus;
                closest_bus = *bus;
            }
        }

        closest_bus * wait_time
    }

    fn part2(&mut self) -> i64 {
        let a: Vec<_> = self.valid_bus_ids_with_indices.iter().map(|(a, _n)| *a as i64).collect();
        let n: Vec<_> = self.valid_bus_ids_with_indices.iter().map(|(_a, n)| *n as i64).collect();
        let n_mult = n.iter().fold(1, |acc, v| acc * v);

        // Version 1
        // let mut base = a[0];
        // let mut incr = n[0];
        // for i in 1..a.len() {
        //     let mut candidate = base;
        //     while candidate < n_mult {
        //         if candidate % n[i] == a[i] {
        //             base = candidate;
        //             incr *= n[i];
        //             break;
        //         }
        //         candidate += incr;
        //     }
        // }
        
        // base

        // Version 2
        let mut x = 0;
        for i in 0..a.len() {
            let ai = a[i];
            let ni = n[i];
            let yi = n_mult / ni;

            // Find zi such that zi = yi^-1 mod ni <> zi yi = 1 mod ni
            let zi = inv_mod(yi, ni);
            x += ai * yi * zi;
        }

        x % n_mult
    }

    pub fn solve(&mut self) {
        println!("========= DAY 13 ========");
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