use std::fmt::Display;

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let mut platform: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();
    let height = platform.len();
    let width = platform[0].len();

    let mut moved = true;
    while moved {
        moved = false;
        for i in 1..height {
            for j in 0..width {
                if matches!((platform[i - 1][j], platform[i][j]), ('.', 'O')) {
                    moved = true;
                    platform[i - 1][j] = 'O';
                    platform[i][j] = '.';
                }
            }
        }
    }

    if vis {
        for line in &platform {
            let line: String = line.iter().collect();
            println!("{line}");
        }
    }

    let load: usize = platform
        .iter()
        .enumerate()
        .map(|(i, line)| (height - i) * (line.iter().filter(|c| matches!(c, 'O')).count()))
        .sum();
    Box::new(load)
}

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    crate::test::aoc_test!(part1, TEST_INPUT, 136);
    crate::test::aoc_test!(part2, TEST_INPUT, "todo");
}
