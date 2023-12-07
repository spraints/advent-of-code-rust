use std::{collections::HashMap, fmt::Display};

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| parse_hand(Part1, line, vis))
        .collect();
    hands.sort_by_key(|h| h.score);
    let total: u64 = hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i as u64 + 1) * h.bid)
        .sum();
    Box::new(total)
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| parse_hand(Part2, line, vis))
        .collect();
    hands.sort_by_key(|h| h.score);
    let total: u64 = hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i as u64 + 1) * h.bid)
        .sum();
    Box::new(total)
}

trait Scorer {
    fn hand_type(&self, hand: &str, vis: bool) -> u64;
    fn card_val(&self, card: char) -> u64;
}

struct Hand {
    score: u64,
    bid: u64,
}

fn parse_hand<S: Scorer>(s: S, line: &str, vis: bool) -> Hand {
    let (hand, bid) = line.trim().split_once(' ').unwrap();
    let bid = bid.parse().unwrap();
    let mut score = 0x100000 * s.hand_type(hand, vis);
    let mut cards = hand.chars();
    score += 0x10000 * s.card_val(cards.next().unwrap());
    score += 0x1000 * s.card_val(cards.next().unwrap());
    score += 0x100 * s.card_val(cards.next().unwrap());
    score += 0x10 * s.card_val(cards.next().unwrap());
    score += s.card_val(cards.next().unwrap());
    assert!(cards.next().is_none());
    if vis {
        println!("{hand} => {score:#x}");
    }
    Hand { score, bid }
}

struct Part1;

impl Scorer for Part1 {
    fn hand_type(&self, hand: &str, vis: bool) -> u64 {
        let mut seen = HashMap::new();
        let mut counts = Vec::new();
        for card in hand.chars() {
            let seen_len = seen.len();
            let i = seen.entry(card).or_insert(seen_len);
            while *i >= counts.len() {
                counts.push(0);
            }
            counts[*i] += 1;
        }
        counts.sort();
        if vis {
            println!("{hand} => {counts:?}");
        }
        match counts[..] {
            [5] => 7,             // five of a kind
            [1, 4] => 6,          // four of a kind
            [2, 3] => 5,          // full house
            [1, 1, 3] => 4,       // three of a kind
            [1, 2, 2] => 3,       // two pair
            [1, 1, 1, 2] => 2,    // one pair
            [1, 1, 1, 1, 1] => 1, // high card
            _ => panic!("{counts:?} is weird!"),
        }
    }

    fn card_val(&self, card: char) -> u64 {
        match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            _ => panic!("illegal card {card:?}"),
        }
    }
}

struct Part2;

impl Scorer for Part2 {
    fn hand_type(&self, hand: &str, vis: bool) -> u64 {
        let mut jokers = 0;
        let mut seen = HashMap::new();
        let mut counts = Vec::new();
        for card in hand.chars() {
            if card == 'J' {
                jokers += 1;
            } else {
                let seen_len = seen.len();
                let i = seen.entry(card).or_insert(seen_len);
                while *i >= counts.len() {
                    counts.push(0);
                }
                counts[*i] += 1;
            }
        }
        if counts.is_empty() {
            counts.push(jokers);
        } else {
            counts.sort();
            let last_i = counts.len() - 1;
            counts[last_i] += jokers;
        }
        if vis {
            println!("{hand} => {counts:?}");
        }
        match counts[..] {
            [5] => 7,             // five of a kind
            [1, 4] => 6,          // four of a kind
            [2, 3] => 5,          // full house
            [1, 1, 3] => 4,       // three of a kind
            [1, 2, 2] => 3,       // two pair
            [1, 1, 1, 2] => 2,    // one pair
            [1, 1, 1, 1, 1] => 1, // high card
            _ => panic!("{counts:?} is weird!"),
        }
    }

    fn card_val(&self, card: char) -> u64 {
        match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            'J' => 1,
            _ => panic!("illegal card {card:?}"),
        }
    }
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    crate::test::aoc_test!(part1, TEST_INPUT, 6440);
    crate::test::aoc_test!(part2, TEST_INPUT, 5905);
}
