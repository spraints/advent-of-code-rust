use std::collections::HashMap;
use std::fmt::Display;

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, _vis: bool) -> Box<dyn Display> {
    let parsed = parse(&input);
    let mut visited: HashMap<(isize, isize), Vec<Dir>> = HashMap::new();
    let mut cur = vec![(Dir::Right, (0, 0))];
    while let Some((dir, pos)) = cur.pop() {
        let e = visited.entry(pos).or_insert_with(Vec::new);
        if !e.contains(&dir) {
            e.push(dir);
            for (new_dir, new_pos) in parsed.step(pos, dir) {
                if parsed.is_valid(new_pos) {
                    cur.push((new_dir, new_pos));
                }
            }
        }
    }
    Box::new(visited.len())
}

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

impl Parsed {
    fn step(&self, pos: (isize, isize), dir: Dir) -> Vec<(Dir, (isize, isize))> {
        match (self.spaces.get(&pos), dir) {
            (None, dir) => vec![go_from(pos, dir)],

            (Some(Device::VSplit), Dir::Right | Dir::Left) => {
                vec![go_from(pos, Dir::Up), go_from(pos, Dir::Down)]
            }
            (Some(Device::VSplit), Dir::Up | Dir::Down) => vec![go_from(pos, dir)],

            (Some(Device::HSplit), Dir::Up | Dir::Down) => {
                vec![go_from(pos, Dir::Left), go_from(pos, Dir::Right)]
            }
            (Some(Device::HSplit), Dir::Left | Dir::Right) => vec![go_from(pos, dir)],

            (Some(Device::NWMirror), Dir::Left) => vec![go_from(pos, Dir::Up)],
            (Some(Device::NWMirror), Dir::Down) => vec![go_from(pos, Dir::Right)],
            (Some(Device::NWMirror), Dir::Right) => vec![go_from(pos, Dir::Down)],
            (Some(Device::NWMirror), Dir::Up) => vec![go_from(pos, Dir::Left)],

            (Some(Device::NEMirror), Dir::Left) => vec![go_from(pos, Dir::Down)],
            (Some(Device::NEMirror), Dir::Down) => vec![go_from(pos, Dir::Left)],
            (Some(Device::NEMirror), Dir::Right) => vec![go_from(pos, Dir::Up)],
            (Some(Device::NEMirror), Dir::Up) => vec![go_from(pos, Dir::Right)],
        }
    }

    fn is_valid(&self, pos: (isize, isize)) -> bool {
        let (r, c) = pos;
        r >= 0 && c >= 0 && r < self.rows && c < self.cols
    }
}

fn go_from(pos: (isize, isize), new_dir: Dir) -> (Dir, (isize, isize)) {
    (new_dir, new_dir.step(pos))
}

impl Dir {
    fn step(&self, pos: (isize, isize)) -> (isize, isize) {
        let (r, c) = pos;
        match self {
            Dir::Right => (r, c + 1),
            Dir::Left => (r, c - 1),
            Dir::Up => (r - 1, c),
            Dir::Down => (r + 1, c),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Dir {
    Right,
    Left,
    Up,
    Down,
}

struct Parsed {
    rows: isize,
    cols: isize,
    spaces: HashMap<(isize, isize), Device>,
}

enum Device {
    VSplit,   // '|'
    HSplit,   // '-'
    NWMirror, // '\'
    NEMirror, // '/'
}

fn parse(input: &str) -> Parsed {
    let mut rows = 0;
    let mut cols = 0;
    let mut spaces = HashMap::new();
    for (row, line) in input.lines().enumerate() {
        let row = row as isize;
        rows = row + 1;
        for (col, ch) in line.trim().chars().enumerate() {
            let col = col as isize;
            cols = col + 1;
            match ch {
                '|' => spaces.insert((row, col), Device::VSplit),
                '-' => spaces.insert((row, col), Device::HSplit),
                '\\' => spaces.insert((row, col), Device::NWMirror),
                '/' => spaces.insert((row, col), Device::NEMirror),
                '.' => None,
                _ => unreachable!("illegal char {ch:?}"),
            };
        }
    }
    Parsed { rows, cols, spaces }
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    crate::test::aoc_test!(part1, TEST_INPUT, 46);
    crate::test::aoc_test!(part2, TEST_INPUT, "todo");
}
