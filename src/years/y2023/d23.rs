use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
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
    let Graph { edges, nodes } = trace(parsed, slippery);
    if vis {
        println!("{} edges, {} nodes", edges.len(), nodes.len());
    }

    let mut to_visit: VecDeque<(Node, HashSet<Pos>)> = VecDeque::new();
    let dest = (parsed.rows - 1, parsed.dest_col);
    to_visit.push_back((dest, [dest].into()));

    let mut path_to_start: Option<HashSet<Pos>> = None;

    let mut count = 0;
    while let Some((n, visited)) = to_visit.pop_front() {
        count += 1;
        if vis {
            println!(
                "[{count}] visiting {n:?} from a {}-long path",
                visited.len()
            );
        }

        // find all edges that lead to the current node.
        for edge_in in edges.iter().filter(|e| e.to == n) {
            if visited.contains(&edge_in.from) {
                if vis {
                    println!("- skip {:?}, it's already in the path", edge_in.from);
                }
                continue;
            }

            let new_path: HashSet<Pos> = visited.union(&edge_in.path).copied().collect();
            if vis {
                println!(
                    "- {:?} -> {n:?} -> ... -> {dest:?} could be {}",
                    edge_in.from,
                    new_path.len()
                );
            }

            if edge_in.from.0 == 0 {
                // This is the start, save this path if it's the longest one we've seen so far.
                if new_path.len() > path_to_start.as_ref().map_or(0, |p| p.len()) {
                    path_to_start = Some(new_path);
                }
            } else {
                // This is not the start, keep going!
                to_visit.push_back((edge_in.from, new_path));
            }
        }
    }

    if vis {
        println!("looped {count} times");
    }

    path_to_start.unwrap()
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
