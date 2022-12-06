use std::fmt::Display;

pub fn part1(input: String, _vis: bool) -> Box<dyn Display> {
    find_tag(&input, 4)
}

fn find_tag(input: &str, size: usize) -> Box<dyn Display> {
    let input = input.as_bytes();
    for (i, vals) in input.windows(size).enumerate() {
        if none_match(vals) {
            return Box::new(i + vals.len());
        }
    }
    Box::new("not found")
}

fn none_match(v: &[u8]) -> bool {
    for i in 0..v.len() {
        for j in (i + 1)..v.len() {
            if v[i] == v[j] {
                return false;
            }
        }
    }
    true
}

pub fn part2(input: String, _vis: bool) -> Box<dyn Display> {
    find_tag(&input, 14)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::aoc_test;

    aoc_test!(a, "mjqjpqmgbljsphdztnvjfqwrcgsmlb", part1 => 7, part2 => 19);
    aoc_test!(b, "bvwbjplbgvbhsrlpgdmjqwftvncz", part1 => 5, part2 => 23);
    aoc_test!(c, "nppdvjthqldpwncqszvftbrmjlhg", part1 => 6, part2 => 23);
    aoc_test!(d, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", part1 => 10, part2 => 29);
    aoc_test!(e, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", part1 => 11, part2 => 26);
}
