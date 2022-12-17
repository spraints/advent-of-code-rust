use std::{
    borrow::Cow,
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fmt::Display,
    io::Write,
};

fn write_dot(valves: &[Valve]) -> std::io::Result<()> {
    let mut f = std::fs::File::create("d16.dot")?;
    writeln!(f, "digraph G {{")?;
    writeln!(f, "rankdir=LR;")?;
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

// 2105 is too low
pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    let (valves, vindices) = parse_input(input);
    let dists = find_distances(&valves, &vindices, vis);

    let start_i = vindices["AA"];
    let mut visited = vec![false; valves.len()];
    visited[start_i] = true;
    let (relieved, paths) = search2(
        (Act::Idle { i: start_i }, Act::Idle { i: start_i }),
        ("AA@1", "AA@1"),
        26,
        0,
        &valves,
        &vindices,
        &dists,
        &mut visited,
        vis,
    );
    if vis {
        println!("best1: {}", paths.0);
        println!("best2: {}", paths.1);
    }
    Box::new(relieved)
}

#[derive(Copy, Clone, Debug)]
enum Act {
    Idle { i: usize },
    Going { dest: usize, arrive_at: Flow },
}

fn search2(
    acts: (Act, Act),
    paths: (&str, &str),
    minutes_remaining: Flow,
    accum: Flow,
    valves: &[Valve],
    vi: &HashMap<String, usize>,
    dists: &[Vec<Option<usize>>],
    visited: &mut [bool],
    vis: bool,
) -> (Flow, (String, String)) {
    if vis {
        println!("[{}]/0 {:?} {}", minutes_remaining, acts.0, paths.0);
        println!("[{}]/1 {:?} {}", minutes_remaining, acts.1, paths.1);
    }
    let mut max = accum;
    let mut max_paths = (Cow::from(paths.0), Cow::from(paths.1));
    let (me_path, him_path) = (paths.0, paths.1);
    match acts {
        (Act::Idle { i: me_i }, Act::Idle { i: him_i }) => {
            let (best, best_path) = search2_2(
                me_i,
                me_path,
                him_i,
                him_path,
                minutes_remaining,
                accum,
                valves,
                vi,
                dists,
                visited,
                vis,
            );
            if best > max {
                max = best;
                max_paths = (Cow::from(best_path.0), Cow::from(best_path.1));
            }
        }
        (
            Act::Idle { i: me_i },
            Act::Going {
                dest: him_dest,
                arrive_at: him_arrive_at,
            },
        ) => {
            let (best, best_path) = search2_1(
                me_i,
                me_path,
                (him_dest, him_arrive_at),
                him_path,
                minutes_remaining,
                accum,
                valves,
                vi,
                dists,
                visited,
                vis,
            );
            if best > max {
                max = best;
                max_paths = (Cow::from(best_path.0), Cow::from(best_path.1));
            }
        }
        _ => unreachable!(),
    };
    (max, (max_paths.0.into_owned(), max_paths.1.into_owned()))
}

fn search2_1(
    idle: usize,
    idle_path: &str,
    going: (usize, Flow),
    going_path: &str,
    minutes_remaining: Flow,
    accum: Flow,
    valves: &[Valve],
    vi: &HashMap<String, usize>,
    dists: &[Vec<Option<usize>>],
    visited: &mut [bool],
    vis: bool,
) -> (usize, (String, String)) {
    let mut max = accum;
    let mut max_path = (Cow::from(idle_path), Cow::from(going_path));
    let (him_dest, him_arrive_at) = going;
    let him_path = going_path;
    for (me_dest, d) in dists[idle].iter().enumerate() {
        if let Some(d) = d {
            if !visited[me_dest] && d + 1 < minutes_remaining {
                let me_arrive_at = minutes_remaining - d - 1;
                let added_accum = me_arrive_at * valves[me_dest].rate;
                let new_accum = accum + added_accum;
                let me_path = format!(
                    "{} -> {}@{}+{}",
                    idle_path, valves[me_dest].name, me_arrive_at, added_accum
                );
                if vis {
                    println!("trying i={},d={} {} ...", me_dest, d, me_path);
                    println!("  other is {}", him_path);
                }

                let (acts, new_min, paths): ((Act, Act), Flow, (&str, &str)) =
                    match me_arrive_at.cmp(&him_arrive_at) {
                        Ordering::Less => (
                            (
                                Act::Idle { i: him_dest },
                                Act::Going {
                                    dest: me_dest,
                                    arrive_at: me_arrive_at,
                                },
                            ),
                            him_arrive_at,
                            (him_path, &me_path),
                        ),
                        Ordering::Equal => (
                            (Act::Idle { i: me_dest }, Act::Idle { i: him_dest }),
                            me_arrive_at,
                            (&me_path, him_path),
                        ),
                        Ordering::Greater => (
                            (
                                Act::Idle { i: me_dest },
                                Act::Going {
                                    dest: him_dest,
                                    arrive_at: him_arrive_at,
                                },
                            ),
                            me_arrive_at,
                            (&me_path, him_path),
                        ),
                    };
                if vis {
                    println!("    acts: {:?}", acts);
                }
                visited[me_dest] = true;
                let (best, best_paths) = search2(
                    acts, paths, new_min, new_accum, valves, vi, dists, visited, vis,
                );
                visited[me_dest] = false;
                if best > max {
                    max = best;
                    max_path = (Cow::from(best_paths.0), Cow::from(best_paths.1));
                }
            }
        }
    }
    (max, (max_path.0.into_owned(), max_path.1.into_owned()))
}

fn search2_2(
    idle1: usize,
    idle1_path: &str,
    idle2: usize,
    idle2_path: &str,
    minutes_remaining: Flow,
    accum: Flow,
    valves: &[Valve],
    vi: &HashMap<String, usize>,
    dists: &[Vec<Option<usize>>],
    visited: &mut [bool],
    vis: bool,
) -> (usize, (String, String)) {
    let mut max = accum;
    let mut max_path = (Cow::from(idle1_path), Cow::from(idle2_path));
    for (me_dest, d) in dists[idle1].iter().enumerate() {
        if let Some(d) = d {
            if !visited[me_dest] && d + 1 < minutes_remaining {
                let me_arrive_at = minutes_remaining - d - 1;
                let added_accum = me_arrive_at * valves[me_dest].rate;
                let new_accum = accum + added_accum;
                let me_path = format!(
                    "{} -> {}@{}+{}",
                    idle1_path, valves[me_dest].name, me_arrive_at, added_accum
                );
                if vis {
                    println!("trying i={},d={} {} ...", me_dest, d, me_path);
                    println!("  other is {}", idle2_path);
                }

                visited[me_dest] = true;
                let (best, best_path) = search2_1(
                    idle2,
                    idle2_path,
                    (me_dest, me_arrive_at),
                    &me_path,
                    minutes_remaining,
                    new_accum,
                    valves,
                    vi,
                    dists,
                    visited,
                    vis,
                );
                visited[me_dest] = false;
                if best > max {
                    max = best;
                    max_path = (Cow::from(best_path.0), Cow::from(best_path.1));
                }
            }
        }
    }
    visited[idle1] = false;
    (max, (max_path.0.into_owned(), max_path.1.into_owned()))
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
        //part1 => 1651,
        part2 => 1707);
}
