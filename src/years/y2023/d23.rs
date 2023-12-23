use std::{collections::HashSet, fmt::Display};

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let parsed = parse(&input, true);
    let longest_path = find_longest_path(&parsed);
    if vis {
        for (r, row) in parsed.map.iter().enumerate() {
            for (c, tile) in row.iter().enumerate() {
                if longest_path.contains(&(r, c)) {
                    print!("O");
                } else {
                    print!("{}", tile);
                }
            }
            println!();
        }
    }
    // I count the start square, but it's not supposed to be counted.
    Box::new(longest_path.len() - 1)
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    let parsed = parse(&input, false);
    let longest_path = find_longest_path(&parsed);
    if vis {
        for (r, row) in parsed.map.iter().enumerate() {
            for (c, tile) in row.iter().enumerate() {
                if longest_path.contains(&(r, c)) {
                    print!("O");
                } else {
                    print!("{}", tile);
                }
            }
            println!();
        }
    }
    // I count the start square, but it's not supposed to be counted.
    Box::new(longest_path.len() - 1)
}

fn find_longest_path(parsed: &Parsed) -> HashSet<(usize, usize)> {
    fn flp(
        parsed: &Parsed,
        progress: HashSet<(usize, usize)>,
        pos: (usize, usize),
    ) -> Option<HashSet<(usize, usize)>> {
        match fill_to_fork(parsed, progress, pos) {
            None => None,
            Some((progress, choices)) => {
                if choices.is_empty() {
                    Some(progress)
                } else {
                    choices
                        .into_iter()
                        .filter_map(|choice| flp(parsed, progress.clone(), choice))
                        .max_by_key(|path| path.len())
                }
            }
        }
    }

    fn fill_to_fork(
        parsed: &Parsed,
        mut progress: HashSet<(usize, usize)>,
        mut pos: (usize, usize),
    ) -> Option<(HashSet<(usize, usize)>, Vec<(usize, usize)>)> {
        loop {
            // assume pos is a legal next step.
            progress.insert(pos);
            if pos.0 == parsed.rows - 1 {
                // We found the end!
                return Some((progress, Vec::new()));
            }
            let neighbors = neighbors(pos, &progress, parsed);
            match neighbors.len() {
                // Dead end.
                0 => return None,
                // On a path without a fork.
                1 => pos = neighbors[0],
                // At a fork.
                _ => return Some((progress, neighbors)),
            };
        }
    }

    fn neighbors(
        pos: (usize, usize),
        progress: &HashSet<(usize, usize)>,
        parsed: &Parsed,
    ) -> Vec<(usize, usize)> {
        let mut res = Vec::with_capacity(2);
        let (r, c) = pos;
        if r > 0 {
            // try going up.
            let p = (r - 1, c);
            if !progress.contains(&p)
                && matches!(
                    parsed.map[p.0][p.1],
                    Tile::Path(SlopeDirection::None | SlopeDirection::Up)
                )
            {
                res.push(p);
            }
        }
        if c > 0 {
            // try going left.
            let p = (r, c - 1);
            if !progress.contains(&p)
                && matches!(
                    parsed.map[p.0][p.1],
                    Tile::Path(SlopeDirection::None | SlopeDirection::Left)
                )
            {
                res.push(p);
            }
        }
        if r < parsed.rows - 1 {
            // try going down.
            let p = (r + 1, c);
            if !progress.contains(&p)
                && matches!(
                    parsed.map[p.0][p.1],
                    Tile::Path(SlopeDirection::None | SlopeDirection::Down)
                )
            {
                res.push(p);
            }
        }
        if c < parsed.cols - 1 {
            // try going right.
            let p = (r, c + 1);
            if !progress.contains(&p)
                && matches!(
                    parsed.map[p.0][p.1],
                    Tile::Path(SlopeDirection::None | SlopeDirection::Right)
                )
            {
                res.push(p);
            }
        }
        res
    }

    flp(parsed, HashSet::new(), (0, parsed.start_col)).unwrap()
}

struct Parsed {
    map: Vec<Vec<Tile>>,
    start_col: usize,
    rows: usize,
    cols: usize,
}

enum Tile {
    Forest,
    Path(SlopeDirection),
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Forest => '#',
                Self::Path(SlopeDirection::None) => '.',
                Self::Path(SlopeDirection::Up) => '^',
                Self::Path(SlopeDirection::Down) => 'v',
                Self::Path(SlopeDirection::Left) => '<',
                Self::Path(SlopeDirection::Right) => '>',
            }
        )
    }
}

enum SlopeDirection {
    None,
    Left,
    Right,
    Up,
    Down,
}

fn parse(input: &str, slippery: bool) -> Parsed {
    let map: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match (c, slippery) {
                    ('#', _) => Tile::Forest,
                    ('.', _) | (_, false) => Tile::Path(SlopeDirection::None),
                    ('>', true) => Tile::Path(SlopeDirection::Right),
                    ('<', true) => Tile::Path(SlopeDirection::Left),
                    ('^', true) => Tile::Path(SlopeDirection::Up),
                    ('v', true) => Tile::Path(SlopeDirection::Down),
                    _ => panic!("unexpected tile {c:?}"),
                })
                .collect()
        })
        .collect();
    let start_col = map[0]
        .iter()
        .position(|t| matches!(t, Tile::Path(_)))
        .unwrap();
    let rows = map.len();
    let cols = map[0].len();
    Parsed {
        map,
        start_col,
        rows,
        cols,
    }
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str = r"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    crate::test::aoc_test!(part1, TEST_INPUT, 94);
    crate::test::aoc_test!(part2, TEST_INPUT, 154);
}
