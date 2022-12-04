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
    let mut s = s.split(&[',', '-']).map(|s| s.parse().unwrap());
    let a1 = s.next().unwrap();
    let b1 = s.next().unwrap();
    let a2 = s.next().unwrap();
    let b2 = s.next().unwrap();
    (a1, b1, a2, b2)
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
