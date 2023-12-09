use std::fmt::Display;

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let mut sum = 0;
    for line in input.lines() {
        let numbers = nums(&line);
        sum += next_number(&numbers, vis, Part::One);
    }
    Box::new(sum)
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    let mut sum = 0;
    for line in input.lines() {
        let numbers = nums(&line);
        sum += next_number(&numbers, vis, Part::Two);
    }
    Box::new(sum)
}

fn next_number(numbers: &Vec<i64>, vis: bool, part: Part) -> i64 {
    let next_num = if numbers.iter().all(|a| *a == 0) {
        0
    } else {
        let diffs: Vec<i64> = numbers.windows(2).map(|x| x[1] - x[0]).collect();

        part.go(&numbers, next_number(&diffs, vis, part))
    };

    if vis {
        println!("{numbers:?} ==> {next_num}");
    }

    next_num
}

#[derive(Clone, Copy)]
enum Part {
    One,
    Two,
}

impl Part {
    fn go(self, nums: &[i64], next_number: i64) -> i64 {
        match self {
            Self::One => *nums.last().unwrap() + next_number,
            Self::Two => *nums.first().unwrap() - next_number,
        }
    }
}

fn nums(line: &str) -> Vec<i64> {
    line.trim()
        .split_whitespace()
        .map(|s| s.parse().expect(&format!("should be a number {s:?}")))
        .collect()
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    crate::test::aoc_test!(part1, TEST_INPUT, 114);
    crate::test::aoc_test!(part2, TEST_INPUT, 2);
}
