use std::fmt::Display;

pub fn part1(input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

pub fn part2(input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::*;

    const EX: &str = r"";

    #[test]
    fn part1_example() {
        dotest("todo", EX, part1);
    }

    #[test]
    fn part2_example() {
        dotest("todo", EX, part2);
    }
}
