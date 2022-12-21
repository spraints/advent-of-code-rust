use std::fmt::Display;

// NOT -6697
// YES 16533
pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let mut values: Vec<(usize, i64)> = input
        .lines()
        .map(|s| s.parse().unwrap())
        .enumerate()
        .collect();
    mix(&mut values, vis);

    let zero = values.iter().position(|&(_, v)| v == 0).unwrap();
    let a = values[(zero + 1000) % values.len()].1;
    let b = values[(zero + 2000) % values.len()].1;
    let c = values[(zero + 3000) % values.len()].1;
    Box::new(a + b + c)
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    const KEY: i64 = 811589153;
    let mut values: Vec<(usize, i64)> = input
        .lines()
        .map(|s| KEY * s.parse::<i64>().unwrap())
        .enumerate()
        .collect();

    if vis {
        println!("Initial arrangement:");
        print_values(&values, Some(4));
    }

    for i in 0..10 {
        if i == 0 {
            return Box::new("BOOOM");
        }
        mix(&mut values, false);
        if vis {
            println!("After round {}:", i + 1);
            print_values(&values, Some(4));
        }
    }

    let zero = values.iter().position(|&(_, v)| v == 0).unwrap();
    let a = values[(zero + 1000) % values.len()].1;
    let b = values[(zero + 2000) % values.len()].1;
    let c = values[(zero + 3000) % values.len()].1;
    Box::new(a + b + c)
}

fn mix(values: &mut Vec<(usize, i64)>, vis: bool) {
    if vis {
        println!("Initial arrangement of {} items:", values.len());
        print_values(values, None);
    }
    let len = values.len() as i64;
    for order in 0..values.len() {
        let (origpos, (_, val)) = values
            .iter()
            .enumerate()
            .find(|(_, (i, _))| *i == order)
            .unwrap();
        let val = *val;
        let mut pos = origpos as i64;
        if val != 0 {
            let offset = val / val.abs();
            for _ in 0..val.abs() {
                let mut newpos = pos + offset;
                if newpos < 0 {
                    newpos = len - 1;
                }
                newpos = newpos % len;
                let swap = values[newpos as usize].clone();
                values[newpos as usize] = values[pos as usize].clone();
                values[pos as usize] = swap;
                pos = newpos;
            }
        }
        if vis {
            println!("moved {} from [{}] to [{}]", val, origpos, pos);
            print_values(values, None);
        }
        if vis {
            println!();
        }
    }
}

fn print_values(mut values: &[(usize, i64)], cap: Option<usize>) {
    if let Some(cap) = cap {
        values = &values[0..cap];
    }
    let mut sep = "";
    for (_, v) in values {
        print!("{}{}", sep, v);
        sep = ", ";
    }
    if cap.is_some() {
        print!("{}...", sep);
    }
    println!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn negative_mod() {
        // This is not what I want, but good to know.
        assert_eq!(-32, -32 % 33);
    }

    crate::test::aoc_test!(example, r"1
2
-10
3
-2
0
11",
        part1 => 3,
        part2 => 1623178306);
}
