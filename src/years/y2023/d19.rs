use std::collections::HashMap;
use std::fmt::Display;

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let (workflows, parts) = parse(&input);

    let mut sum = 0;
    for p in parts {
        if vis {
            print!("{p}: ");
        }
        let mut wf = workflows.get("in").unwrap();
        loop {
            if vis {
                print!("{} -> ", wf.name);
            }
            match wf.apply(&p) {
                Destination::Reject => {
                    if vis {
                        println!("R");
                    }
                    break;
                }
                Destination::Accept => {
                    let psum = p.x + p.m + p.a + p.s;
                    if vis {
                        println!("A ({psum})");
                    }
                    sum += psum;
                    break;
                }
                Destination::Workflow(name) => {
                    wf = workflows.get(name).unwrap();
                }
            };
        }
    }

    Box::new(sum)
}

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

fn parse(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let workflow_re = regex::Regex::new(r"(.+)\{(.*)\}").unwrap();
    let rule_re = regex::Regex::new(r"(.*)(<|>)(\d+):(.*)").unwrap();

    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let workflows = workflows
        .lines()
        .map(|l| {
            let c = workflow_re.captures(l).unwrap();
            let (_, [name, rules]) = c.extract();
            let rules = rules
                .split(',')
                .map(|rule| match rule_re.captures(rule) {
                    None => Rule {
                        cond: Condition::Always,
                        dest: parse_dest(rule),
                    },
                    Some(c) => {
                        let (_, [field, op, limit, dest]) = c.extract();
                        let cond = if op == "<" {
                            Condition::Lt
                        } else {
                            Condition::Gt
                        };
                        let field = match field {
                            "x" => Field::X,
                            "m" => Field::M,
                            "a" => Field::A,
                            "s" => Field::S,
                            _ => panic!("illegal field {field:?}"),
                        };
                        Rule {
                            cond: cond(field, limit.parse().unwrap()),
                            dest: parse_dest(dest),
                        }
                    }
                })
                .collect();
            (
                name.to_owned(),
                Workflow {
                    name: name.to_owned(),
                    rules,
                },
            )
        })
        .collect();

    let parts = parts
        .lines()
        .map(|l| {
            let l = l.trim().trim_end_matches('}').trim_start_matches('{');
            let mut part = Part {
                x: 0,
                m: 0,
                a: 0,
                s: 0,
            };
            for p in l.split(',') {
                let (t, v) = p.split_once('=').unwrap();
                let v = v.parse().unwrap();
                match t {
                    "x" => part.x = v,
                    "m" => part.m = v,
                    "a" => part.a = v,
                    "s" => part.s = v,
                    _ => panic!("illegal field name in {l:?}"),
                };
            }
            part
        })
        .collect();

    (workflows, parts)
}

fn parse_dest(dest: &str) -> Destination {
    match dest {
        "A" => Destination::Accept,
        "R" => Destination::Reject,
        s => Destination::Workflow(s.to_owned()),
    }
}

type Num = u32;

struct Part {
    x: Num,
    m: Num,
    a: Num,
    s: Num,
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}x={},m={},a={},s={}{}",
            '{', self.x, self.m, self.a, self.s, '}'
        )
    }
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn apply(&self, part: &Part) -> &Destination {
        for rule in &self.rules {
            match &rule.cond {
                Condition::Gt(f, n) => {
                    if f.get(part) > *n {
                        return &rule.dest;
                    }
                }
                Condition::Lt(f, n) => {
                    if f.get(part) < *n {
                        return &rule.dest;
                    }
                }
                Condition::Always => return &rule.dest,
            };
        }
        unreachable!()
    }
}

struct Rule {
    cond: Condition,
    dest: Destination,
}

enum Condition {
    Gt(Field, Num),
    Lt(Field, Num),
    Always,
}

enum Field {
    X,
    M,
    A,
    S,
}

impl Field {
    fn get(&self, part: &Part) -> Num {
        match self {
            Self::X => part.x,
            Self::M => part.m,
            Self::A => part.a,
            Self::S => part.s,
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::X => "x",
                Self::M => "m",
                Self::A => "a",
                Self::S => "s",
            }
        )
    }
}

enum Destination {
    Workflow(String),
    Reject,
    Accept,
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    crate::test::aoc_test!(part1, TEST_INPUT, 19114);
    crate::test::aoc_test!(part2, TEST_INPUT, "todo");
}
