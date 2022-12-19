use std::{collections::HashMap, fmt::Display};

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let total: usize = input
        .lines()
        .map(parse)
        .map(|bp| bp.n * quality_level(&bp, 24, vis))
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
    //let minutes = minutes - minutes + 10; // XXX
    let mut robots = [1, 0, 0, 0];
    let minerals = [0; 4];
    let mut cache = HashMap::new();
    ql2(bp, minutes, &mut robots, minerals, &mut cache, vis)
}

fn ql2(
    bp: &Blueprint,
    minutes: usize,
    robots: &mut [usize],
    minerals: [usize; 4],
    cache: &mut HashMap<State, Quality>,
    vis: bool,
) -> Quality {
    if minutes < 1 {
        println!("  => {:?}", minerals);
        return minerals[Mineral::Geode as usize];
    }

    let st = state(minutes, robots, &minerals);
    if let Some(res) = cache.get(&st) {
        if vis {
            println!(
                "[{}] minutes={} already tried robots={:?} minerals={:?} => {}",
                bp.n, minutes, robots, minerals, res
            );
        }
        return *res;
    }
    if vis {
        println!(
            "[{}] minutes={} try robots={:?} minerals={:?}",
            bp.n, minutes, robots, minerals
        );
    }

    let step = robots[Mineral::Geode as usize];
    let mut best = 0;

    for rc in &bp.robot_costs {
        if let Some(minerals) = rc.buy(&minerals) {
            if vis {
                println!("    minutes={} try buying {:?}", minutes, rc);
            }
            let minerals = collect(minerals, robots);
            robots[rc.produces as usize] += 1;
            best = best.max(ql2(bp, minutes - 1, robots, minerals, cache, vis));
            robots[rc.produces as usize] -= 1;
        }
    }

    best = best.max(ql2(
        bp,
        minutes - 1,
        robots,
        collect(minerals, robots),
        cache,
        vis,
    ));

    best += step;

    cache.insert(st, best);
    best
}

fn collect(mut minerals: [usize; 4], robots: &[usize]) -> [usize; 4] {
    for (i, m) in minerals.iter_mut().enumerate() {
        *m += robots[i];
    }
    minerals
}

type State = (usize, [usize; 4], [usize; 4]);

fn state(minutes: usize, robots: &[usize], minerals: &[usize]) -> State {
    (
        minutes,
        [robots[0], robots[1], robots[2], robots[3]],
        [minerals[0], minerals[1], minerals[2], minerals[3]],
    )
}

type Quality = usize;

#[derive(Debug)]
struct Blueprint {
    n: usize,
    robot_costs: Vec<RobotCost>,
}

#[derive(Debug)]
struct RobotCost {
    produces: Mineral,
    costs: [usize; 4],
}

impl RobotCost {
    fn buy(&self, minerals: &[usize]) -> Option<[usize; 4]> {
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
