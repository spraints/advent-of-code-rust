use std::fmt::Display;

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let vals: Vec<u32> = input.lines().map(|line| get_cal(line)).collect();
    if vis {
        println!("{:?}", vals);
    }
    let sum: u32 = vals.into_iter().sum();
    Box::new(sum)
}

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

fn get_cal(line: &str) -> u32 {
    let mut numbers = Vec::new();
    for c in line.chars() {
        println!("{c}");
        if c >= '0' && c <= '9' {
            numbers.push((c as u32) - ('0' as u32));
        }
    }
    println!("=> {:?}", numbers);
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
        part2 => "todo");
}
