use std::fmt::Display;

pub fn part1(input: String, _vis: bool) -> Box<dyn Display> {
    let total = input.lines().map(decode).sum();
    Box::new(encode(total))
}

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
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

fn encode(mut num: usize) -> String {
    let mut res = Vec::new();
    while num > 0 {
        let rem = num % 5;
        num /= 5;
        match rem {
            0 => res.push('0'),
            1 => res.push('1'),
            2 => res.push('2'),
            3 => {
                num += 1;
                res.push('=')
            }
            4 => {
                num += 1;
                res.push('-')
            }
            _ => unreachable!(),
        };
    }
    res.iter().rev().collect()
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
