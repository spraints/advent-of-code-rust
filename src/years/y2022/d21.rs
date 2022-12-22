use std::{collections::HashMap, fmt::Display};

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let rules: HashMap<String, Rule> = input.lines().map(parse).collect();
    Box::new(solve(&rules, "root", vis))
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    let rules: HashMap<String, Rule> = input.lines().map(parse).collect();
    if vis {
        match rules.get("root") {
            Some(Rule::Add(a, b)) => {
                println!("{}:", a);
                println!("  {}", substitute(a, &rules));
                println!("{}:", b);
                println!("  {}", substitute(b, &rules));
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

fn solve(rules: &HashMap<String, Rule>, target: &str, vis: bool) -> i64 {
    fn resolve(
        rules: &HashMap<String, Rule>,
        memo: &mut HashMap<String, i64>,
        target: &str,
        vis: bool,
    ) -> i64 {
        if let Some(val) = memo.get(target) {
            return *val;
        }
        if vis {
            println!("getting {}", target);
        }
        match rules.get(target).unwrap() {
            Rule::Const(val) => *val,
            Rule::Add(arg1, arg2) => {
                resolve(rules, memo, arg1, vis) + resolve(rules, memo, arg2, vis)
            }
            Rule::Sub(arg1, arg2) => {
                resolve(rules, memo, arg1, vis) - resolve(rules, memo, arg2, vis)
            }
            Rule::Mul(arg1, arg2) => {
                resolve(rules, memo, arg1, vis) * resolve(rules, memo, arg2, vis)
            }
            Rule::Div(arg1, arg2) => {
                resolve(rules, memo, arg1, vis) / resolve(rules, memo, arg2, vis)
            }
        }
    }

    let mut memo = HashMap::new();
    resolve(rules, &mut memo, target, vis)
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
        part2 => "todo");
}
