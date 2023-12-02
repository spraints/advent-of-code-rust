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
    use super::*;

    crate::test::aoc_tests!(example, r"",
        part1 => "todo",
        part2 => "todo");
}
