use std::fmt::Display;

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str = r"";

    crate::test::aoc_test!(part1, TEST_INPUT, "todo");
    crate::test::aoc_test!(part2, TEST_INPUT, "todo");
}
