use std::fmt::Display;

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let (board, path) = input.split_once("\n\n").unwrap();

    let board = parse_board(board);
    let path = parse_path(path);

    let mut pos = find_start(&board);
    let mut dir = Dir::Right;

    for m in path {
        match m {
            Move::L => {
                dir = match dir {
                    Dir::Up => Dir::Left,
                    Dir::Left => Dir::Down,
                    Dir::Down => Dir::Right,
                    Dir::Right => Dir::Up,
                }
            }
            Move::R => {
                dir = match dir {
                    Dir::Up => Dir::Right,
                    Dir::Right => Dir::Down,
                    Dir::Down => Dir::Left,
                    Dir::Left => Dir::Up,
                }
            }
            Move::Go(dist) => pos = walk(&board, pos, dir, dist),
        }
    }

    let (row, col) = pos;
    Box::new((row + 1) * 1000 + (col + 1) * 4 + dir as usize)
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    Box::new("todo")
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
        let (dr, dc) = match self {
            Dir::Right => (0, 1),
            Dir::Down => (1, 0),
            Dir::Left => (0, -1),
            Dir::Up => (-1, 0),
        };
        for _no_infinite_loop in 0..(board.tiles.len() + board.tiles[0].len()) {
            let nextr = match pr + dr {
                r if r < 0 => r + board.tiles.len() as isize,
                r => r % board.tiles.len() as isize,
            };
            let nextc = match pc + dc {
                c if c < 0 => c + board.tiles[0].len() as isize,
                c => c % board.tiles[0].len() as isize,
            };
            println!("({}, {}) ... {:?} ... ({}, {})", pr, pc, self, nextr, nextc);
            match board
                .tiles
                .get(nextr as usize)
                .and_then(|r| r.get(nextc as usize))
            {
                None | Some(Tile::None) => {
                    pr = nextr;
                    pc = nextc;
                }
                Some(Tile::Open) => return Some((nextr as usize, nextc as usize)),
                Some(Tile::Wall) => return None,
            };
        }
        unreachable!()
    }
}

struct Board {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Debug)]
enum Tile {
    None,
    Open,
    Wall,
}

fn find_start(board: &Board) -> Coord {
    (
        0,
        board.tiles[0]
            .iter()
            .position(|x| matches!(x, Tile::Open))
            .unwrap(),
    )
}

fn parse_board(input: &str) -> Board {
    fn parse_line(s: &str) -> Vec<Tile> {
        s.chars()
            .map(|c| match c {
                ' ' => Tile::None,
                '.' => Tile::Open,
                '#' => Tile::Wall,
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
                write!(
                    f,
                    "{}",
                    match t {
                        Tile::None => ' ',
                        Tile::Open => '.',
                        Tile::Wall => '#',
                    }
                )?;
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
