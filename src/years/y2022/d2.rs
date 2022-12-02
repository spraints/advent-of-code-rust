use std::{collections::BinaryHeap, fmt::Display};

pub fn part1(input: String) -> anyhow::Result<Box<dyn Display>> {
    let mut score: u64 = 0;
    for line in input.lines() {
        let opp = line.chars().nth(0).unwrap();
        let me = line.chars().nth(2).unwrap();
        score += match (opp, me) {
            ('A', 'X') => 1 + 3,
            ('B', 'X') => 1 + 0,
            ('C', 'X') => 1 + 6,
            ('A', 'Y') => 2 + 6,
            ('B', 'Y') => 2 + 3,
            ('C', 'Y') => 2 + 0,
            ('A', 'Z') => 3 + 0,
            ('B', 'Z') => 3 + 6,
            ('C', 'Z') => 3 + 3,
            _ => panic!("illegal {}", line),
        };
    }
    Ok(Box::new(score))
}

pub fn part2(input: String) -> anyhow::Result<Box<dyn Display>> {
    let mut score: u64 = 0;
    for line in input.lines() {
        let opp = line.chars().nth(0).unwrap();
        let me = line.chars().nth(2).unwrap();
        score += match (opp, me) {
            // A = opp Rock
            // B = opp Paper
            // C = opp Scissors
            // X = Lose
            // Y = Draw
            // Z = Win
            ('A', 'X') => 3 + 0,
            ('B', 'X') => 1 + 0,
            ('C', 'X') => 2 + 0,
            ('A', 'Y') => 1 + 3,
            ('B', 'Y') => 2 + 3,
            ('C', 'Y') => 3 + 3,
            ('A', 'Z') => 2 + 6,
            ('B', 'Z') => 3 + 6,
            ('C', 'Z') => 1 + 6,
            _ => panic!("illegal {}", line),
        };
    }
    Ok(Box::new(score))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::*;

    #[test]
    fn part1_example() {
        dotest(
            15,
            r"A Y
B X
C Z",
            part1,
        );
    }

    #[test]
    fn part2_example() {
        dotest(
            12,
            r"A Y
B X
C Z",
            part2,
        );
    }
}
