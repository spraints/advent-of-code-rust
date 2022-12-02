use std::fmt::Display;

#[aoc(year = 2021, day = 1, part = 1)]
pub fn part1(input: String) -> Box<dyn Display> {
    let values: Vec<u32> = input.lines().map(|l| l.trim().parse().unwrap()).collect();
    Box::new(values.windows(2).filter(|x| x[0] < x[1]).count())
}

#[aoc(year = 2021, day = 1, part = 2)]
pub fn part2(input: String) -> Box<dyn Display> {
    let values: Vec<u32> = input.lines().map(|l| l.trim().parse().unwrap()).collect();
    let sums: Vec<u32> = values.windows(3).map(|x| x[0] + x[1] + x[2]).collect();
    Box::new(sums.windows(2).filter(|x| x[0] < x[1]).count())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::dotest;

    #[test]
    fn part1_example() {
        dotest(
            7,
            r"199
200
208
210
200
207
240
269
260
263",
            part1,
        );
    }

    #[test]
    fn part2_example() {
        dotest(
            5,
            r"199
200
208
210
200
207
240
269
260
263",
            part2,
        );
    }
}
