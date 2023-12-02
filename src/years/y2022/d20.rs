use std::fmt::Display;

// YES 16533
// NOT -6697
pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let mut values: Vec<(usize, i64)> = input
        .lines()
        .map(|s| s.parse().unwrap())
        .enumerate()
        .collect();

    mix(&mut values, vis);

    Box::new(score(values))
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    let mut values: Vec<(usize, i64)> = input
        .lines()
        .map(|s| s.parse::<i64>().unwrap() * 811589153)
        .enumerate()
        .collect();

    if vis {
        println!("Initial arrangement of {} items:", values.len());
        print_values(&values);
        println!();
    }

    for i in 0..10 {
        mix(&mut values, false);

        if vis {
            let mut rv = values.clone();
            let zero = find_zero(&rv);
            rv.rotate_left(zero);
            println!("After round {}", i + 1);
            print_values(&rv);
        }
    }

    // 15045 is too low
    // 4789999181006 is right
    Box::new(score(values))
}

fn find_zero(values: &[(usize, i64)]) -> usize {
    values.iter().position(|&(_, v)| v == 0).unwrap()
}

fn score(values: Vec<(usize, i64)>) -> i64 {
    let zero = find_zero(&values);
    let a = values[(zero + 1000) % values.len()].1;
    let b = values[(zero + 2000) % values.len()].1;
    let c = values[(zero + 3000) % values.len()].1;
    a + b + c
}

fn mix(values: &mut Vec<(usize, i64)>, vis: bool) {
    if vis {
        println!("Initial arrangement of {} items:", values.len());
        print_values(values);
    }
    for order in 0..values.len() {
        mix1(values, order, vis);
    }
}

fn mix1(values: &mut Vec<(usize, i64)>, order: usize, vis: bool) {
    fn find(values: &[(usize, i64)], order: usize) -> (usize, i64) {
        let (pos, (_, val)) = values
            .iter()
            .enumerate()
            .find(|(_, (i, _))| *i == order)
            .unwrap();
        (pos, *val)
    }

    let (pos, val) = find(values, order);
    let len = values.len() as i64;
    if val == 0 {
        return;
    }
    let mut newpos = (pos as i64 + val) % (len - 1);
    if newpos < 0 {
        newpos += len - 1;
    }
    assert!(
        newpos >= 0 && newpos < (len - 1),
        "newpos={} values.len={}",
        newpos,
        values.len()
    );
    let newpos = newpos as usize;
    let e = values.remove(pos);
    values.insert(newpos, e);
    /*
    if newpos < pos {
        values[newpos..=pos].rotate_right(1);
    } else {
        values[pos..=newpos].rotate_left(1);
    }
    */
    if vis {
        println!("moved {} from [{}] to [{}]", val, pos, newpos);
        print_values(values);
    }
}

