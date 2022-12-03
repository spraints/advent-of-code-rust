use std::{
    collections::{BTreeSet, HashSet},
    fmt::Display,
};

pub fn part1(input: String) -> Box<dyn Display> {
    let sacks = input.lines().map(compartmentalize);
    let common = sacks.map(compare_compartments);
    let priorities = common.map(priority);
    let total_priority: u32 = priorities.sum();
    Box::new(total_priority)
}

pub fn part2(input: String) -> Box<dyn Display> {
    let sacks = input.lines().collect::<Vec<&str>>();
    let groups = sacks.chunks(3);
    let common = groups.map(compare_groups);
    let priorities = common.map(priority);
    let total_priority: u32 = priorities.sum();
    Box::new(total_priority)
}

pub fn part1_set(input: String) -> Box<dyn Display> {
    let sacks = input.lines().map(compartmentalize);
    let common = sacks.map(compare_compartments_set);
    let total_priority: u32 = common.sum();
    Box::new(total_priority)
}

pub fn part2_set(input: String) -> Box<dyn Display> {
    let sacks = input.lines().collect::<Vec<&str>>();
    let groups = sacks.chunks(3);
    let priorities = groups.map(compare_groups_set);
    let total_priority: u32 = priorities.sum();
    Box::new(total_priority)
}

pub fn part2_set2(input: String) -> Box<dyn Display> {
    let sacks = input.lines().collect::<Vec<&str>>();
    let groups = sacks.chunks(3);
    let common = groups.map(compare_groups_set2);
    let priorities = common.map(priority);
    let total_priority: u32 = priorities.sum();
    Box::new(total_priority)
}

pub fn part2_bytes(input: String) -> Box<dyn Display> {
    let mut priorities = vec![0; 256];
    let little_a = b'a' as usize;
    let big_a = b'A' as usize;
    for i in 0..26 {
        priorities[little_a + i] = i + 1;
        priorities[big_a + i] = i + 27;
    }
    let mut sacks = input.lines();
    let mut total_priority = 0;
    loop {
        let a = match sacks.next() {
            Some(s) => s.as_bytes(),
            None => break,
        };
        let b = sacks.next().unwrap().as_bytes();
        let c = sacks.next().unwrap().as_bytes();
        'search: for ac in a {
            for bc in b.iter() {
                if ac == bc {
                    for cc in c.iter() {
                        if ac == cc {
                            total_priority += priorities[*ac as usize];
                            break 'search;
                        }
                    }
                }
            }
        }
    }
    Box::new(total_priority)
}

fn compartmentalize(line: &str) -> (&str, &str) {
    line.split_at(line.len() / 2)
}

fn compare_compartments(ab: (&str, &str)) -> char {
    let (a, b) = ab;
    for ac in a.chars() {
        for bc in b.chars() {
            if ac == bc {
                return ac;
            }
        }
    }
    panic!("No match in {:?}", ab);
}

fn compare_compartments_set(ab: (&str, &str)) -> u32 {
    let (a, b) = ab;
    common(compare_two(score(a), score(b)))
}

fn common(s: BTreeSet<u32>) -> u32 {
    s.into_iter().next().unwrap()
}

fn compare_two(a: BTreeSet<u32>, b: BTreeSet<u32>) -> BTreeSet<u32> {
    a.intersection(&b).copied().collect()
}

fn score(s: &str) -> BTreeSet<u32> {
    s.chars().map(priority).collect()
}

fn compare_groups(abc: &[&str]) -> char {
    let a = abc[0];
    let b = abc[1];
    let c = abc[2];
    for ac in a.chars() {
        for bc in b.chars() {
            if ac == bc {
                for cc in c.chars() {
                    if ac == cc {
                        return ac;
                    }
                }
            }
        }
    }
    panic!("No match in {:?}", abc);
}

fn compare_groups_set(abc: &[&str]) -> u32 {
    let reduced = abc
        .iter()
        .map(|s| score(s))
        .reduce(|a, b| compare_two(a, b))
        .unwrap();
    common(reduced)
}

fn compare_groups_set2(abc: &[&str]) -> char {
    let common = reduce(abc[0].chars().collect(), abc[1].chars());
    let mut common = reduce(common.collect(), abc[2].chars());
    common.next().unwrap()
}

fn reduce(a: HashSet<char>, b: std::str::Chars) -> Reduction {
    Reduction { a, b }
}

struct Reduction<'a> {
    a: HashSet<char>,
    b: std::str::Chars<'a>,
}

impl<'a> Iterator for Reduction<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.b.next() {
                None => return None,
                Some(c) if self.a.contains(&c) => return Some(c),
                _ => continue,
            };
        }
    }
}

fn priority(c: char) -> u32 {
    const LOWER: u32 = 'a' as u32 - 1;
    const UPPER: u32 = 'A' as u32 - 27;
    c as u32 - if c.is_lowercase() { LOWER } else { UPPER }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::*;

    const EX: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_example() {
        dotest(157, EX, part1);
        dotest(157, EX, part1_set);
    }

    #[test]
    fn part2_example() {
        dotest(70, EX, part2);
        dotest(70, EX, part2_set);
        dotest(70, EX, part2_set2);
        dotest(70, EX, part2_bytes);
    }
}
