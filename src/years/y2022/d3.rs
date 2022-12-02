use std::fmt::Display;

pub fn part1(input: String) -> anyhow::Result<Box<dyn Display>> {
    Ok(Box::new(input))
}

pub fn part2(input: String) -> anyhow::Result<Box<dyn Display>> {
    Ok(Box::new(input))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::*;

    const EX: &str = r"";

    #[test]
    fn part1_example() {
        dotest(EX, EX, part1);
    }

    #[test]
    fn part2_example() {
        dotest(EX, EX, part2);
    }
}
