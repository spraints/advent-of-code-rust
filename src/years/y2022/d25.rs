use std::fmt::Display;

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let total = input.lines().map(decode).sum();
    Box::new(encode(total))
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

fn decode(line: &str) -> usize {
    line.chars().fold(0isize, |res, ch| {
        res * 5
            + match ch {
                '1' => 1,
                '2' => 2,
                '0' => 0,
                '=' => -2,
                '-' => -1,
                ch => unreachable!("illegal char {:?} in {:?}", ch, line),
            }
    }) as usize
}

fn encode(num: usize) -> usize {
    num
}

#[cfg(test)]
mod test {
    use super::*;

    crate::test::aoc_test!(example, r"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122",
        part1 => "2=-1=0",
        part2 => "todo");
}
