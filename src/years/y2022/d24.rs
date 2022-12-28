use std::fmt::Display;

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let board = parse_input(&input);
    let mut you = (0, board[0].iter().position(|sq| *sq == EMPTY).unwrap());
    if vis {
        println!("Initial state:");
        show_state(&board, &you, 0);
    }

    you = (1, 1);
    if vis {
        println!();
        println!("Minute 1:");
        show_state(&board, &you, 1);
    }

    Box::new("todo")
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    Box::new("todo")
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
            return WALL;
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
        part2 => "todo");
}
