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

    let mut moves = ['n', 's', 'w', 'e'];

    for round in 0..10 {
        occupied = play_round(occupied, &moves).0;
        moves.rotate_left(1);
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

    let mut moves = ['n', 's', 'w', 'e'];

    for round in 0.. {
        if vis && round % 100 == 0 {
            print!(".");
        }
        let (new_field, any_moved) = play_round(occupied, &moves);
        if !any_moved {
            if vis {
                println!();
            }
            return Box::new(round + 1);
        }
        occupied = new_field;
        moves.rotate_left(1);
    }
    unreachable!()
}

fn play_round(init: HashSet<Coord>, moves: &[char]) -> (HashSet<Coord>, bool) {
    let moves = init
        .iter()
        .map(|elf| (elf.clone(), maybe_move(&init, elf, moves)));
    let mut res: HashMap<Coord, Coord> = HashMap::new();
    let mut off_limits = HashSet::new();
    let mut count = 0;
    for (from, to) in moves {
        if off_limits.contains(&to) {
            if let Some(move_back) = res.remove(&to) {
                count -= 1;
                res.insert(move_back, move_back);
            }
            res.insert(from, from);
        } else {
            if from != to {
                count += 1;
            }
            off_limits.insert(to);
            res.insert(to, from);
        }
    }
    (res.keys().copied().collect(), count > 0)
}

fn maybe_move(field: &HashSet<Coord>, elf: &Coord, moves: &[char]) -> Coord {
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
        return (r, c);
    }
    for m in moves {
        match m {
            'n' => {
                if !nw && !n && !ne {
                    return (r - 1, c);
                }
            }
            's' => {
                if !sw && !s && !se {
                    return (r + 1, c);
                }
            }
            'e' => {
                if !ne && !e && !se {
                    return (r, c + 1);
                }
            }
            'w' => {
                if !nw && !w && !sw {
                    return (r, c - 1);
                }
            }
            _ => unreachable!(),
        };
    }
    (r, c)
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
        part2 => 20);
}
