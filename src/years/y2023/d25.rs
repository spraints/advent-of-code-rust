use std::collections::{HashMap, HashSet};
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
            v1.insert(n.to_string());
        } else {
            v2.insert(n.to_string());
        }
    }
    // Calculate internal and external "costs" for each node.
    // c(a,b) = 1 if connected or 0 otherwise.
    // E(a) = sum(c(a, v) for v in V(2) (when a is in V(1)).
    // I(a) = sum(c(a, v) for v in V(1) (when a is in V(1)).
    // This implementation is a shortcut because we can skip all the pairs where we know there
    // isn't an edge.
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
    if vis {
        println!("INITIAL COSTS");
        for n in &nodes {
            println!("{n}: I={:?} E={:?}", int_costs.get(n), ext_costs.get(n));
        }
    }
    // Calculate c(a,b) for every pair. This will simplify things later.
    let mut cabs: HashMap<(&str, &str), i32> = HashMap::new();
    for (from, tos) in &edges {
        for to in tos {
            cabs.insert((from, to), 1);
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
            println!("V1 = {:?}", sorted(&v1));
            println!("V2 = {:?}", sorted(&v2));
        }
        // Find maximum: g(a,b) = D(a) + D(b) - 2*c(a, b)
        let mut best = None;
        for n1 in &v1 {
            if locked.contains(n1) {
                continue;
            }
            for n2 in &v2 {
                if locked.contains(n2) {
                    continue;
                }
                let cab = cabs.get(&(n1, n2)).copied().unwrap_or(0);
                let gab = d.get(n1).unwrap() + d.get(n2).unwrap() - 2 * cab;
                if best.is_none() || matches!(best, Some((n, _, _)) if gab > n) {
                    best = Some((gab, n1, n2));
                }
            }
        }
        if vis {
            println!("best: {best:?}");
        }
        match best {
            Some((gab, a, b)) if gab > 0 => {
                let a = a.clone();
                let b = b.clone();
                assert!(v1.remove(&a));
                assert!(v2.remove(&b));
                for x in &v1 {
                    let dx = d.get_mut(x).unwrap();
                    let cxa = cabs.get(&(x, &a)).copied().unwrap_or(0);
                    let cxb = cabs.get(&(x, &b)).copied().unwrap_or(0);
                    *dx += 2 * cxa - 2 * cxb;
                }
                for y in &v2 {
                    let dy = d.get_mut(y).unwrap();
                    let cyb = cabs.get(&(y, &b)).copied().unwrap_or(0);
                    let cya = cabs.get(&(y, &a)).copied().unwrap_or(0);
                    *dy += 2 * cyb - 2 * cya;
                }
                v1.insert(b.clone());
                v2.insert(a.clone());
                locked.insert(a);
                locked.insert(b);
            }
            _ => break,
        }
    }

    if vis {
        println!("---FINISHED!---");
        println!("V1 = {:?}", sorted(&v1));
        println!("V2 = {:?}", sorted(&v2));
        for a in &v1 {
            for b in &v2 {
                if cabs.get(&(a, b)).is_some() {
                    println!("{a} connects to {b}");
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

fn sorted(v: &HashSet<String>) -> Vec<String> {
    let mut res: Vec<String> = v.iter().cloned().collect();
    res.sort();
    res
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
