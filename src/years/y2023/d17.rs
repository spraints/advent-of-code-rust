use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fmt::Display,
};

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, _vis: bool) -> Box<dyn Display> {
    Box::new(solve(
        &input,
        Config {
            min_run: 1,
            max_run: 3,
        },
    ))
}

pub fn part2(input: String, _vis: bool) -> Box<dyn Display> {
    Box::new(solve(
        &input,
        Config {
            min_run: 4,
            max_run: 10,
        },
    ))
}

fn solve(input: &str, cfg: Config) -> u64 {
    let parsed = parse(input);

    // solve with Dijkstra's algorithm where nodes are the individual squares
    // and edges are sets of 1..3 blocks plus a left or right turn.

    let mut dist = HashMap::new(); // assume infinity.
    let mut to_visit = BinaryHeap::new();
    dist.insert(((0, 0), Dir::Start), 0);
    to_visit.push(State {
        cost: 0,
        pos: (0, 0),
        next_dir: Dir::Right,
    });
    to_visit.push(State {
        cost: 0,
        pos: (0, 0),
        next_dir: Dir::Down,
    });

    let dest = (parsed.height - 1, parsed.width - 1);

    while let Some(State {
        cost,
        pos,
        next_dir,
    }) = to_visit.pop()
    {
        if pos == dest {
            return cost;
        }
        match dist.get(&(pos, next_dir)) {
            Some(d) if *d < cost => continue,
            _ => (),
        };

        for st in &parsed.neighbors(pos, next_dir, cost, &cfg) {
            let e = dist.entry((st.pos, st.next_dir)).or_insert_with(|| {
                to_visit.push(*st);
                st.cost
            });
            if *e > st.cost {
                *e = st.cost;
                to_visit.push(*st);
            }
        }
    }

    panic!("no path found!")
}

struct Config {
    min_run: usize,
    max_run: usize,
}

fn parse(input: &str) -> Parsed {
    let costs: Vec<Vec<u64>> = input
        .lines()
        .map(|line| {
            line.trim()
                .as_bytes()
                .iter()
                .map(|c| (c - b'0') as u64)
                .collect()
        })
        .collect();
    let height = costs.len();
    let width = costs.len();
    Parsed {
        costs,
        height,
        width,
    }
}

struct Parsed {
    costs: Vec<Vec<u64>>,
    height: usize,
    width: usize,
}

impl Parsed {
    fn neighbors(
        &self,
        pos: (usize, usize),
        dir: Dir,
        start_cost: u64,
        cfg: &Config,
    ) -> Vec<State> {
        let mut res = Vec::new();
        self._neighbors(pos, dir, start_cost, cfg.min_run, cfg.max_run, &mut res);
        res
    }

    fn _neighbors(
        &self,
        pos: (usize, usize),
        dir: Dir,
        start_cost: u64,
        min_run: usize,
        max_run: usize,
        res: &mut Vec<State>,
    ) {
        let nmr = if min_run > 0 { min_run - 1 } else { 0 };
        if max_run > 0 {
            if let Some((i, j)) = self.step(pos, dir) {
                let cost = self.costs[i][j];
                self._neighbors((i, j), dir, start_cost + cost, nmr, max_run - 1, res);
            }
        }
        if min_run == 0 {
            res.push(State {
                pos,
                cost: start_cost,
                next_dir: dir.turn_left(),
            });
            res.push(State {
                pos,
                cost: start_cost,
                next_dir: dir.turn_right(),
            });
        }
    }

    fn step(&self, pos: (usize, usize), dir: Dir) -> Option<(usize, usize)> {
        match (pos, dir) {
            ((i, j), Dir::Up) if i > 0 => Some((i - 1, j)),
            ((i, j), Dir::Down) if i + 1 < self.height => Some((i + 1, j)),
            ((i, j), Dir::Left) if j > 0 => Some((i, j - 1)),
            ((i, j), Dir::Right) if j + 1 < self.width => Some((i, j + 1)),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u64,
    pos: (usize, usize),
    next_dir: Dir,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Note that the ordering is flipped.
        // Cost is the most important, we compare the others because this needs to be != if the
        // States aren't identical.
        (other.cost, other.pos, other.next_dir).cmp(&(self.cost, self.pos, self.next_dir))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    Start,
}

impl Dir {
    fn turn_left(&self) -> Dir {
        match self {
            Dir::Up => Dir::Left,
            Dir::Left => Dir::Down,
            Dir::Down => Dir::Right,
            Dir::Right => Dir::Up,
            _ => unreachable!(),
        }
    }

    fn turn_right(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Left => Dir::Up,
            Dir::Down => Dir::Left,
            Dir::Right => Dir::Down,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    crate::test::aoc_test!(part1, TEST_INPUT, 102);
    crate::test::aoc_test!(part2, TEST_INPUT, 94);
}
