use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let mut exposed = HashMap::new();
    let mut covered = HashSet::new();
    for cube in input.lines() {
        let (x, y, z) = parse(cube);
        if vis {
            println!("! ({}, {}, {})", x, y, z);
        }
        covered.insert((x, y, z));
        if let Some(c) = exposed.remove(&(x, y, z)) {
            if vis {
                println!("  - cover {} ({}, {}, {})", c, x, y, z);
            }
        }
        add_side(&mut exposed, &covered, vis, (x + 1, y, z));
        add_side(&mut exposed, &covered, vis, (x - 1, y, z));
        add_side(&mut exposed, &covered, vis, (x, y + 1, z));
        add_side(&mut exposed, &covered, vis, (x, y - 1, z));
        add_side(&mut exposed, &covered, vis, (x, y, z + 1));
        add_side(&mut exposed, &covered, vis, (x, y, z - 1));
        if vis {
            println!("exposed={} covered={}", exposed.len(), covered.len());
        }
    }
    for p in &exposed {
        println!("** {:?}", p);
    }
    Box::new(exposed.values().sum::<usize>())
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    let covered: HashSet<P> = input.lines().map(parse).collect();
    let (maxx, maxy, maxz) = covered
        .iter()
        .copied()
        .reduce(|(ax, ay, az), (bx, by, bz)| (ax.max(bx), ay.max(by), az.max(bz)))
        .unwrap();
    if vis {
        println!("max: ({},{},{})", maxx, maxy, maxz);
    }

    fn e<I: Iterator<Item = C>, F: Fn(C) -> P>(covered: &HashSet<P>, r: I, next: F) -> u16 {
        for c in r {
            if covered.contains(&next(c)) {
                return 0;
            }
        }
        1
    }

    let mut exposed = 0;
    for p in &covered {
        let (x, y, z) = p.clone();
        let before = exposed;
        exposed += e(&covered, 0..x, |x| (x, y, z));
        exposed += e(&covered, (x + 1)..=maxx, |x| (x, y, z));
        exposed += e(&covered, 0..y, |y| (x, y, z));
        exposed += e(&covered, (y + 1)..=maxy, |y| (x, y, z));
        exposed += e(&covered, 0..z, |z| (x, y, z));
        exposed += e(&covered, (z + 1)..=maxz, |z| (x, y, z));
        if vis {
            println!("{:?} => {}", p, exposed - before);
        }
    }
    Box::new(exposed)
}

fn add_side(exposed: &mut HashMap<P, usize>, covered: &HashSet<P>, vis: bool, p: P) {
    if !covered.contains(&p) {
        if vis {
            println!("  + expose {:?}", p);
        }
        let c = exposed.entry(p).or_insert(0);
        *c += 1;
    }
}

type P = (C, C, C);
type C = i32;
fn parse(line: &str) -> P {
    let mut parts = line.split(',');
    let x = parts.next().unwrap().parse().unwrap();
    let y = parts.next().unwrap().parse().unwrap();
    let z = parts.next().unwrap().parse().unwrap();
    (x, y, z)
}

#[cfg(test)]
mod test {
    use super::*;

    crate::test::aoc_test!(smol, r"1,1,1
2,1,1",
        part1 => 10);

    crate::test::aoc_test!(example, r"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5",
            part1 => 64,
            part2 => 58);
}
