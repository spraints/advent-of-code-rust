// TODO
// - framework for adding days, parts.
// - framework for testing with example inputs.
//
// - 'cargo test' - runs all unit tests
// - 'cargo run' - run the current day
// - 'cargo run --day 1 --part 1' - run just day 1, part 1, from this year.
// - 'cargo run --year Y [--day D] [--part 1|2]' - run all or part of a year.
//    needed.
// - 'cargo run --all' - run everything.
// - 'cargo run --set-token TOKEN' - stash my auth token.
//
// https://github.com/gobanos/aoc-runner-derive/blob/master/src/lib.rs if attr is useful.

mod curday;
mod input;
mod registry;
mod solutionset;
mod solver;
mod token;
mod years;

#[cfg(test)]
mod test;

use std::{collections::BTreeSet, fmt::Display};

use chrono::Datelike;
use clap::Parser;
use curday::aoc_now;
use input::get_input;
use solutionset::SolutionSet;
use solver::Solver;
use token::{get_token, set_token};

#[macro_use]
extern crate advent_of_code_registry;

fn main() {
    let cli = Cli::parse();
    if let Err(e) = match cli.set_token {
        Some(token) => set_token(token),
        None => do_run(cli),
    } {
        eprintln!("error: {e}");
    }
}

fn do_run(cli: Cli) -> anyhow::Result<()> {
    let mut runner = Runner::new(cli.visualize);
    /*
     * TODO:
     * register here like:
     *   register![
     *     y2021::day1part1,
     *     y2021::day1part2,
     *     y2021::day2part1,
     *     y2021::day2part2,
     *     ...
     *   ]
     * even better would be to not need to list everything, let register! scan all the modules.
     *
     * annotate days like:
     *   #[aoc(year = 2021, day = 1, part = 1, label = "optional extra string")]
     *   pub fn day1part1(...
     */
    years::register(&mut runner);
    runner.run(cli)
}

struct Runner {
    visualize: bool,
    solvers: BTreeSet<Solver>,
}

impl Runner {
    fn new(visualize: bool) -> Self {
        Self {
            visualize,
            solvers: BTreeSet::new(),
        }
    }

    fn run(self, mut cli: Cli) -> anyhow::Result<()> {
        let token = get_token()?;
        let now = aoc_now();
        cli.set_today(&now);

        fn res(cli: &Cli, res: Box<dyn Display>) -> Box<dyn Display> {
            if cli.no_spoilers {
                Box::new("(result hidden)")
            } else {
                res
            }
        }

        let mut total_time = std::time::Duration::ZERO;
        let mut count = 0;
        for solver in self.solvers {
            let Solver {
                year,
                day,
                part,
                label,
                f,
                slow,
            } = solver;
            if cli.matches(year, day, part) {
                if is_future(&now, year, day) {
                    println!("{year}: Dec {day:02}: part {part}: (future)");
                    continue;
                }
                if slow && !cli.run_slow_parts() {
                    println!("{year}: Dec {day:02}: part {part}: (skipped because it's slow)");
                    continue;
                }
                count += 1;
                let input = get_input(year, day, &token)?;
                let now = std::time::Instant::now();
                let result = f(input, self.visualize);
                let elapsed = now.elapsed();
                total_time += elapsed;
                println!(
                    "{}: Dec {:02}: part {}: {} ({:.2?}){}",
                    year,
                    day,
                    part,
                    res(&cli, result),
                    elapsed,
                    match label {
                        Some(s) => format!(" ({})", s),
                        None => "".to_string(),
                    },
                );
            }
        }
        if count == 0 {
            println!("No matches found! {:?}", cli);
        } else {
            let avg = total_time / count;
            println!("total time: {total_time:.2?} / avg: {avg:.2?}");
        }
        Ok(())
    }
}

fn is_future<D: Datelike>(now: &D, year: i32, day: u32) -> bool {
    year > now.year() || (year == now.year() && day > now.day())
}

