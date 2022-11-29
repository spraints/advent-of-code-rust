// TODO
// - framework for adding days, parts.
// - framework for testing with example inputs.
//
// - 'cargo test' - runs all unit tests
// - 'cargo run [--year Y] [--day D] [--part 1|2]' - run the current day, downloading the input if needed.
//
// https://github.com/gobanos/aoc-runner-derive/blob/master/src/lib.rs if attr is useful.

mod curday;
mod download;

use std::path::PathBuf;

use chrono::{DateTime, Datelike, FixedOffset, NaiveDate};
use clap::{Parser, Subcommand};
use curday::aoc_now;
use download::download;

fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Some(Command::Download { wait, year }) => do_download(year, wait).unwrap(),
        Some(Command::SetToken { token }) => do_set_token(token).unwrap(),
        None => todo!(),
    };
}

// Alias the types that chrono uses for parts of the date.
type Year = i32;
type Day = u32;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Download {
        #[arg(short, long)]
        wait: bool,

        #[arg(short, long)]
        year: Option<Year>,
        //day: Option<Day>,
        //part: Option<Part>,
    },

    SetToken {
        token: String,
    },
}

fn do_download(year: Option<Year>, wait: bool) -> anyhow::Result<()> {
    let token = match read_token() {
        Ok(token) => token,
        Err(err) => {
            eprint!(
                "error: {}\nUse 'cargo run set-token TOKEN' to set your token.\n",
                err
            );
            std::process::exit(1);
        }
    };
    let client = reqwest::blocking::Client::new();
    for date in dates(year, wait) {
        download(&date, &token, &client)?;
    }
    Ok(())
}

fn dates(year: Option<Year>, wait: bool) -> DateIter {
    let now = aoc_now();
    match (year, wait, now.year(), now.month(), now.day()) {
        (None, true, y, 11, 30) => DateIter {
            year: y,
            stop_after: 1,
            now,
            next_day: 1,
        },
        (Some(y), true, cy, 11, 30) if y == cy => DateIter {
            year: y,
            stop_after: 1,
            now,
            next_day: 1,
        },
        (None, true, y, 12, d) => DateIter {
            year: y,
            stop_after: d + 1,
            now,
            next_day: 1,
        },
        (Some(y), true, cy, 12, d) if cy == y => DateIter {
            year: y,
            stop_after: d + 1,
            now,
            next_day: 1,
        },
        (None, false, y, 12, d) => DateIter {
            year: y,
            stop_after: d,
            now,
            next_day: 1,
        },
        (Some(y), false, cy, 12, d) if cy == y => DateIter {
            year: y,
            stop_after: d,
            now,
            next_day: 1,
        },
        (None, _, y, _, _) => DateIter {
            year: y - 1,
            stop_after: 25,
            now,
            next_day: 1,
        },
        (Some(y), _, _, _, _) => DateIter {
            year: y,
            stop_after: 25,
            now,
            next_day: 1,
        },
    }
}

struct DateIter {
    year: Year,
    stop_after: Day,
    now: DateTime<FixedOffset>,
    next_day: Day,
}

impl Iterator for DateIter {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_day > self.stop_after {
            return None;
        }
        if self.next_day == self.stop_after
            && self.year == self.now.year()
            && self.next_day > self.now.day()
        {
            panic!("need to sleep for a bit");
        }
        let this_day = self.next_day;
        self.next_day = self.next_day + 1;
        NaiveDate::from_ymd_opt(self.year, 12, this_day)
    }
}

fn do_set_token(token: String) -> anyhow::Result<()> {
    let cfg_file = token_path()?;
    std::fs::write(&cfg_file, token)?;
    println!("wrote token to {:?}", cfg_file);
    Ok(())
}

fn read_token() -> anyhow::Result<String> {
    Ok(std::fs::read_to_string(token_path()?)?)
}

fn token_path() -> anyhow::Result<PathBuf> {
    let app_dirs = platform_dirs::AppDirs::new(Some("advent-of-code"), false).unwrap();
    let mut cfg_file = app_dirs.config_dir;
    cfg_file.push("token");
    Ok(cfg_file)
}
