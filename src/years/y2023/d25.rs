use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::Display;

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let (nodes, edges) = parse(&input);
    if vis {
        println!("{} nodes", nodes.len());
        println!("{} edges", edges.len());
    }
    // Based on https://patterns.eecs.berkeley.edu/?page_id=571#Partitioning_Algorithms
    // Split the nodes into two arbitrary sub-graphs.
    let mut v1 = HashSet::new();
    let mut v2 = HashSet::new();
    for (i, n) in nodes.iter().enumerate() {
        if i % 2 == 0 {
            v1.insert(n);
        } else {
            v2.insert(n);
        }
    }
    // Calculate internal and external "costs" for each node.
    // c(a,b) = 1 if connected or 0 otherwise.
    // E(a) = sum(c(a, v) for v in V(2) (when a is in V(1)).
    // I(a) = sum(c(a, v) for v in V(1) (when a is in V(1)).
    let mut ext_costs = HashMap::new();
    let mut int_costs = HashMap::new();
    for (n, tos) in &edges {
        let int = if v1.contains(n) { &v1 } else { &v2 };
        for to in tos {
            let e = if int.contains(to) {
                int_costs.entry(n.to_string())
            } else {
                ext_costs.entry(n.to_string())
            };
            let e = e.or_insert(0);
            *e += 1;
        }
    }
    // D(a) = E(a) - I(i)
    let mut d = HashMap::new();
    for n in &nodes {
        d.insert(
            n,
            ext_costs.get(n).copied().unwrap_or(0) - int_costs.get(n).copied().unwrap_or(0),
        );
    }
    let mut locked = HashSet::new();
    loop {
        if vis {
            println!("---ITERATION---");
            println!("V1 = {v1:?}");
            println!("V2 = {v2:?}");
        }
        // g(a,b) = D(a) + D(b) - 2*c(a, b)
        let mut g = BinaryHeap::new();
        for n1 in &v1 {
            if locked.contains(n1) {
                continue;
            }
            for n2 in &v2 {
                if locked.contains(n2) {
                    continue;
                }
                let cab = match edges.get(*n1).map(|tos| tos.contains(n2)) {
                    Some(true) => 1,
                    _ => 0,
                };
                let gab = d.get(n1).unwrap() + d.get(n2).unwrap() - 2 * cab;
                g.push((gab, cab, n1, n2));
            }
        }
        match g.pop() {
            None => break,
            Some((gab, cab, a, b)) => {
                if vis {
                    println!("best: {a} vs {b} {gab} ({cab})");
                }
                locked.insert(a);
                locked.insert(b);
                if locked.len() > 0 {
                    // todo replace this with a swap
                    break;
                }
            }
        }
    }

    Box::new("todo")
}

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

fn parse(input: &str) -> (HashSet<String>, HashMap<String, Vec<String>>) {
    let mut nodes = HashSet::new();
    let mut edges = HashMap::new();
    for line in input.lines() {
        let (from, tos) = line.trim().split_once(": ").unwrap();
        nodes.insert(from.to_owned());
        for to in tos.split_whitespace() {
            nodes.insert(to.to_owned());
            edges
                .entry(from.to_owned())
                .or_insert_with(Vec::new)
                .push(to.to_owned());
            edges
                .entry(to.to_owned())
                .or_insert_with(Vec::new)
                .push(from.to_owned());
        }
    }
    (nodes, edges)
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str = r"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    crate::test::aoc_test!(part1, TEST_INPUT, 54);
    crate::test::aoc_test!(part2, TEST_INPUT, "todo");
}
