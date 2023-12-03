use std::collections::HashSet;
use std::fmt::Display;

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let things = parse(&input);

    let mut symbols = HashSet::new();
    for t in &things {
        if matches!(t.th, TH::Symbol(..)) {
            symbols.insert((t.row, t.col));
        }
    }
    if vis {
        println!("symbols at {symbols:?}");
    }
    let mut sum = 0;
    'things: for t in &things {
        if let TH::Number(n) = t.th {
            if vis {
                println!("is {n} ({}, {}) near a symbol?", t.row, t.col);
            }
            let min_row = if t.row > 0 { t.row - 1 } else { t.row };
            let max_row = t.row + 1;
            let min_col = if t.col > 0 { t.col - 1 } else { t.col };
            let max_col = t.col + t.len;
            for r in min_row..=max_row {
                for c in min_col..=max_col {
                    if symbols.contains(&(r, c)) {
                        sum += n;
                        if vis {
                            println!("yes! ({r}, {c}) sum is now {sum}");
                        }
                        continue 'things;
                    }
                }
            }
            if vis {
                println!("no!");
            }
        }
    }
    Box::new(sum)
}

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

fn parse(input: &str) -> Vec<Thing> {
    let n_re = regex::Regex::new("[0-9]+").unwrap();
    let sym_re = regex::Regex::new("[^0-9.]").unwrap();
    let mut things = Vec::new();
    for (row, line) in input.lines().enumerate() {
        for m in n_re.find_iter(line) {
            things.push(Thing {
                row,
                col: m.start(),
                len: m.len(),
                th: TH::Number(m.as_str().parse().unwrap()),
            });
        }
        for m in sym_re.find_iter(line) {
            things.push(Thing {
                row,
                col: m.start(),
                len: 1,
                th: TH::Symbol(m.as_str().chars().next().unwrap()),
            });
        }
    }
    things
}

#[derive(Debug)]
struct Thing {
    row: usize,
    col: usize,
    len: usize,
    th: TH,
}

#[derive(Debug)]
enum TH {
    Symbol(char),
    Number(u32),
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    crate::test::aoc_test!(part1, TEST_INPUT, 4361);
    crate::test::aoc_test!(part2, TEST_INPUT, "todo");
}
