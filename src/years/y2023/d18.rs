use std::{
    cmp::{max, min},
    collections::HashSet,
    fmt::Display,
};

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let directions = parse(&input);
    let edges = dig(&directions, vis);
    let filled = fill(&edges, vis);
    if vis {
        println!("{}", filled.len());
    }
    Box::new("todo")
}

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

struct DigRes {
    trench: HashSet<(isize, isize)>,
    min_row: isize,
    max_row: isize,
    min_col: isize,
    max_col: isize,
}

fn dig(directions: &[Direction], vis: bool) -> DigRes {
    let mut pos = (0, 0);
    let mut trench = HashSet::new();
    trench.insert(pos);
    let mut min_row = 0;
    let mut max_row = 0;
    let mut min_col = 0;
    let mut max_col = 0;
    for d in directions {
        let off = match d.dir {
            Dir::Up => (1, 0),
            Dir::Down => (-1, 0),
            Dir::Left => (0, -1),
            Dir::Right => (0, 1),
        };
        for _ in 0..d.dist {
            pos = (pos.0 + off.0, pos.1 + off.1);
            trench.insert(pos);
        }
        min_row = min(min_row, pos.0);
        max_row = max(max_row, pos.0);
        min_col = min(min_col, pos.1);
        max_col = max(max_col, pos.1);
    }
    if vis {
        if vis {
            println!("after digging:");
        }
        for r in min_row..=max_row {
            for c in min_col..=max_col {
                print!("{}", if trench.contains(&(r, c)) { '#' } else { '.' });
            }
            println!();
        }
    }
    DigRes {
        trench,
        min_row,
        max_row,
        min_col,
        max_col,
    }
}

fn fill(edges: &DigRes, vis: bool) -> HashSet<(isize, isize)> {
    if vis {
        println!("after filling:");
    }
    let mut filled = HashSet::new();
    for r in edges.min_row..=edges.max_row {
        for c in edges.min_col..=edges.max_col {
            let pos = (r, c);
            let included = edges.trench.contains(&pos) || surrounds(&edges, pos);
            if included {
                filled.insert(pos);
            }
            if vis {
                print!("{}", if included { '#' } else { '.' });
            }
        }
        println!();
    }
    filled
}

fn surrounds(edges: &DigRes, pos: (isize, isize)) -> bool {
    let mut cross_r = 0;
    let mut crossing_r = false;
    for r in edges.min_row..pos.0 {
        todo!()
    }
    pos.0 == edges.min_row // todo
}

fn parse(input: &str) -> Vec<Direction> {
    fn parse_line(line: &str) -> Direction {
        let mut parts = line.split_whitespace();
        let dir = match parts.next() {
            Some("D") => Dir::Down,
            Some("L") => Dir::Left,
            Some("U") => Dir::Up,
            Some("R") => Dir::Right,
            s => panic!("expected direction but got {s:?}"),
        };
        let dist = parts.next().unwrap().parse().unwrap();
        let color = parts.next().unwrap().to_owned();
        Direction { dir, dist, color }
    }
    input.lines().map(parse_line).collect()
}

struct Direction {
    dir: Dir,
    dist: isize,
    color: String,
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str = r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    crate::test::aoc_test!(part1, TEST_INPUT, 62);
    crate::test::aoc_test!(part2, TEST_INPUT, "todo");
}
