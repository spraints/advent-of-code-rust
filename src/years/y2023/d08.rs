use std::{collections::HashMap, fmt::Display};

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let (directions, map_input) = input.split_once("\n\n").unwrap();
    let directions = directions.trim();
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

    let mut cur: &str = "AAA";
    let mut steps: u32 = 0;
    loop {
        for dir in directions.chars() {
            steps += 1;
            let (l, r) = map.get(cur).unwrap();
            cur = match dir {
                'L' => l,
                'R' => r,
                _ => panic!("illegal direction {dir:?}"),
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

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    crate::test::aoc_test!(part1, TEST_INPUT, 2);
    crate::test::aoc_test!(part2, TEST_INPUT, "todo");

    const TEST_INPUT2: &'static str = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    crate::test::aoc_test!(part1, part1_2, TEST_INPUT2, 6);
}
