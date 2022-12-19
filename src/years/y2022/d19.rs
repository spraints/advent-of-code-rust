use std::{collections::HashSet, fmt::Display};

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let total: usize = input
        .lines()
        .map(parse)
        .map(|bp| bp.n * quality_level(&bp, 24, vis) as usize)
        .sum();
    Box::new(total)
}

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

fn quality_level(bp: &Blueprint, minutes: usize, vis: bool) -> Quality {
    if vis {
        println!("{:?}", bp);
    }
    let robots = [1, 0, 0, 0];
    let minerals = [0; 4];
    let mut seen = HashSet::new();

    let mut to_try = Vec::new();
    to_try.push(State {
        elapsed: 0,
        robots,
        minerals,
    });

    let mut best = 0;
    while let Some(st) = to_try.pop() {
        if st.elapsed == minutes {
            let geodes = st.minerals[Mineral::Geode as usize];
            println!(" => {} {:?}", geodes, st);
            best = best.max(geodes);
            continue;
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
                let mut robots = st.robots.clone();
                robots[rc.produces as usize] += 1;
                to_try.push(State {
                    elapsed: st.elapsed + 1,
                    robots,
                    minerals,
                });
            }
        }

        to_try.push(State {
            elapsed: st.elapsed + 1,
            robots: st.robots.clone(),
            minerals: collect(st.minerals.clone(), &st.robots),
        });
    }

    best
}

type RobotCount = u8; // max is 24
type MineralCount = u16; // max is 24 * 24

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct State {
    elapsed: usize,
    robots: [RobotCount; 4],
    minerals: [MineralCount; 4],
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

#[cfg(test)]
mod test {
    use super::*;

    fn quality_level(s: &str) -> (usize, Quality) {
        let bp = parse(s);
        (bp.n, super::quality_level(&bp, 24, true))
    }

    crate::test::aoc_test!(example, r"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.",
        part1 => 33,
        part2 => "todo");

    #[test]
    fn test_quality_level() {
        assert_eq!((1,9),quality_level("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian."));
        assert_eq!((2,12),quality_level("Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."));
    }
}
