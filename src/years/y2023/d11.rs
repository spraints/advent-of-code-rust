use std::{collections::HashSet, fmt::Display};

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    solve(input, vis, 2)
}

pub fn part2(input: String, _vis: bool) -> Box<dyn Display> {
    solve(input, false, 1000000)
}

fn solve(input: String, vis: bool, expansion: usize) -> Box<dyn Display> {
    let universe = parse(&input);
    if vis {
        println!("INPUT:");
        println!("{universe}");
    }
    let universe = expand(universe, expansion);
    if vis {
        println!("EXPANDED:");
        println!("{universe}");
    }
    let mut total_dist = 0;
    for (i, g1) in universe.galaxies.iter().enumerate() {
        for g2 in &universe.galaxies[i + 1..] {
            let dist = distance(g1, g2);
            if vis {
                println!("{g1:?} -> {g2:?} in {dist}");
            }
            total_dist += dist;
        }
    }
    Box::new(total_dist)
}

fn distance(g1: &(usize, usize), g2: &(usize, usize)) -> usize {
    let (i1, j1) = g1;
    let (i2, j2) = g2;
    fn diff(a: &usize, b: &usize) -> usize {
        if a > b {
            a - b
        } else {
            b - a
        }
    }
    diff(i1, i2) + diff(j1, j2)
}

fn expand(universe: Universe, expansion: usize) -> Universe {
    let mut empty_rows: HashSet<usize> = (0..universe.rows).collect();
    let mut empty_cols: HashSet<usize> = (0..universe.cols).collect();
    for (i, j) in &universe.galaxies {
        empty_rows.remove(i);
        empty_cols.remove(j);
    }

    let offoff = expansion - 1;

    let mut row_offsets = Vec::with_capacity(universe.rows);
    let mut cur_row_offset = 0;
    for i in 0..universe.rows {
        if empty_rows.contains(&i) {
            cur_row_offset += offoff;
        }
        row_offsets.push(cur_row_offset);
    }

    let mut col_offsets = Vec::with_capacity(universe.cols);
    let mut cur_col_offset = 0;
    for i in 0..universe.cols {
        if empty_cols.contains(&i) {
            cur_col_offset += offoff;
        }
        col_offsets.push(cur_col_offset);
    }

    Universe {
        rows: universe.rows + cur_row_offset,
        cols: universe.cols + cur_col_offset,
        galaxies: universe
            .galaxies
            .into_iter()
            .map(|(i, j)| (i + row_offsets[i], j + col_offsets[j]))
            .collect(),
    }
}

fn parse(input: &str) -> Universe {
    let mut rows = 0;
    let mut cols = 0;
    let mut galaxies = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let line = line.trim();
        cols = line.len();
        rows += 1;
        for (j, ch) in line.chars().enumerate() {
            if ch == '#' {
                galaxies.push((i, j));
            }
        }
    }
    Universe {
        rows,
        cols,
        galaxies,
    }
}

#[derive(Debug)]
struct Universe {
    rows: usize,
    cols: usize,
    galaxies: Vec<(usize, usize)>,
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.rows {
            if i > 0 {
                writeln!(f)?;
            }
            for j in 0..self.cols {
                if self.galaxies.contains(&(i, j)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    crate::test::aoc_test!(part1, TEST_INPUT, 374);
    //crate::test::aoc_test!(part2, TEST_INPUT, "todo");
}
