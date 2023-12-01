use std::fmt::Display;

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let key = [
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];
    Box::new(
        input
            .lines()
            .map(|line| get_cal_new(line, &key, vis))
            .sum::<u32>(),
    )
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    let key = [
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    Box::new(
        input
            .lines()
            .map(|line| get_cal_new(line, &key, vis))
            .sum::<u32>(),
    )
}

fn get_cal_new(line: &str, key: &[(&'static str, u32)], vis: bool) -> u32 {
    let mut first = None;
    let mut last = None;
    for k in key {
        first = match (line.find(k.0), first) {
            (Some(i), None) => Some((i, k.1)),
            (Some(i), Some((j, _))) if i < j => Some((i, k.1)),
            (_, x) => x,
        };
        last = match (line.rfind(k.0), last) {
            (Some(i), None) => Some((i, k.1)),
            (Some(i), Some((j, _))) if i > j => Some((i, k.1)),
            (_, x) => x,
        };
    }
    if vis {
        println!("{line} => {first:?} {last:?}");
    }
    first.unwrap().1 * 10 + last.unwrap().1
}

pub fn orig_part1(input: String, vis: bool) -> Box<dyn Display> {
    let vals: Vec<u32> = input.lines().map(|line| get_cal(line, vis)).collect();
    if vis {
        println!("{:?}", vals);
    }
    let sum: u32 = vals.into_iter().sum();
    Box::new(sum)
}

pub fn orig_part2(input: String, vis: bool) -> Box<dyn Display> {
    let vals: Vec<u32> = input.lines().map(|line| get_cal2(line, vis)).collect();
    if vis {
        println!("{:?}", vals);
    }
    let sum: u32 = vals.into_iter().sum();
    Box::new(sum)
}

fn get_cal(line: &str, vis: bool) -> u32 {
    let mut numbers = Vec::new();
    for c in line.chars() {
        if vis {
            println!("{c}");
        }
        if c >= '0' && c <= '9' {
            numbers.push((c as u32) - ('0' as u32));
        }
    }
    if vis {
        println!("=> {:?}", numbers);
    }
    numbers.first().unwrap() * 10 + numbers.last().unwrap()
}

fn get_cal2(mut line: &str, vis: bool) -> u32 {
    let orig = line;
    let mut numbers = Vec::new();
    while line != "" {
        if line.starts_with("0") {
            numbers.push(0);
        } else if line.starts_with("zero") {
            numbers.push(0);
        } else if line.starts_with("1") {
            numbers.push(1);
        } else if line.starts_with("one") {
            numbers.push(1);
        } else if line.starts_with("2") {
            numbers.push(2);
        } else if line.starts_with("two") {
            numbers.push(2);
        } else if line.starts_with("3") {
            numbers.push(3);
        } else if line.starts_with("three") {
            numbers.push(3);
        } else if line.starts_with("4") {
            numbers.push(4);
        } else if line.starts_with("four") {
            numbers.push(4);
        } else if line.starts_with("5") {
            numbers.push(5);
        } else if line.starts_with("five") {
            numbers.push(5);
        } else if line.starts_with("6") {
            numbers.push(6);
        } else if line.starts_with("six") {
            numbers.push(6);
        } else if line.starts_with("7") {
            numbers.push(7);
        } else if line.starts_with("seven") {
            numbers.push(7);
        } else if line.starts_with("8") {
            numbers.push(8);
        } else if line.starts_with("eight") {
            numbers.push(8);
        } else if line.starts_with("9") {
            numbers.push(9);
        } else if line.starts_with("nine") {
            numbers.push(9);
        }
        line = &line[1..];
    }
    if vis {
        println!("{orig} => {:?}", numbers);
    }
    numbers.first().unwrap() * 10 + numbers.last().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    crate::test::aoc_test!(example, r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        part1 => 142,
        orig_part1 => 142);

    crate::test::aoc_test!(part2_example, r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        part2 => 281,
        orig_part2 => 281);
}
