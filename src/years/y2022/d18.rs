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
    if vis {
        for p in &exposed {
            println!("** {:?}", p);
        }
    }
    Box::new(exposed.values().sum::<usize>())
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    let mut exposed = HashMap::new();
    let mut covered = HashSet::new();
    let mut maxx = 0;
    let mut maxy = 0;
    let mut maxz = 0;
    for cube in input.lines() {
        let (x, y, z) = parse(cube);
        maxx = maxx.max(x);
        maxy = maxy.max(y);
        maxz = maxz.max(z);

        covered.insert((x, y, z));
        exposed.remove(&(x, y, z));

        add_side(&mut exposed, &covered, vis, (x + 1, y, z));
        add_side(&mut exposed, &covered, vis, (x - 1, y, z));
        add_side(&mut exposed, &covered, vis, (x, y + 1, z));
        add_side(&mut exposed, &covered, vis, (x, y - 1, z));
        add_side(&mut exposed, &covered, vis, (x, y, z + 1));
        add_side(&mut exposed, &covered, vis, (x, y, z - 1));
    }

    let mut free = HashSet::new();
    let mut trapped = HashSet::new();
    let mut total_exposed = 0;
    let mut connected = HashSet::new();
    let mut to_check = Vec::new();
    'exp: for (p, n) in exposed {
        if vis {
            println!("checking {:?} exposed={}...", p, n);
        }
        connected.clear();
        to_check.clear();
        to_check.push(p);
        while let Some(p) = to_check.pop() {
            if p.0 < 0 || p.1 < 0 || p.2 < 0 || p.0 > maxx || p.1 > maxy || p.2 > maxz {
                // escape!
                if vis {
                    println!("... escape via {:?}!", p);
                }
                for p in connected.drain() {
                    free.insert(p);
                }
                total_exposed += n;
                continue 'exp;
            }
            if connected.contains(&p) {
                // Already checked!
                continue;
            }
            if free.contains(&p) {
                // woo hoo! we're connected to a free square, so we are free too!
                if vis {
                    println!("... escape, found a way to {:?}", p);
                }
                for p in connected.drain() {
                    free.insert(p);
                }
                total_exposed += n;
                continue 'exp;
            }
            if trapped.contains(&p) {
                // womp we are trapped.
                if vis {
                    println!("... trapped :(");
                }
                assert!(
                    connected.is_empty(),
                    "expect no other spaces in the current search"
                );
                for p in connected.drain() {
                    trapped.insert(p);
                }
                continue 'exp;
            }
            if !covered.contains(&p) {
                // This is an empty space. Search its neighbors.
                add_neighbors(&mut to_check, &connected, &p);
                connected.insert(p);
            }
        }
        // Trapped!
        if vis {
            println!("... trapped, along with {} others", connected.len());
        }
        for p in connected.drain() {
            trapped.insert(p);
        }
    }
    Box::new(total_exposed)
}

fn add_neighbors(to_check: &mut Vec<P>, seen: &HashSet<P>, p: &P) {
    fn a(to_check: &mut Vec<P>, seen: &HashSet<P>, p: P) {
        if !seen.contains(&p) {
            to_check.push(p);
        }
    }
    let (x, y, z) = p;
    a(to_check, seen, (x - 1, *y, *z));
    a(to_check, seen, (x + 1, *y, *z));
    a(to_check, seen, (*x, y - 1, *z));
    a(to_check, seen, (*x, y + 1, *z));
    a(to_check, seen, (*x, *y, z - 1));
    a(to_check, seen, (*x, *y, z + 1));
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
