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
    use crate::test::*;

    #[test]
    fn part1_example() {
        dotest(7, "mjqjpqmgbljsphdztnvjfqwrcgsmlb", part1);
        dotest(5, "bvwbjplbgvbhsrlpgdmjqwftvncz", part1);
        dotest(6, "nppdvjthqldpwncqszvftbrmjlhg", part1);
        dotest(10, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", part1);
        dotest(11, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", part1);
    }

    #[test]
    fn part2_example() {
        dotest(19, "mjqjpqmgbljsphdztnvjfqwrcgsmlb", part2);
        dotest(23, "bvwbjplbgvbhsrlpgdmjqwftvncz", part2);
        dotest(23, "nppdvjthqldpwncqszvftbrmjlhg", part2);
        dotest(29, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", part2);
        dotest(26, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", part2);
    }
}
