use std::collections::HashMap;
use std::fmt::Display;

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let g = parse(&input);

    let g = tilt_north(g);

    if vis {
        for line in &g.platform {
            let line: String = line.iter().collect();
            println!("{line}");
        }
    }

    Box::new(load(g))
}

pub fn part2(input: String, _vis: bool) -> Box<dyn Display> {
    let mut g = parse(&input);

    const CYCLES: usize = 1000000000;
    let mut positions = HashMap::new();
    let mut i = 0;
    let mut found = false;
    loop {
        i += 1;
        g = tilt_north(g);
        g = tilt_west(g);
        g = tilt_south(g);
        g = tilt_east(g);
        if !found {
            match positions.get(&g.platform).cloned() {
                None => {
                    positions.insert(g.platform.clone(), i);
                }
                Some(j) => {
                    let repeat_interval = i - j;
                    i += repeat_interval * ((CYCLES - i) / repeat_interval);
                    found = true;
                }
            };
        }
        if i < CYCLES {
            continue;
        } else if i == CYCLES {
            return Box::new(load(g));
        } else {
            panic!("huh?!?! {i} vs {CYCLES}");
        }
    }
}

fn load(g: Parsed) -> usize {
    g.platform
        .iter()
        .enumerate()
        .map(|(i, line)| (g.height - i) * (line.iter().filter(|c| matches!(c, 'O')).count()))
        .sum()
}

fn tilt_north(g: Parsed) -> Parsed {
    let Parsed {
        mut platform,
        height,
        width,
    } = g;
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
    Parsed {
        platform,
        height,
        width,
    }
}

fn tilt_south(g: Parsed) -> Parsed {
    let Parsed {
        mut platform,
        height,
        width,
    } = g;
    let mut moved = true;
    while moved {
        moved = false;
        for i in (1..height).rev() {
            for j in 0..width {
                if matches!((platform[i - 1][j], platform[i][j]), ('O', '.')) {
                    moved = true;
                    platform[i - 1][j] = '.';
                    platform[i][j] = 'O';
                }
            }
        }
    }
    Parsed {
        platform,
        height,
        width,
    }
}

fn tilt_west(g: Parsed) -> Parsed {
    let Parsed {
        mut platform,
        height,
        width,
    } = g;
    let mut moved = true;
    while moved {
        moved = false;
        for j in 1..width {
            for i in 0..height {
                if matches!((platform[i][j - 1], platform[i][j]), ('.', 'O')) {
                    moved = true;
                    platform[i][j - 1] = 'O';
                    platform[i][j] = '.';
                }
            }
        }
    }
    Parsed {
        platform,
        height,
        width,
    }
}

fn tilt_east(g: Parsed) -> Parsed {
    let Parsed {
        mut platform,
        height,
        width,
    } = g;
    let mut moved = true;
    while moved {
        moved = false;
        for j in (1..width).rev() {
            for i in 0..height {
                if matches!((platform[i][j - 1], platform[i][j]), ('O', '.')) {
                    moved = true;
                    platform[i][j - 1] = '.';
                    platform[i][j] = 'O';
                }
            }
        }
    }
    Parsed {
        platform,
        height,
        width,
    }
}

fn parse(input: &str) -> Parsed {
    let platform: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();
    let height = platform.len();
    let width = platform[0].len();
    Parsed {
        platform,
        width,
        height,
    }
}

struct Parsed {
    platform: Vec<Vec<char>>,
    height: usize,
    width: usize,
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
    crate::test::aoc_test!(part2, TEST_INPUT, 64);
}
