use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let parsed = parse(&input);
    let longest_path = find_longest_path(&parsed, true, vis);
    Box::new(longest_path)
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    // Hint: Maybe a simple Dijkstra would be faster?
    // https://www.reddit.com/r/adventofcode/comments/18ufl0o/algorithms_for_each_day/
    let parsed = parse(&input);
    let longest_path = find_longest_path(&parsed, false, vis);
    Box::new(longest_path)
}

fn find_longest_path(parsed: &Parsed, slippery: bool, vis: bool) -> usize {
    let graph = trace(parsed, slippery);
    if vis {
        println!(
            "{} edges, {} nodes",
            graph.count_edges(),
            graph.count_nodes()
        );
    }

    let mut to_visit: VecDeque<(Node, usize, Vec<Node>)> = VecDeque::new();
    let src = (0, parsed.start_col);
    to_visit.push_back((src, 0, vec![src]));

    let mut maxcost: Option<(usize, Vec<Node>)> = None;

    let mut count = 0;
    while let Some((n, cost, visited)) = to_visit.pop_front() {
        count += 1;
        if vis {
            println!("[{count}] visiting {n:?} from a {cost}-long path ({visited:?})");
        }

        if n.0 == parsed.rows - 1 {
            if cost > maxcost.as_ref().map_or(0, |mc| mc.0) {
                maxcost = Some((cost, visited.clone()));
            }
            continue;
        }

        let edges = graph
            .nodes
            .get(&n)
            .unwrap_or_else(|| panic!("expected to find edges from {n:?}"));
        for edge in edges {
            if visited.contains(&edge.to) {
                if vis {
                    println!(" - skip {:?}, it's already in this list", edge.to);
                }
            } else {
                if vis {
                    println!(" - consider {:?} (+{})", edge.to, edge.path.len());
                }
                let mut extended = visited.clone();
                extended.push(edge.to);
                to_visit.push_back((edge.to, cost + edge.path.len(), extended));
            }
        }
    }

    let (maxcost, path) = maxcost.unwrap();

    if vis {
        println!("looped {count} times");
        let mut longest_path: HashSet<Pos> = HashSet::new();
        for w in path.windows(2) {
            let from = &w[0];
            let to = &w[1];
            let edge = graph
                .nodes
                .get(from)
                .unwrap()
                .iter()
                .find(|e| e.to == *to)
                .unwrap();
            longest_path.extend(edge.path.iter());
        }

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

    maxcost
}

fn trace(parsed: &Parsed, slippery: bool) -> Graph {
    let mut nodes = HashMap::new();
    for (r, row) in parsed.map.iter().enumerate() {
        for (c, tile) in row.iter().enumerate() {
            if matches!(tile, Tile::Path(_)) {
                let from = (r, c);
                if r == 0 || is_fork(from, parsed) {
                    for (to, path) in walk(from, parsed, slippery) {
                        let edges = nodes.entry(from).or_insert_with(Vec::new);
                        edges.push(Edge {
                            to,
                            path: path.iter().cloned().collect(),
                        })
                    }
                }
            }
        }
    }
    Graph { nodes }
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
                res.push((to, visited[1..].iter().cloned().collect()));
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
                res.push((to, visited[1..].iter().cloned().collect()));
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
                res.push((to, visited[1..].iter().cloned().collect()));
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
                res.push((to, visited[1..].iter().cloned().collect()));
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
    nodes: HashMap<Node, Vec<Edge>>,
}

impl Graph {
    fn count_edges(&self) -> usize {
        self.nodes
            .values()
            .fold(0, |count, edges| count + edges.len())
    }

    fn count_nodes(&self) -> usize {
        self.nodes.len()
    }
}

type Pos = (usize, usize);
type Node = Pos;

struct Edge {
    to: Node,
    path: HashSet<Pos>, // does not include 'from'
}

struct Parsed {
    map: Vec<Vec<Tile>>,
    start_col: usize,
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

    Parsed {
        map,
        start_col,
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
