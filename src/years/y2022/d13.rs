use std::fmt::Display;

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let mut sum = 0;
    for (i, pair) in input.split("\n\n").enumerate() {
        let (left, right) = pair.split_once('\n').unwrap();
        let (left, right) = (parse(left, vis), parse(right, vis));
        if left < right {
            sum += i + 1;
        }
    }
    Box::new(sum)
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    let mut packets: Vec<Packet> = input
        .lines()
        .filter(|s| *s != "")
        .map(|s| parse(s, vis))
        .collect();
    let p1 = parse("[[2]]", vis);
    let p2 = parse("[[6]]", vis);
    packets.push(p1.clone());
    packets.push(p2.clone());
    packets.sort();
    let mut res = 1;
    for (i, p) in packets.iter().enumerate() {
        if *p == p1 || *p == p2 {
            if vis {
                println!("found {:?} at {}", p, i);
            }
            res *= i + 1;
        }
    }
    Box::new(res)
}

fn parse(s: &str, vis: bool) -> Packet {
    let s = s.trim();
    if vis {
        println!("PARSING {}", s);
    }
    let mut parents = Vec::new();
    let mut cur = Vec::new();
    for tok in tokens(s) {
        if vis {
            println!("TOKEN: {:?}", tok);
        }
        match tok {
            Token::Open => {
                parents.push(cur);
                cur = Vec::new()
            }
            Token::Close => {
                let child = cur;
                cur = parents.pop().unwrap();
                cur.push(Packet::List(child));
            }
            Token::Value(s) => cur.push(Packet::Value(s.parse().unwrap())),
        };
    }
    let res = cur.into_iter().next().unwrap();
    if vis {
        println!("{} => {:?}", s, res);
    }
    res
}

fn tokens(s: &str) -> Tokens {
    Tokens { s }
}

struct Tokens<'a> {
    s: &'a str,
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.s.starts_with(',') {
            self.s = &self.s[1..];
        }
        if self.s == "" {
            None
        } else if self.s.starts_with('[') {
            self.s = &self.s[1..];
            Some(Token::Open)
        } else if self.s.starts_with(']') {
            self.s = &self.s[1..];
            Some(Token::Close)
        } else {
            match (self.s.find(','), self.s.find(']')) {
                (None, None) => {
                    let val = self.s;
                    self.s = "";
                    Some(Token::Value(val))
                }
                (Some(comma), None) => {
                    let val = &self.s[0..comma];
                    self.s = &self.s[comma + 1..];
                    Some(Token::Value(val))
                }
                (Some(comma), Some(close)) if comma < close => {
                    let val = &self.s[0..comma];
                    self.s = &self.s[comma + 1..];
                    Some(Token::Value(val))
                }
                (_, Some(close)) => {
                    let val = &self.s[0..close];
                    self.s = &self.s[close..];
                    Some(Token::Value(val))
                }
            }
        }
    }
}

#[derive(Debug)]
enum Token<'a> {
    Open,
    Close,
    Value(&'a str),
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum Packet {
    List(Vec<Packet>),
    Value(u8),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let res = match (self, other) {
            (Self::Value(l), Self::Value(r)) => l.cmp(r),
            (Self::List(l), Self::List(r)) => cmp_list(l, r),
            (Self::List(l), r) => Self::List(l.clone()).cmp(&Self::List(vec![r.clone()])),
            (l, Self::List(r)) => Self::List(vec![l.clone()]).cmp(&Self::List(r.clone())),
        };
        //println!("{:?} < {:?} => {:?}", self, other, res);
        res
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
        part2 => 140);

    #[test]
    fn parse() {
        use super::{parse, Packet};
        assert_eq!(Packet::List(Vec::new()), parse("[]", true));
        assert_eq!(Packet::Value(1), parse("1", true));
        assert_eq!(Packet::Value(10), parse("10", true));
        assert_eq!(
            Packet::List(vec![Packet::Value(1), Packet::List(vec![Packet::Value(2)])]),
            parse("[1,[2]]", true)
        );
    }

    #[test]
    fn cmp() {
        use super::Packet;
        fn lt(a: Packet, b: Packet) {
            assert!(a < b, "expect {:?} to be less than {:?}", a, b);
        }
        fn eq(a: Packet, b: Packet) {
            assert!(a == b, "expect {:?} to be equal to {:?}", a, b);
        }
        lt(
            Packet::List(Vec::new()),
            Packet::List(vec![Packet::Value(1)]),
        );
        lt(Packet::Value(1), Packet::Value(2));
        eq(Packet::Value(1), Packet::Value(1));
    }
}
