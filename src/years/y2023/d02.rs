use std::cmp::max;
use std::fmt::Display;

pub fn part1_regexp(input: String, vis: bool) -> Box<dyn Display> {
    let mut res = 0;
    for (id, moves) in parse_games(&input, vis) {
        if moves
            .into_iter()
            .all(|m| m.red <= 12 && m.green <= 13 && m.blue <= 14)
        {
            res += id;
        }
    }
    Box::new(res)
}

pub fn part2_regexp(input: String, vis: bool) -> Box<dyn Display> {
    let mut res = 0;
    for (_, moves) in parse_games(&input, false) {
        if vis {
            println!("{moves:?}");
        }
        let req = moves
            .into_iter()
            .reduce(|a, b| Move {
                red: max(a.red, b.red),
                green: max(a.green, b.green),
                blue: max(a.blue, b.blue),
            })
            .unwrap();
        if vis {
            println!(" ==> {req:?}");
        }
        res += req.red * req.green * req.blue;
    }
    Box::new(res)
}

#[derive(Debug)]
struct Move {
    red: u32,
    green: u32,
    blue: u32,
}

fn parse_games(input: &str, vis: bool) -> Vec<(u32, Vec<Move>)> {
    let game_re = regex::Regex::new(r"Game (?<id>\d+): (?<moves>.*)").unwrap();
    let move_re = regex::Regex::new(r"(?<count>\d+) (?<color>\w+)").unwrap();

    let mut res = Vec::new();

    for line in input.lines() {
        if vis {
            println!("parse_game({line:?})");
        }
        let vals = game_re.captures(line).unwrap();
        let id = vals.name("id").unwrap().as_str().parse().unwrap();
        let mut moves = Vec::new();
        for m in vals.name("moves").unwrap().as_str().split(';') {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for part in m.split(',') {
                let part_vals = move_re.captures(part).unwrap();
                let count = part_vals.name("count").unwrap().as_str().parse().unwrap();
                match part_vals.name("color").unwrap().as_str() {
                    "red" => red = count,
                    "green" => green = count,
                    "blue" => blue = count,
                    _ => panic!("illegal game {line:?} (part: {part})"),
                };
            }
            moves.push(Move { red, green, blue });
        }
        if vis {
            println!("=> ({id}, {moves:?})");
        }
        res.push((id, moves));
    }

    res
}

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

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    let games = input.lines().map(|l| parse_game(l, false));
    let mut res = 0;
    for (_, moves) in games {
        if vis {
            println!("{moves:?}");
        }
        let mut cubes = (0, 0, 0);
        for m in moves {
            cubes = (max(cubes.0, m.0), max(cubes.1, m.1), max(cubes.2, m.2));
        }
        if vis {
            println!(" ==> {cubes:?}");
        }
        res += cubes.0 * cubes.1 * cubes.2;
    }
    Box::new(res)
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
        part1_regexp => 8,
        part2 => 2286,
        part2_regexp => 2286);
}