impl SolutionSet for Runner {
    fn add<F>(&mut self, year: i32, day: u32, part: u8, label: Option<&'static str>, f: F)
    where
        F: Fn(String, bool) -> Box<dyn std::fmt::Display> + 'static,
    {
        self.solvers.insert(Solver {
            year,
            day,
            part,
            label,
            f: Box::new(f),
            slow: false,
        });
    }

    fn add_slow<F>(&mut self, year: i32, day: u32, part: u8, label: Option<&'static str>, f: F)
    where
        F: Fn(String, bool) -> Box<dyn std::fmt::Display> + 'static,
    {
        self.solvers.insert(Solver {
            year,
            day,
            part,
            label,
            f: Box::new(f),
            slow: true,
        });
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Set your token instead of solving any puzzles. This is the value of the session cookie on adventofcode.com.
    #[arg(long)]
    set_token: Option<String>,

    /// Run all solvers.
    #[arg(long)]
    all: bool,

    /// Run this year's solvers.
    #[arg(long)]
    this_year: bool,

    /// Hide solutions.
    #[arg(long)]
    no_spoilers: bool,

    /// Run all of the given year's solvers, unless --day is set.
    #[arg(short, long)]
    year: Option<i32>,

    /// Run the given day's solvers.
    #[arg(short, long)]
    day: Option<u32>,

    /// Run either part 1 or 2.
    #[arg(short, long)]
    part: Option<u8>,

    /// Include visualizations.
    #[arg(short, long)]
    visualize: bool,

    /// Run parts that are considered 'slow'.
    #[arg(long)]
    include_slow: bool,

    /// YYYY or YYYY/DD or DD to run.
    spec: Option<String>,
}

impl Cli {
    fn set_today<D: Datelike>(&mut self, today: &D) {
        if let Some(spec) = &self.spec {
            if self.year.is_some() || self.day.is_some() {
                panic!("arg conflict: SPEC may not be provided when --year or --day are used");
            }
            match spec.split_once('/') {
                None => {
                    if spec.len() >= 4 {
                        self.year = Some(spec.parse().unwrap());
                    } else {
                        self.day = Some(spec.parse().unwrap());
                    }
                }
                Some((y, d)) => {
                    self.year = Some(y.parse().unwrap());
                    self.day = Some(d.parse().unwrap());
                }
            };
        }

        let today_year = today.year();
        let today_day = today.day();
        match (
            self.year.is_some(),
            self.day.is_some(),
            self.this_year,
            today.month(),
        ) {
            // No args, today is a day on the advent calendar => run today only.
            // No args, today is not on the calendar => run nothing.
            (false, false, false, _) => {
                self.year = Some(today_year);
                self.day = Some(today_day);
            }
            // --this-year, today is still December => run today's.
            (false, false, true, 12) => {
                self.year = Some(today_year);
            }
            // --this-year, today is not December => run last year's.
            (false, false, true, _) => {
                self.year = Some(today_year - 1);
            }
            // --year Y => run everything from year Y.
            (true, _, _, _) => (),
            // --day D, today is December => run this year's day D.
            (false, true, _, 12) => {
                self.year = Some(today_year);
            }
            // --day D, today is not December => run last year's day D.
            (false, true, _, _) => {
                self.year = Some(today_year - 1);
            }
        }
        if self.year.is_none() && self.day.is_none() && today.month() == 12 {
            self.year = Some(today.year());
            if !self.this_year {
                self.day = Some(today.day());
            }
        }
    }

    fn matches(&self, year: i32, day: u32, part: u8) -> bool {
        if self.all {
            return true;
        }
        match (self.year, self.day, self.part) {
            (None, None, None) => false,
            (Some(y), _, _) if y != year => false,
            (_, Some(d), _) if d != day => false,
            (_, _, Some(p)) if p != part => false,
            _ => true,
        }
    }

    fn run_slow_parts(&self) -> bool {
        return self.include_slow;
    }
}
