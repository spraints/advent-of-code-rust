use std::fmt::Display;

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

#[derive(Default, Debug, Clone, Copy)]
enum Space {
    #[default]
    Empty,
    Filled,
}

impl Space {
    fn c(&self, filled: char) -> char {
        match self {
            Space::Empty => '.',
            Space::Filled => filled,
        }
    }

    fn is_none(&self) -> bool {
        matches!(self, Space::Empty)
    }

    fn is_filled(&self) -> bool {
        matches!(self, Space::Filled)
    }
}

struct Rock(Vec<Vec<Space>>);
fn parse_rock(rs: &str) -> Rock {
    Rock(
        rs.lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => Space::Empty,
                        '#' => Space::Filled,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect(),
    )
}

impl Rock {
    fn width(&self) -> usize {
        self.0[0].len()
    }
}

impl Rock {
    fn bottom(&self) -> &[Space] {
        &self.0[self.0.len() - 1]
    }
}

impl Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn fmt_row(row: &Vec<Space>) -> String {
            row.iter().map(|x| x.c('#')).collect()
        }
        let rows: Vec<String> = self.0.iter().map(fmt_row).collect();
        write!(f, "{}", rows.join("/"))
    }
}

#[derive(Debug, Clone, Copy)]
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

struct Cavern(Vec<Vec<Space>>);

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let puffs: Vec<Puff> = input.trim().chars().map(parse_puff).collect();
    let rocks: Vec<Rock> = ROCKS.split("\n\n").map(parse_rock).collect();
    //let mut cavern = vec![vec![None; 7]; 3];
    let mut cavern = Cavern(Vec::new());
    let mut puffs = forever(puffs);
    for i in 0..2022 {
        rock_fall(&mut cavern, &rocks[i % rocks.len()], &mut puffs, vis);
        if vis && i < 12 {
            print_cavern(&cavern);
        }
    }
    Box::new(tower_height(&cavern))
}

fn rock_fall<'a, I: Iterator<Item = Puff>>(
    cavern: &mut Cavern,
    rock: &Rock,
    puffs: &'a mut I,
    vis: bool,
) {
    let need = 4 - empty_space(cavern);
    println!("add {} rows to cavern", need);
    cavern
        .0
        .resize(cavern.0.len() + need, vec![Default::default(); 7]);
    let mut height = cavern.0.len() - 1;
    let mut from_left = 2;
    let max_from_left = 7 - rock.width();
    loop {
        //if vis && cavern.0.len() < 10 {
        //    print_falling_rock(cavern, rock, height, from_left);
        //}
        let puff = puffs.next().unwrap();
        from_left = match (puff, from_left, max_from_left - from_left) {
            (Puff::Left, 0, _) => 0,
            (Puff::Left, from_left, _) => from_left - 1,
            (Puff::Right, from_left, 0) => from_left,
            (Puff::Right, from_left, _) => from_left + 1,
        };
        assert!(
            from_left + rock.width() <= 7,
            "puff={:?} from_left={} rock_width={}",
            puff,
            from_left,
            rock.width(),
        );
        if height == 0 || is_landed(cavern, rock, height, from_left) {
            for (i, row) in rock.0.iter().enumerate() {
                for (j, x) in row.iter().enumerate() {
                    if x.is_filled() {
                        assert!(
                            cavern.0[height + i][from_left + j].is_none(),
                            "expected empty space at ({},{}) height={} from_left={} rock={}",
                            i,
                            j,
                            height,
                            from_left,
                            rock,
                        );
                        cavern.0[height + i][from_left + j] = *x;
                    }
                }
            }
            return;
        }
        height -= 1;
        if vis {
            println!("... fall");
        }
    }
}

fn is_landed(cavern: &Cavern, rock: &Rock, height: usize, from_left: usize) -> bool {
    println!("height={} from_left={}", height, from_left);
    for (i, x) in rock.bottom().iter().enumerate() {
        println!(
            "  i={} x={:?} c[h]={:?} c[h+1]={:?}",
            i,
            x,
            cavern.0.get(height - 1).map(|cr| cr.get(from_left + i)),
            cavern.0.get(height).map(|cr| cr.get(from_left + i))
        );
        match (
            x,
            cavern.0[height - 1][from_left + i],
            cavern.0.get(height).map(|cr| cr[from_left + i]),
        ) {
            (Space::Empty, Space::Filled, _) => return true,
            (Space::Filled, _, Some(Space::Filled)) => return true,
            _ => (),
        };
    }
    println!("  NO CONTACT");
    false
}

fn tower_height(cavern: &Cavern) -> usize {
    cavern.0.len() - empty_space(cavern)
}

fn empty_space(cavern: &Cavern) -> usize {
    for (i, row) in cavern.0.iter().rev().enumerate() {
        if row.iter().any(Space::is_filled) {
            return i;
        }
    }
    cavern.0.len()
}

fn print_cavern(cavern: &Cavern) {
    for r in cavern.0.iter().rev() {
        print!("|");
        for c in r {
            print!("{}", c.c('#'));
        }
        println!("|");
    }
    println!("+-------+");
}

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

struct Forever<T> {
    items: Vec<T>,
    len: usize,
    i: usize,
}

fn forever<T>(items: Vec<T>) -> Forever<T> {
    let len = items.len();
    Forever { items, i: 0, len }
}

impl<T: Copy> Iterator for Forever<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        self.i += 1;
        Some(self.items[i % self.len])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    crate::test::aoc_test!(example, r">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>",
        part1 => 3068,
        part2 => "todo");
}
