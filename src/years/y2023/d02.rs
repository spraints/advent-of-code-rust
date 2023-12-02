use std::fmt::Display;

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let games = input.lines().map(|l| parse_game(l, vis));
    let mut res = 0;
    for (id, moves) in games {
        let mut possible = true;
        for m in moves {
            if m.0 > 12 || m.1 > 13 || m.2 > 14 {
                possible = false;
            }
        }
        if possible {
            res += id;
        }
    }
    Box::new(res)
}

fn parse_game(line: &str, vis: bool) -> (u32, Vec<(u32, u32, u32)>) {
    if vis {
        println!("parse_game({line:?})");
    }
    let (id, moves) = line.split_once(':').unwrap();
    if vis {
        println!(" id = {id}");
    }
    let (_, id) = id.split_once(' ').unwrap();
    if vis {
        println!(" id = {id:?}");
    }
    let id = id.parse().unwrap();
    let moves = moves.split(';').map(|m| parse_move(m, vis)).collect();
    if vis {
        println!(" => ({id}, {moves:?})");
    }
    (id, moves)
}

fn parse_move(m: &str, vis: bool) -> (u32, u32, u32) {
    let mut res = (0, 0, 0);
    for group in m.split(',') {
        if vis {
            println!("  parse {group:?}");
        }
        let (count, color) = group.trim().split_once(' ').unwrap();
        let count = count.parse().unwrap();
        if vis {
            println!("  -> count = {count}");
        }
        match color.trim() {
            "red" => res.0 = count,
            "green" => res.1 = count,
            "blue" => res.2 = count,
            _ => panic!("bad {m}"),
        };
    }
    res
}

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

#[cfg(test)]
mod test {
    use super::*;

    crate::test::aoc_test!(example, r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        part1 => 8,
        part2 => "todo");
}
