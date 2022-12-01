use std::fs;

use reqwest::blocking::Client;

pub fn get_input(year: i32, day: u32, token: Option<&str>) -> anyhow::Result<String> {
    let input_file = file_for(year, day);
    match (fs::read_to_string(&input_file), token) {
        (Ok(s), _) => return Ok(s),
        (Err(e), None) => return Err(e.into()),
        _ => {}
    };
    let url = url_for(year, day);
    println!("downloading {} ...", url);
    let resp = Client::new()
        .get(&url)
        .header("Cookie", format!("session={}", token.unwrap()))
        .send()?;
    anyhow::ensure!(resp.status().is_success(), "{}: {}", url, resp.status());
    fs::create_dir_all(year_dir(year))?;
    let s = resp.text()?;
    fs::write(&input_file, &s)?;
    Ok(s)
}

fn url_for(year: i32, day: u32) -> String {
    format!("https://adventofcode.com/{}/day/{}/input", year, day)
}

fn year_dir(year: i32) -> String {
    format!("inputs/{}", year)
}

fn file_for(year: i32, day: u32) -> String {
    format!("{}/{}", year_dir(year), day)
}
