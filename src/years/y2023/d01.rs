use std::fmt::Display;

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let vals: Vec<u32> = input.lines().map(|line| get_cal(line, vis)).collect();
    if vis {
        println!("{:?}", vals);
    }
    let sum: u32 = vals.into_iter().sum();
    Box::new(sum)
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
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
            line = &line[1..];
        } else if line.starts_with("zero") {
            numbers.push(0);
            line = &line[4..];
        } else if line.starts_with("1") {
            numbers.push(1);
            line = &line[1..];
        } else if line.starts_with("one") {
            numbers.push(1);
            line = &line[3..];
        } else if line.starts_with("2") {
            numbers.push(2);
            line = &line[1..];
        } else if line.starts_with("two") {
            numbers.push(2);
            line = &line[3..];
        } else if line.starts_with("3") {
            numbers.push(3);
            line = &line[1..];
        } else if line.starts_with("three") {
            numbers.push(3);
            line = &line[5..];
        } else if line.starts_with("4") {
            numbers.push(4);
            line = &line[1..];
        } else if line.starts_with("four") {
            numbers.push(4);
            line = &line[4..];
        } else if line.starts_with("5") {
            numbers.push(5);
            line = &line[1..];
        } else if line.starts_with("five") {
            numbers.push(5);
            line = &line[4..];
        } else if line.starts_with("6") {
            numbers.push(6);
            line = &line[1..];
        } else if line.starts_with("six") {
            numbers.push(6);
            line = &line[3..];
        } else if line.starts_with("7") {
            numbers.push(7);
            line = &line[1..];
        } else if line.starts_with("seven") {
            numbers.push(7);
            line = &line[5..];
        } else if line.starts_with("8") {
            numbers.push(8);
            line = &line[1..];
        } else if line.starts_with("eight") {
            numbers.push(8);
            line = &line[5..];
        } else if line.starts_with("9") {
            numbers.push(9);
            line = &line[1..];
        } else if line.starts_with("nine") {
            numbers.push(9);
            line = &line[4..];
        } else {
            line = &line[1..];
        }
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
        part1 => 142);

    crate::test::aoc_test!(part2_example, r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        part2 => 281);
}
