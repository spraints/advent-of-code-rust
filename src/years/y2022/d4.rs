use std::fmt::Display;

pub fn part1(input: String) -> Box<dyn Display> {
    fn completely_contains(x: &(u8, u8, u8, u8)) -> bool {
        let (a1, b1, a2, b2) = x;
        let res = (a1 <= a2 && b1 >= b2) || (a2 <= a1 && b2 >= b1);
        //println!("{:?} => {:?}", x, res);
        res
    }
    Box::new(input.lines().map(parse).filter(completely_contains).count())
}

pub fn part2(input: String) -> Box<dyn Display> {
    fn overlaps(x: &(u8, u8, u8, u8)) -> bool {
        let (a1, b1, a2, b2) = x;
        let res = (a1 <= a2 && b1 >= a2) || (a2 <= a1 && b2 >= a1);
        //println!("{:?} => {:?}", x, res);
        res
    }
    Box::new(input.lines().map(parse).filter(overlaps).count())
}

fn parse(s: &str) -> (u8, u8, u8, u8) {
    let (a, b) = s.split_once(",").unwrap();
    let (a1, b1) = a.split_once("-").unwrap();
    let (a2, b2) = b.split_once("-").unwrap();
    (
        a1.parse().unwrap(),
        b1.parse().unwrap(),
        a2.parse().unwrap(),
        b2.parse().unwrap(),
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::*;

    const EX: &str = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn part1_example() {
        dotest(2, EX, part1);
    }

    #[test]
    fn part2_example() {
        dotest(4, EX, part2);
    }
}
