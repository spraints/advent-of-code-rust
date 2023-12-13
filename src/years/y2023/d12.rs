use std::fmt::Display;

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let res: usize = input.lines().map(|line| solve(line, vis)).sum();
    Box::new(res)
}

fn solve(line: &str, vis: bool) -> usize {
    let (conditions, counts) = line.trim().split_once(' ').unwrap();
    let conditions: Vec<Cond> = conditions.chars().map(Cond::from).collect();
    let counts: Vec<u16> = counts.split(',').map(|n| n.parse().unwrap()).collect();
    let mut solutions = Vec::new();
    find_solutions(
        &conditions,
        &counts,
        false,
        String::with_capacity(conditions.len()),
        &mut solutions,
    );
    if vis {
        println!("> {line}");
        for sol in &solutions {
            println!("  {sol}");
        }
    }
    solutions.len()
}

#[derive(Debug, Clone, Copy)]
enum Cond {
    Unknown,
    Ok,
    Broken,
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

fn find_solutions(
    conditions: &[Cond],
    counts: &[u16],
    in_broken_chunk: bool,
    prev: String,
    solutions: &mut Vec<String>,
) {
    fn ext(prev: &String, ch: char) -> String {
        let mut s = prev.clone();
        s.push(ch);
        s
    }
    match (conditions.is_empty(), counts.is_empty()) {
        (true, false) | (false, true) => (),
        (true, true) => solutions.push(prev),
        (false, false) => {
            let mut c = counts.clone();
            match (conditions[0], counts[0], in_broken_chunk) {
                (Cond::Unknown, 0, true) => find_solutions(&conditions[1..], &counts[1..], false, ext(&prev, '.'),solutions),
                (Cond::Unknown, n, false) => {
                    find_solutions(&conditions[1..], counts, false, ext(&prev, '.'),solutions),
                    c[0] = n - 1;
                    find_solutions(&conditions[1..], &c, true, ext(&prev, '.'),solutions),
                    find_solutions(&k
            Cond::Unknown => {
                if in_brok
            },
            Cond::Ok => todo!(),
            Cond::Broken => todo!(),
        },
    };
}

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
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
    crate::test::aoc_test!(part2, TEST_INPUT, "todo");
}
