use std::borrow::Cow;
use std::collections::HashMap;
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
    if vis {
        println!("{line}");
    }

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
    if vis && mult > 1 {
        println!("++ {conditions}");
        println!("## {counts}");
    }

    let conditions: Vec<Cond> = conditions.chars().map(Cond::from).collect();
    let counts: Vec<u16> = counts.split(',').map(|n| n.parse().unwrap()).collect();

    let mut memo = HashMap::new();

    let res = s2(&conditions, &counts, false, &mut memo);
    if vis {
        println!("  >> {res}");
    }
    res
}

fn s2(
    conditions: &[Cond],
    counts: &[u16],
    in_broken: bool,
    memo: &mut HashMap<(Vec<Cond>, Vec<u16>, bool), usize>,
) -> usize {
    let res = match (conditions.is_empty(), counts.is_empty()) {
        (true, true) => 1,
        (true, false) => {
            if counts.iter().all(|x| *x == 0) {
                1
            } else {
                0
            }
        }
        (false, true) => {
            if conditions.iter().all(|x| !matches!(x, Cond::Broken)) {
                1
            } else {
                0
            }
        }
        (false, false) => {
            let k = (conditions.to_vec(), counts.to_vec(), in_broken);
            if let Some(v) = memo.get(&k) {
                return *v;
            }
            let res = match (in_broken, conditions[0], counts[0]) {
                // It's not possible to be at count==0 without being in a broken region.
                (false, _, 0) => panic!("illegal!!"),
                // If this block is OK and the previous one was too, just keep going.
                (false, Cond::Ok, _) => s2(&conditions[1..], counts, false, memo),
                // If this block is OK and the previous one wasn't, then the current count should be 0.
                // Keep going.
                (true, Cond::Ok, 0) => s2(&conditions[1..], &counts[1..], false, memo),
                // If this block is OK and the previous one wasn't and we're still expecting more
                // broken ones, stop this branch of the search.
                (true, Cond::Ok, 1_u16..=u16::MAX) => 0,
                // If this block is broken and the current count is 0, we're over budget, stop this
                // branch of the search.
                (true, Cond::Broken, 0) => 0,
                // If this block is broken, reduce the current count and look for more.
                (_, Cond::Broken, n @ 1_u16..=u16::MAX) => {
                    s2(&conditions[1..], &c(n - 1, &counts[1..]), true, memo)
                }
                // If this block is unknown and the previous one was broken and the count is 0, this
                // one must be OK.
                (true, Cond::Unknown, 0) => s2(&conditions[1..], &counts[1..], false, memo),
                // If this block is unknown and the previous one was broken and there are still more
                // broken ones needed, this one must be broken.
                (true, Cond::Unknown, n) => {
                    s2(&conditions[1..], &c(n - 1, &counts[1..]), true, memo)
                }
                // If this block is unknown and the previous one wasn't, this can either be broken or
                // not.
                (false, Cond::Unknown, n) => {
                    // Pretend that it is Ok.
                    s2(&conditions[1..], &counts, false, memo)
                    // Pretend that it is not Ok.
                    + s2(&conditions[1..], &c(n - 1, &counts[1..]), true, memo)
                }
            };
            memo.insert(k, res);
            res
        }
    };
    //println!("maybe? {} {counts:?} => {res}", Cond::str(conditions));
    res
}

fn c(val: u16, vals: &[u16]) -> Vec<u16> {
    let mut res = Vec::with_capacity(vals.len() + 1);
    res.push(val);
    res.extend_from_slice(vals);
    res
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Cond {
    Unknown,
    Ok,
    Broken,
}

impl Cond {
    fn str(conditions: &[Self]) -> String {
        let mut res = String::with_capacity(conditions.len());
        for c in conditions {
            res.push(match c {
                Self::Unknown => '?',
                Self::Ok => '.',
                Self::Broken => '#',
            })
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

    crate::test::aoc_test!(part2, short, r".# 1", 1);
}
