use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fmt::Display,
    io::Write,
};

fn write_dot(valves: &[Valve]) -> std::io::Result<()> {
    let mut f = std::fs::File::create("d16.dot")?;
    writeln!(f, "digraph G {{")?;
    //writeln!(f, "rankdir=LR;")?;
    for v in valves {
        if v.name == "AA" {
            writeln!(
                f,
                "{} [label=\"{}\\nrate={}\",shape=box];",
                v.name, v.name, v.rate
            )?;
        } else if v.rate > 0 {
            writeln!(
                f,
                "{} [label=\"{}\\nrate={}\",color=red];",
                v.name, v.name, v.rate
            )?;
        } else {
            writeln!(f, "{} [label=\"{}\"];", v.name, v.name)?;
        }
    }
    for v in valves {
        for n in &v.neighbors {
            writeln!(f, "{} -> {};", v.name, n)?;
        }
    }
    writeln!(f, "}}")
}

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    Box::new(solve(input, vis, 30, 1))
}

// 2105 is too low
pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    Box::new(solve(input, vis, 26, 2))
}

#[derive(Eq, Debug)]
struct State {
    possible: Flow,
    actual: Flow,
    minutes_remaining: Flow,
    actors: Vec<Act>,
    visited: Visited,
    paths: String,
}

impl State {
    fn cmpkey(&self) -> (Flow, Flow, Flow) {
        (self.possible, self.actual, self.minutes_remaining)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmpkey().cmp(&other.cmpkey())
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        // compare everything except path
        self.possible == other.possible
            && self.actual == other.actual
            && self.minutes_remaining == other.minutes_remaining
            && self.actors == other.actors
            && self.visited == other.visited
    }
}

struct Game {
    valves: Vec<Valve>,
    vindices: HashMap<String, usize>,
    dists: Vec<Vec<Option<usize>>>,
}

type Visited = u64;

fn new_visited(size: usize) -> Visited {
    assert!(
        size < Visited::BITS as usize,
        "cannot track {} nodes in {} bits",
        size,
        Visited::BITS
    );
    0
}

fn set_visited(visited: &Visited, i: usize) -> Visited {
    assert!(i < Visited::BITS as usize);
    visited | (1 << i)
}

fn is_visited(visited: &Visited, i: usize) -> bool {
    visited & (1 << i) != 0
}

fn solve(input: String, vis: bool, minutes: Flow, actors: usize) -> Flow {
    if vis {
        println!("~~~ 2022 day 16, minutes={} actors={} ~~~", minutes, actors);
    }
    let (valves, vindices) = parse_input(input);
    let dists = find_distances(&valves, &vindices, vis);

    if vis {
        write_dot(&valves).unwrap();
    }

    let game = Game {
        valves,
        vindices,
        dists,
    };

    let mut states = BinaryHeap::new();
    let visited = new_visited(game.valves.len());
    let i = game.vindices["AA"];
    let actors = vec![Act::Idle { i }; actors];
    let possible = possible_flow(minutes, &actors, &game, &visited);
    states.push(State {
        possible,
        actual: 0,
        minutes_remaining: minutes,
        actors,
        visited,
        paths: "".to_string(),
    });

    let mut steps = 0;

    while let Some(st) = states.pop() {
        steps += 1;
        let State {
            possible,
            actual,
            minutes_remaining,
            actors,
            visited,
            paths,
        } = st;
        if vis {
            let visited_names: Vec<&str> = game
                .valves
                .iter()
                .enumerate()
                .filter_map(|(i, v)| {
                    if is_visited(&visited, i) {
                        Some(&*v.name)
                    } else {
                        None
                    }
                })
                .collect();
            let actors: Vec<String> = actors
                .iter()
                .map(|a| match a {
                    Act::Idle { i } => format!("@{}", game.valves[*i].name),
                    Act::Going { dest, arrive_at } => {
                        format!("->{},{}", game.valves[*dest].name, arrive_at)
                    }
                })
                .collect();
            println!(
                "@{} possible={} actual={} actors={} visited={}",
                minutes_remaining,
                possible,
                actual,
                actors.join(","),
                visited_names.join(",")
            );
        }
        if possible == actual {
            if vis {
                println!("optimal path ({} steps):", steps);
                println!("{}", paths);
            }
            return possible;
        }
        if all_moving(&actors) {
            let earliest_arrive_at = actors.iter().map(Act::must_arrive_at).max().unwrap();
            let actors = actors
                .into_iter()
                .map(|a| match a {
                    Act::Going { dest, arrive_at } if arrive_at == earliest_arrive_at => {
                        Act::Idle { i: dest }
                    }
                    a => a,
                })
                .collect();
            states.push(State {
                possible,
                actual,
                minutes_remaining: earliest_arrive_at,
                actors,
                visited,
                paths,
            });
            continue;
        }
        let mut seen_actors = vec![false; game.valves.len()];
        for (actor_i, actor) in actors.iter().enumerate() {
            if let Act::Idle { i: loc } = actor {
                if seen_actors[*loc] {
                    continue;
                }
                seen_actors[*loc] = true;

                for step in possible_dests(*loc, minutes_remaining, &visited, &game) {
                    let Step {
                        dest,
                        arrive_at,
                        added_flow,
                    } = step;
                    let visited = set_visited(&visited, dest);
                    let mut actors = actors.clone();
                    actors[actor_i] = Act::Going { dest, arrive_at };
                    let actual = actual + added_flow;
                    let possible =
                        actual + possible_flow(minutes_remaining, &actors, &game, &visited);
                    states.push(State {
                        possible,
                        actual,
                        minutes_remaining,
                        actors,
                        visited,
                        paths: format!(
                            "{}[{}] -> {} rate={} arrive_at={} added_flow={}\n",
                            paths,
                            actor_i,
                            game.valves[dest].name,
                            game.valves[dest].rate,
                            arrive_at,
                            added_flow
                        ),
                    });
                }
            }
        }
        states.push(State {
            possible: actual,
            actual,
            minutes_remaining,
            actors,
            visited,
            paths,
        });
    }
    unreachable!();
}

