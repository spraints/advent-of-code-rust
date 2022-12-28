use std::fmt::Display;

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let board = parse_input(&input);
    let mut you = (
        0,
        board[0]
            .iter()
            .position(|sq| matches!(sq, Square::Empty))
            .unwrap(),
    );
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

type Board = Vec<Vec<Square>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Square {
    Wall,
    Empty,
    Blizzard {
        up: bool,
        down: bool,
        left: bool,
        right: bool,
    },
}

const UP: Square = Square::Blizzard {
    up: true,
    down: false,
    left: false,
    right: false,
};
const DOWN: Square = Square::Blizzard {
    up: false,
    down: true,
    left: false,
    right: false,
};
const LEFT: Square = Square::Blizzard {
    up: false,
    down: false,
    left: true,
    right: false,
};
const RIGHT: Square = Square::Blizzard {
    up: false,
    down: false,
    left: false,
    right: true,
};

fn parse_input(input: &str) -> Board {
    fn parse_line(line: &str) -> Vec<Square> {
        line.chars()
            .map(|c| match c {
                '#' => Square::Wall,
                '.' => Square::Empty,
                '>' => RIGHT,
                'v' => DOWN,
                '<' => LEFT,
                '^' => UP,
                c => unreachable!("illegal input char {:?}", c),
            })
            .collect()
    }
    input.lines().map(parse_line).collect()
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

    if matches!(board[r][c], Square::Wall) {
        assert!(*you != pos);
        return '#';
    }

    if r == 0 || c == 0 || r + 1 == board.len() || c + 1 == board[0].len() {
        assert!(matches!(board[r][c], Square::Empty));
        if *you == pos {
            return 'E';
        } else {
            return '.';
        }
    }

    let brows = board.len() - 2;
    let bcols = board[0].len() - 2;
    let roff = elapsed % brows;
    let coff = elapsed % bcols;
    //println!("r={} roff={} brows={}", r, roff, brows);
    let is_up = matches!(
        board[(r + roff - 1) % brows + 1][c],
        Square::Blizzard { up: true, .. }
    );
    let is_down = matches!(
        board[(r + brows - roff - 1) % brows + 1][c],
        Square::Blizzard { down: true, .. }
    );
    let is_left = matches!(
        board[r][(c + coff - 1) % bcols + 1],
        Square::Blizzard { left: true, .. }
    );
    let is_right = matches!(
        board[r][(c + bcols - coff - 1) % bcols + 1],
        Square::Blizzard { right: true, .. }
    );

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
