use std::io;
use std::cmp::{Ord, Ordering};
use std::str::FromStr;

#[derive(Debug)]
struct Hand {
    cards: [u32; 5],
    bid: u64,
}

impl Hand {
    // returns:
    // 1 for High card
    // 2 for One pair
    // 3 for Two pair
    // 4 for Three of a kind
    // 5 for Full house
    // 6 for Four of a kind
    // 7 for Five of a kind
    fn get_type(&self) -> u8 {
        let mut occurrences : [u32; 13] = [0; 13];
        for card in self.cards {
            occurrences[(card - 1) as usize] += 1;
        }
        if occurrences[0] == 0 {
            // full house without jokers
            for i in 1..13 {
                if occurrences[i] == 3 {
                    for j in 1..13 {
                        if occurrences[j] == 2 {
                            return 5;
                        }
                    }
                }
            }
            // two pair without jokers
            for i in 1..13 {
                if occurrences[i] == 2 {
                    for j in 1..13 {
                        if occurrences[j] == 2 && i != j{
                            if occurrences[0] == 1 {
                                return 5;
                            }
                            return 3;
                        }
                    }
                }
            }
        }
        // two pair without jokers plus one joker makes a full house
        if occurrences[0] == 1 {
            for i in 1..13 {
                if occurrences[i] == 2 {
                    for j in 1..13 {
                        if occurrences[j] == 2 && i != j {
                            return 5;
                        }
                    }
                }
            }
        }

        // otherwise we just take the remaining n-of-a-kind including jokers
        let n = occurrences[1..].iter().max().unwrap() + occurrences[0];
        match n {
            1 => { return 1; },
            2 => { return 2; },
            3 => { return 4; },
            4 => { return 6; },
            5 => { return 7; },
            _ => { panic!(); },
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_type = self.get_type();
        let other_type = other.get_type();
        let type_ord = self_type.cmp(&other_type);
        if type_ord != Ordering::Equal {
            return type_ord;
        }
        return self.cards.cmp(&other.cards);
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand { }

#[derive(Debug, PartialEq, Eq)]
struct ParseHandError;

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hand = Hand { cards: [0; 5], bid: 0 };
        let mut split = s.split_whitespace();
        let cards_str = split.next().unwrap();
        hand.bid = u64::from_str_radix(split.next().unwrap(), 10).unwrap();
        for (i, card_char) in cards_str.chars().enumerate() {
            hand.cards[i] = card_char.to_digit(10).unwrap_or_else( { ||
                match card_char {
                    'J' => 1,
                    'T' => 10,
                    'Q' => 11,
                    'K' => 12,
                    'A' => 13,
                    _ => panic!(),
                }
            });
        }
        return Ok(hand);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut hands = vec![];

    for line in stdin.lines() {
        let line_str = line.unwrap();
        let hand: Hand = line_str.parse().unwrap();
        match hands.binary_search(&hand) {
            Ok(i) => { hands.insert(i, hand); }
            Err(i) => { hands.insert(i, hand); }
        }
    }

    let mut winnings = 0u64;
    for (rank, hand) in hands.iter().enumerate() {
        winnings += (rank as u64 + 1) * hand.bid
    }

    println!("{}", winnings);
}