fn print_values(values: &[(usize, i64)]) {
    let mut sep = "";
    for (_, v) in values.iter() {
        print!("{}{}", sep, v);
        sep = ", ";
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
        assert_eq!(-2, -32 % 5);
    }

    crate::test::aoc_tests!(example, r"1
2
-3
3
-2
0
4",
        part1 => 3,
        part2 => 1623178306);

    // Order doesn't matter. If they're the same at any point, things are good.
    fn check(mut left: Vec<i64>, right: Vec<i64>) {
        for _ in 0..left.len() {
            if left == right {
                return;
            }
            left.rotate_right(1);
        }
        assert_eq!(left, right);
    }

    #[test]
    fn test_mix() {
        fn mix(values: Vec<i64>) -> Vec<i64> {
            let mut values = values.into_iter().enumerate().collect();
            super::mix(&mut values, false);
            values.into_iter().map(|(_, v)| v).collect()
        }

        check(vec![0, 0, 0, 1, 0], mix(vec![0, 0, 1, 0, 0]));
        check(vec![0, -1, 0, 0, 0], mix(vec![0, 0, -1, 0, 0]));

        check(vec![0, 4, 0, 0, 0], mix(vec![0, 0, 4, 0, 0]));
        check(vec![0, 0, 0, -4, 0], mix(vec![0, 0, -4, 0, 0]));

        check(vec![0, 0, 0, 6, 0], mix(vec![0, 0, 6, 0, 0]));
        check(vec![0, -6, 0, 0, 0], mix(vec![0, 0, -6, 0, 0]));

        check(vec![0, 9, 0, 0, 0], mix(vec![0, 0, 9, 0, 0]));
        check(vec![0, 0, 0, -9, 0], mix(vec![0, 0, -9, 0, 0]));
    }

    const MIX1_VIS: bool = true;

    #[test]
    fn test_mix1() {
        fn mix1(values: Vec<i64>, pos: usize) -> Vec<i64> {
            let mut values: Vec<(usize, i64)> = values.into_iter().enumerate().collect();
            if MIX1_VIS {
                println!("------");
                println!("mix1 test input");
                print_values(&values);
            }
            super::mix1(&mut values, pos, MIX1_VIS);
            values.into_iter().map(|(_, v)| v).collect()
        }

        check(vec![4, 5, 6, 1, 7], mix1(vec![4, 5, 1, 6, 7], 2));
        check(vec![4, -1, 5, 6, 7], mix1(vec![4, 5, -1, 6, 7], 2));

        check(vec![8, 4, 7, 6, 9], mix1(vec![9, 8, 4, 7, 6], 2));
        check(vec![6, 9, 8, -4, 7], mix1(vec![9, 8, -4, 7, 6], 2));

        check(vec![2, 3, 4, 6, 1], mix1(vec![1, 2, 6, 3, 4], 2));
        check(vec![4, -6, 1, 2, 3], mix1(vec![1, 2, -6, 3, 4], 2));

        check(vec![3, 9, 4, 1, 2], mix1(vec![1, 2, 9, 3, 4], 2));
        check(vec![3, 4, 1, -9, 2], mix1(vec![1, 2, -9, 3, 4], 2));

        // pos = 2
        // val = 44
        // len = 5
        // remove(2) => [1,2,3,4]
        // insert((pos+val)%(len-1) => 2) => [1,2,44,3,4]
        check(vec![2, 44, 3, 4, 1], mix1(vec![1, 2, 44, 3, 4], 2));
        // remove(2) => [1,2,3,4]
        // insert(3) => [1,2,3,45,4]
        check(vec![2, 3, 45, 4, 1], mix1(vec![1, 2, 45, 3, 4], 2));
        // remove(2) => [1,2,3,4]
        // insert(0) => [46,1,2,3,4]
        check(vec![2, 3, 4, 46, 1], mix1(vec![1, 2, 46, 3, 4], 2));
        // remove(2) => [1,2,3,4]
        // insert(1) => [1,47,2,3,4]
        check(vec![2, 3, 4, 1, 47], mix1(vec![1, 2, 47, 3, 4], 2));
        // remove(2) => [1,2,3,4]
        // insert(2) => [1,2,48,3,4]
        check(vec![48, 3, 4, 1, 2], mix1(vec![1, 2, 48, 3, 4], 2));

        // remove(pos) => remove(2)
        // insert((pos + val) mod len) => insert((2 - 44) mod 5) => insert(-42 mod 5) => insert(3)
        check(vec![4, 1, 2, -44, 3], mix1(vec![1, 2, -44, 3, 4], 2));
        check(vec![4, 1, -45, 2, 3], mix1(vec![1, 2, -45, 3, 4], 2));
        check(vec![4, -46, 1, 2, 3], mix1(vec![1, 2, -46, 3, 4], 2));
        check(vec![-47, 4, 1, 2, 3], mix1(vec![1, 2, -47, 3, 4], 2));
        check(vec![3, 4, 1, 2, -48], mix1(vec![1, 2, -48, 3, 4], 2));

        check(
            vec![1, 2, 3, -2, -3, 0, 4],
            mix1(vec![1, -3, 2, 3, -2, 0, 4], 1),
        );

        /*
        // val = 49
        // newpos = 51 % 5 = 1
        // rotateleft 2 (49, 3, 4, 1, 2), [0..=1].rotateright(1) => (3, 49, 4, 1, 2)
        assert_eq!(vec![3, 49, 4, 1, 2], mix1(vec![1, 2, 49, 3, 4], 2));
        // rotateleft 3 (
        assert_eq!(vec![4, 54, 1, 2, 3], mix1(vec![1, 2, 54, 3, 4], 2));
        // rotateleft 0
        assert_eq!(vec![1, 59, 2, 3, 4], mix1(vec![1, 2, 59, 3, 4], 2));
        // rotateleft 1
        assert_eq!(vec![2, 64, 3, 4, 1], mix1(vec![1, 2, 64, 3, 4], 2));

        assert_eq!(vec![4, 1, 2, -64, 3], mix1(vec![1, 2, -64, 3, 4], 2));
        */
    }
}
