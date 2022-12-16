use std::{collections::HashMap, fmt::Display};

pub fn part1(input: String, _vis: bool) -> Box<dyn Display> {
    let (valves, vindices) = parse_input(input);
    Box::new(vindices.len())
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
    let line = line.strip_prefix("Valve ").unwrap();
    let (name, line) = line.split_once(' ').unwrap();
    let line = line.strip_prefix("has flow rate=").unwrap();
    let (rate, line) = line.split_once(';').unwrap();
    println!("line: {:?}", line);
    let line = match line.strip_prefix(" tunnels lead to valves ") {
        Some(s) => s,
        None => line.strip_prefix(" tunnel leads to valve ").unwrap(),
    };
    let neighbors = line.split(", ").map(|s| s.to_owned()).collect();
    Valve {
        name: name.to_owned(),
        rate: rate.parse().unwrap(),
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
