use std::fmt::Display;

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let mut sum = 0;
    for line in input.lines() {
        let numbers = nums(&line);
        sum += next_number(&numbers, vis);
    }
    Box::new(sum)
}

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

fn next_number(numbers: &Vec<i64>, vis: bool) -> i64 {
    let next_num = if numbers.iter().all(|a| *a == 0) {
        0
    } else {
        let diffs: Vec<i64> = numbers.windows(2).map(|x| x[1] - x[0]).collect();

        numbers.last().unwrap() + next_number(&diffs, vis)
    };

    if vis {
        println!("{numbers:?} ==> {next_num}");
    }

    next_num
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
    crate::test::aoc_test!(part2, TEST_INPUT, "todo");
}
