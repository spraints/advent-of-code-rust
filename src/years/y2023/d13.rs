use std::{fmt::Display, iter::zip};

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let mut res = 0;
    for pattern in input.split("\n\n") {
        if vis {
            println!("{pattern}");
        }
        let pattern: Vec<&[u8]> = pattern.lines().map(|l| l.trim().as_bytes()).collect();
        for row in 0..pattern.len() - 1 {
            if is_v(&pattern, row) {
                let row = row + 1;
                if vis {
                    println!(" v {row}");
                }
                res += row * 100;
            }
        }
        for col in 0..pattern[0].len() - 1 {
            if is_h(&pattern, col) {
                let col = col + 1;
                if vis {
                    println!(" > {col}");
                }
                res += col;
            }
        }
    }
    Box::new(res)
}

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

fn is_v(pattern: &[&[u8]], row: usize) -> bool {
    // 0..=row is one size, row+1..pattern.len() is the other side.
    let (top, bottom) = pattern.split_at(row + 1);
    zip(top.iter().rev(), bottom.iter()).all(|(a, b)| a == b)
}

fn is_h(pattern: &[&[u8]], col: usize) -> bool {
    pattern.iter().all(|row| {
        let (left, right) = row.split_at(col + 1);
        zip(left.iter().rev(), right.iter()).all(|(a, b)| a == b)
    })
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    crate::test::aoc_test!(part1, TEST_INPUT, 405);
    crate::test::aoc_test!(part2, TEST_INPUT, "todo");
}
