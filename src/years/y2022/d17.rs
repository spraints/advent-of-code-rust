use std::{fmt::Display, marker::PhantomData};

const ROCKS: &str = r"####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##";

type Rock = Vec<Vec<Option<()>>>;
fn parse_rock(rs: &str) -> Rock {
    rs.lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => None,
                    '#' => Some(()),
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

#[derive(Debug)]
enum Puff {
    Left,
    Right,
}

fn parse_puff(c: char) -> Puff {
    match c {
        '>' => Puff::Right,
        '<' => Puff::Left,
        _ => unreachable!(),
    }
}

type Cavern = Vec<Vec<Option<()>>>;

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let puffs: Vec<Puff> = input.trim().chars().map(parse_puff).collect();
    let rocks: Vec<Rock> = ROCKS.split("\n\n").map(parse_rock).collect();
    let mut cavern = vec![vec![None; 7]; 3];
    let mut puffs = forever(puffs);
    for (i, r) in forever(rocks).take(2022).enumerate() {
        rock_fall(&mut cavern, r, &mut puffs);
        if vis && i < 12 {
            print_cavern(&cavern);
        }
    }
    Box::new(tower_height(&cavern))
}

fn rock_fall<'a, I: Iterator<Item = &'a Puff>>(cavern: &mut Cavern, rock: &Rock, puffs: &'a mut I) {
    // todo
}

fn tower_height(cavern: &Cavern) -> usize {
    cavern.len() - empty_space(cavern)
}

fn empty_space(cavern: &Cavern) -> usize {
    for (i, row) in cavern.iter().enumerate() {
        if row.iter().any(Option::is_some) {
            return i;
        }
    }
    cavern.len()
}

fn print_cavern(cavern: &Cavern) {
    for r in cavern.iter().rev() {
        print!("|");
        for c in r {
            match c {
                Some(..) => print!("."),
                None => print!("#"),
            };
        }
        println!("|");
    }
    println!("+-------+");
}

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

struct Forever<'a, T> {
    items: Vec<T>,
    len: usize,
    i: usize,
    _x: PhantomData<&'a T>,
}

fn forever<T>(items: Vec<T>) -> Forever<'_, T> {
    Forever {
        items,
        i: 0,
        len: items.len(),
        _x: PhantomData,
    }
}

impl<'a, T> Iterator for Forever<'a, T> {
    type Item = &'a T;

    fn next(&'a mut self) -> Option<Self::Item> {
        let i = self.i;
        self.i += 1;
        Some(&self.items[i % self.len])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    crate::test::aoc_test!(example, r">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>",
        part1 => 3068,
        part2 => "todo");
}
