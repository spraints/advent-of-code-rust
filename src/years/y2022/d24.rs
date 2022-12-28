use std::fmt::Display;

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let game = parse_input(&input);
    let you = None;
    show_state(&game, &you);
    Box::new("todo")
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

type Coord = (usize, usize);

type Square = [bool; 4];
struct Game {
    blizzards: Vec<Vec<Square>>,
    start_col: usize,
    exit_col: usize,
}

const UP: usize = 0;
const LEFT: usize = 1;
const DOWN: usize = 2;
const RIGHT: usize = 3;

fn parse_input(input: &str) -> Game {
    let mut lines = input.lines();

    let start_col = find_gap(lines.next().unwrap());

    let mut blizzards = Vec::new();
    for line in lines {
        assert_eq!("#", &line[0..1]);
        if &line[1..2] == "#" {
            let exit_col = find_gap(line);
            return Game {
                blizzards,
                start_col,
                exit_col,
            };
        }
        blizzards.push(parse_blizzards(line));
    }
    unreachable!()
}

fn find_gap(wall: &str) -> usize {
    wall[1..].chars().position(|c| c == '.').unwrap()
}

fn parse_blizzards(line: &str) -> Vec<Square> {
    fn init_square(dir: usize) -> Square {
        let mut sq = [false; 4];
        sq[dir] = true;
        sq
    }
    line[1..line.len() - 1]
        .chars()
        .map(|c| match c {
            '.' => [false; 4],
            '^' => init_square(UP),
            '>' => init_square(RIGHT),
            'v' => init_square(DOWN),
            '<' => init_square(LEFT),
            c => unreachable!("illegal char {}", c),
        })
        .collect()
}

fn show_state(game: &Game, you: &Option<Coord>) {
    println!("start_col: {}, exit_col: {}", game.start_col, game.exit_col);
    println!("blizz.len: {}", game.blizzards.len());
    let num_rows = game.blizzards.len() + 2;
    let num_cols = game.blizzards.len() + 2;
    for r in 0..num_rows {
        for c in 0..num_cols {
            if r == 0 || r + 1 == num_rows || c == 0 || c + 1 == num_cols {
                match you {
                    None if r == 0 && c == game.start_col + 1 => print!("E"),
                    Some((yr, yc)) if *yr == r && *yc == c => print!("E"),
                    _ if r == 0 && c == game.start_col + 1 => print!("."),
                    _ if r > 0 && c == game.exit_col + 1 => print!("."),
                    _ => print!("#"),
                };
            } else {
                match (you, game.blizzards[r - 1][c - 1]) {
                    (Some((yr, yc)), [false, false, false, false])
                        if *yr == r - 1 && *yc == c - 1 =>
                    {
                        print!("E")
                    }
                    (Some((yr, yc)), _) if *yr == r - 1 && *yc == c - 1 => print!("*"),
                    (_, [true, false, false, false]) => print!("^"),
                    (_, [false, true, false, false]) => print!("<"),
                    (_, [false, false, true, false]) => print!("v"),
                    (_, [false, false, false, true]) => print!(">"),
                    (_, [false, false, false, false]) => print!("."),
                    (_, sq) => print!("{}", sq.iter().filter(|x| **x).count()),
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
