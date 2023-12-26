use std::collections::HashSet;
use std::fmt::Display;

use num::rational::Ratio;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

const NODE_LABEL_LEN: usize = 3;

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let g = parse(&input);
    if vis {
        println!("{} nodes", g.nodes.len());
        println!("{} edges", g.edges.len());
    }

    let inv_sqrt_2 = Ratio::new(99, 70).recip();
    let mut rng = thread_rng();

    let g = fastmincut(g, &inv_sqrt_2, &mut rng);
    if vis {
        println!("final nodes = {:?}", g.nodes);
        println!("final edges = {:?}", g.edges);
    }
    assert_eq!(2, g.nodes.len());
    assert_eq!(3, g.edges.len());
    let (_, a) = node_size(&g.nodes[0]);
    let (_, b) = node_size(&g.nodes[1]);
    Box::new(a * b)
}

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

// Karger-Stein algorithm.
// https://en.wikipedia.org/wiki/Karger%27s_algorithm#Karger%E2%80%93Stein_algorithm
fn fastmincut(g: Graph, inv_sqrt_2: &Ratio<usize>, rng: &mut ThreadRng) -> Graph {
    if g.nodes.len() <= 6 {
        contract(g, 2, rng)
    } else {
        let t = (inv_sqrt_2 * g.nodes.len() + 1).ceil().to_integer();
        let g1 = fastmincut(contract(g.clone(), t, rng), inv_sqrt_2, rng);
        let g2 = fastmincut(contract(g, t, rng), inv_sqrt_2, rng);
        if g1.edges.len() < g2.edges.len() {
            g1
        } else {
            g2
        }
    }
}

fn contract(g: Graph, t: usize, rng: &mut ThreadRng) -> Graph {
    let Graph {
        mut nodes,
        mut edges,
    } = g;
    while nodes.len() > t {
        let (from, to) = edges.choose(rng).unwrap();
        let from = from.to_string();
        let to = to.to_string();
        let (prefix, from_size) = node_size(&from);
        let (_, to_size) = node_size(&to);
        let combined = format!("{prefix}-{}", from_size + to_size);
        //println!("DELETEME: {from} + {to} => {combined}");
        nodes = nodes
            .into_iter()
            .filter_map(|node| match node {
                node if node == from => Some(combined.clone()),
                node if node == to => None,
                node => Some(node),
            })
            .collect();
        edges = edges
            .into_iter()
            .filter_map(|edge| match edge {
                (a, b) if a == from && b == to => None,
                (a, b) if b == from && a == to => None,
                (a, b) if a == from || a == to => Some((combined.clone(), b)),
                (a, b) if b == from || b == to => Some((a, combined.clone())),
                (a, b) => Some((a, b)),
            })
            .collect();
        //println!("DELETEME: -------- t={t}");
        //println!("DELETEME: nodes = {nodes:?}");
        //println!("DELETEME: edges = {edges:?}");
    }
    Graph { nodes, edges }
}

fn node_size(node: &str) -> (&str, usize) {
    if node.len() == NODE_LABEL_LEN {
        (node, 1)
    } else {
        let (node, num) = node.split_once('-').unwrap();
        (node, num.parse().unwrap())
    }
}

#[derive(Clone)]
struct Graph {
    nodes: Vec<String>,
    edges: Vec<(String, String)>,
}

fn parse(input: &str) -> Graph {
    let mut nodes = HashSet::new();
    let mut edges = Vec::new();
    for line in input.lines() {
        let (from, tos) = line.trim().split_once(": ").unwrap();
        assert_eq!(NODE_LABEL_LEN, from.len());
        nodes.insert(from.to_owned());
        for to in tos.split_whitespace() {
            assert_eq!(NODE_LABEL_LEN, to.len());
            nodes.insert(to.to_owned());
            edges.push((from.to_owned(), to.to_owned()));
        }
    }
    let mut nodes: Vec<String> = nodes.into_iter().collect();
    nodes.sort();
    Graph { nodes, edges }
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
