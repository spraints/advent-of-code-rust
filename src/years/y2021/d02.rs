use std::fmt::Display;

pub fn part1(input: String, _vis: bool) -> Box<dyn Display> {
    let mut depth = 0;
    let mut pos = 0;
    for c in parse(&input) {
        match c {
            Command::Forward(n) => pos += n,
            Command::Down(n) => depth += n,
            Command::Up(n) => depth -= n,
        };
    }
    Box::new(format!("{}", depth * pos))
}

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

fn parse(s: &str) -> Vec<Command> {
    s.lines().map(|line| parse_line(line)).collect()
}

fn parse_line(s: &str) -> Command {
    let mut parts = s.split(' ');
    let cmd = parts.next().unwrap();
    let val = parts.next().unwrap().parse().unwrap();
    match cmd {
        "forward" => Command::Forward(val),
        "up" => Command::Up(val),
        "down" => Command::Down(val),
        _ => panic!("illegal command '{s}'"),
    }
}

enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[cfg(test)]
mod test {
    use super::*;

    crate::test::aoc_test!(example, r"forward 5
down 5
forward 8
up 3
down 8
forward 2",
        part1 => "150",
        part2 => "todo");
}
