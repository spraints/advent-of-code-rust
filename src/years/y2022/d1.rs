use std::{collections::BinaryHeap, fmt::Display};

pub fn part1(input: String) -> Box<dyn Display> {
    Box::new(sum_first_n(groups(input), 1))
}

fn groups(input: String) -> BinaryHeap<u32> {
    input.split("\n\n").map(sum_group).collect()
}

fn sum_group(group: &str) -> u32 {
    group
        .lines()
        .map(|s| s.trim().parse::<u32>().unwrap())
        .sum()
}

fn sum_first_n(mut groups: BinaryHeap<u32>, n: usize) -> u32 {
    let mut res = 0;
    for _ in 0..n {
        res += groups.pop().unwrap();
    }
    res
}

pub fn part2(input: String) -> Box<dyn Display> {
    Box::new(sum_first_n(groups(input), 3))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::dotest;

    #[test]
    fn part1_example() {
        dotest(
            24000,
            r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
            part1,
        );
    }

    #[test]
    fn part2_example() {
        dotest(
            45000,
            r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
            part2,
        );
    }
}
