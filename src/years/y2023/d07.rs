use std::{collections::HashMap, fmt::Display};

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let mut hands: Vec<Hand> = input.lines().map(|line| parse_hand(line, vis)).collect();
    hands.sort_by_key(|h| h.score);
    let total: u64 = hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i as u64 + 1) * h.bid)
        .sum();
    Box::new(total)
}

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

struct Hand {
    score: u64,
    bid: u64,
}

fn parse_hand(line: &str, vis: bool) -> Hand {
    let (hand, bid) = line.trim().split_once(' ').unwrap();
    let bid = bid.parse().unwrap();
    let mut score = 0x100000 * hand_type(hand, vis);
    let mut cards = hand.chars();
    score += 0x10000 * card_val(cards.next().unwrap());
    score += 0x1000 * card_val(cards.next().unwrap());
    score += 0x100 * card_val(cards.next().unwrap());
    score += 0x10 * card_val(cards.next().unwrap());
    score += card_val(cards.next().unwrap());
    assert!(cards.next().is_none());
    if vis {
        println!("{hand} => {score:#x}");
    }
    Hand { score, bid }
}

fn hand_type(hand: &str, vis: bool) -> u64 {
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

fn card_val(card: char) -> u64 {
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

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    crate::test::aoc_test!(part1, TEST_INPUT, 6440);
    crate::test::aoc_test!(part2, TEST_INPUT, "todo");
}
