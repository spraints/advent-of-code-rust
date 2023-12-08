use std::{collections::HashMap, fmt::Display};

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let (directions, map) = parse(input, vis);

    let mut cur: &str = "AAA";
    let mut steps: u32 = 0;
    loop {
        for dir in &directions {
            steps += 1;
            let (l, r) = map.get(cur).unwrap();
            cur = match dir {
                Dir::L => l,
                Dir::R => r,
            };
            if cur == "ZZZ" {
                return Box::new(steps);
            }
            //if steps > 100 {
            //    panic!("not found");
            //}
        }
    }
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    let (directions, map) = parse(input, vis);
    let mut cur: Vec<&str> = map
        .keys()
        .filter(|s| s.ends_with("A"))
        .map(|s| s.as_str())
        .collect();
    let tracks = cur.len();
    if vis {
        println!("start => {cur:?}");
    }
    let mut steps = 0;
    loop {
        for dir in &directions {
            steps += 1;
            let mut done = true;
            for i in 0..tracks {
                let (l, r) = map.get(cur[i]).unwrap();
                cur[i] = match dir {
                    Dir::L => l,
                    Dir::R => r,
                };
                if !cur[i].ends_with("Z") {
                    done = false;
                }
            }
            if vis {
                println!("[{steps}] (done? {done}) => {cur:?}");
            }
            if done {
                return Box::new(steps);
            }
            //if steps > 100 {
            //    panic!("not found");
            //}
        }
    }
}

fn parse(input: String, vis: bool) -> (Vec<Dir>, HashMap<String, (String, String)>) {
    let (directions, map_input) = input.split_once("\n\n").unwrap();
    let directions = directions.trim().chars().map(|d| d.into()).collect();
    let mut map = HashMap::new();
    let line_re = regex::Regex::new(r"^(...) = \((...), (...)\)").unwrap();
    // I'm not sure why this doesn't work :( it only gets the first match.
    //for (_, [from, l, r]) in line_re.captures_iter(&map_input).map(|c| c.extract()) {
    //    println!("FUFUIFUDIF");
    //    map.insert(from.to_owned(), (l.to_owned(), r.to_owned()));
    //}
    for line in map_input.lines() {
        let (_, [from, l, r]) = line_re.captures(line).unwrap().extract();
        map.insert(from.to_owned(), (l.to_owned(), r.to_owned()));
    }
    if vis {
        println!("directions = {directions:?}");
        println!("map = {map:?}");
        println!("map_input = {map_input}");
    }
    (directions, map)
}

#[derive(Debug)]
enum Dir {
    L,
    R,
}

impl From<char> for Dir {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::L,
            'R' => Self::R,
            _ => panic!("illegal direction {c:?}"),
        }
    }
}

#[cfg(test)]
mod test {
    crate::test::aoc_test!(
        part1,
        r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        2
    );

    crate::test::aoc_test!(
        part1,
        part1_2,
        r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        6
    );

    crate::test::aoc_test!(
        part2,
        r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        6
    );
}
