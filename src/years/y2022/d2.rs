use std::fmt::Display;

pub fn part1(input: String) -> Box<dyn Display> {
    let mut score: u64 = 0;
    for line in input.lines() {
        let (opp, me) = line.split_once(' ').unwrap();
        score += match (opp, me) {
            ("A", "X") => 1 + 3,
            ("B", "X") => 1, // + 0,
            ("C", "X") => 1 + 6,
            ("A", "Y") => 2 + 6,
            ("B", "Y") => 2 + 3,
            ("C", "Y") => 2, // + 0,
            ("A", "Z") => 3, // + 0,
            ("B", "Z") => 3 + 6,
            ("C", "Z") => 3 + 3,
            _ => panic!("illegal {}", line),
        };
    }
    Box::new(score)
}

pub fn part1_nosplit(input: String) -> Box<dyn Display> {
    let mut score: u64 = 0;
    for line in input.lines() {
        score += match line.trim() {
            "A X" => 1 + 3,
            "B X" => 1, // + 0,
            "C X" => 1 + 6,
            "A Y" => 2 + 6,
            "B Y" => 2 + 3,
            "C Y" => 2, // + 0,
            "A Z" => 3, // + 0,
            "B Z" => 3 + 6,
            "C Z" => 3 + 3,
            _ => panic!("illegal {}", line),
        };
    }
    Box::new(score)
}

pub fn part2(input: String) -> Box<dyn Display> {
    let mut score: u64 = 0;
    for line in input.lines() {
        let (opp, me) = line.split_once(' ').unwrap();
        score += match (opp, me) {
            // A = opp Rock
            // B = opp Paper
            // C = opp Scissors
            // X = Lose
            // Y = Draw
            // Z = Win
            ("A", "X") => 3, // + 0,
            ("B", "X") => 1, // + 0,
            ("C", "X") => 2, // + 0,
            ("A", "Y") => 1 + 3,
            ("B", "Y") => 2 + 3,
            ("C", "Y") => 3 + 3,
            ("A", "Z") => 2 + 6,
            ("B", "Z") => 3 + 6,
            ("C", "Z") => 1 + 6,
            _ => panic!("illegal {}", line),
        };
    }
    Box::new(score)
}

pub fn part1alt(input: String) -> Box<dyn Display> {
    Box::new(input.lines().map(score_line1).sum::<u32>())
}

fn score_line1(line: &str) -> u32 {
    let (opp, me) = parse_line(line);
    let res = (me + 4 - opp) % 3;
    me + 1 + res * 3
}

pub fn part2alt(input: String) -> Box<dyn Display> {
    Box::new(input.lines().map(score_line2).sum::<u32>())
}

fn score_line2(line: &str) -> u32 {
    let (opp, res) = parse_line(line);
    let me = (res + 2 + opp) % 3;
    me + 1 + res * 3
}

fn parse_line(line: &str) -> (u32, u32) {
    const A: u32 = 'A' as u32;
    const X: u32 = 'X' as u32;
    let mut c = line.chars();
    let opp = c.next().unwrap() as u32 - A;
    let me = c.skip(1).next().unwrap() as u32 - X;
    (opp, me)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::*;

    const EX: &str = r"A Y
B X
C Z";

    #[test]
    fn part1_example() {
        dotest(15, EX, part1);
        dotest(15, EX, part1alt);
        dotest(15, EX, part1_nosplit);
    }

    #[test]
    fn part2_example() {
        dotest(12, EX, part2);
        dotest(12, EX, part2alt);
    }
}
