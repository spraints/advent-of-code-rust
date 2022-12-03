use std::fmt::Display;

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

fn compartmentalize(line: &str) -> (&str, &str) {
    line.split_at(line.len() / 2)
}

fn compare_compartments(ab: (&str, &str)) -> char {
    let (a, b) = ab;
    let mut a: Vec<char> = a.chars().collect();
    a.sort_unstable();
    let mut b: Vec<char> = b.chars().collect();
    b.sort_unstable();
    for ac in a {
        for bc in &b {
            if ac == *bc {
                return ac;
            }
        }
    }
    panic!("No match in {:?}", ab);
}

fn compare_groups(abc: &[&str]) -> char {
    let mut a: Vec<char> = abc[0].chars().collect();
    a.sort_unstable();
    let mut b: Vec<char> = abc[1].chars().collect();
    b.sort_unstable();
    let mut c: Vec<char> = abc[2].chars().collect();
    c.sort_unstable();
    for ac in a {
        for bc in &b {
            if ac == *bc {
                for cc in &c {
                    if ac == *cc {
                        return ac;
                    }
                }
            }
        }
    }
    panic!("No match in {:?}", abc);
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
    }

    #[test]
    fn part2_example() {
        dotest(70, EX, part2);
    }
}
