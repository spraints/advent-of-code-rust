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
    let res = match rules.get("root") {
        Some(Rule::Op { arg1, arg2, .. }) => {
            if vis {
                println!("{}:", arg1);
                println!("  {}", substitute(arg1, &rules));
                println!("  => {:?}", memo.get(arg1));
                println!("{}:", arg2);
                println!("  {}", substitute(arg2, &rules));
                println!("  => {:?}", memo.get(arg2));
            }
            match (memo.get(arg1), memo.get(arg2)) {
                (Some(Some(answer)), Some(None)) => what_is_humn(*answer, &rules, &memo, arg2, vis),
                (Some(None), Some(Some(answer))) => what_is_humn(*answer, &rules, &memo, arg1, vis),
                (memo1, memo2) => unreachable!("womp womp a={:?} b={:?}", memo1, memo2),
            }
        }
        x => unreachable!("unexpected root: {:?}", x),
    };
    Box::new(res)
}

fn what_is_humn(
    answer: i64,
    rules: &HashMap<String, Rule>,
    memo: &HashMap<String, Option<i64>>,
    cur: &str,
    vis: bool,
) -> i64 {
    match rules.get(cur) {
        Some(Rule::Op { arg1, arg2, op }) => {
            let m1 = memo.get(arg1);
            let m2 = memo.get(arg2);
            if vis {
                println!(
                    "solving for {} = {}({:?}) {} {}({:?})",
                    answer, arg1, m1, op, arg2, m2
                );
            }
            match (m1, m2) {
                // Still more to do!
                (Some(Some(arg1)), Some(None)) => {
                    what_is_humn(op.unapply1(answer, arg1), rules, memo, arg2, vis)
                }
                (Some(None), Some(Some(arg2))) => {
                    what_is_humn(op.unapply2(answer, arg2), rules, memo, arg1, vis)
                }
                // This is it!
                (Some(Some(arg1)), None) if arg2 == "humn" => op.unapply1(answer, arg1),
                (None, Some(Some(arg2))) if arg1 == "humn" => op.unapply2(answer, arg2),
                // Nope! These shouldn't happen!
                x => unreachable!("unexpected: {:?}", x),
            }
        }
        Some(x) => unreachable!("should not have a {:?} here", x),
        None => unreachable!("illegal reference to {}", cur),
    }
}

fn substitute(x: &str, rules: &HashMap<String, Rule>) -> String {
    if x == "humn" {
        return x.to_string();
    }
    match rules.get(x).unwrap() {
        Rule::Const(val) => format!("{}", val),
        Rule::Op { op, arg1, arg2 } => format!(
            "({} {} {})",
            substitute(arg1, rules),
            op,
            substitute(arg2, rules),
        ),
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
        return None;
    }
    if let Some(val) = memo.get(target) {
        return *val;
    }
    if vis {
        println!("getting {}", target);
    }
    fn step(arg1: Option<i64>, arg2: Option<i64>, op: Op) -> Option<i64> {
        match (arg1, arg2) {
            (Some(arg1), Some(arg2)) => Some(op.apply(arg1, arg2)),
            _ => None,
        }
    }
    let res = match rules.get(target).unwrap() {
        Rule::Const(val) => Some(*val),
        Rule::Op { op, arg1, arg2 } => step(
            solve(rules, memo, arg1, vis, skip),
            solve(rules, memo, arg2, vis, skip),
            *op,
        ),
    };
    memo.insert(target.to_string(), res);
    res
}

fn parse(s: &str) -> (String, Rule) {
    let (name, expr) = s.split_once(": ").unwrap();
    let parts: Vec<&str> = expr.split(' ').collect();
    let rule = match parts.get(1) {
        None => Rule::Const(parts[0].parse().unwrap()),
        Some(op) => Rule::Op {
            op: parse_op(op),
            arg1: parts[0].to_string(),
            arg2: parts[2].to_string(),
        },
        _ => unreachable!("can't parse {:?}", s),
    };
    (name.to_owned(), rule)
}

fn parse_op(s: &str) -> Op {
    match s {
        "+" => Op::Add,
        "-" => Op::Sub,
        "*" => Op::Mul,
        "/" => Op::Div,
        s => unreachable!("illegal op {:?}", s),
    }
}

#[derive(Debug)]
enum Rule {
    Const(i64),
    Op { op: Op, arg1: String, arg2: String },
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Self::Add => a + b,
            Self::Sub => a - b,
            Self::Mul => a * b,
            Self::Div => a / b,
        }
    }

    fn unapply1(&self, answer: i64, arg1: &i64) -> i64 {
        match self {
            // arg1 + ? = answer
            Self::Add => answer - arg1,
            // arg1 * ? = answer
            Self::Mul => answer / arg1,
            // arg1 - ? = answer
            Self::Sub => arg1 - answer,
            _ => todo!("{} {} {} = {}", arg1, self, "?", answer),
        }
    }

    fn unapply2(&self, answer: i64, arg2: &i64) -> i64 {
        match self {
            // ? + arg2 = answer
            Self::Add => answer - arg2,
            Self::Sub => answer + arg2,
            Self::Mul => answer / arg2,
            Self::Div => answer * arg2,
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
        }
    }
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
