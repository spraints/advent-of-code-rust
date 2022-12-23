use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::ops::{Add, Sub};

pub fn part1(input: String, _vis: bool) -> Box<dyn Display> {
    let (board, path) = input.split_once("\n\n").unwrap();

    let board = parse_board(board);
    let path = parse_path(path);

    let mut pos = find_start(&board);
    let mut dir = Dir::Right;

    for m in path {
        match m {
            Move::L => dir = dir.l(),
            Move::R => dir = dir.r(),
            Move::Go(dist) => pos = walk(&board, pos, dir, dist),
        }
    }

    let (row, col) = pos;
    Box::new((row + 1) * 1000 + (col + 1) * 4 + dir as usize)
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    let (board, path) = input.split_once("\n\n").unwrap();

    let board = parse_board(board);
    let path = parse_path(path);
    let edge_len = if board.tiles.len() < 50 { 4 } else { 50 };

    let mut pos = find_start(&board);
    let mut dir = Dir::Right;

    let corners = find_corners(&board, pos);
    //let jumps = match_edges(&board);
    let edges = trace_edges(&board, pos, edge_len);

    if vis {
        for (row, tilerow) in board.tiles.iter().enumerate() {
            for (col, tile) in tilerow.iter().enumerate() {
                match (tile, edges.get(&(row, col))) {
                    (None, None) => print!(" "),
                    (Some(t), None) => print!("{}", t),
                    (Some(_), Some(n)) => print!("{}", n),
                    x => unreachable!("{:?}", x),
                };
            }
            println!();
        }
        println!("{} corners found", corners.len());
    }

    for m in path {
        match m {
            Move::L => dir = dir.l(),
            Move::R => dir = dir.r(),
            Move::Go(dist) => (pos, dir) = walk2(&board, &edges, pos, dir, dist, edge_len),
        }
    }

    let (row, col) = pos;
    Box::new((row + 1) * 1000 + (col + 1) * 4 + dir as usize)
}

type Colors = HashMap<Coord, u8>;

fn trace_edges(board: &Board, pos: Coord, edge_len: usize) -> Colors {
    fn should_explore<T>(
        board: &Board,
        res: &HashMap<Coord, T>,
        row: impl TryInto<usize>,
        col: impl TryInto<usize>,
    ) -> bool {
        match (row.try_into(), col.try_into()) {
            (Ok(row), Ok(col)) => !res.contains_key(&(row, col)) && get(board, row, col).is_some(),
            _ => false,
        }
    }

    let mut res = Colors::new();
    let mut todo = vec![pos];
    let mut n = 0;
    while let Some(pos) = todo.pop() {
        if let Some(n) = res.get(&pos) {
            println!("SKIP {:?} ({})", pos, n);
            continue;
        }
        if get(board, pos.0, pos.1).is_none() {
            println!("DONT LOOK {:?}", pos);
            continue;
        }
        let (r, c) = (pos.0 as isize, pos.1 as isize);
        let upleft = should_explore(board, &res, r - 1, c - 1);
        let up = should_explore(board, &res, r - 1, c);
        let upright = should_explore(board, &res, r - 1, c + 1);
        let right = should_explore(board, &res, r, c + 1);
        let downright = should_explore(board, &res, r + 1, c + 1);
        let down = should_explore(board, &res, r + 1, c);
        let downleft = should_explore(board, &res, r + 1, c - 1);
        let left = should_explore(board, &res, r, c - 1);
        if !upleft && downright {
            println!("DR from {:?}", pos);
            for r in 0..edge_len {
                for c in 0..edge_len {
                    let ok = res.insert((pos.0 + r, pos.1 + c), n).is_none();
                    assert!(ok);
                }
            }
            todo.push((pos.0, pos.1 + edge_len));
            todo.push((pos.0 + edge_len, pos.1));
            n += 1;
        } else if upleft && !downright {
            println!("UL from {:?}", pos);
            for r in 0..edge_len {
                for c in 0..edge_len {
                    let ok = res.insert((pos.0 - r, pos.1 - c), n).is_none();
                    assert!(ok);
                }
            }
            if pos.0 >= edge_len {
                todo.push((pos.0 - edge_len, pos.1));
            }
            if pos.1 >= edge_len {
                todo.push((pos.0, pos.1 - edge_len));
            }
            n += 1;
        } else if !upright && downleft {
            println!("DL from {:?}", pos);
            for r in 0..edge_len {
                for c in 0..edge_len {
                    let ok = res.insert((pos.0 + r, pos.1 - c), n).is_none();
                    assert!(ok);
                }
            }
            todo.push((pos.0 + edge_len, pos.1));
            if pos.1 >= edge_len {
                todo.push((pos.0, pos.1 - edge_len));
            }
            n += 1;
        } else if upright && !downleft {
            println!("UR from {:?}", pos);
            for r in 0..edge_len {
                for c in 0..edge_len {
                    let ok = res.insert((pos.0 - r, pos.1 + c), n).is_none();
                    assert!(ok);
                }
            }
            if pos.0 >= edge_len {
                todo.push((pos.0 - edge_len, pos.1));
            }
            if pos.0 >= edge_len {
                todo.push((pos.0, pos.1 - edge_len));
            }
            n += 1;
        }
        if up {
            todo.push((pos.0 - 1, pos.1));
            println!("  todo: U {:?}", todo[todo.len() - 1]);
        }
        if down {
            todo.push((pos.0 + 1, pos.1));
            println!("  todo: D {:?}", todo[todo.len() - 1]);
        }
        if left {
            todo.push((pos.0, pos.1 - 1));
            println!("  todo: L {:?}", todo[todo.len() - 1]);
        }
        if right {
            todo.push((pos.0, pos.1 + 1));
            println!("  todo: R {:?}", todo[todo.len() - 1]);
        }
    }
    res
}