fn possible_flow(minutes_remaining: Flow, actors: &[Act], game: &Game, visited: &Visited) -> Flow {
    let x = game.valves.iter();
    let x = x.enumerate();
    let x = x.filter(|(i, v)| !is_visited(visited, *i) && v.rate > 0);
    let x = x.map(|(to, v)| {
        let a = actors.iter();
        let a = a.filter_map(|a| {
            let (rate, can_start_going, dist) = match a {
                Act::Idle { i: from } => (v.rate, minutes_remaining, game.dists[*from][to]),
                Act::Going {
                    dest: from,
                    arrive_at,
                } => (v.rate, *arrive_at, game.dists[*from][to]),
            };
            dist.and_then(|dist| {
                if dist < can_start_going {
                    Some(rate * (can_start_going - dist))
                } else {
                    None
                }
            })
        });
        a.max().unwrap_or(0)
    });
    x.sum::<Flow>()
}

fn all_moving(actors: &[Act]) -> bool {
    actors.iter().all(|a| matches!(a, Act::Going { .. }))
}

struct Step {
    dest: usize,
    arrive_at: Flow,
    added_flow: Flow,
}

fn possible_dests(
    from: usize,
    minutes_remaining: Flow,
    visited: &Visited,
    game: &Game,
) -> Vec<Step> {
    fn step(
        dest: usize,
        dist: &Option<usize>,
        minutes_remaining: Flow,
        visited: &Visited,
        game: &Game,
    ) -> Option<Step> {
        match (dist, is_visited(visited, dest)) {
            (Some(dist), false) if *dist < minutes_remaining => {
                let arrive_at = minutes_remaining - dist - 1;
                if arrive_at > 0 {
                    let v = &game.valves[dest];
                    if v.rate > 0 {
                        Some(Step {
                            added_flow: arrive_at * v.rate,
                            dest,
                            arrive_at,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    game.dists[from]
        .iter()
        .enumerate()
        .filter_map(|(to, dist)| step(to, dist, minutes_remaining, visited, game))
        .collect()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Act {
    Idle { i: usize },
    Going { dest: usize, arrive_at: Flow },
}

impl Act {
    fn must_arrive_at(&self) -> Flow {
        if let Act::Going { arrive_at, .. } = self {
            return *arrive_at;
        }
        unreachable!()
    }
}

// Figure out how far it is from every pair of valves to each other.
fn find_distances(v: &[Valve], vi: &HashMap<String, usize>, vis: bool) -> Vec<Vec<Option<usize>>> {
    let mut dists = vec![vec![None; v.len()]; v.len()];
    for (from, valve) in v.iter().enumerate() {
        if valve.name == "AA" || valve.rate > 0 {
            for (to, dest_valve) in v.iter().enumerate() {
                if dest_valve.rate > 0 && dest_valve.name != valve.name {
                    if dists[from][to].is_none() {
                        let d = Some(get_dist(from, to, v, vi, vis));
                        dists[from][to] = d;
                        dists[to][from] = d;
                    }
                }
            }
        }
    }
    dists
}

fn get_dist(from: usize, to: usize, v: &[Valve], vi: &HashMap<String, usize>, _vis: bool) -> usize {
    let mut dists: Vec<usize> = vec![usize::MAX; v.len()];
    let mut heap = BinaryHeap::new();

    dists[from] = 0;
    heap.push((0isize, from));

    while let Some((dist, i)) = heap.pop() {
        let dist = -dist as usize;
        if i == to {
            return dist;
        }
        if dist > dists[i] {
            continue;
        }
        let next_dist = dist + 1;
        let next_idist = -(next_dist as isize);
        for neighbor in &v[i].neighbors {
            let ni = &vi[neighbor];
            if dists[*ni] > next_dist {
                dists[*ni] = next_dist;
                heap.push((next_idist, *ni));
            }
        }
    }

    unreachable!()
}

fn parse_input(input: String) -> (Vec<Valve>, HashMap<String, usize>) {
    let valves: Vec<Valve> = input.lines().map(parse_valve).collect();
    let vindices = valves
        .iter()
        .enumerate()
        .map(|(i, v)| (v.name.to_string(), i))
        .collect();
    (valves, vindices)
}

struct Valve {
    name: String,
    neighbors: Vec<String>,
    rate: Flow,
}

type Flow = usize;

fn parse_valve(line: &str) -> Valve {
    // line looks like this:
    // Valve CC has flow rate=2; tunnels lead to valves DD, BB
    // 0     1  2   3    4       5       6    7  8      9
    let line: Vec<&str> = line.split(' ').collect();
    let name = line[1].to_owned();
    let rate = line[4]
        .strip_prefix("rate=")
        .unwrap()
        .strip_suffix(";")
        .unwrap()
        .parse()
        .unwrap();
    let neighbors = line
        .iter()
        .skip(9)
        .map(|s| s.strip_suffix(',').unwrap_or(s).to_owned())
        .collect();
    Valve {
        name,
        rate,
        neighbors,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    crate::test::aoc_test!(example, r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II",
        part1 => 1651,
        part2 => 1707);
}
