use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let mut score = 0;
    for line in input.lines() {
        let (_, vals) = line.split_once(':').unwrap();
        let (winning, mine) = vals.split_once('|').unwrap();
        if vis {
            println!("winning={winning:?} mine={mine:?}");
        }
        let winning: HashSet<u32> = winning
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        let mine: HashSet<u32> = mine
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        let n = winning.intersection(&mine).count() as u32;
        if n > 0 {
            score += 2_u32.pow(n - 1);
        }
    }
    Box::new(score)
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    let mut score = 0;
    let mut wins = VecDeque::new();
    for line in input.lines() {
        let copies = 1 + wins.pop_front().unwrap_or(0);
        if vis {
            println!("{copies} copies of {line:?}");
        }
        score += copies;

        let (_, vals) = line.split_once(':').unwrap();
        let (winning, mine) = vals.split_once('|').unwrap();
        let winning: HashSet<u32> = winning
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        let mine: HashSet<u32> = mine
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        let n = winning.intersection(&mine).count();
        if n > 0 {
            if vis {
                println!("--> {copies} more copies of the next {n} cards!");
            }
            for i in 0..n {
                if wins.len() > i {
                    wins[i] += copies;
                } else {
                    wins.push_back(copies);
                }
            }
        }
    }
    Box::new(score)
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    crate::test::aoc_test!(part1, TEST_INPUT, 13);
    crate::test::aoc_test!(part2, TEST_INPUT, 30);
}
