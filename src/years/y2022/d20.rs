use std::fmt::Display;

// YES 16533
// NOT -6697
pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let mut values: Vec<(usize, i32)> = input
        .lines()
        .map(|s| s.parse().unwrap())
        .enumerate()
        .collect();

    mix(&mut values, vis);

    Box::new(score(values))
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    Box::new("todo")
}

fn score(values: Vec<(usize, i32)>) -> i32 {
    let zero = values.iter().position(|&(_, v)| v == 0).unwrap();
    let a = values[(zero + 1000) % values.len()].1;
    let b = values[(zero + 2000) % values.len()].1;
    let c = values[(zero + 3000) % values.len()].1;
    a + b + c
}

fn mix(values: &mut Vec<(usize, i32)>, vis: bool) {
    if vis {
        println!("Initial arrangement of {} items:", values.len());
        let mut sep = "";
        for (_, v) in values.iter() {
            print!("{}{}", sep, v);
            sep = ", ";
        }
        println!();
    }
    for order in 0..values.len() {
        mix1(values, order, vis);
    }
}

fn mix1(values: &mut Vec<(usize, i32)>, order: usize, vis: bool) {
    let len = values.len() as i32;
    let (origpos, (_, val)) = values
        .iter()
        .enumerate()
        .find(|(_, (i, _))| *i == order)
        .unwrap();
    let origpos = origpos;
    let val = *val;
    let mut pos = origpos as i32;
    if val != 0 {
        let offset = val / val.abs();
        //let val = offset * (val.abs() % len);
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
        let mut sep = "";
        for (_, v) in values.iter() {
            print!("{}{}", sep, v);
            sep = ", ";
        }
        println!();
    }
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

    #[test]
    fn test_mix() {
        fn mix(values: Vec<i32>) -> Vec<i32> {
            let mut values = values.into_iter().enumerate().collect();
            super::mix(&mut values, false);
            values.into_iter().map(|(_, v)| v).collect()
        }

        assert_eq!(vec![0, 0, 0, 1, 0], mix(vec![0, 0, 1, 0, 0]));
        assert_eq!(vec![0, -1, 0, 0, 0], mix(vec![0, 0, -1, 0, 0]));

        assert_eq!(vec![0, 4, 0, 0, 0], mix(vec![0, 0, 4, 0, 0]));
        assert_eq!(vec![0, 0, 0, -4, 0], mix(vec![0, 0, -4, 0, 0]));

        assert_eq!(vec![0, 0, 0, 6, 0], mix(vec![0, 0, 6, 0, 0]));
        assert_eq!(vec![0, -6, 0, 0, 0], mix(vec![0, 0, -6, 0, 0]));

        assert_eq!(vec![0, 9, 0, 0, 0], mix(vec![0, 0, 9, 0, 0]));
        assert_eq!(vec![0, 0, 0, -9, 0], mix(vec![0, 0, -9, 0, 0]));
    }

    #[test]
    fn test_mix1() {
        fn mix1(values: Vec<i32>, pos: usize) -> Vec<i32> {
            let mut values = values.into_iter().enumerate().collect();
            super::mix1(&mut values, pos, false);
            values.into_iter().map(|(_, v)| v).collect()
        }

        assert_eq!(vec![4, 5, 6, 1, 7], mix1(vec![4, 5, 1, 6, 7], 2));
        assert_eq!(vec![4, -1, 5, 6, 7], mix1(vec![4, 5, -1, 6, 7], 2));

        assert_eq!(vec![8, 4, 7, 6, 9], mix1(vec![9, 8, 4, 7, 6], 2));
        assert_eq!(vec![6, 9, 8, -4, 7], mix1(vec![9, 8, -4, 7, 6], 2));

        assert_eq!(vec![2, 3, 4, 6, 1], mix1(vec![1, 2, 6, 3, 4], 2));
        assert_eq!(vec![4, -6, 1, 2, 3], mix1(vec![1, 2, -6, 3, 4], 2));

        assert_eq!(vec![3, 9, 4, 1, 2], mix1(vec![1, 2, 9, 3, 4], 2));
        assert_eq!(vec![3, 4, 1, -9, 2], mix1(vec![1, 2, -9, 3, 4], 2));
    }
}
