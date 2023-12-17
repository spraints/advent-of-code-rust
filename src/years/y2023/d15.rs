use std::fmt::Display;

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    Box::new(
        input
            .trim()
            .split(',')
            .map(|step| {
                let h = hash(step, false);
                if vis {
                    println!("{step:?} -> {h}");
                }
                h as u64
            })
            .sum::<u64>(),
    )
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    let pattern = regex::Regex::new("\\A(.*)(-|=([0-9]+))\\z").unwrap();
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];
    fn show_boxes<T: std::fmt::Debug>(boxes: &[Vec<T>]) {
        for (i, b) in boxes.iter().enumerate() {
            if !b.is_empty() {
                println!("Box {i}: {b:?}");
            }
        }
    }
    for step in input.trim().split(',') {
        let m = pattern.captures(step).expect("{step} should match!");
        let label = m.get(1).unwrap().as_str();
        let box_num = hash(label, false);
        let b = boxes
            .get_mut(box_num as usize)
            .expect("should have box {box_num}");
        match m.get(3) {
            None => {
                // "-" => remove this label.
                if let Some(i) = b.iter().position(|(l, _)| l == &label) {
                    b.remove(i);
                }
            }
            Some(focal_length) => {
                // "=\d"
                let focal_length: usize = focal_length.as_str().parse().unwrap();
                match b.iter().position(|(l, _)| l == &label) {
                    None => b.push((label, focal_length)),
                    Some(i) => b[i] = (label, focal_length),
                };
            }
        }
        if vis {
            println!("After {step:?}:");
            show_boxes(&boxes);
        }
    }
    let mut focusing_power: usize = 0;
    for (i, b) in boxes.into_iter().enumerate() {
        for (j, (l, fl)) in b.into_iter().enumerate() {
            let j = j + 1;
            let fp = (i + 1) * j * fl;
            if vis {
                println!("- {l} (box {i}) * {j} (slot) * {fl} (focal length) = {fp}");
            }
            focusing_power += fp;
        }
    }
    Box::new(focusing_power)
}

fn hash(input: &str, vis: bool) -> u8 {
    let input = input.as_bytes();
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
    const TEST_INPUT: &'static str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7\n";

    crate::test::aoc_test!(part1, TEST_INPUT, 1320);
    crate::test::aoc_test!(part2, TEST_INPUT, 145);

    #[test]
    fn test_hash() {
        assert_eq!(super::hash("HASH", true), 52);
    }
}
