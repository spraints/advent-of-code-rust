use std::collections::HashMap;
use std::fmt::Display;

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let circuit = parse(&input);
    if vis {
        println!("parsed {} modules", circuit.modules.len());
        let mut lines = input.lines();
        for m in &circuit.modules {
            if let Some(line) = lines.next() {
                println!("{line}");
            }
            println!("{m:?}");
        }
    }
    Box::new("todo")
}

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

struct Circuit {
    // name to index in modules.
    module_index: HashMap<String, usize>,
    modules: Vec<Module>,

    flip_flop_states: Vec<bool>,

    // (usize,usize) to index in inputs.
    input_index: HashMap<(usize, usize), usize>,
    input_states: Vec<bool>,
}

#[derive(Debug)]
struct Module {
    name: String,
    mod_type: ModuleType,
    dests: Vec<String>,
    inputs: Vec<String>,
}

#[derive(Debug)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

fn parse(input: &str) -> Circuit {
    let mut modules: Vec<Module> = input.lines().map(parse_module).collect();

    let mut module_index = HashMap::new();
    let mut inputs = HashMap::new();
    for (i, m) in modules.iter().enumerate() {
        module_index.insert(m.name.clone(), i);
        for dest in &m.dests {
            let e = inputs.entry(dest.clone()).or_insert_with(|| Vec::new());
            e.push(m.name.clone());
        }
    }

    let mut input_index: HashMap<(usize, usize), usize> = HashMap::new();
    for (k, v) in inputs {
        let to = {
            let e = module_index.entry(k.to_owned()).or_insert_with(|| {
                modules.push(Module {
                    name: k.to_owned(),
                    mod_type: ModuleType::Broadcaster,
                    dests: Vec::new(),
                    inputs: Vec::new(),
                });
                modules.len() - 1
            });
            *e
        };
        let m = modules.get_mut(to).unwrap();
        m.inputs = v.clone();
        for from_name in v {
            let from = module_index.get(&from_name).unwrap();
            let i = input_index.len();
            input_index.insert((*from, to), i);
        }
    }

    let flip_flop_states = vec![false; modules.len()];
    let input_states = vec![false; input_index.len()];

    Circuit {
        module_index,
        modules,
        flip_flop_states,
        input_index,
        input_states,
    }
}

fn parse_module(line: &str) -> Module {
    let (mt, dests) = line.trim().split_once(" -> ").unwrap();
    let (mod_type, name) = match mt {
        "broadcaster" => (ModuleType::Broadcaster, mt.to_owned()),
        _ => {
            let mut c = mt.chars();
            let t = c.next().unwrap();
            let name = c.collect();
            let mt = match t {
                '%' => ModuleType::FlipFlop,
                '&' => ModuleType::Conjunction,
                _ => unreachable!("illegal module type {t:?}"),
            };
            (mt, name)
        }
    };
    let dests = dests.split(',').map(|s| s.trim().to_owned()).collect();
    Module {
        name,
        mod_type,
        dests,
        inputs: Vec::new(),
    }
}

#[cfg(test)]
mod test {
    crate::test::aoc_test!(
        part1,
        simple1,
        r"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a",
        32000000
    );

    crate::test::aoc_test!(
        part1,
        complex,
        r"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output",
        11687500
    );
}
