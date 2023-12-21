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
    let mut circuit = parse(&input);
    for i in 1..10 {
        if vis {
            println!("---- CYCLE ----");
        }
        let (_, _, rx) = cycle(&mut circuit, true);
        if vis {
            println!("{i}: {rx:?}");
            for m in circuit.modules.values() {
                match m.mod_type {
                    ModuleType::Broadcaster => (),
                    ModuleType::FlipFlop => {
                        println!("{}: {:?}", m.name, circuit.get_flip_flop_state(m))
                    }
                    ModuleType::Conjunction => {
                        print!("{}:", m.name);
                        for input_name in &m.inputs {
                            let input = circuit
                                .input_states
                                .get(&(m.name.clone(), input_name.to_owned()))
                                .unwrap_or(&false);
                            print!(" {input_name}:{input}");
                        }
                        println!();
                    }
                };
            }
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

        if let Some(m) = circuit.modules.get(&dest) {
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
            };
        }
    }

    (low_pulses, high_pulses, (rx_low, rx_high))
}

struct Circuit {
    modules: HashMap<String, Module>,
    flip_flop_states: HashMap<String, bool>,
    input_states: HashMap<(String, String), bool>,
}

impl Circuit {
    fn update(&mut self, src: &str, dest: &str, pulse: bool) {
        //if src == "button" && dest == "broadcaster" {
        //    return;
        //}
        if let Some(m) = self.modules.get(dest) {
            let mod_type = m.mod_type;
            match mod_type {
                ModuleType::Broadcaster => (),
                ModuleType::FlipFlop => {
                    if !pulse {
                        let e = self
                            .flip_flop_states
                            .entry(dest.to_owned())
                            .or_insert(false);
                        *e = !*e;
                    }
                }
                ModuleType::Conjunction => {
                    self.input_states
                        .insert((src.to_owned(), dest.to_owned()), pulse);
                }
            };
        }
    }

    fn get_flip_flop_state(&self, m: &Module) -> bool {
        match self.flip_flop_states.get(&m.name) {
            None => false,
            Some(b) => *b,
        }
    }

    fn resolve_conjunction(&self, m: &Module) -> bool {
        let all_high = m.inputs.iter().all(|input_name| {
            *self
                .input_states
                .get(&(input_name.to_owned(), m.name.clone()))
                .unwrap_or(&false)
        });
        !all_high
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

    let mut inputs = HashMap::new();
    for m in modules.iter() {
        for dest in &m.dests {
            let e = inputs.entry(dest.clone()).or_insert_with(|| Vec::new());
            e.push(m.name.clone());
        }
    }

    for m in modules.iter_mut() {
        if let Some(v) = inputs.remove(&m.name) {
            m.inputs = v;
        }
    }

    let modules = modules.into_iter().map(|m| (m.name.clone(), m)).collect();
    let flip_flop_states = HashMap::new();
    let input_states = HashMap::new();

    Circuit {
        modules,
        flip_flop_states,
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
