use std::{fmt::Display, fs, path::Path};

use chrono::Datelike;
use reqwest::blocking::Client;

pub fn download<D: Datelike + Display>(
    date: &D,
    token: &str,
    client: &Client,
) -> anyhow::Result<String> {
    let dest_file = file_for(date);
    if matches!(Path::new(&dest_file).try_exists(), Ok(true)) {
        return Ok(dest_file);
    }
    // sleep_if_needed(date);
    println!("download {} ...", date);
    let url = url_for(date);
    let resp = client
        .get(&url)
        .header("Cookie", format!("session={}", token))
        .send()?;
    anyhow::ensure!(resp.status().is_success(), "{}: {}", url, resp.status());
    fs::create_dir_all(year_dir(date))?;
    fs::write(&dest_file, resp.text()?)?;
    Ok(dest_file)
}

fn url_for<D: Datelike>(date: &D) -> String {
    format!(
        "https://adventofcode.com/{}/day/{}/input",
        date.year(),
        date.day()
    )
}

fn year_dir<D: Datelike>(date: &D) -> String {
    format!("inputs/{}", date.year())
}

fn file_for<D: Datelike>(date: &D) -> String {
    format!("{}/{}", year_dir(date), date.day())
}
