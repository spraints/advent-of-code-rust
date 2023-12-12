use std::collections::{HashMap, VecDeque};
use std::fmt::Display;

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    if vis {
        println!("{input}");
    }
    let tiles = parse(&input);
    let start = find_start(&tiles);
    if vis {
        println!("start = {start:?}");
    }
    let mut visited: HashMap<(usize, usize), u64> = HashMap::new();
    visited.insert(start, 0);
    let mut positions = VecDeque::new();
    positions.push_back(start);
    loop {
        match positions.pop_front() {
            None => break,
            Some(pos) => {
                let n_dist = 1 + visited.get(&pos).unwrap();
                let ns = neighbors(&tiles, &pos);
                if vis {
                    println!("{pos:?} --> ({n_dist})  {ns:?}");
                }
                for n in ns {
                    let e = visited.entry(n.clone()).or_insert(n_dist);
                    if *e >= n_dist {
                        *e = n_dist;
                        positions.push_back(n);
                    }
                }
            }
        };
    }
    Box::new(*visited.values().max().unwrap())
}

fn neighbors(tiles: &[Vec<Tile>], pos: &(usize, usize)) -> Vec<(usize, usize)> {
    //println!("find neighbors from {pos:?}");
    fn st(
        tiles: &[Vec<Tile>],
        start: &(usize, usize),
        choices: Vec<(usize, usize)>,
    ) -> Vec<(usize, usize)> {
        //println!("looking for start neighbors in {choices:?}");
        choices
            .into_iter()
            .filter(|p| neighbors(tiles, &p).contains(start))
            .collect()
    }
    let t = &tiles[pos.0][pos.1];
    //println!(" (it's a {t:?})");
    match t {
        Tile::Vertical => {
            if pos.0 == 0 {
                Vec::new()
            } else {
                vec![(pos.0 - 1, pos.1), (pos.0 + 1, pos.1)]
            }
        }
        Tile::Horizontal => {
            if pos.1 == 0 {
                Vec::new()
            } else {
                vec![(pos.0, pos.1 - 1), (pos.0, pos.1 + 1)]
            }
        }
        Tile::NE => {
            if pos.0 == 0 {
                Vec::new()
            } else {
                vec![(pos.0 - 1, pos.1), (pos.0, pos.1 + 1)]
            }
        }
        Tile::NW => {
            if pos.0 == 0 || pos.1 == 0 {
                Vec::new()
            } else {
                vec![(pos.0 - 1, pos.1), (pos.0, pos.1 - 1)]
            }
        }
        Tile::SW => {
            if pos.1 == 0 {
                Vec::new()
            } else {
                vec![(pos.0 + 1, pos.1), (pos.0, pos.1 - 1)]
            }
        }
        Tile::SE => vec![(pos.0 + 1, pos.1), (pos.0, pos.1 + 1)],
        Tile::Ground => Vec::new(),
        Tile::Start => match pos {
            (0, 0) => vec![(0, 1), (1, 0)],
            (0, j) => st(&tiles, pos, vec![(1, *j), (0, *j - 1), (0, *j + 1)]),
            (i, 0) => st(&tiles, pos, vec![(*i, 1), (*i - 1, 0), (*i + 1, 0)]),
            (i, j) => st(
                &tiles,
                pos,
                vec![(*i - 1, *j), (*i + 1, *j), (*i, *j - 1), (*i, *j + 1)],
            ),
        },
    }
}

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

fn find_start(tiles: &[Vec<Tile>]) -> (usize, usize) {
    for (i, row) in tiles.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if matches!(tile, Tile::Start) {
                return (i, j);
            }
        }
    }
    panic!("no start found in {tiles:?}");
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    fn parse_line(line: &str) -> Vec<Tile> {
        line.trim()
            .chars()
            .map(|c| match c {
                '|' => Tile::Vertical,
                '-' => Tile::Horizontal,
                'L' => Tile::NE,
                'J' => Tile::NW,
                '7' => Tile::SW,
                'F' => Tile::SE,
                '.' => Tile::Ground,
                'S' => Tile::Start,
                _ => panic!("illegal tile {c:?}"),
            })
            .collect()
    }
    input.lines().map(parse_line).collect()
}

#[derive(Debug)]
enum Tile {
    Vertical,   // |
    Horizontal, // -
    NE,         // L
    NW,         // J
    SW,         // 7
    SE,         // F
    Ground,     // .
    Start,      // S
}

#[cfg(test)]
mod test {
    crate::test::aoc_test!(
        part1,
        simple_part1,
        r"-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
        4
    );

    crate::test::aoc_test!(
        part1,
        complex_part1,
        r"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
        8
    );
}