//fn match_edges(board: &Board) -> HashMap<(isize, isize), Coord> {
//    let mut pos = find_start(board);
//    let mut dir = Dir::Right;
//    let mut res = HashMap::new();
//    while !res.contains(&pos) {
//        j
//    }
//}

fn find_corners(board: &Board, pos: Coord) -> HashSet<Coord> {
    let mut to_visit = vec![pos];
    let mut corners = HashSet::new();

    while let Some(pos) = to_visit.pop() {
        if !corners.insert(pos) {
            continue;
        }
        let (row, col) = (pos.0 as isize, pos.1 as isize);
        match (
            get(board, row - 1, col),
            get(board, row, col - 1),
            get(board, row + 1, col),
            get(board, row, col + 1),
        ) {
            (Some(_up), Some(_left), None, None) => {
                to_visit.push(find_edge(board, (row, col), (-1, 0)));
                to_visit.push(find_edge(board, (row, col), (0, -1)));
            }
            (None, Some(_left), Some(_down), None) => {
                to_visit.push(find_edge(board, (row, col), (0, -1)));
                to_visit.push(find_edge(board, (row, col), (1, 0)));
            }
            (None, None, Some(_down), Some(_right)) => {
                to_visit.push(find_edge(board, (row, col), (1, 0)));
                to_visit.push(find_edge(board, (row, col), (0, 1)));
            }
            (Some(_up), None, None, Some(_right)) => {
                to_visit.push(find_edge(board, (row, col), (0, 1)));
                to_visit.push(find_edge(board, (row, col), (-1, 0)));
            }
            (Some(_up), Some(_left), Some(_down), Some(_right)) => {
                match (
                    get(board, row - 1, col - 1),
                    get(board, row - 1, col + 1),
                    get(board, row + 1, col - 1),
                    get(board, row + 1, col + 1),
                ) {
                    (None, Some(_ur), Some(_dl), Some(_dr)) => {
                        to_visit.push(find_edge(board, (row, col), (-1, 0)));
                        to_visit.push(find_edge(board, (row, col), (0, -1)));
                    }
                    (Some(_ul), None, Some(_dl), Some(_dr)) => {
                        to_visit.push(find_edge(board, (row, col), (-1, 0)));
                        to_visit.push(find_edge(board, (row, col), (0, 1)));
                    }
                    (Some(_ul), Some(_ur), None, Some(_dr)) => {
                        to_visit.push(find_edge(board, (row, col), (1, 0)));
                        to_visit.push(find_edge(board, (row, col), (0, -1)));
                    }
                    (Some(_ul), Some(_ur), Some(_dl), None) => {
                        to_visit.push(find_edge(board, (row, col), (1, 0)));
                        to_visit.push(find_edge(board, (row, col), (0, 1)));
                    }
                    (ul, ur, dl, dr) => {
                        unreachable!("ul={:?} ur={:?} dl={:?} dr={:?}", ul, ur, dl, dr)
                    }
                };
            }
            (u, l, d, r) => unreachable!(
                "shouldn't be able to go l={:?} r={:?} u={:?} d={:?} from {:?}",
                l, r, u, d, pos
            ),
        }
    }

    corners
}

fn find_edge(board: &Board, mut pos: (isize, isize), step: (isize, isize)) -> Coord {
    loop {
        let next = (pos.0 + step.0, pos.1 + step.1);
        match get(board, next.0, next.1) {
            None => return (pos.0 as usize, pos.1 as usize),
            Some(_) => {
                if count_neighbors(board, next.0, next.1) == 4 {
                    return (next.0 as usize, next.1 as usize);
                } else {
                    pos = next;
                }
            }
        };
    }
}

fn get(board: &Board, row: impl TryInto<usize>, col: impl TryInto<usize>) -> Option<Tile> {
    match (row.try_into(), col.try_into()) {
        (Ok(row), Ok(col)) => board
            .tiles
            .get(row)
            .and_then(|r| r.get(col))
            .and_then(|t| *t),
        _ => None,
    }
}

