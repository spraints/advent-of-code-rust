use std::fmt::Display;

pub fn part1(input: String, _vis: bool) -> Box<dyn Display> {
    Box::new(solve(input, 20, 3))
}

pub fn part2(input: String, _vis: bool) -> Box<dyn Display> {
    Box::new(solve(input, 10000, 1))
}

fn solve(input: String, iterations: usize, backoff: Worry) -> usize {
    let mut monkeys = parse_monkeys(&input);
    for _ in 0..iterations {
        go_around(&mut monkeys, backoff);
    }
    let mut inspections: Vec<usize> = monkeys.iter().map(|m| m.inspections).collect();
    inspections.sort();
    let a = inspections.pop().unwrap();
    let b = inspections.pop().unwrap();
    a * b
}

fn go_around(monkeys: &mut Vec<Monkey>, backoff: Worry) {
    let common: Worry = monkeys.iter().map(|m| m.test).product();
    for i in 0..monkeys.len() {
        let nitems = monkeys[i].items.len();
        for j in 0..nitems {
            let worry = (monkeys[i].change_worry(monkeys[i].items[j]) / backoff) % common;
            let dest = monkeys[i].do_test(worry);
            monkeys[dest].items.push(worry);
        }
        monkeys[i].items.clear();
        monkeys[i].inspections += nitems;
    }
}

type Worry = u64;

struct Monkey {
    items: Vec<Worry>,
    op: Op,
    test: Test,
    if_true: Action,
    if_false: Action,
    inspections: usize,
}

impl Monkey {
    fn change_worry(&self, item: Worry) -> Worry {
        match self.op {
            Op::Square => item * item,
            Op::Mult(n) => item * n,
            Op::Add(n) => item + n,
        }
    }

    fn do_test(&self, item: Worry) -> Action {
        if item % self.test == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

enum Op {
    Add(Worry),
    Mult(Worry),
    Square,
}

type Test = Worry; // divisor

type Action = usize;

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    fn parse_starting_items(s: &str) -> Vec<Worry> {
        s.strip_prefix("  Starting items: ")
            .unwrap()
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect()
    }
    fn parse_operation(s: &str) -> Op {
        let expr = s.strip_prefix("  Operation: new = old ").unwrap();
        match &expr[0..1] {
            "*" => match &expr[2..] {
                "old" => Op::Square,
                n => Op::Mult(n.parse().unwrap()),
            },
            "+" => Op::Add(expr[2..].parse().unwrap()),
            _ => unreachable!(),
        }
    }
    fn parse_test(s: &str) -> Test {
        s.strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse()
            .unwrap()
    }
    fn parse_cond(s: &str) -> Action {
        s.split_once(':')
            .unwrap()
            .1
            .strip_prefix(" throw to monkey ")
            .unwrap()
            .parse()
            .unwrap()
    }
    input
        .split("\n\n")
        .map(|chunk| {
            let mut lines = chunk.lines();
            lines.next().unwrap();
            let items = parse_starting_items(lines.next().unwrap());
            let op = parse_operation(lines.next().unwrap());
            let test = parse_test(lines.next().unwrap());
            let if_true = parse_cond(lines.next().unwrap());
            let if_false = parse_cond(lines.next().unwrap());
            Monkey {
                items,
                op,
                test,
                if_true,
                if_false,
                inspections: 0,
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    crate::test::aoc_test!(example, r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1",
        part1 => 10605,
        part2 => 2713310158usize);
}
