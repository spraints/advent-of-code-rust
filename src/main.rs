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
mod solutionset;
mod test;
mod token;
mod years;

use chrono::Datelike;
use clap::Parser;
use curday::aoc_now;
use input::get_input;
use solutionset::SolutionSet;
use token::{get_token, set_token};

fn main() {
    let cli = Cli::parse();
    match cli.set_token {
        Some(token) => set_token(token).unwrap(),
        None => do_run(cli).unwrap(),
    }
}

fn do_run(cli: Cli) -> anyhow::Result<()> {
    let mut runner = Runner::new();
    years::y2021::register(&mut runner);
    // todo register solvers
    runner.run(cli)
}

#[derive(Default)]
struct Runner {
    solvers: Vec<Solver>,
}

impl Runner {
    fn new() -> Self {
        Default::default()
    }

    fn run(self, mut cli: Cli) -> anyhow::Result<()> {
        let token = get_token()?;
        cli.set_today(aoc_now());
        for solver in self.solvers {
            if cli.matches(solver.year, solver.day, solver.part) {
                println!(
                    "{}/{}/{}: {}",
                    solver.year,
                    solver.day,
                    solver.part,
                    (solver.f)(get_input(solver.year, solver.day, &token)?)?
                );
            }
        }
        Ok(())
    }
}

impl SolutionSet for &mut Runner {
    fn add<F>(self, year: i32, day: u32, part: u8, f: F)
    where
        F: Fn(String) -> anyhow::Result<String> + 'static,
    {
        self.solvers.push(Solver {
            year,
            day,
            part,
            f: Box::new(f),
        });
    }
}

struct Solver {
    year: i32,
    day: u32,
    part: u8,
    f: Box<dyn Fn(String) -> anyhow::Result<String>>,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Set your token instead of solving any puzzles. This is the value of the session cookie on adventofcode.com.
    #[arg(long)]
    set_token: Option<String>,

    /// Run all solvers.
    #[arg(long)]
    all: bool,

    /// Run all of the given year's solvers, unless --day is set.
    #[arg(short, long)]
    year: Option<i32>,

    /// Run the given day's solvers.
    #[arg(short, long)]
    day: Option<u32>,

    /// Run either part 1 or 2.
    #[arg(short, long)]
    part: Option<u8>,
}

impl Cli {
    fn set_today<D: Datelike>(&mut self, today: D) {
        if self.year.is_none() && self.day.is_none() && today.month() == 12 {
            self.year = Some(today.year());
            self.day = Some(today.day());
        }
    }

    fn matches(&self, year: i32, day: u32, part: u8) -> bool {
        if self.all {
            return true;
        }
        match (self.year, self.day, self.part) {
            (Some(y), _, _) if y != year => false,
            (_, Some(d), _) if d != day => false,
            (_, _, Some(p)) if p != part => false,
            _ => true,
        }
    }
}
