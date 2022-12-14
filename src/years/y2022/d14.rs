use std::{fmt::Display, ops::RangeInclusive};

pub fn part1(input: String, _vis: bool) -> Box<dyn Display> {
    let rocks = input.lines().map(parse_rock).collect();
    let (mut space, max_depth) = create_space(rocks);
    for grains in 0.. {
        let mut sand_depth = 0;
        let mut sand_col = 500;
        'grain: loop {
            if sand_depth == max_depth {
                return Box::new(grains);
            } else if space[sand_depth + 1][sand_col] == Space::Empty {
                sand_depth += 1;
            } else if space[sand_depth + 1][sand_col - 1] == Space::Empty {
                sand_depth += 1;
                sand_col -= 1;
            } else if space[sand_depth + 1][sand_col + 1] == Space::Empty {
                sand_depth += 1;
                sand_col += 1;
            } else {
                space[sand_depth][sand_col] = Space::Sand;
                break 'grain;
            }
        }
    }
    unreachable!()
}

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

fn create_space(rocks: Vec<Vec<Coord>>) -> (Vec<Vec<Space>>, usize) {
    let max_depth = rocks.iter().fold(0, |m, r| {
        r.iter()
            .fold(m, |m, Coord(_, d)| if m > *d { m } else { *d })
    });

    let mut space = vec![vec![Space::Empty; 1000]; max_depth + 1];

    fn fill_range(a: usize, b: usize) -> RangeInclusive<usize> {
        if a < b {
            a..=b
        } else {
            b..=a
        }
    }
    for rock in rocks {
        for pair in rock.windows(2) {
            let Coord(a_col, a_depth) = pair[0];
            let Coord(b_col, b_depth) = pair[1];
            if a_col == b_col {
                for depth in fill_range(a_depth, b_depth) {
                    space[depth][a_col] = Space::Rock;
                }
            } else if a_depth == b_depth {
                for col in fill_range(a_col, b_col) {
                    space[a_depth][col] = Space::Rock;
                }
            } else {
                unreachable!()
            }
        }
    }

    (space, max_depth)
}

struct Coord(usize, usize);

#[derive(Clone, PartialEq)]
enum Space {
    Empty,
    Sand,
    Rock,
}

fn parse_rock(line: &str) -> Vec<Coord> {
    fn parse_coord(s: &str) -> Coord {
        let (col, depth) = s.split_once(',').unwrap();
        Coord(col.parse().unwrap(), depth.parse().unwrap())
    }
    line.split(" -> ").map(parse_coord).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    crate::test::aoc_test!(example, r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9",
        part1 => 24,
        part2 => "todo");
}
