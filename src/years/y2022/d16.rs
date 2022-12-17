use std::{
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

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
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

fn get_dist(from: usize, to: usize, v: &[Valve], vi: &HashMap<String, usize>, vis: bool) -> usize {
    let mut dists: Vec<usize> = vec![usize::MAX; v.len()];
    let mut heap = BinaryHeap::new();

    dists[from] = 0;
    heap.push((0isize, from));

    if vis {
        println!("{} -> {}", v[from].name, v[to].name);
    }
    while let Some((dist, i)) = heap.pop() {
        let dist = -dist as usize;
        if vis {
            println!("... {} dist={}", v[i].name, dist);
        }
        if i == to {
            if vis {
                println!("=> {}", dist);
            }
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
                if vis {
                    println!(
                        "  + {} dist={} (was {})",
                        v[*ni].name, next_dist, dists[*ni]
                    );
                }
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
        part2 => "todo");
}
