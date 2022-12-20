use std::fmt::Display;

// NOT -6697
pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let mut values: Vec<(usize, i32)> = input
        .lines()
        .map(|s| s.parse().unwrap())
        .enumerate()
        .collect();
    if vis {
        println!("Initial arrangement:");
        let mut sep = "";
        for (_, v) in &values {
            print!("{}{}", sep, v);
            sep = ", ";
        }
        println!();
    }
    for order in 0..values.len() {
        let (pos, (_, val)) = values
            .iter()
            .enumerate()
            .find(|(_, (i, _))| *i == order)
            .unwrap();
        let pos = pos as i32;
        let mut newpos = pos + val;
        let len = values.len() as i32;
        if vis {
            println!("{} moves from [{}] to [{}]", val, pos, newpos,);
        }
        if newpos < 0 {
            newpos -= 1;
            while newpos < 0 {
                newpos += len;
            }
        }
        newpos = newpos % len;
        if newpos < pos {
            values[(newpos as usize)..=(pos as usize)].rotate_right(1);
        } else if newpos > pos {
            values[(pos as usize)..=(newpos as usize)].rotate_left(1);
        }

        if vis {
            let mut sep = "";
            for (_, v) in &values {
                print!("{}{}", sep, v);
                sep = ", ";
            }
            println!();
        }
    }

    let zero = values.iter().position(|&(_, v)| v == 0).unwrap();
    let a = values[(zero + 1000) % values.len()].1;
    let b = values[(zero + 2000) % values.len()].1;
    let c = values[(zero + 3000) % values.len()].1;
    Box::new(a + b + c)
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    Box::new("todo")
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
-3
3
-2
0
4",
        part1 => 3,
        part2 => "todo");
}
