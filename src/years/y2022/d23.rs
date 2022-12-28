use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

type Coord = (isize, isize);

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let mut occupied: HashSet<Coord> = input
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '#')
                .map(move |(c, _)| (r as isize, c as isize))
        })
        .collect();

    if vis {
        println!("initial state");
        print_field(&occupied);
    }

    for round in 0..10 {
        occupied = play_round(occupied);
        if vis {
            println!();
            println!("after round {}", round + 1);
            print_field(&occupied);
        }
    }
    let (r_min, r_max, c_min, c_max) = extents(&occupied);
    let area = (r_max - r_min + 1) * (c_max - c_min + 1);
    Box::new(area as usize - occupied.len())
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

fn play_round(init: HashSet<Coord>) -> HashSet<Coord> {
    let moves = init.iter().map(|elf| (elf.clone(), maybe_move(&init, elf)));
    let mut res: HashMap<Coord, Coord> = HashMap::new();
    let mut off_limits = HashSet::new();
    for (from, to) in moves {
        if off_limits.contains(&to) {
            if let Some(move_back) = res.remove(&to) {
                res.insert(move_back, move_back);
            }
            res.insert(from, from);
        } else {
            off_limits.insert(to);
            res.insert(to, from);
        }
    }
    res.keys().copied().collect()
}

fn maybe_move(field: &HashSet<Coord>, elf: &Coord) -> Coord {
    let (r, c) = (elf.0, elf.1);
    let n = field.contains(&(r - 1, c));
    let ne = field.contains(&(r - 1, c + 1));
    let e = field.contains(&(r, c + 1));
    let se = field.contains(&(r + 1, c + 1));
    let s = field.contains(&(r + 1, c));
    let sw = field.contains(&(r + 1, c - 1));
    let w = field.contains(&(r, c - 1));
    let nw = field.contains(&(r - 1, c - 1));
    if !n && !ne && !e && !se && !s && !sw && !w && !nw {
        (r, c)
    } else if !nw && !n && !ne {
        (r - 1, c)
    } else if !sw && !s && !se {
        (r + 1, c)
    } else if !nw && !w && !sw {
        (r, c - 1)
    } else if !ne && !e && !se {
        (r, c + 1)
    } else {
        // unreachable!()
        (r, c)
    }
}

fn extents(field: &HashSet<Coord>) -> (isize, isize, isize, isize) {
    let rows: Vec<isize> = field.iter().map(|(r, _)| r).copied().collect();
    let cols: Vec<isize> = field.iter().map(|(_, c)| c).copied().collect();
    let r_min = rows.iter().min().unwrap();
    let r_max = rows.iter().max().unwrap();
    let c_min = cols.iter().min().unwrap();
    let c_max = cols.iter().max().unwrap();
    (*r_min, *r_max, *c_min, *c_max)
}

fn print_field(field: &HashSet<Coord>) {
    let (r_min, r_max, c_min, c_max) = extents(field);
    for r in r_min..=r_max {
        for c in c_min..=c_max {
            if field.contains(&(r, c)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    crate::test::aoc_test!(example, r"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..",
        part1 => 110,
        part2 => "todo");
}
