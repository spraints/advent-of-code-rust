use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let parsed = parse(&input);
    let longest_path = find_longest_path(&parsed, true, vis);
    if vis {
        for (r, row) in parsed.map.iter().enumerate() {
            for (c, tile) in row.iter().enumerate() {
                if longest_path.contains(&(r, c)) {
                    print!("O");
                } else {
                    print!("{}", tile);
                }
            }
            println!();
        }
    }
    // I count the start square, but it's not supposed to be counted.
    Box::new(longest_path.len() - 1)
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    let parsed = parse(&input);
    let longest_path = find_longest_path(&parsed, false, vis);
    if vis {
        for (r, row) in parsed.map.iter().enumerate() {
            for (c, tile) in row.iter().enumerate() {
                if longest_path.contains(&(r, c)) {
                    print!("O");
                } else {
                    print!("{}", tile);
                }
            }
            println!();
        }
    }
    Box::new(longest_path.len() - 1)
}

fn find_longest_path(parsed: &Parsed, slippery: bool, vis: bool) -> HashSet<Pos> {
    let Graph { edges, nodes: _ } = trace(parsed, slippery);

    // max_paths is the longest path from 'key' to the bottom row.
    let mut max_paths: HashMap<Pos, HashSet<Pos>> = HashMap::new();

    let mut to_visit = VecDeque::new();
    to_visit.push_back((parsed.rows - 1, parsed.dest_col));

    // does this need to sort 'to_visit'??
    while let Some(n) = to_visit.pop_front() {
        if vis {
            println!("visiting {n:?}");
        }

        // assume max_paths has already been populated for n.
        // the one time it isn't is the case where n is the dest node.
        let max_path_from_n: HashSet<Pos> =
            max_paths.get(&n).cloned().unwrap_or_else(|| [n].into());

        // find all edges that lead to the current node.
        for edge_in in edges.iter().filter(|e| e.to == n) {
            if vis {
                println!(
                    "- considering {:?} to {:?} (len = {})",
                    edge_in.from,
                    edge_in.to,
                    edge_in.path.len()
                );
            }
            if max_path_from_n.contains(&edge_in.from) {
                // don't revisit n.
                if vis {
                    println!(
                        "  ! this would be a loop because {n:?} -> FIN already includes {:?}",
                        edge_in.from
                    );
                }
                continue;
            }

            // What if we add 'edge_in' -> 'n' to n's longest path?
            let new_path: HashSet<Pos> = max_path_from_n.union(&edge_in.path).copied().collect();

            // Is that longer than edge_in's longest path so far?
            let edge_in_longest_path = max_paths.get(&edge_in.from).map_or(0, |p| p.len());
            if vis {
                println!(
                    "  {:?} -> FIN was {}; {:?} -> {n:?} -> FIN is {}",
                    edge_in.from,
                    edge_in_longest_path,
                    edge_in.from,
                    new_path.len()
                );
            }
            if new_path.len() > edge_in_longest_path {
                if vis {
                    println!("  updated!");
                }
                max_paths.insert(edge_in.from, new_path);
                to_visit.push_back(edge_in.from);
            }
        }
    }

    max_paths.remove(&(0, parsed.start_col)).unwrap()
}

fn trace(parsed: &Parsed, slippery: bool) -> Graph {
    let mut nodes = HashSet::new();
    let mut edges = Vec::new();
    for (r, row) in parsed.map.iter().enumerate() {
        for (c, tile) in row.iter().enumerate() {
            if matches!(tile, Tile::Path(_)) {
                let from = (r, c);
                if r == 0 || is_fork(from, parsed) {
                    for (to, path) in walk(from, parsed, slippery) {
                        nodes.insert(to);
                        edges.push(Edge {
                            from,
                            to,
                            path: path.iter().cloned().collect(),
                        })
                    }
                }
            }
        }
    }
    Graph { nodes, edges }
}

// Returns all edges from 'from' like this: (to, edge)
fn walk(from: Pos, parsed: &Parsed, slippery: bool) -> Vec<(Node, HashSet<Pos>)> {
    let mut res = Vec::with_capacity(3);
    walk2(from, parsed, slippery, &mut Vec::new(), &mut res);
    res
}

