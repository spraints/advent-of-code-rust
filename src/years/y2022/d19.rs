use std::{
    collections::{BinaryHeap, HashSet},
    fmt::Display,
};

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let total: usize = input
        .lines()
        .map(parse)
        .map(|bp| bp.n * quality_level(&bp, 24, vis) as usize)
        .sum();
    Box::new(total)
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    let res: usize = input
        .lines()
        .take(3)
        .map(parse)
        .map(|bp| quality_level(&bp, 32, vis) as usize)
        .product();
    Box::new(res)
}

fn quality_level(bp: &Blueprint, minutes: usize, vis: bool) -> Quality {
    if vis {
        println!("{:?}", bp);
    }
    let robots = [1, 0, 0, 0];
    let minerals = [0; 4];

    let mut to_try = BinaryHeap::new();
    to_try.push(State {
        elapsed: 0,
        minutes,
        robots,
        minerals,
    });

    let mut seen = HashSet::new();

    while let Some(st) = to_try.pop() {
        if st.elapsed == minutes {
            let geodes = st.minerals[Mineral::Geode as usize];
            if vis {
                println!(" => {} {:?}", geodes, st);
            }
            return geodes;
        }

        if seen.contains(&st) {
            continue;
        }
        seen.insert(st.clone());

        for rc in &bp.robot_costs {
            if let Some(minerals) = rc.buy(&st.minerals) {
                if vis {
                    println!(
                        "{:width$}buy {:?} for {:?} at {:?}",
                        ' ',
                        rc.produces,
                        rc.costs,
                        st,
                        width = st.elapsed
                    );
                }
                let minerals = collect(minerals, &st.robots);
                let mut robots = st.robots;
                robots[rc.produces as usize] += 1;
                to_try.push(State {
                    elapsed: st.elapsed + 1,
                    minutes: st.minutes,
                    robots,
                    minerals,
                });
            }
        }

        to_try.push(State {
            elapsed: st.elapsed + 1,
            minutes: st.minutes,
            robots: st.robots,
            minerals: collect(st.minerals, &st.robots),
        });
    }

    unreachable!("please")
}

type RobotCount = u8; // max is 24
type MineralCount = u16; // max is 24 * 24

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct State {
    elapsed: usize,
    minutes: usize,
    robots: [RobotCount; 4],
    minerals: [MineralCount; 4],
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score().cmp(&other.score())
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn score(&self) -> MineralCount {
        let remaining = (self.minutes - self.elapsed) as MineralCount;
        let per_step = (self.robots[Mineral::Geode as usize]) as MineralCount;
        let max_per_step = per_step + remaining;
        let crazy_potential = (per_step + max_per_step) * remaining / 2;
        self.minerals[Mineral::Geode as usize] + crazy_potential
    }
}

fn collect(mut minerals: [MineralCount; 4], robots: &[RobotCount]) -> [MineralCount; 4] {
    for (i, m) in minerals.iter_mut().enumerate() {
        *m += robots[i] as MineralCount;
    }
    minerals
}

type Quality = MineralCount;

#[derive(Debug)]
struct Blueprint {
    n: usize,
    robot_costs: Vec<RobotCost>,
}

#[derive(Debug)]
struct RobotCost {
    produces: Mineral,
    costs: [MineralCount; 4],
}

impl RobotCost {
    fn buy(&self, minerals: &[MineralCount]) -> Option<[MineralCount; 4]> {
        let mut res = [0; 4];
        for (i, cost) in self.costs.iter().enumerate() {
            let m = &minerals[i];
            if cost > m {
                return None;
            }
            res[i] = m - cost;
        }
        Some(res)
    }
}

#[derive(Clone, Copy, Debug)]
enum Mineral {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}

fn parse(s: &str) -> Blueprint {
    let mut words = s.split(' ');
    words.next().unwrap(); // "Blueprint"
    let n = words
        .next()
        .unwrap()
        .strip_suffix(':')
        .unwrap()
        .parse()
        .unwrap();
    let mut robot_costs = Vec::new();
    while words.next().is_some()
    /* "Each" */
    {
        let produces = parse_mineral(words.next().unwrap());
        words.next().unwrap(); // "robot"
        words.next().unwrap(); // "costs"
        let mut costs = [0; 4];
        loop {
            let cost = words.next().unwrap().parse().unwrap();
            let mineral = words.next().unwrap();
            let (mineral, finished) = match mineral.strip_suffix('.') {
                Some(m) => (m, true),
                None => (mineral, false),
            };
            let mineral = parse_mineral(mineral);
            costs[mineral as usize] = cost;
            if finished {
                break;
            } else {
                words.next().unwrap(); // "and"
            }
        }
        robot_costs.push(RobotCost { produces, costs });
    }
    Blueprint { n, robot_costs }
}

fn parse_mineral(mineral: &str) -> Mineral {
    match mineral {
        "ore" => Mineral::Ore,
        "clay" => Mineral::Clay,
        "obsidian" => Mineral::Obsidian,
        "geode" => Mineral::Geode,
        s => unreachable!("unexpected mineral {:?}", s),
    }
}

/*
 * too slow
#[cfg(test)]
mod test {
    use super::*;

    fn quality_level(s: &str) -> (usize, Quality) {
        let bp = parse(s);
        (bp.n, super::quality_level(&bp, 24, false))
    }

    crate::test::aoc_test!(example, r"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.",
        part1 => 33);

    #[test]
    fn test_quality_level() {
        assert_eq!((1,9),quality_level("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian."));
        assert_eq!((2,12),quality_level("Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."));
    }
}
*/
