use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Write};

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    if vis {
        println!("{input}");
    }
    let tiles = parse(&input);
    let visited = trace(&tiles, vis);
    Box::new(*visited.values().max().unwrap())
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    if vis {
        println!("{input}");
    }
    let tiles = parse(&input);
    let visited = trace(&tiles, false);

    let mut marked: HashMap<(usize, usize), Fin> =
        visited.keys().map(|pos| (*pos, Fin::Wall)).collect();

    for (i, row) in tiles.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            marked.entry((i, j)).or_insert_with(|| {
                let mut last_t = None;
                let mut count = 0;
                let left = &row[0..j];
                if !left.contains(&Tile::Start) {
                    // Check the tiles to the left.
                    for (_, t) in left
                        .into_iter()
                        .enumerate()
                        .filter(|(j, t)| visited.contains_key(&(i, *j)) && **t != Tile::Horizontal)
                    {
                        (last_t, count) = match (last_t, t) {
                            (None, Tile::Vertical) => (None, count + 1),
                            (None, Tile::NE) | (None, Tile::SE) => (Some(*t), count),
                            (Some(Tile::NE), Tile::SW) | (Some(Tile::SE), Tile::NW) => {
                                (None, count + 1)
                            }
                            (Some(Tile::NE), Tile::NW) | (Some(Tile::SE), Tile::SW) => {
                                (None, count)
                            }
                            _ => {
                                panic!("illegal {t:?} after {last_t:?}")
                            }
                        }
                    }
                } else {
                    // Check the tiles above.
                    let above = tiles[0..i].iter().map(|row| row[j]);
                    for (ii, t) in above
                        .enumerate()
                        .filter(|(i, t)| visited.contains_key(&(*i, j)) && *t != Tile::Vertical)
                    {
                        (last_t, count) = match (last_t, t) {
                            (None, Tile::Horizontal) => (None, count + 1),
                            (None, Tile::SW) | (None, Tile::SE) => (Some(t), count),
                            (Some(Tile::SW), Tile::NE) | (Some(Tile::SE), Tile::NW) => {
                                (None, count + 1)
                            }
                            (Some(Tile::SE), Tile::NE) | (Some(Tile::SW), Tile::NW) => {
                                (None, count)
                            }
                            _ => {
                                let mut details = String::new();
                                for (i, row) in tiles[0..i].iter().enumerate() {
                                    let t = row[j];
                                    let v = visited.get(&(i, j));
                                    writeln!(&mut details, "({i},{j}) {t:?} {v:?}").unwrap();
                                }
                                panic!("({ii},{j}) illegal {t:?} after {last_t:?}\n{details}")
                            }
                        }
                    }
                }
                // // TODO: fix this block.
                // // Right now, it only counts walls that it crosses like '|'.
                // // It needs to also count walls like 'F--J' and 'L--7'.
                // // If it encounters S, it needs to use 0..i instead.
                // let walls = (0..j)
                //     .filter(|j| visited.contains_key(&(i, *j)))
                //     .map(|j| &row[j])
                //     .filter(|t| matches!(t, Tile::Vertical))
                //     .count();
                // if i == 0 {
                //     println!("{:?} => {walls}", &row[0..=j]);
                // }
                if count % 2 == 0 {
                    Fin::O
                } else {
                    Fin::I
                }
            });
        }
    }

    if vis {
        println!("--- inner and outer ---");
        for (i, row) in tiles.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                match marked.get(&(i, j)) {
                    Some(Fin::Wall) => print!("{tile}"),
                    Some(Fin::I) => print!("I"),
                    Some(Fin::O) => print!("O"),
                    None => print!("."),
                }
            }
            println!();
        }
    }

    let res: usize = marked.values().filter(|v| matches!(v, Fin::I)).count();
    Box::new(res)
}

#[derive(Debug)]
enum Fin {
    Wall,
    I,
    O,
}

fn trace(tiles: &[Vec<Tile>], vis: bool) -> HashMap<(usize, usize), u64> {
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
    visited
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

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Tile::Vertical => "|",
            Tile::Horizontal => "-",
            Tile::NE => "L",
            Tile::NW => "J",
            Tile::SW => "7",
            Tile::SE => "F",
            Tile::Ground => ".",
            Tile::Start => "S",
        };
        write!(f, "{}", s)
    }
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

    crate::test::aoc_test!(
        part2,
        part2_1,
        r"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        4
    );

    crate::test::aoc_test!(
        part2,
        part2_2,
        r".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
        8
    );

    crate::test::aoc_test!(
        part2,
        part2_3,
        r"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
        10
    );
}
