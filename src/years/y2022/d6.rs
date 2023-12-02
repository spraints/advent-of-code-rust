use std::fmt::Display;

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    find_tag(&input, 4, vis)
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    find_tag(&input, 14, vis)
}

fn find_tag(input: &str, size: usize, vis: bool) -> Box<dyn Display> {
    let input = input.as_bytes();
    for (i, vals) in input.windows(size).enumerate() {
        if none_match(vals) {
            let j = i + size;
            if vis {
                unsafe {
                    println!(
                        "{} [ {} ] {}",
                        std::str::from_utf8_unchecked(&input[..i]),
                        std::str::from_utf8_unchecked(&input[i..j]),
                        std::str::from_utf8_unchecked(&input[j..])
                    )
                };
            }
            return Box::new(i + vals.len());
        }
    }
    Box::new("not found")
}

fn none_match(v: &[u8]) -> bool {
    for i in 0..v.len() {
        for j in (i + 1)..v.len() {
            if v[i] == v[j] {
                return false;
            }
        }
    }
    true
}

pub fn part1_alt(input: String, vis: bool) -> Box<dyn Display> {
    find_tag2(&input, 4, vis)
}

pub fn part2_alt(input: String, vis: bool) -> Box<dyn Display> {
    find_tag2(&input, 14, vis)
}

fn find_tag2(input: &str, size: usize, vis: bool) -> Box<dyn Display> {
    if vis {
        println!("input: {}", input);
        println!("size: {}", size);
    }
    let input = input.as_bytes();
    let mut min_good = size;
    'search: for i in 1..input.len() {
        if vis {
            println!("[{}] min = {}", i, min_good);
        }
        if i == min_good {
            let s = i - size;
            if vis {
                unsafe {
                    println!(
                        "{} [ {} ] {}",
                        std::str::from_utf8_unchecked(&input[..s]),
                        std::str::from_utf8_unchecked(&input[s..i]),
                        std::str::from_utf8_unchecked(&input[i..])
                    )
                };
            }
            return Box::new(i);
        }
        let c = input[i];
        for off in 1..size {
            if off > i {
                if vis {
                    println!("{}: {}: too short", i, unsafe {
                        std::str::from_utf8_unchecked(&input[0..=i])
                    });
                }
                continue 'search;
            }
            let j = i - off;
            if input[j] == c {
                let maybe = j + size + 1;
                if maybe > min_good {
                    min_good = maybe;
                }
                if vis {
                    unsafe {
                        println!(
                            "{}/{}: {}: match -> {} -> {}",
                            j,
                            i,
                            std::str::from_utf8_unchecked(&input[j..=i]),
                            maybe,
                            min_good,
                        )
                    };
                }
                continue 'search;
            }
        }
        if vis && i >= size {
            unsafe {
                println!(
                    "{}: {}: no match for {}",
                    i,
                    std::str::from_utf8_unchecked(&input[i - size..=i]),
                    std::char::from_u32_unchecked(c as u32),
                )
            };
        }
    }
    Box::new("not found")
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::aoc_tests;

    aoc_tests!(a, "mjqjpqmgbljsphdztnvjfqwrcgsmlb", part1 => 7, part2 => 19);
    aoc_tests!(b, "bvwbjplbgvbhsrlpgdmjqwftvncz", part1 => 5, part2 => 23);
    aoc_tests!(c, "nppdvjthqldpwncqszvftbrmjlhg", part1 => 6, part2 => 23);
    aoc_tests!(d, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", part1 => 10, part2 => 29);
    aoc_tests!(e, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", part1 => 11, part2 => 26);

    aoc_tests!(alt_a, "mjqjpqmgbljsphdztnvjfqwrcgsmlb", part1_alt => 7, part2_alt => 19);
    aoc_tests!(alt_b, "bvwbjplbgvbhsrlpgdmjqwftvncz", part1_alt => 5, part2_alt => 23);
    aoc_tests!(alt_c, "nppdvjthqldpwncqszvftbrmjlhg", part1_alt => 6, part2_alt => 23);
    aoc_tests!(alt_d, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", part1_alt => 10, part2_alt => 29);
    aoc_tests!(alt_e, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", part1_alt => 11, part2_alt => 26);
}
