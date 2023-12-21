use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let parsed = parse(&input);
    let res = solve(parsed, 64, vis);
    Box::new(res)
}

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

fn solve(parsed: Parsed, steps: usize, vis: bool) -> usize {
    let mut possible = HashSet::new();
    for (pos, st) in &parsed.map {
        if matches!(st, Tile::Start) {
            possible.insert(*pos);
        }
    }

    fn maybe_push(
        new_possible: &mut HashSet<(isize, isize)>,
        parsed: &Parsed,
        pos: (isize, isize),
    ) {
        if matches!(parsed.map.get(&pos), Some(Tile::Garden | Tile::Start)) {
            new_possible.insert(pos);
        }
    }

    for i in 1..=steps {
        let mut new_possible = HashSet::new();
        for pos in possible {
            maybe_push(&mut new_possible, &parsed, (pos.0, pos.1 + 1));
            maybe_push(&mut new_possible, &parsed, (pos.0, pos.1 - 1));
            maybe_push(&mut new_possible, &parsed, (pos.0 + 1, pos.1));
            maybe_push(&mut new_possible, &parsed, (pos.0 - 1, pos.1));
        }
        possible = new_possible;
        if vis {
            println!("---AFTER STEP {i}---");
            for r in 0..parsed.rows {
                for c in 0..parsed.cols {
                    print!(
                        "{}",
                        match (possible.contains(&(r, c)), parsed.map.get(&(r, c))) {
                            (true, _) => 'O',
                            (false, Some(Tile::Garden)) => '.',
                            (false, Some(Tile::Rock)) => '#',
                            (false, Some(Tile::Start)) => 'S',
                            _ => unreachable!(),
                        }
                    );
                }
                println!();
            }
        }
    }

    possible.len()
}

struct Parsed {
    map: HashMap<(isize, isize), Tile>,
    rows: isize,
    cols: isize,
}

enum Tile {
    Start,
    Garden,
    Rock,
}

fn parse(input: &str) -> Parsed {
    let mut map = HashMap::new();
    let mut rows = 0;
    let mut cols = 0;
    for (r, line) in input.lines().enumerate() {
        rows = max(rows, 1 + r as isize);
        for (c, ch) in line.trim().chars().enumerate() {
            cols = max(cols, 1 + c as isize);
            map.insert(
                (r as isize, c as isize),
                match ch {
                    'S' => Tile::Start,
                    '.' => Tile::Garden,
                    '#' => Tile::Rock,
                    _ => panic!("unexpected tile {ch:?} at row={r} col={c}"),
                },
            );
        }
    }
    Parsed { map, rows, cols }
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str = r"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test_part1() {
        assert_eq!(16, super::solve(super::parse(TEST_INPUT), 6, true));
    }

    crate::test::aoc_test!(part2, TEST_INPUT, "todo");
}
