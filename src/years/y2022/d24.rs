use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fmt::Display,
};

const MOVES: [(isize, isize); 5] = [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)];

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let board = parse_input(&input);
    let you_start = (0, board[0].iter().position(|sq| *sq == EMPTY).unwrap());
    let bottom_row = board.len() - 1;

    if vis {
        println!("Initial state:");
        show_state(&board, &you_start, 0);

        println!();
        println!("Minute 1 (after waiting):");
        show_state(&board, &you_start, 1);
    }

    let mut tried = HashSet::new();
    let mut todo = BinaryHeap::new();
    todo.push(Reverse(State {
        elapsed: 0,
        pos: you_start,
    }));

    while let Some(Reverse(st)) = todo.pop() {
        if vis {
            println!("try {:?}", st);
        }
        let State { elapsed, pos } = st;
        if tried.contains(&(pos, elapsed)) {
            continue;
        }
        tried.insert((pos, elapsed));

        if pos.0 == bottom_row {
            return Box::new(elapsed);
        }

        let elapsed = elapsed + 1;
        let (r, c) = (pos.0 as isize, pos.1 as isize);
        for (dr, dc) in &MOVES {
            let nr = r + dr;
            let nc = c + dc;
            if nr >= 0 && nc >= 0 {
                let pos = (nr as usize, nc as usize);
                if pos.0 < board.len() && pos.1 <= board[0].len() {
                    if is_empty(&board, elapsed, pos) {
                        let next = State { pos, elapsed };
                        if vis {
                            println!(" -> {:?}", next);
                        }
                        todo.push(Reverse(next));
                    }
                }
            }
        }
    }

    unreachable!()
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    let board = parse_input(&input);
    let you_start = (0, board[0].iter().position(|sq| *sq == EMPTY).unwrap());
    let bottom_row = board.len() - 1;
    let you_finish = (
        bottom_row,
        board[bottom_row]
            .iter()
            .position(|sq| *sq == EMPTY)
            .unwrap(),
    );

    let elapsed = go(&board, 0, you_start, you_finish);
    if vis {
        println!("got to the end after {} minutes", elapsed);
    }
    let elapsed = go(&board, elapsed, you_finish, you_start);
    if vis {
        println!("got back to the start after {} minutes", elapsed);
    }
    let elapsed = go(&board, elapsed, you_start, you_finish);
    if vis {
        println!("got back to the end after {} minutes", elapsed);
    }

    Box::new(elapsed)
}

fn go(board: &Board, elapsed: usize, start: Coord, finish: Coord) -> usize {
    let mut tried = HashSet::new();
    let mut todo = BinaryHeap::new();
    todo.push(Reverse(State {
        elapsed,
        pos: start,
    }));

    while let Some(Reverse(st)) = todo.pop() {
        let State { elapsed, pos } = st;
        if tried.contains(&(pos, elapsed)) {
            continue;
        }
        tried.insert((pos, elapsed));

        if pos == finish {
            return elapsed;
        }

        let elapsed = elapsed + 1;
        let (r, c) = (pos.0 as isize, pos.1 as isize);
        for (dr, dc) in &MOVES {
            let nr = r + dr;
            let nc = c + dc;
            if nr >= 0 && nc >= 0 {
                let pos = (nr as usize, nc as usize);
                if pos.0 < board.len() && pos.1 <= board[0].len() {
                    if is_empty(&board, elapsed, pos) {
                        let next = State { pos, elapsed };
                        todo.push(Reverse(next));
                    }
                }
            }
        }
    }

    unreachable!()
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
struct State {
    elapsed: usize,
    pos: Coord,
}

type Coord = (usize, usize);

type Board = Vec<Vec<char>>;

const WALL: char = '#';
const EMPTY: char = '.';
const UP: char = '^';
const LEFT: char = '<';
const DOWN: char = 'v';
const RIGHT: char = '>';

const EXPEDITION: char = 'E';

fn parse_input(input: &str) -> Board {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn show_state(board: &Board, you: &Coord, elapsed: usize) {
    let rows = board.len();
    let cols = board[0].len();
    for r in 0..rows {
        for c in 0..cols {
            print!("{}", render(board, you, elapsed, (r, c)));
        }
        println!();
    }
}

fn is_empty(board: &Board, elapsed: usize, pos: Coord) -> bool {
    get(board, elapsed, pos) == EMPTY
}

fn get(board: &Board, elapsed: usize, pos: Coord) -> char {
    let (r, c) = pos;

    if board[r][c] == WALL {
        return WALL;
    }

    if r == 0 || c == 0 || r + 1 == board.len() || c + 1 == board[0].len() {
        assert!(board[r][c] == EMPTY);
        return EMPTY;
    }

    let brows = board.len() - 2;
    let bcols = board[0].len() - 2;
    let roff = elapsed % brows;
    let coff = elapsed % bcols;

    let is_up = board[(r + roff - 1) % brows + 1][c] == UP;
    let is_down = board[(r + brows - roff - 1) % brows + 1][c] == DOWN;
    let is_left = board[r][(c + coff - 1) % bcols + 1] == LEFT;
    let is_right = board[r][(c + bcols - coff - 1) % bcols + 1] == RIGHT;

    match (is_up, is_down, is_left, is_right) {
        (false, false, false, false) => EMPTY,
        (true, false, false, false) => UP,
        (false, true, false, false) => DOWN,
        (false, false, true, false) => LEFT,
        (false, false, false, true) => RIGHT,
        (a, b, c, d) => match a as u8 + b as u8 + c as u8 + d as u8 {
            2 => '2',
            3 => '3',
            4 => '4',
            n => unreachable!(
                "up:{} down:{} left:{} right:{} -> count={}",
                is_up, is_down, is_left, is_right, n
            ),
        },
    }
}

fn render(board: &Board, you: &Coord, elapsed: usize, pos: Coord) -> char {
    let (r, c) = pos;

    if board[r][c] == WALL {
        assert!(*you != pos);
        return WALL;
    }

    if r == 0 || c == 0 || r + 1 == board.len() || c + 1 == board[0].len() {
        assert!(board[r][c] == EMPTY);
        if *you == pos {
            return EXPEDITION;
        } else {
            return board[r][c];
        }
    }

    let brows = board.len() - 2;
    let bcols = board[0].len() - 2;
    let roff = elapsed % brows;
    let coff = elapsed % bcols;
    //println!("r={} roff={} brows={}", r, roff, brows);
    let is_up = board[(r + roff - 1) % brows + 1][c] == UP;
    let is_down = board[(r + brows - roff - 1) % brows + 1][c] == DOWN;
    let is_left = board[r][(c + coff - 1) % bcols + 1] == LEFT;
    let is_right = board[r][(c + bcols - coff - 1) % bcols + 1] == RIGHT;

    if *you == pos {
        assert!(!is_up);
        assert!(!is_down);
        assert!(!is_right);
        assert!(!is_left);
        return 'E';
    }

    match (is_up, is_down, is_left, is_right) {
        (false, false, false, false) => '.',
        (true, false, false, false) => '^',
        (false, true, false, false) => 'v',
        (false, false, true, false) => '<',
        (false, false, false, true) => '>',
        (a, b, c, d) => match a as u8 + b as u8 + c as u8 + d as u8 {
            2 => '2',
            3 => '3',
            4 => '4',
            n => unreachable!(
                "up:{} down:{} left:{} right:{} -> count={}",
                is_up, is_down, is_left, is_right, n
            ),
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    crate::test::aoc_test!(example, r"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#",
        part1 => 18,
        part2 => 54);
}
