use std::fmt::Display;

pub fn part1(input: String, _vis: bool) -> Box<dyn Display> {
    let mut sum = 0;
    for (i, pair) in input.split("\n\n").enumerate() {
        let (left, right) = pair.split_once('\n').unwrap();
        let (left, right) = (parse(left), parse(right));
        if left < right {
            sum += i + 1;
        }
    }
    Box::new(sum)
}

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

fn parse(s: &str) -> Packet {
    parse_item(s, 0).0
}

fn parse_item(mut s: &str, level: usize) -> (Packet, &str) {
    let os = s;
    if s.starts_with('[') {
        let mut items = Vec::new();
        s = &s[1..];
        while !s.starts_with(']') {
            println!("[{}] '{}'", level, s);
            let parsed = parse_item(s, level + 1);
            items.push(parsed.0);
            s = parsed.1;
        }
        let res = (Packet::List(items), &s[1..]);
        println!("[{}] '{}' => {:?}", level, os, res);
        res
    } else {
        let (val, rest): (&str, &str) = match (s.find(','), s.find(']')) {
            (None, None) => (s, ""),
            (Some(i), Some(j)) if i < j => (&s[0..i], &s[i + 1..]),
            (Some(i), None) => (&s[0..i], &s[i + 1..]),
            (_, Some(i)) => (&s[0..i], &s[i..]),
        };
        println!("[{}] '{}': VALUE: '{}'/'{}'", level, os, val, rest);
        let res = (Packet::Value(val.parse().unwrap()), rest);
        println!("[{}] '{}' => {:?}", level, os, res);
        res
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum Packet {
    List(Vec<Packet>),
    Value(u8),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Value(l), Self::Value(r)) => l.cmp(r),
            (Self::List(l), Self::List(r)) => cmp_list(l, r),
            (Self::List(l), r) => Self::List(l.clone()).cmp(&Self::List(vec![r.clone()])),
            (l, Self::List(r)) => Self::List(vec![l.clone()]).cmp(&Self::List(r.clone())),
        }
    }
}

fn cmp_list(l: &[Packet], r: &[Packet]) -> std::cmp::Ordering {
    for (l, r) in std::iter::zip(l, r) {
        match l.cmp(r) {
            std::cmp::Ordering::Equal => continue,
            o => return o,
        }
    }
    l.len().cmp(&r.len())
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    crate::test::aoc_test!(example, r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]",
        part1 => 13,
        part2 => "todo");

    #[test]
    fn parse() {
        use super::{parse, Packet};
        assert_eq!(Packet::List(Vec::new()), parse("[]"));
        assert_eq!(Packet::Value(1), parse("1"));
        assert_eq!(Packet::Value(10), parse("10"));
    }
}
