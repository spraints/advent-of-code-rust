use std::fmt::Display;

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let mut board = parse_input(&input);
    let mut other_board = board.clone();
    let mut you = (
        0,
        board[0]
            .iter()
            .position(|sq| matches!(sq, Square::Empty))
            .unwrap(),
    );
    if vis {
        println!("Initial state:");
        show_state(&board, &you);
    }

    you = (1, 1);
    (board, other_board) = step_storms(board, other_board);
    if vis {
        println!();
        println!("Minute 1:");
        show_state(&board, &you);
    }

    Box::new("todo")
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

fn step_storms(old_board: Board, mut new_board: Board) -> (Board, Board) {
    let rows = old_board.len();
    let cols = old_board[0].len();
    for r in 1..(rows - 1) {
        for c in 1..(cols - 1) {
            new_board[r][c] = Square::Empty;
        }
    }
    let brows = rows - 2;
    let bcols = cols - 2;
    for r in 1..(rows - 1) {
        for c in 1..(cols - 1) {
            if let Square::Blizzard {
                up,
                down,
                left,
                right,
            } = old_board[r][c]
            {
                if up {
                    new_board[(r + brows - 2) % brows + 1][c].set_up();
                }
                if down {
                    new_board[r % brows + 1][c].set_down();
                }
                if left {
                    new_board[r][(c + bcols - 2) % bcols + 1].set_left();
                }
                if right {
                    new_board[r][c % bcols + 1].set_right();
                }
            }
        }
    }
    (new_board, old_board)
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

impl Square {
    fn set_up(&mut self) {
        let new = match self {
            Self::Empty => UP,
            Self::Blizzard {
                up: false,
                down,
                left,
                right,
            } => Self::Blizzard {
                up: true,
                down: *down,
                left: *left,
                right: *right,
            },
            illegal => unreachable!("can't add an 'up' blizzard to {:?}", illegal),
        };
        *self = new
    }

    fn set_down(&mut self) {
        let new = match self {
            Self::Empty => DOWN,
            Self::Blizzard {
                down: false,
                up,
                left,
                right,
            } => Self::Blizzard {
                down: true,
                up: *up,
                left: *left,
                right: *right,
            },
            illegal => unreachable!("can't add a 'down' blizzard to {:?}", illegal),
        };
        *self = new;
    }

    fn set_left(&mut self) {
        let new = match self {
            Self::Empty => LEFT,
            Self::Blizzard {
                left: false,
                up,
                down,
                right,
            } => Self::Blizzard {
                left: true,
                up: *up,
                down: *down,
                right: *right,
            },
            illegal => unreachable!("can't add a 'left' blizzard to {:?}", illegal),
        };
        *self = new;
    }

    fn set_right(&mut self) {
        let new = match self {
            Self::Empty => RIGHT,
            Self::Blizzard {
                right: false,
                up,
                down,
                left,
            } => Self::Blizzard {
                right: true,
                up: *up,
                down: *down,
                left: *left,
            },
            illegal => unreachable!("can't add a 'right' blizzard to {:?}", illegal),
        };
        *self = new;
    }
}

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

fn show_state(board: &Board, you: &Coord) {
    let (yr, yc) = *you;
    for (r, row) in board.iter().enumerate() {
        for (c, sq) in row.iter().enumerate() {
            if r == yr && c == yc {
                assert!(matches!(sq, Square::Empty));
                print!("E");
            } else {
                match sq {
                    Square::Wall => print!("#"),
                    Square::Empty => print!("."),
                    sq if *sq == RIGHT => print!(">"),
                    sq if *sq == DOWN => print!("v"),
                    sq if *sq == LEFT => print!("<"),
                    sq if *sq == UP => print!("^"),
                    Square::Blizzard {
                        up,
                        down,
                        left,
                        right,
                    } => print!("{}", *up as u8 + *down as u8 + *left as u8 + *right as u8),
                };
            }
        }
        println!();
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
