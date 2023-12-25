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
    Box::new("todo")
}

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

fn parse(input: &str) -> (HashSet<String>, HashMap<String, String>) {
    let mut nodes = HashSet::new();
    let mut edges = HashMap::new();
    for line in input.lines() {
        let (from, tos) = line.trim().split_once(": ").unwrap();
        nodes.insert(from.to_owned());
        for to in tos.split_whitespace() {
            nodes.insert(to.to_owned());
            edges.insert(from.to_owned(), to.to_owned());
            edges.insert(to.to_owned(), from.to_owned());
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
