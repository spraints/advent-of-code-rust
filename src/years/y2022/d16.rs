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
    let (valves, vindices) = parse_input(input);
    write_dot(&valves).unwrap();
    let dists = find_distances(&valves, &vindices, vis);
    if vis {
        print!("  ");
        for v in &valves {
            print!("{:3}", v.name);
        }
        println!();
        for (i, r) in dists.iter().enumerate() {
            print!("{}", valves[i].name);
            for c in r {
                match c {
                    None => print!("   "),
                    Some(d) => print!("{:3}", d),
                };
            }
            println!();
        }
    }
    let start_i = vindices["AA"];
    let mut visited = vec![false; valves.len()];
    visited[start_i] = true;
    let (dist, path) = search(
        start_i,
        0,
        30,
        &valves,
        &vindices,
        &dists,
        &mut visited,
        vis,
    );
    if vis {
        println!("optimal path: {}", path);
    }
    // 1199 is too low.
    Box::new(dist)
}

fn search(
    start: usize,
    accum: Flow,
    minutes: Flow,
    valves: &[Valve],
    vi: &HashMap<String, usize>,
    dists: &[Vec<Option<usize>>],
    visited: &mut [bool],
    vis: bool,
) -> (Flow, String) {
    let disp_minute = 31 - minutes;
    let mut max = accum;
    let mut max_path = format!("{}({})", valves[start].name, valves[start].rate);
    visited[start] = true;
    for (i, d) in dists[start].iter().enumerate() {
        if let Some(d) = d {
            /*
            if !visited[i] && vis {
                println!(
                    "{} minutes left, can i get from {} to {} (need {} minutes)?",
                    minutes, valves[start].name, valves[i].name, d
                );
            }
            */
            if !visited[i] && d + 1 < minutes {
                let new_minutes = minutes - d - 1;
                let added_accum = new_minutes * valves[i].rate;
                let new_accum = accum + added_accum;
                if vis {
                    println!(
                        "[{}] {} -> {} (rate={}) takes {} minutes to move, increases flow by {} to {}",
                        disp_minute,
                        valves[start].name,
                        valves[i].name,
                        valves[i].rate,
                        d,
                        added_accum,
                        new_accum
                    );
                }
                let (best, path) =
                    search(i, new_accum, new_minutes, valves, vi, dists, visited, vis);
                if best > max {
                    max = best;
                    max_path = format!(
                        "{}({}) -{}-> {}",
                        valves[start].name, valves[start].rate, d, path
                    );
                }
            }
        }
    }
    visited[start] = false;
    if vis {
        println!(
            "[{}] from {} max is {} ({})",
            31 - minutes,
            valves[start].name,
            max,
            max_path
        );
    }
    (max, max_path)
}

#[derive(Eq, PartialEq, Debug)]
struct State {
    possible: Flow,
    actual: Flow,
    minutes_remaining: Flow,
    actors: Vec<Act>,
    visited: Vec<bool>,
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

struct Game {
    valves: Vec<Valve>,
    vindices: HashMap<String, usize>,
    dists: Vec<Vec<Option<usize>>>,
}

// 2105 is too low
pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    Box::new(solve(input, vis, 26, 2))
}

fn solve(input: String, vis: bool, minutes: Flow, actors: usize) -> Flow {
    let (valves, vindices) = parse_input(input);
    let dists = find_distances(&valves, &vindices, vis);

    let game = Game {
        valves,
        vindices,
        dists,
    };

    let mut states = BinaryHeap::new();
    let visited = vec![false; game.valves.len()];
    let possible = possible_flow(minutes, &game, &visited);
    let i = game.vindices["AA"];
    states.push(State {
        possible,
        actual: 0,
        minutes_remaining: minutes,
        actors: vec![Act::Idle { i }; actors],
        visited,
        paths: "".to_string(),
    });

    while let Some(st) = states.pop() {
        let State {
            possible,
            actual,
            minutes_remaining,
            actors,
            visited,
            paths,
        } = st;
        if vis {
            let visited: Vec<&str> = visited
                .iter()
                .enumerate()
                .filter_map(|(i, b)| {
                    if *b {
                        Some(&*game.valves[i].name)
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
                visited.join(",")
            );
        }
        if possible == actual {
            if vis {
                println!("optimal path:");
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
        for (actor_i, actor) in actors.iter().enumerate() {
            if let Act::Idle { i: loc } = actor {
                for step in possible_dests(*loc, minutes_remaining, &visited, &game) {
                    let Step {
                        dest,
                        arrive_at,
                        added_flow,
                    } = step;
                    let mut visited = visited.clone();
                    visited[dest] = true;
                    let mut actors = actors.clone();
                    actors[actor_i] = Act::Going { dest, arrive_at };
                    states.push(State {
                        possible: actual
                            + added_flow
                            + possible_flow(minutes_remaining, &game, &visited),
                        actual: actual + added_flow,
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
            minutes_remaining: 0,
            actors: Vec::new(),
            visited: Vec::new(),
            paths,
        });
    }
    unreachable!();
}

fn possible_flow(minutes_remaining: Flow, game: &Game, visited: &[bool]) -> Flow {
    let unvisited_rates: Flow = game
        .valves
        .iter()
        .enumerate()
        .filter(|(i, _)| !visited[*i])
        .map(|(_, v)| v.rate)
        .sum();
    minutes_remaining * unvisited_rates
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
    visited: &[bool],
    game: &Game,
) -> Vec<Step> {
    fn step(
        dest: usize,
        dist: &Option<usize>,
        minutes_remaining: Flow,
        visited: &[bool],
        game: &Game,
    ) -> Option<Step> {
        match (dist, visited[dest]) {
            (Some(dist), false) if *dist < minutes_remaining => {
                let arrive_at = minutes_remaining - dist - 1;
                if arrive_at > 0 {
                    let v = &game.valves[dest];
                    Some(Step {
                        added_flow: arrive_at * v.rate,
                        dest,
                        arrive_at,
                    })
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
