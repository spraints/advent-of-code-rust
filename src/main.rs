// TODO
// - framework for adding days, parts.
// - framework for testing with example inputs.
//
// - 'cargo test' - runs all unit tests
// - 'cargo run [--year Y] [--day D] [--part 1|2]' - run the current day, downloading the input if
//    needed.
// - 'cargo run --all [--year Y] [--day D] [--part 1|2]' - run everything, maybe filtered down a
//    bit.
// - 'cargo run --set-token TOKEN' - stash my auth token.
//
// https://github.com/gobanos/aoc-runner-derive/blob/master/src/lib.rs if attr is useful.

mod curday;
mod download;
mod token;

use clap::Parser;
use token::set_token;

fn main() {
    let cli = Cli::parse();
    match cli.set_token {
        Some(token) => set_token(token).unwrap(),
        None => do_run(cli).unwrap(),
    }
}

fn do_run(cli: Cli) -> anyhow::Result<()> {
    //let runner = Runner::new();
    // todo register solvers
    //runner.run(cli)
    Ok(())
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
    day: Option<i32>,

    /// Run either part 1 or 2.
    #[arg(short, long)]
    part: Option<u8>,
}
