use std::{collections::BTreeSet, fmt::Display};

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
    }
}
