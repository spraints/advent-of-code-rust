use std::{collections::HashMap, fmt::Display};

pub fn part1(input: String, _vis: bool) -> Box<dyn Display> {
    let rules: Vec<(String, Rule)> = input.lines().map(parse).collect();
    Box::new(solve(&rules, "root"))
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

fn solve(rules: &Vec<(String, Rule)>, target: &str) -> i32 {
    let mut values: HashMap<String, i32> = HashMap::new();
    while !values.contains_key(target) {
        for (name, rule) in rules {
            if !values.contains_key(name) {
                match rule {
                    Rule::Const(val) => {
                        values.insert(name.clone(), *val);
                        ()
                    }
                    Rule::Add(a, b) => try_eval(&mut values, name, a, b, |a, b| a + b),
                    Rule::Sub(a, b) => try_eval(&mut values, name, a, b, |a, b| a - b),
                    Rule::Mul(a, b) => try_eval(&mut values, name, a, b, |a, b| a * b),
                    Rule::Div(a, b) => try_eval(&mut values, name, a, b, |a, b| a / b),
                };
            }
        }
    }
    values[target]
}

fn try_eval<F>(
    values: &mut HashMap<String, i32>,
    target: &str,
    operand1: &str,
    operand2: &str,
    op: F,
) where
    F: Fn(i32, i32) -> i32,
{
    if let (Some(a), Some(b)) = (values.get(operand1), values.get(operand2)) {
        let res = op(*a, *b);
        values.insert(target.to_string(), res);
    }
}

fn parse(s: &str) -> (String, Rule) {
    let (name, op) = s.split_once(": ").unwrap();
    let op: Vec<&str> = op.split(' ').collect();
    let rule = match op.get(1) {
        None => Rule::Const(op[0].parse().unwrap()),
        Some(&"+") => Rule::Add(op[0].to_owned(), op[1].to_owned()),
        Some(&"-") => Rule::Sub(op[0].to_owned(), op[1].to_owned()),
        Some(&"*") => Rule::Mul(op[0].to_owned(), op[1].to_owned()),
        Some(&"/") => Rule::Div(op[0].to_owned(), op[1].to_owned()),
        _ => unreachable!("can't parse {:?}", s),
    };
    (name.to_owned(), rule)
}

enum Rule {
    Const(i32),
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
        part2 => "todo");
}
