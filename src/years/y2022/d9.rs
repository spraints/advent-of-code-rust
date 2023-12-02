use std::{cmp::Ordering, collections::HashSet, fmt::Display};

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    doit(input, vis, 2)
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    doit(input, vis, 10)
}

fn doit(input: String, vis: bool, size: usize) -> Box<dyn Display> {
    let mut positions = vec![(0, 0); size];
    let mut visited = HashSet::new();
    visited.insert((0, 0));
    for line in input.lines() {
        let ((dx, dy), steps) = parse(line);
        for _ in 0..steps {
            let (mut hx, mut hy) = positions[0];
            if vis {
                println!("head: ({},{}) + ({},{})", hx, hy, dx, dy);
            }
            hx += dx;
            hy += dy;
            positions[0] = (hx, hy);
            if vis {
                println!("  [{}] ({},{})", 0, hx, hy);
            }
            for (i, pos) in positions.iter_mut().enumerate().skip(1) {
                let (mut tx, mut ty) = pos;
                if ty - hy > 1 {
                    ty -= 1;
                    tx += match tx.cmp(&hx) {
                        Ordering::Less => 1,
                        Ordering::Equal => 0,
                        Ordering::Greater => -1,
                    };
                } else if hy - ty > 1 {
                    ty += 1;
                    tx += match tx.cmp(&hx) {
                        Ordering::Less => 1,
                        Ordering::Equal => 0,
                        Ordering::Greater => -1,
                    };
                } else if hx - tx > 1 {
                    tx += 1;
                    ty += match ty.cmp(&hy) {
                        Ordering::Less => 1,
                        Ordering::Equal => 0,
                        Ordering::Greater => -1,
                    };
                } else if tx - hx > 1 {
                    tx -= 1;
                    ty += match ty.cmp(&hy) {
                        Ordering::Less => 1,
                        Ordering::Equal => 0,
                        Ordering::Greater => -1,
                    };
                } else {
                    break;
                }
                if vis {
                    println!("  [{}] ({},{})", i, tx, ty);
                }
                *pos = (tx, ty);
                hx = tx;
                hy = ty;
            }
            visited.insert(*positions.last().unwrap());
        }
        if vis {
            println!("{} => {:?}", line, positions);
        }
    }
    Box::new(visited.len())
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

    crate::test::aoc_tests!(example, r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
        part1 => 13,
        part2 => 1);

    crate::test::aoc_tests!(longer, r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20", part2 => 36);

    // test input, part1 => 6087, part2 => 2493
}
