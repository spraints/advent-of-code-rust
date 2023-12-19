use std::cmp::{max, min};
use std::fmt::Display;

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let directions = parse(&input);
    let edges = dig(&directions, vis);
    let filled = fill(&edges, vis);
    Box::new(filled)
}

pub fn part2(input: String, _vis: bool) -> Box<dyn Display> {
    fn flip(directions: Vec<Direction>) -> Vec<Direction> {
        directions
            .into_iter()
            .map(|d| {
                // 012345678
                // (#abcde0)
                let dist = isize::from_str_radix(&d.color[2..7], 16).unwrap();
                let dir = match &d.color[7..8] {
                    "0" => Dir::Right,
                    "1" => Dir::Down,
                    "2" => Dir::Left,
                    "3" => Dir::Up,
                    _ => panic!("unrecognized final digit in {:?}", d.color),
                };
                Direction {
                    dist,
                    dir,
                    color: d.color,
                }
            })
            .collect()
    }

    let directions = parse(&input);
    let directions = flip(directions);
    let edges = dig(&directions, false);
    let filled = fill(&edges, false);
    Box::new(filled)
}

struct DigRes {
    vertices: Vec<(isize, isize)>,
    perimeter: isize,
}

fn dig(directions: &[Direction], vis: bool) -> DigRes {
    let mut pos = (0, 0);
    let mut vertices = Vec::with_capacity(directions.len() + 1);
    vertices.push(pos);
    let mut perimeter = 1;
    let mut min_row = 0;
    let mut max_row = 0;
    let mut min_col = 0;
    let mut max_col = 0;
    for d in directions {
        let dir = match d.dir {
            Dir::Up => (1, 0),
            Dir::Down => (-1, 0),
            Dir::Left => (0, -1),
            Dir::Right => (0, 1),
        };
        pos = (pos.0 + dir.0 * d.dist, pos.1 + dir.1 * d.dist);
        vertices.push(pos);
        perimeter += d.dist;
        min_row = min(min_row, pos.0);
        max_row = max(max_row, pos.0);
        min_col = min(min_col, pos.1);
        max_col = max(max_col, pos.1);
    }
    if vis {
        fn walk(from: &(isize, isize), to: &(isize, isize)) -> Vec<(isize, isize)> {
            let (r1, r2) = if from.0 > to.0 {
                (to.0, from.0)
            } else {
                (from.0, to.0)
            };
            let (c1, c2) = if from.1 > to.1 {
                (to.1, from.1)
            } else {
                (from.1, to.1)
            };
            let mut res = Vec::new();
            for r in r1..=r2 {
                for c in c1..=c2 {
                    res.push((r, c));
                }
            }
            res
        }
        if vis {
            println!("after digging:");
        }
        let height = max_row - min_row + 1;
        let width = max_col - min_col + 1;
        //println!("rows {min_row} .. {max_row} => {height}");
        //println!("cols {min_col} .. {max_col} => {width}");
        let mut trenches = vec![vec!['.'; width as usize]; height as usize];
        for pair in vertices.windows(2) {
            let from = pair[0];
            let to = pair[1];
            for (r, c) in walk(&from, &to) {
                trenches[(r - min_row) as usize][(c - min_col) as usize] = '#';
            }
        }
        for (r, c) in walk(&pos, &(0, 0)) {
            trenches[(r - min_row) as usize][(c - min_col) as usize] = '#';
        }
        for row in trenches {
            let row: String = row.into_iter().collect();
            println!("{row}");
        }
    }
    DigRes {
        vertices,
        perimeter,
    }
}

fn fill(edges: &DigRes, vis: bool) -> isize {
    if vis {
        println!("after filling:");
    }
    // from https://www.linkedin.com/advice/1/how-do-you-calculate-area-perimeter-irregular-polygon#:~:text=To%20calculate%20the%20area%20of,is%20the%20number%20of%20vertices.
    fn term(from: &(isize, isize), to: &(isize, isize)) -> isize {
        from.0 * to.1 - to.0 * from.1
    }
    let mut a = 0;
    for pair in edges.vertices.windows(2) {
        a += term(&pair[0], &pair[1]);
    }
    a += term(
        edges.vertices.last().unwrap(),
        edges.vertices.first().unwrap(),
    );
    a.abs() / 2 + edges.perimeter / 2 + 1
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
    crate::test::aoc_test!(part2, TEST_INPUT, "952408144115");
}
