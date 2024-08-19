use std::fs::read_to_string;
use std::io::{self, Write};
use std::collections::HashSet;

#[allow(unused_imports)]
use itertools::Itertools;

pub struct Solution {
    contents: String,
}

impl Solution {
    pub fn init() -> Self {

        Self {
            contents: read_to_string("inputs/day22.txt").unwrap(),
        }
    }

    fn part1(&mut self) -> Card {
        let (player1_str, player2_str) = self.contents.split_once("\r\n\r\n").unwrap();
        let mut player1 = Deck::parse(player1_str);
        let mut player2 = Deck::parse(player2_str);
        
        while !player1.has_lost() && !player2.has_lost() {
            let card1 = player1.draw();
            let card2 = player2.draw();
            if card1 > card2 {
                player1.append((card1, card2));
            } else if card2 > card1 {
                player2.append((card2, card1));
            } else {
                panic!("Equal cards ??");
            }
        }

        player1.score() + player2.score()
    }

    fn part2(&mut self) -> Card {
        let (player1_str, player2_str) = self.contents.split_once("\r\n\r\n").unwrap();
        let player1 = Deck::parse(player1_str);
        let player2 = Deck::parse(player2_str);
        let winning_decks = self.play_game((player1, player2), &mut HashSet::new());

        winning_decks.0.score() + winning_decks.1.score()
    }

    fn play_game(&mut self, mut players: (Deck, Deck), seen_states: &mut HashSet<(Deck, Deck)>) -> (Deck, Deck) {
        while !players.0.has_lost() && !players.1.has_lost() {
            if seen_states.contains(&players) {
                // Game ends in a win for player 1
                // println!("Player 1 wins the game because already seen ??!");
                players.1 = Deck(vec![]);
                return players; 
            }

            seen_states.insert(players.clone());

            // println!("\nPlayer 1's deck: {:?}", players.0);
            // println!("Player 2's deck: {:?}", players.1);

            let player1_card = players.0.draw();
            let player2_card = players.1.draw();
            if players.0.len() >= player1_card && players.1.len() >= player2_card {
                // Play a sub-game
                // println!("Playing a sub-game to determine the winner...");
                let subdecks = (players.0.subdeck(player1_card), players.1.subdeck(player2_card));
                let winning_decks = self.play_game(subdecks, &mut HashSet::new());
                if winning_decks.0.has_lost() {
                    // println!("...Player 2 wins the round!");
                    players.1.append((player2_card, player1_card));
                } else {
                    // println!("...Player 1 wins the round!");
                    players.0.append((player1_card, player2_card));
                }
            } else {
                // The winner of the round is the player with the higher-value card
                // println!("Player 1 plays: {}", player1_card);
                // println!("Player 2 plays: {}", player2_card);

                if player1_card > player2_card {
                    // println!("Player 1 wins the round!");
                    players.0.append((player1_card, player2_card));
                } else {
                    // println!("Player 2 wins the round!");
                    players.1.append((player2_card, player1_card));
                }

            }
        }

        players
    }

    pub fn solve(&mut self) {
        println!("========= DAY 22 ========");
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

type Card = usize;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Deck(Vec<Card>);

impl Deck {
    fn parse(input: &str) -> Self {
        Self(input.lines().skip(1).map(|line| line.parse().unwrap()).collect())
    }

    fn draw(&mut self) -> Card {
        self.0.remove(0)
    }

    fn peek(&self) -> Card {
        self.0[0]
    }

    fn append(&mut self, (card_a, card_b): (Card, Card)) {
        self.0.push(card_a);
        self.0.push(card_b);
    }

    fn has_lost(&self) -> bool {
        self.0.is_empty()
    }

    fn score(&self) -> Card {
        self.0.iter()
              .rev()
              .enumerate()
              .map(|(i, card)| (i+1) * card)
              .sum()
    }

    fn subdeck(&self, size: usize) -> Deck {
        Deck(self.0[0..size].to_vec())
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}