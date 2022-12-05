use std::fmt::Display;

pub fn part1(input: String, _vis: bool) -> Box<dyn Display> {
    Box::new(format!("do something with {} chars", input.len()))
}

pub fn part2(input: String, _vis: bool) -> Box<dyn Display> {
    Box::new(format!("do something with {} chars", input.len()))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::*;

    const EX: &str = r"";

    #[test]
    fn part1_example() {
        dotest("do something with 0 chars", EX, part1);
    }

    #[test]
    fn part2_example() {
        dotest("do something with 0 chars", EX, part2);
    }
}
