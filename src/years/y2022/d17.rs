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
    let cavern_width = 7;
    for i in 0..2022 {
        rock_fall(
            &mut cavern,
            cavern_width,
            &rocks[i % rocks.len()],
            &mut puffs,
            vis,
        );
        if vis && i < 12 {
            print_cavern(&cavern);
        }
    }
    Box::new(tower_height(&cavern))
}

fn rock_fall<'a, I: Iterator<Item = Puff>>(
    cavern: &mut Cavern,
    cavern_width: usize,
    rock: &Rock,
    puffs: &'a mut I,
    vis: bool,
) {
    let need = 4 - empty_space(cavern);
    if vis {
        println!("add {} rows to cavern", need);
    }
    cavern.0.resize(
        cavern.0.len() + need,
        vec![Default::default(); cavern_width],
    );

    #[derive(Clone, Copy, Debug)]
    struct State {
        height: usize,
        from_left: usize,
    }

    fn puff(puff: Puff, cavern: &Cavern, rock: &Rock, state: State, cavern_width: usize) -> State {
        match puff {
            Puff::Left => {
                if state.from_left == 0 {
                    return state;
                }
                let maybe = State {
                    from_left: state.from_left - 1,
                    ..state
                };
                if collides(cavern, rock, maybe) {
                    return state;
                } else {
                    return maybe;
                }
            }
            Puff::Right => {
                if state.from_left + rock.width() == cavern_width {
                    return state;
                }
                let maybe = State {
                    from_left: state.from_left + 1,
                    ..state
                };
                if collides(cavern, rock, maybe) {
                    return state;
                } else {
                    return maybe;
                }
            }
        };
    }

    fn collides(cavern: &Cavern, rock: &Rock, state: State) -> bool {
        for (i, row) in rock.0.iter().rev().enumerate() {
            for (j, x) in row.iter().enumerate() {
                if matches!(x, Space::Filled)
                    && matches!(
                        cavern
                            .0
                            .get(state.height + i)
                            .map(|cr| cr[state.from_left + j]),
                        Some(Space::Filled)
                    )
                {
                    return true;
                }
            }
        }
        false
    }

    let mut state = State {
        height: cavern.0.len() - 1,
        from_left: 2,
    };

    loop {
        let p = puffs.next().unwrap();
        state = puff(p, cavern, rock, state, cavern_width);
        if vis {
            println!("{:?} => {:?}", p, state);
        }
        if state.height == 0
            || collides(
                cavern,
                rock,
                State {
                    height: state.height - 1,
                    ..state
                },
            )
        {
            let State { height, from_left } = state;
            for (i, row) in rock.0.iter().rev().enumerate() {
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
        state.height -= 1;
        if vis {
            println!("... fall");
        }
    }
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
