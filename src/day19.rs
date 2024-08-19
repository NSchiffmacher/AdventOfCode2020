use std::fs::read_to_string;
use std::io::{self, Write};
use std::collections::HashMap;

#[allow(unused_imports)]
use itertools::Itertools;

pub struct Solution {
    rules: HashMap<usize, Rule>,
    messages: Vec<String>,
}

impl Solution {
    pub fn init() -> Self {
        let content = read_to_string("inputs/day19.txt").unwrap();

        // Parse 
        let (rules_lines, messages_lines) = content.split("\n\n").collect_tuple().unwrap();
        let rules = parse_rules(&rules_lines.lines().collect_vec());
        let messages = messages_lines.lines().map(|x| x.to_string()).collect_vec();


        Self {
            rules,
            messages,
        }
    }

    fn _check_rule(&self, rule: &Rule, x: &str, i: usize, r: usize) -> Vec<usize> {
        let mut res = vec![];
        match rule {
            Rule::Char(c) => {
                if i < x.len() && x.chars().nth(i).unwrap() == *c {
                    res.push(i + 1);
                }
            },
            Rule::Seq(seq) => {
                if r == seq.len() {
                    res.push(i);
                } else {
                    for j in self._check_rule(self.rules.get(&seq[r]).unwrap(), x, i, 0) {
                        res.extend(self._check_rule(rule, x, j, r + 1));
                    }
                }
            },
            Rule::Or(rules) => {
                for rule in rules {
                    res.extend(self._check_rule(rule, x, i, 0))
                }
            }
        }

        res
    }

    fn check_rule(&self, x: &str) -> bool {
        let initial_rule = self.rules.get(&0).unwrap();
        for i in self._check_rule(initial_rule, x, 0, 0) {
            if i == x.len() {
                return true;
            }
        }
        false
    }

    fn part1(&mut self) -> usize {
        self.messages.iter().filter(|x| self.check_rule(x)).count()
    }

    fn part2(&mut self) -> usize {
        self.rules.insert( 8, Rule::Or(vec![Rule::Seq(vec![42]), Rule::Seq(vec![42, 8])]));
        self.rules.insert(11, Rule::Or(vec![Rule::Seq(vec![42, 31]), Rule::Seq(vec![42, 11, 31])]));
        self.messages.iter().filter(|x| self.check_rule(x)).count()
    }

    pub fn solve(&mut self) {
        println!("========= DAY 19 ========");
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

#[derive(Debug)]
enum Rule {
    Char(char),
    Seq(Vec<usize>), 
    Or(Vec<Rule>),
}

fn parse_rules(lines: &[&str]) -> HashMap<usize, Rule> {
    let mut rules = HashMap::new();

    for rule in lines.iter() {
        // Find the rule ID 
        let (id_str, rule_str) = rule.split(':').collect_tuple().unwrap();
        let id = id_str.parse::<usize>().unwrap();

        // Parse the rule
        if !rule_str.contains('"') {
            // Seq rule
            let mut seq_rules = vec![];
            for seq in rule_str.split('|') {
                let seq = seq.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect_vec();
                seq_rules.push(Rule::Seq(seq));
            }
            rules.insert(id, Rule::Or(seq_rules));

        } else {
            // Char rule
            let c = rule_str
                .trim()
                .strip_prefix('"')
                .unwrap()
                .strip_suffix('"')
                .unwrap()
                .chars()
                .next()
                .unwrap();
            rules.insert(id, Rule::Char(c));
        }
    }

    rules
}

// fn match_rules(message: &str, rules: &HashMap<usize, Rule>) -> bool {

// }