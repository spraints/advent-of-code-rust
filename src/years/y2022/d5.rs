use std::fmt::Display;

type Stack = Vec<char>;
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let (mut stacks, moves) = parse(&input);
    if vis {
        show_stacks(&stacks);
    }
    for m in moves {
        if vis {
            println!("{}", m);
        }
        for _ in 0..m.count {
            let moved = stacks[m.from].pop().unwrap();
            stacks[m.to].push(moved);
        }
        if vis {
            show_stacks(&stacks);
        }
    }
    Box::new(
        stacks
            .into_iter()
            .map(|mut s| s.pop().unwrap())
            .collect::<String>(),
    )
}

pub fn part1_transpose(input: String, vis: bool) -> Box<dyn Display> {
    let (mut stacks, moves) = parse_transpose(&input);
    if vis {
        show_stacks(&stacks);
    }
    for m in moves {
        if vis {
            println!("{}", m);
        }
        for _ in 0..m.count {
            let moved = stacks[m.from].pop().unwrap();
            stacks[m.to].push(moved);
        }
        if vis {
            show_stacks(&stacks);
        }
    }
    Box::new(
        stacks
            .into_iter()
            .map(|mut s| s.pop().unwrap())
            .collect::<String>(),
    )
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    let (mut stacks, moves) = parse(&input);
    if vis {
        show_stacks(&stacks);
    }
    for m in moves {
        if vis {
            println!("{}", m);
        }
        let i = stacks[m.from].len() - m.count;
        let mut moved = stacks[m.from].split_off(i);
        stacks[m.to].append(&mut moved);
        if vis {
            show_stacks(&stacks);
        }
    }
    Box::new(
        stacks
            .into_iter()
            .map(|mut s| s.pop().unwrap())
            .collect::<String>(),
    )
}

pub fn part2_slices(input: String, vis: bool) -> Box<dyn Display> {
    let (mut stacks, moves) = parse(&input);
    if vis {
        show_stacks(&stacks);
    }
    for m in moves {
        if vis {
            println!("{}", m);
        }
        let new_len = stacks[m.from].len() - m.count;
        for i in 0..m.count {
            let moved = stacks[m.from][new_len + i];
            stacks[m.to].push(moved);
        }
        stacks[m.from].resize(new_len, Default::default());
        if vis {
            show_stacks(&stacks);
        }
    }
    Box::new(
        stacks
            .into_iter()
            .map(|mut s| s.pop().unwrap())
            .collect::<String>(),
    )
}

fn show_stacks(stacks: &Vec<Stack>) {
    let mut i = stacks.iter().map(|s| s.len()).max().unwrap();
    while i > 0 {
        i -= 1;
        for stack in stacks {
            match stack.get(i) {
                None => print!("    "),
                Some(c) => print!("[{}] ", c),
            };
        }
        println!();
    }
    for i in 0..stacks.len() {
        print!(" {}  ", i + 1);
    }
    println!();
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "move {} from {} to {}",
            self.count,
            self.from + 1,
            self.to + 1
        )
    }
}

fn parse(s: &str) -> (Vec<Stack>, Vec<Move>) {
    fn update_stacks(stacks: &mut Vec<Stack>, line: &str) {
        let mut line = line.chars();
        let mut i = 0;
        loop {
            match line.next() {
                Some('[') => {
                    let c = line.next().unwrap();
                    if i >= stacks.len() {
                        stacks.resize(i + 1, Default::default());
                    }
                    stacks[i].insert(0, c);
                    for _ in 0..2 {
                        if line.next().is_none() {
                            return;
                        }
                    }
                }
                Some(' ') => {
                    for _ in 0..3 {
                        if line.next().is_none() {
                            return;
                        }
                    }
                }
                _ => return,
            };

            i += 1;
        }
    }
    let mut stacks = Vec::new();
    let mut moves = Vec::new();
    let mut state = 0;
    for line in s.lines() {
        match (state, line) {
            (0, l) if l.starts_with(" 1 ") => state = 1,
            (0, l) => update_stacks(&mut stacks, l),
            (1, _) => state = 2,
            (2, l) => moves.push(parse_move(l)),
            _ => unreachable!(),
        };
    }
    (stacks, moves)
}

fn parse_transpose(s: &str) -> (Vec<Stack>, Vec<Move>) {
    fn parse_crates(line: &str) -> Vec<u8> {
        let line = line.as_bytes();
        let count = line.len() / 4;
        (0..=count).map(|i| line[i * 4 + 1]).collect()
    }
    fn transpose(raw_stacks: Vec<Vec<u8>>) -> Vec<Stack> {
        let mut raw_stacks: Vec<_> = raw_stacks
            .into_iter()
            .rev()
            .map(|v| v.into_iter())
            .collect();
        let mut res = Vec::new();
        loop {
            let stack: Stack = raw_stacks
                .iter_mut()
                .filter_map(|level| match level.next() {
                    None | Some(b' ') => None,
                    Some(b) => Some(unsafe { char::from_u32_unchecked(b as u32) }),
                })
                .collect();
            if stack.is_empty() {
                break;
            }
            res.push(stack);
        }
        res
    }
    let mut stacks = Vec::new();
    let mut moves = Vec::new();
    let mut state = 0;
    for line in s.lines() {
        match (state, line) {
            (0, l) if l.starts_with(" 1 ") => state = 1,
            (0, l) => stacks.push(parse_crates(l)),
            (1, _) => state = 2,
            (2, l) => moves.push(parse_move(l)),
            _ => unreachable!(),
        };
    }
    (transpose(stacks), moves)
}

fn parse_move(line: &str) -> Move {
    let mut words = line.split(' ');
    words.next().unwrap();
    let count = words.next().unwrap().parse().unwrap();
    words.next().unwrap();
    let from = words.next().unwrap().parse::<usize>().unwrap() - 1;
    words.next().unwrap();
    let to = words.next().unwrap().parse::<usize>().unwrap() - 1;
    Move { count, from, to }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::*;

    const EX: &str = r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_example() {
        dotest("CMZ", EX, part1);
        dotest("CMZ", EX, part1_transpose);
    }

    #[test]
    fn part2_example() {
        dotest("MCD", EX, part2);
        dotest("MCD", EX, part2_slices);
    }
}
