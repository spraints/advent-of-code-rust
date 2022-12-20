use std::fmt::Display;

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

#[cfg(test)]
mod test {
    use super::*;

    crate::test::aoc_test!(example, r"",
        part1 => "todo",
        part2 => "todo");
}
