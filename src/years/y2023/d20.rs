use std::collections::{HashMap, VecDeque};
use std::fmt::Display;

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let mut circuit = parse(&input);
    let mut low = 0;
    let mut high = 0;
    for i in 0..1000 {
        if vis && i < 4 {
            println!("---- CYCLE ----");
        }
        let (l, h, _) = cycle(&mut circuit, vis && i < 4);
        low += l;
        high += h;
    }
    if vis {
        println!("low pulses: {low}");
        println!("high pulses: {high}");
    }
    Box::new(low * high)
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    fn vb(i: &[bool]) -> String {
        let mut res = String::with_capacity(i.len());
        for x in i {
            res.push(if *x { '1' } else { '0' });
        }
        res
    }
    let mut circuit = parse(&input);
    for i in 1..10 {
        let (_, _, rx) = cycle(&mut circuit, false);
        if vis {
            println!("{i}: {rx:?}");
            println!("  ff:  {}", vb(&circuit.flip_flop_states));
            println!("  inp: {}", vb(&circuit.input_states));
        }
    }
    Box::new("todo")
}

type RxCount = (usize, usize);

fn cycle(circuit: &mut Circuit, vis: bool) -> (usize, usize, RxCount) {
    let mut pending: VecDeque<(String, String, bool)> =
        vec![("button".to_owned(), "broadcaster".to_owned(), false)].into();
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    let mut rx_low = 0;
    let mut rx_high = 0;

    while let Some((src, dest, pulse)) = pending.pop_front() {
        if vis {
            println!("{src} -{}-> {dest}", if pulse { "high" } else { "low" });
        }

        if pulse {
            high_pulses += 1;
        } else {
            low_pulses += 1;
        }
        if dest == "rx" {
            if pulse {
                rx_low += 1;
            } else {
                rx_high += 1;
            }
        }

        circuit.update(&src, &dest, pulse);

        let m = circuit.get_mod(&dest);
        match m.mod_type {
            ModuleType::Broadcaster => {
                for new_dest in &m.dests {
                    pending.push_back((dest.clone(), new_dest.to_string(), pulse));
                }
            }
            ModuleType::FlipFlop => {
                if !pulse {
                    let new_pulse = circuit.get_flip_flop_state(m);
                    for new_dest in &m.dests {
                        pending.push_back((dest.clone(), new_dest.to_string(), new_pulse));
                    }
                }
            }
            ModuleType::Conjunction => {
                let new_pulse = circuit.resolve_conjunction(m);
                for new_dest in &m.dests {
                    pending.push_back((dest.clone(), new_dest.to_string(), new_pulse));
                }
            }
        }
    }

    (low_pulses, high_pulses, (rx_low, rx_high))
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

impl Circuit {
    fn update(&mut self, src: &str, dest: &str, pulse: bool) {
        if src == "button" && dest == "broadcaster" {
            return;
        }
        let src = *self
            .module_index
            .get(src)
            .unwrap_or_else(|| panic!("expected to find module {src:?}"));
        let dest = *self.module_index.get(dest).unwrap();
        let mod_type = self.modules[dest].mod_type;
        match mod_type {
            ModuleType::Broadcaster => (),
            ModuleType::FlipFlop => {
                if !pulse {
                    self.flip_flop_states[dest] = !self.flip_flop_states[dest];
                }
            }
            ModuleType::Conjunction => {
                let i = self.input_index.get(&(src, dest)).unwrap();
                self.input_states[*i] = pulse;
            }
        };
    }

    fn get_mod(&self, name: &str) -> &Module {
        let i = self.module_index.get(name).unwrap();
        &self.modules[*i]
    }

    fn get_flip_flop_state(&self, m: &Module) -> bool {
        let i = self.module_index.get(&m.name).unwrap();
        self.flip_flop_states[*i]
    }

    fn resolve_conjunction(&self, m: &Module) -> bool {
        let i = self.module_index.get(&m.name).unwrap();
        // if it remembers high pulses for all inputs, it sends a low pulse;
        // otherwise, it sends a high pulse.
        !m.inputs.iter().all(|input| {
            let j = self.module_index.get(input).unwrap();
            let k = self.input_index.get(&(*j, *i)).unwrap();
            self.input_states[*k]
        })
    }
}

#[derive(Debug)]
struct Module {
    name: String,
    mod_type: ModuleType,
    dests: Vec<String>,
    inputs: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
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
