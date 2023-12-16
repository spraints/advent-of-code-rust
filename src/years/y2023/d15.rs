use std::fmt::Display;

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    Box::new(
        input
            .split(',')
            .map(|step| {
                let h = hash(step.as_bytes(), false);
                if vis {
                    println!("{step} -> {h}");
                }
                h as u64
            })
            .sum::<u64>(),
    )
}

pub fn part2(_input: String, _vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

fn hash(input: &[u8], vis: bool) -> u8 {
    let mut current_value: u8 = 0;
    for c in input {
        current_value = current_value.wrapping_add(*c).wrapping_mul(17);
        if vis {
            println!("--> {c} --> {current_value}");
        }
    }
    current_value
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    crate::test::aoc_test!(part1, TEST_INPUT, 1320);
    crate::test::aoc_test!(part2, TEST_INPUT, "todo");

    #[test]
    fn test_hash() {
        assert_eq!(super::hash("HASH".as_bytes(), true), 52);
    }
}