fn walk2(
    from: Pos,
    parsed: &Parsed,
    slippery: bool,
    visited: &mut Vec<Pos>,
    res: &mut Vec<(Node, HashSet<Pos>)>,
) {
    visited.push(from);
    let (r, c) = from;
    if r > 0 {
        let to = (r - 1, c);
        if !visited.contains(&to)
            && can_visit(to, parsed, |d| !slippery || matches!(d, SlopeDirection::Up))
        {
            if is_fork(to, parsed) {
                visited.push(to);
                res.push((to, visited.iter().cloned().collect()));
                visited.pop();
            } else {
                walk2(to, parsed, slippery, visited, res);
            }
        }
    }
    if r < parsed.rows - 1 {
        let to = (r + 1, c);
        if !visited.contains(&to)
            && can_visit(to, parsed, |d| {
                !slippery || matches!(d, SlopeDirection::Down)
            })
        {
            if to.0 == parsed.rows - 1 || is_fork(to, parsed) {
                visited.push(to);
                res.push((to, visited.iter().cloned().collect()));
                visited.pop();
            } else {
                walk2(to, parsed, slippery, visited, res);
            }
        }
    }
    if c > 0 {
        let to = (r, c - 1);
        if !visited.contains(&to)
            && can_visit(to, parsed, |d| {
                !slippery || matches!(d, SlopeDirection::Left)
            })
        {
            if is_fork(to, parsed) {
                visited.push(to);
                res.push((to, visited.iter().cloned().collect()));
                visited.pop();
            } else {
                walk2(to, parsed, slippery, visited, res);
            }
        }
    }
    if c < parsed.cols - 1 {
        let to = (r, c + 1);
        if !visited.contains(&to)
            && can_visit(to, parsed, |d| {
                !slippery || matches!(d, SlopeDirection::Right)
            })
        {
            if is_fork(to, parsed) {
                visited.push(to);
                res.push((to, visited.iter().cloned().collect()));
                visited.pop();
            } else {
                walk2(to, parsed, slippery, visited, res);
            }
        }
    }
    visited.pop();
}

fn can_visit<WS: Fn(&SlopeDirection) -> bool>(p: Pos, parsed: &Parsed, wont_slip: WS) -> bool {
    match &parsed.map[p.0][p.1] {
        Tile::Path(SlopeDirection::None) => true,
        Tile::Path(d) if wont_slip(d) => true,
        _ => false,
    }
}

fn is_fork(p: Pos, parsed: &Parsed) -> bool {
    let (r, c) = p;
    let mut neighbors = 0;
    if r > 0 && matches!(parsed.map[r - 1][c], Tile::Path(_)) {
        neighbors += 1;
    }
    if c > 0 && matches!(parsed.map[r][c - 1], Tile::Path(_)) {
        neighbors += 1;
    }
    if r < parsed.rows - 1 && matches!(parsed.map[r + 1][c], Tile::Path(_)) {
        neighbors += 1;
    }
    if c < parsed.cols - 1 && matches!(parsed.map[r][c + 1], Tile::Path(_)) {
        neighbors += 1;
    }
    neighbors > 2
}

struct Graph {
    edges: Vec<Edge>,
    nodes: HashSet<Node>,
}

type Pos = (usize, usize);
type Node = Pos;

struct Edge {
    from: Node,
    to: Node,
    path: HashSet<Pos>, // includes 'from' and 'to'.
}

struct Parsed {
    map: Vec<Vec<Tile>>,
    start_col: usize,
    dest_col: usize,
    rows: usize,
    cols: usize,
}

#[derive(Debug)]
enum Tile {
    Forest,
    Path(SlopeDirection),
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Forest => '#',
                Self::Path(SlopeDirection::None) => '.',
                Self::Path(SlopeDirection::Up) => '^',
                Self::Path(SlopeDirection::Down) => 'v',
                Self::Path(SlopeDirection::Left) => '<',
                Self::Path(SlopeDirection::Right) => '>',
            }
        )
    }
}

#[derive(Debug)]
enum SlopeDirection {
    None,
    Left,
    Right,
    Up,
    Down,
}

fn parse(input: &str) -> Parsed {
    let map: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    '#' => Tile::Forest,
                    '.' => Tile::Path(SlopeDirection::None),
                    '>' => Tile::Path(SlopeDirection::Right),
                    '<' => Tile::Path(SlopeDirection::Left),
                    '^' => Tile::Path(SlopeDirection::Up),
                    'v' => Tile::Path(SlopeDirection::Down),
                    _ => panic!("unexpected tile {c:?}"),
                })
                .collect()
        })
        .collect();

    let rows = map.len();
    let cols = map[0].len();

    let start_col = map[0]
        .iter()
        .position(|t| matches!(t, Tile::Path(_)))
        .unwrap();
    let dest_col = map[rows - 1]
        .iter()
        .position(|t| matches!(t, Tile::Path(_)))
        .unwrap();

    Parsed {
        map,
        start_col,
        dest_col,
        rows,
        cols,
    }
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str = r"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    crate::test::aoc_test!(part1, TEST_INPUT, 94);
    crate::test::aoc_test!(part2, TEST_INPUT, 154);
}
