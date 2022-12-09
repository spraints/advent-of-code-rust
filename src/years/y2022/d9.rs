use std::{collections::HashSet, fmt::Display};

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let (mut hx, mut hy) = (0, 0);
    let (mut tx, mut ty) = (0, 0);
    let mut visited = HashSet::new();
    visited.insert((tx, ty));
    for line in input.lines() {
        let ((dx, dy), steps) = parse(line);
        for _ in 0..steps {
            hx += dx;
            hy += dy;
            if ty - hy > 1 {
                tx = hx;
                ty -= 1;
            } else if hy - ty > 1 {
                tx = hx;
                ty += 1;
            } else if hx - tx > 1 {
                ty = hy;
                tx += 1;
            } else if tx - hx > 1 {
                ty = hy;
                tx -= 1;
            }
            visited.insert((tx, ty));
            if vis {
                println!("H=({},{}), T=({},{})", hx, hy, tx, ty);
            }
        }
    }
    Box::new(visited.len())
}

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

fn parse(line: &str) -> ((isize, isize), usize) {
    let (dir, steps) = line.split_once(' ').unwrap();
    (
        match dir {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => unreachable!(),
        },
        steps.parse().unwrap(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    crate::test::aoc_test!(example, r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
        part1 => 13,
        part2 => "todo");
}
