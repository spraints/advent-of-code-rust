use std::fmt::Display;
use std::iter::zip;

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let races = parse(&input);
    let res: u64 = races.iter().map(|r| score(r, vis)).product();
    Box::new(res)
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    let sp = regex::Regex::new(" ").unwrap();
    let input = sp.replace_all(&input, "");
    let races = parse(&input);
    let res: u64 = races.iter().map(|r| score(r, vis)).product();
    Box::new(res)
}

fn score(race: &Race, vis: bool) -> u64 {
    // speed = x
    // dist = speed * (time - x)
    // break even when 0 = speed * (time - x) - best
    // y = x * (time - x) - best
    // y = x * time - x * x - best
    // y = -x^2 + time * x - best
    // a = -1
    // b = time
    // c = -best
    // x = (-b +/- sqrt(b^2 - 4ac)) / 2a
    let a = -1 as f64;
    let b = race.time as f64;
    let c = -1.0 * (race.best as f64);
    let s = (b * b - 4.0 * a * c).sqrt();
    let x1 = (-b + s) / 2.0 * a;
    let x2 = (-b - s) / 2.0 * a;
    if vis {
        println!("0 = {a} * x^2 + {b} * x + {c}");
        println!("time={} best={} => [ {x1}, {x2} ]", race.time, race.best);
        //for i in 1..race.time {
        //    println!("  press={i} ===> {}", i * (race.time - i));
        //}
    }
    let mut x1 = x1.ceil() as u64;
    if x1 * (race.time - x1) <= race.best {
        x1 += 1;
    }
    let mut x2 = x2.floor() as u64;
    if x2 * (race.time - x2) <= race.best {
        x2 -= 1;
    }
    1 + x2 - x1
}

fn parse(input: &str) -> Vec<Race> {
    let re = regex::Regex::new(r"\d+").unwrap();
    let mut lines = input.lines();

    let times = re.find_iter(lines.next().unwrap()).map(|m| m.as_str());
    let bests = re.find_iter(lines.next().unwrap()).map(|m| m.as_str());
    assert!(lines.next().is_none());

    zip(times, bests).map(mkrace).collect()
}

fn mkrace(t: (&str, &str)) -> Race {
    let (time, best) = t;
    let time = time.parse().unwrap();
    let best = best.parse().unwrap();
    Race { time, best }
}

struct Race {
    time: u64,
    best: u64,
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str = r"Time:      7  15   30
Distance:  9  40  200";

    crate::test::aoc_test!(part1, TEST_INPUT, 288);
    crate::test::aoc_test!(part2, TEST_INPUT, 71503);
}