fn count_neighbors<C>(board: &Board, row: C, col: C) -> usize
where
    C: TryInto<usize> + Sub<isize, Output = C> + Add<isize, Output = C> + Copy,
{
    [
        get(board, row + 1, col),
        get(board, row, col + 1),
        get(board, row - 1, col),
        get(board, row, col - 1),
    ]
    .iter()
    .filter(|x| x.is_some())
    .count()
}

fn walk2(
    board: &Board,
    colors: &Colors,
    pos: Coord,
    mut dir: Dir,
    dist: usize,
    edge_len: usize,
) -> (Coord, Dir) {
    let (mut r, mut c) = (pos.0 as isize, pos.1 as isize);
    for _ in 0..dist {
        let (dr, dc) = dir.d();
        let (newr, newc) = (r + dr, c + dc);
        match get(board, newr, newc) {
            Some(Tile::Wall) => break,
            Some(Tile::Open) => (r, c) = (newr, newc),
            None => todo!(
                "suck dir={:?} dest={:?} color={}",
                dir,
                (newr, newc),
                colors.get(&(r as usize, c as usize)).unwrap()
            ),
        };
    }
    ((r as usize, c as usize), dir)
}

fn walk(board: &Board, mut pos: Coord, dir: Dir, dist: usize) -> Coord {
    for _ in 0..dist {
        match dir.step(board, pos) {
            None => break,
            Some(p) => pos = p,
        };
    }
    pos
}

type Coord = (usize, usize);

#[derive(Clone, Copy, Debug)]
enum Dir {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Dir {
    fn step(&self, board: &Board, pos: Coord) -> Option<Coord> {
        let (mut pr, mut pc) = (pos.0 as isize, pos.1 as isize);
        let (dr, dc) = self.d();
        for _no_infinite_loop in 0..(board.tiles.len() + board.tiles[0].len()) {
            let nextr = match pr + dr {
                r if r < 0 => r + board.tiles.len() as isize,
                r => r % board.tiles.len() as isize,
            };
            let nextc = match pc + dc {
                c if c < 0 => c + board.tiles[0].len() as isize,
                c => c % board.tiles[0].len() as isize,
            };
            match get(board, nextr, nextc) {
                None => {
                    pr = nextr;
                    pc = nextc;
                }
                Some(Tile::Open) => return Some((nextr as usize, nextc as usize)),
                Some(Tile::Wall) => return None,
            };
        }
        unreachable!()
    }

    fn d(&self) -> (isize, isize) {
        match self {
            Dir::Right => (0, 1),
            Dir::Down => (1, 0),
            Dir::Left => (0, -1),
            Dir::Up => (-1, 0),
        }
    }

    fn l(&self) -> Self {
        match self {
            Dir::Up => Dir::Left,
            Dir::Left => Dir::Down,
            Dir::Down => Dir::Right,
            Dir::Right => Dir::Up,
        }
    }

    fn r(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}

struct Board {
    tiles: Vec<Vec<Option<Tile>>>,
}

#[derive(Clone, Copy, Debug)]
enum Tile {
    Open,
    Wall,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Open => '.',
                Tile::Wall => '#',
            }
        )
    }
}

fn find_start(board: &Board) -> Coord {
    (
        0,
        board.tiles[0]
            .iter()
            .position(|x| matches!(x, Some(Tile::Open)))
            .unwrap(),
    )
}

fn parse_board(input: &str) -> Board {
    fn parse_line(s: &str) -> Vec<Option<Tile>> {
        s.chars()
            .map(|c| match c {
                ' ' => None,
                '.' => Some(Tile::Open),
                '#' => Some(Tile::Wall),
                _ => unreachable!(),
            })
            .collect()
    }
    let tiles = input.lines().map(parse_line).collect();
    Board { tiles }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in &self.tiles {
            for t in r {
                match t {
                    None => write!(f, " "),
                    Some(t) => write!(f, "{}", t),
                }?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

type Path = Vec<Move>;

enum Move {
    Go(usize),
    R,
    L,
}

fn parse_path(s: &str) -> Path {
    let mut res = Path::new();
    let mut cur = 0;
    for c in s.trim().chars() {
        match c {
            '0' => cur *= 10,
            '1' => cur = cur * 10 + 1,
            '2' => cur = cur * 10 + 2,
            '3' => cur = cur * 10 + 3,
            '4' => cur = cur * 10 + 4,
            '5' => cur = cur * 10 + 5,
            '6' => cur = cur * 10 + 6,
            '7' => cur = cur * 10 + 7,
            '8' => cur = cur * 10 + 8,
            '9' => cur = cur * 10 + 9,
            'L' => {
                res.push(Move::Go(cur));
                res.push(Move::L);
                cur = 0;
            }
            'R' => {
                res.push(Move::Go(cur));
                res.push(Move::R);
                cur = 0;
            }
            c => unreachable!("unexpected {:?} in {:?}", c, s),
        };
    }
    res.push(Move::Go(cur));
    res
}

#[cfg(test)]
mod test {
    use super::*;

    crate::test::aoc_test!(example, r"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5",
        part1 => 6032,
        part2 => "todo");
}
