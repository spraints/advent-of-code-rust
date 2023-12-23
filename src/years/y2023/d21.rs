use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fmt::Display;

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let parsed = parse(&input);
    let res = solve(Part1, parsed, 64, vis);
    Box::new(res)
}

pub fn part2(input: String, _vis: bool) -> Box<dyn Display> {
    let parsed = parse(&input);
    let res = solve(Part2, parsed, 26501365, false);
    Box::new(res)
}

trait Part {
    fn maybe_push(
        &self,
        new_possible: &mut HashSet<(isize, isize)>,
        parsed: &Parsed,
        pos: (isize, isize),
    );
}

#[derive(Clone, Copy)]
struct Part1;
#[derive(Clone, Copy)]
struct Part2;

impl Part for Part1 {
    fn maybe_push(
        &self,
        new_possible: &mut HashSet<(isize, isize)>,
        parsed: &Parsed,
        pos: (isize, isize),
    ) {
        if matches!(parsed.map.get(&pos), Some(Tile::Garden | Tile::Start)) {
            new_possible.insert(pos);
        }
    }
}

impl Part for Part2 {
    fn maybe_push(
        &self,
        new_possible: &mut HashSet<(isize, isize)>,
        parsed: &Parsed,
        pos: (isize, isize),
    ) {
        let map_pos = (modwrap(pos.0, parsed.rows), modwrap(pos.1, parsed.cols));
        if matches!(parsed.map.get(&map_pos), Some(Tile::Garden | Tile::Start)) {
            new_possible.insert(pos);
        }
    }
}

struct State {
    min_row: isize,
    max_row: isize,
    min_col: isize,
    max_col: isize,
    possible: HashSet<(isize, isize)>,
}

fn solve(p: impl Part + Copy, parsed: Parsed, steps: usize, vis: bool) -> usize {
    let mut state = initial_state(&parsed);

    for i in 1..=steps {
        state = step(state, p, &parsed);
        if vis {
            println!("---AFTER STEP {i}---");
            for r in state.min_row..=state.max_row {
                for c in state.min_col..=state.max_col {
                    print!(
                        "{}",
                        match (
                            state.possible.contains(&(r, c)),
                            parsed
                                .map
                                .get(&(modwrap(r, parsed.rows), modwrap(c, parsed.cols)))
                        ) {
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

    state.possible.len()
}

fn step(state: State, p: impl Part, parsed: &Parsed) -> State {
    let State {
        mut min_row,
        mut max_row,
        mut min_col,
        mut max_col,
        possible,
    } = state;

    let mut new_possible = HashSet::new();
    for pos in possible {
        min_row = min(min_row, pos.0 - 1);
        max_row = max(max_row, pos.0 + 1);
        min_col = min(min_col, pos.1 - 1);
        max_col = max(max_col, pos.1 + 1);
        p.maybe_push(&mut new_possible, &parsed, (pos.0, pos.1 + 1));
        p.maybe_push(&mut new_possible, &parsed, (pos.0, pos.1 - 1));
        p.maybe_push(&mut new_possible, &parsed, (pos.0 + 1, pos.1));
        p.maybe_push(&mut new_possible, &parsed, (pos.0 - 1, pos.1));
    }

    State {
        min_row,
        max_row,
        min_col,
        max_col,
        possible: new_possible,
    }
}

fn initial_state(parsed: &Parsed) -> State {
    let mut possible = HashSet::new();
    for (pos, st) in &parsed.map {
        if matches!(st, Tile::Start) {
            possible.insert(*pos);
        }
    }

    let min_row = 0;
    let max_row = parsed.rows - 1;
    let min_col = 0;
    let max_col = parsed.rows - 1;

    State {
        min_row,
        max_row,
        min_col,
        max_col,
        possible,
    }
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

fn modwrap(n: isize, d: isize) -> isize {
    (d + (n % d)) % d
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
    fn mod_assumptions() {
        assert_eq!(0, super::modwrap(0, 100));
        assert_eq!(1, super::modwrap(1, 100));
        assert_eq!(1, super::modwrap(1001, 100));
        assert_eq!(1, super::modwrap(-99, 100));
        assert_eq!(1, super::modwrap(-9999999, 100));
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            16,
            super::solve(super::Part1, super::parse(TEST_INPUT), 6, true)
        );
    }

    #[test]
    fn test_part2_6() {
        assert_eq!(
            16,
            super::solve(super::Part2, super::parse(TEST_INPUT), 6, false)
        );
    }

    #[test]
    fn test_part2_10() {
        assert_eq!(
            50,
            super::solve(super::Part2, super::parse(TEST_INPUT), 10, true)
        );
    }

    #[test]
    fn test_part2_50() {
        assert_eq!(
            1594,
            super::solve(super::Part2, super::parse(TEST_INPUT), 50, false)
        );
    }

    #[test]
    fn test_part2_100() {
        assert_eq!(
            6536,
            super::solve(super::Part2, super::parse(TEST_INPUT), 100, false)
        );
    }

    //#[test]
    //fn test_part2_500() {
    //    assert_eq!(
    //        167004,
    //        super::solve(super::Part2, super::parse(TEST_INPUT), 500, false)
    //    );
    //}

    //#[test]
    //fn test_part2_1000() {
    //    assert_eq!(
    //        668697,
    //        super::solve(super::Part2, super::parse(TEST_INPUT), 1000, false)
    //    );
    //}

    //#[test]
    //fn test_part2_5000() {
    //    assert_eq!(
    //        16733044,
    //        super::solve(super::Part2, super::parse(TEST_INPUT), 5000, false)
    //    );
    //}
}
