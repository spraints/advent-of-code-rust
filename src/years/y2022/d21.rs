use std::{collections::HashMap, fmt::Display};

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let rules: HashMap<String, Rule> = input.lines().map(parse).collect();
    let mut memo = HashMap::new();
    Box::new(solve(&rules, &mut memo, "root", vis, None).unwrap())
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    let rules: HashMap<String, Rule> = input.lines().map(parse).collect();
    let mut memo = HashMap::new();
    solve(&rules, &mut memo, "root", true, Some("humn"));
    if vis {
        match rules.get("root") {
            Some(Rule::Add(a, b)) => {
                println!("{}:", a);
                println!("  {}", substitute(a, &rules));
                println!("  => {:?}", memo.get(a));
                println!("{}:", b);
                println!("  {}", substitute(b, &rules));
                println!("  => {:?}", memo.get(b));
            }
            x => unreachable!("unexpected root: {:?}", x),
        }
    }
    Box::new("todo")
}

fn substitute(x: &str, rules: &HashMap<String, Rule>) -> String {
    if x == "humn" {
        return x.to_string();
    }
    match rules.get(x).unwrap() {
        Rule::Const(val) => format!("{}", val),
        Rule::Add(a, b) => format!("({} + {})", substitute(a, rules), substitute(b, rules)),
        Rule::Sub(a, b) => format!("({} - {})", substitute(a, rules), substitute(b, rules)),
        Rule::Mul(a, b) => format!("({} * {})", substitute(a, rules), substitute(b, rules)),
        Rule::Div(a, b) => format!("({} / {})", substitute(a, rules), substitute(b, rules)),
    }
}

fn solve(
    rules: &HashMap<String, Rule>,
    memo: &mut HashMap<String, Option<i64>>,
    target: &str,
    vis: bool,
    skip: Option<&str>,
) -> Option<i64> {
    if matches!(skip, Some(x) if x == target) {
        println!(" ... skip {}", target);
        return None;
    }
    if let Some(val) = memo.get(target) {
        return *val;
    }
    if vis {
        println!("getting {}", target);
    }
    fn step<F: Fn(i64, i64) -> i64>(
        s: &str,
        op1: Option<i64>,
        op2: Option<i64>,
        f: F,
    ) -> Option<i64> {
        println!(" {} => {:?} ~ {:?}", s, op1, op2);
        match (op1, op2) {
            (Some(op1), Some(op2)) => Some(f(op1, op2)),
            _ => None,
        }
    }
    let res = match rules.get(target).unwrap() {
        Rule::Const(val) => Some(*val),
        Rule::Add(arg1, arg2) => step(
            target,
            solve(rules, memo, arg1, vis, skip),
            solve(rules, memo, arg2, vis, skip),
            |a, b| a + b,
        ),
        Rule::Sub(arg1, arg2) => step(
            target,
            solve(rules, memo, arg1, vis, skip),
            solve(rules, memo, arg2, vis, skip),
            |a, b| a - b,
        ),
        Rule::Mul(arg1, arg2) => step(
            target,
            solve(rules, memo, arg1, vis, skip),
            solve(rules, memo, arg2, vis, skip),
            |a, b| a * b,
        ),
        Rule::Div(arg1, arg2) => step(
            target,
            solve(rules, memo, arg1, vis, skip),
            solve(rules, memo, arg2, vis, skip),
            |a, b| a / b,
        ),
    };
    memo.insert(target.to_string(), res);
    res
}

fn parse(s: &str) -> (String, Rule) {
    let (name, op) = s.split_once(": ").unwrap();
    let op: Vec<&str> = op.split(' ').collect();
    let rule = match op.get(1) {
        None => Rule::Const(op[0].parse().unwrap()),
        Some(&"+") => Rule::Add(op[0].to_owned(), op[2].to_owned()),
        Some(&"-") => Rule::Sub(op[0].to_owned(), op[2].to_owned()),
        Some(&"*") => Rule::Mul(op[0].to_owned(), op[2].to_owned()),
        Some(&"/") => Rule::Div(op[0].to_owned(), op[2].to_owned()),
        _ => unreachable!("can't parse {:?}", s),
    };
    (name.to_owned(), rule)
}

#[derive(Debug)]
enum Rule {
    Const(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

#[cfg(test)]
mod test {
    use super::*;

    crate::test::aoc_test!(example, r"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32",
        part1 => 152,
        part2 => 301);
}
