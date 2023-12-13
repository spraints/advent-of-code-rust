use std::borrow::Cow;
use std::fmt::Display;

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let res: usize = input.lines().map(|line| solve(line, vis, 1)).sum();
    Box::new(res)
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    let res: usize = input.lines().map(|line| solve(line, vis, 5)).sum();
    Box::new(res)
}

fn solve(line: &str, vis: bool, mult: usize) -> usize {
    let (conditions, counts) = line.trim().split_once(' ').unwrap();

    fn r(s: &str, c: char, mult: usize) -> Cow<'_, str> {
        match mult {
            0 => "".into(),
            1 => s.into(),
            _ => {
                let mut res = String::with_capacity(s.len() * mult + mult);
                res.push_str(s);
                for i in 1..mult {
                    res.push(c);
                    res.push_str(s);
                }
                res.into()
            }
        }
    }
    let conditions = r(conditions, '?', mult);
    let counts = r(counts, ',', mult);

    let mut conditions: Vec<Cond> = conditions.chars().map(Cond::from).collect();
    let counts: Vec<u16> = counts.split(',').map(|n| n.parse().unwrap()).collect();
    if vis {
        println!("> {line}");
    }
    try_each(0, &mut conditions, &counts, vis)
}

fn try_each(i: usize, conditions: &mut [Cond], counts: &[u16], vis: bool) -> usize {
    match conditions.get(i).cloned() {
        None => {
            if get_counts(conditions) == counts {
                if vis {
                    println!("{}", Cond::str(conditions));
                }
                1
            } else {
                0
            }
        }
        Some(Cond::Ok) | Some(Cond::Broken) => try_each(i + 1, conditions, counts, vis),
        Some(Cond::Unknown) => {
            let mut res = 0;
            conditions[i] = Cond::Ok;
            res += try_each(i + 1, conditions, counts, vis);
            conditions[i] = Cond::Broken;
            res += try_each(i + 1, conditions, counts, vis);
            conditions[i] = Cond::Unknown;
            res
        }
    }
}

fn get_counts(conditions: &[Cond]) -> Vec<u16> {
    let mut broken_len = 0;
    let mut res = Vec::new();
    for c in conditions {
        match c {
            Cond::Unknown => panic!("no unknowns allowed! {conditions:?}"),
            Cond::Ok => {
                if broken_len > 0 {
                    res.push(broken_len);
                    broken_len = 0;
                }
            }
            Cond::Broken => broken_len += 1,
        };
    }
    if broken_len > 0 {
        res.push(broken_len);
    }
    res
}

#[derive(Debug, Clone, Copy)]
enum Cond {
    Unknown,
    Ok,
    Broken,
}

impl Cond {
    fn str(conditions: &[Self]) -> String {
        let mut res = String::with_capacity(conditions.len());
        for (i, c) in conditions.iter().enumerate() {
            res.push(match c {
                Self::Unknown => '?',
                Self::Ok => '.',
                Self::Broken => '#',
            });
        }
        res
    }
}

impl From<char> for Cond {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Ok,
            '#' => Self::Broken,
            '?' => Self::Unknown,
            _ => panic!("illegal condition {c:?}"),
        }
    }
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    crate::test::aoc_test!(part1, TEST_INPUT, 21);
    crate::test::aoc_test!(part2, TEST_INPUT, 525152);
}
