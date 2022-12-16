use std::{collections::HashMap, fmt::Display};

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let (valves, vindices) = parse_input(input);

    let open = vec![false; valves.len()];
    Box::new(search(0, &valves, &vindices, &open, 30, vis, 0))
}

fn search(
    pos: usize,
    valves: &[Valve],
    vi: &HashMap<String, usize>,
    open: &[bool],
    mut steps_left: u8,
    vis: bool,
    released: Flow,
) -> Flow {
    if steps_left == 1 {
        return released;
    }
    steps_left -= 1;

    let valve = &valves[pos];

    let steps = 30 - steps_left as usize;
    if vis {
        println!(
            "{:width$} visiting {} (rate={}, open={}) (already released {})",
            steps,
            valve.name,
            valve.rate,
            open[pos],
            released,
            width = steps
        );
    }

    let mut max_released = released;

    if !open[pos] && valve.rate > 0 {
        if vis {
            println!(
                "{:width$} try opening {} for rate={}",
                steps,
                valve.name,
                valve.rate,
                width = steps
            );
        }
        let added = valve.rate * steps_left as Flow;
        let mut open = open.to_vec();
        open[pos] = true;
        let max = search(pos, valves, vi, &open, steps_left, vis, released + added);
        if max > max_released {
            max_released = max;
        }
        return max_released;
    }

    for n in &valve.neighbors {
        if vis {
            println!("{:width$} try visiting {}", steps, n, width = steps);
        }
        let max = search(vi[n], valves, vi, open, steps_left, vis, released);
        if max > max_released {
            max_released = max;
        }
    }

    max_released
}

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
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

type Flow = u32;

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
