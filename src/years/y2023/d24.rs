use std::fmt::Display;

use num::rational::Ratio;

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    Box::new(solve1(
        &input,
        (200000000000000, 200000000000000),
        (400000000000000, 400000000000000),
        vis,
    ))
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    let stones = parse(&input);
    if vis {
        println!("{stones:?}");
    }
    // Looking for (xS,yS,zS) @ (dxS,dyS,dzS).
    // t0 where (xS+t0*dxS, yS+t0*dyS, zS+t0*dzS) == (x0+t0*dx0, y0+t0*dy0, z0+t0*dz0)
    // t1 where (xS+t1*dxS, yS+t1*dyS, zS+t1*dzS) == (x1+t1*dx1, y1+t1*dy1, z1+t1*dz1)
    // ...
    // tN where (xS+tN*dxS, yS+tN*dyS, zS+tN*dzS) == (xN+tN*dxN, yN+tN*dyN, zN+tN*dzN)
    //
    // 3*N equations, 6+N unknowns.
    // xS+t0*dxS = x0+t0*dx0 => xS - x0 + t0*(dxS - dx0) = 0
    // xS+t0*dxS = x0+t0*dx0 => xS + t0*dxS - t0*dx0 = x0
    // I want to put this into a matrix, but would need coefficients of unknowns, and t0*dxS
    // doesn't help me here.
    //
    // Maybe I can solve for the first three lines and then that will be the solution for all?
    // 0: xS + t0*dxS = x0 + t0*dx0 (1 eq [xS, t0, dxS] 3 unk)
    // 1: yS + t0*dyS = y0 + t0*dy0 (2 eq [xS, t0, dxS, yS, dyS] 5 unk)
    // 2: zS + t0*dzS = z0 + t0*dz0 (3 eq [xS, t0, dxS, yS, dyS, zS, dzS] 7 unk)
    // 3: xS + t1*dxS = x1 + t1*dx1 (4 eq [xS, t0, dxS, yS, dyS, zS, dzS, t1] 8 unk)
    // 4: yS + t1*dyS = y1 + t1*dy1 (5 eq [xS, t0, dxS, yS, dyS, zS, dzS, t1] 8 unk)
    // 5: zS + t1*dzS = z1 + t1*dz1 (6 eq [xS, t0, dxS, yS, dyS, zS, dzS, t1] 8 unk)
    // 6: xS + t2*dxS = x2 + t2*dx2 (7 eq [xS, t0, dxS, yS, dyS, zS, dzS, t1, t2] 9 unk)
    // 7: yS + t2*dyS = y2 + t2*dy2 (8 eq [xS, t0, dxS, yS, dyS, zS, dzS, t1, t2] 9 unk)
    // 8: zS + t2*dzS = z2 + t2*dz2 (9 eq [xS, t0, dxS, yS, dyS, zS, dzS, t1, t2] 9 unk)
    // !! 9 eq, 9 unk !!
    // 0:
    // t0*dxS - t0*dx0 = x0 - xS
    // t0*(dxS - dx0) = x0 - xS
    // t0 = (x0 - xS) / (dxS - dx0)
    // 1:
    // yS + t0*dyS = y0 + t0*dy0
    // yS + dyS * (x0 - xS) / (dxS - dx0) = y0 + dy0 * (x0 - xS) / (dxS - dx0)
    // :puke:
    //
    // Let's try adding a new var (dxSN = dxS * tN)
    // 0: xS + dxS0 = x0 + dx0 * t0 => 3 new unknowns [xS, dxS0, t0]
    // 1: yS + dyS0 = y0 + dy0 * t0 => 2 new unknowns [yS, dyS0]
    // 2: zS + dzS0 = z0 + dz0 * t0 => 2 new unknowns [zS, dzS0]
    //
    // 3: xS + dxS1 = x1 + dx1 * t1 => 2 new unknowns [dxS1, t1]
    // 4: yS + dyS1 = y1 + dy1 * t1 => 1 new unknowns [dyS1]
    // 5: zS + dzS1 = z1 + dz1 * t1 => 1 new unknowns [dzS1]
    // ... (3 + 4N vars, 3N equations)
    // 3N+0: dxS0 = dxS * t0
    // 3N+1: dyS0 = dyS * t0
    // 3N+2: dzS0 = dzS * t0
    // ... (3N more equations)
    Box::new("todo")
}

type N = i128;

fn solve1(input: &str, min: (N, N), max: (N, N), vis: bool) -> usize {
    let stones = parse(&input);
    let mut res = 0;
    for (i, a) in stones.iter().enumerate() {
        for b in &stones[i + 1..] {
            if vis {
                println!();
                println!("Hailstone A: {a}");
                println!("Hailstone B: {b}");
            }
            if intersects(a, b, min, max, vis) {
                res += 1;
            }
        }
    }
    res
}

fn intersects(a: &Hailstone, b: &Hailstone, min: (N, N), max: (N, N), vis: bool) -> bool {
    let a_line = slope_intercept(a);
    let b_line = slope_intercept(b);
    let x_range = min.0..=max.0;
    let y_range = min.1..=max.1;
    match intersection(a_line, b_line) {
        None => {
            if vis {
                println!("Hailstones' paths do not intersect");
            }
            false
        }
        Some((x, y))
            if !x_range.contains(&x.to_integer()) || !y_range.contains(&y.to_integer()) =>
        {
            if vis {
                println!("Hailstones' paths intersect outside the area (at x={x}, y={y})")
            }
            false
        }
        Some((x, y)) if is_past(a, (x, y)) => {
            if vis {
                println!("Hailstones' paths intersect in the past for A (at x={x}, y={y})");
            }
            false
        }
        Some((x, y)) if is_past(b, (x, y)) => {
            if vis {
                println!("Hailstones' paths intersect in the past for B (at x={x}, y={y})");
            }
            false
        }
        Some((x, y)) => {
            if vis {
                println!("Hailstones' paths intersect inside the area (at x={x}, y={y})");
            }
            true
        }
    }
}

fn is_past(stone: &Hailstone, point: (Ratio<N>, Ratio<N>)) -> bool {
    // (x0, y0) vs (x, y)
    let x0 = stone.position.0;
    let dx = stone.velocity.0;
    let xp = point.0;
    let t = (xp - x0) / dx;
    //println!("TODO: {stone} => ({},{}) AT t={t}", point.0, point.1);
    t < 0.into()
}

fn intersection(a: SlopeIntercept, b: SlopeIntercept) -> Option<(Ratio<N>, Ratio<N>)> {
    if a.slope == b.slope {
        if a.intercept == b.intercept {
            panic!("lines are the same! This is unexpected!");
        }
        return None;
    }
    // y = m1 * x + b1
    // y = m2 * x + b2
    // 0 = (m1 - m2) * x + b1 - b2
    // (b2 - b1) / (m1 - m2) = x
    let x = (b.intercept - a.intercept) / (a.slope - b.slope);
    let y1 = a.slope * x + a.intercept;
    let y2 = b.slope * x + b.intercept;
    assert_eq!(y1, y2);
    Some((x, y1))
}

struct SlopeIntercept {
    slope: Ratio<N>,
    intercept: Ratio<N>,
}

fn slope_intercept(stone: &Hailstone) -> SlopeIntercept {
    // have (x,y), (dx,dy) want y=mx+b
    // m = dy/dx
    // b = y - (dy/dx)*x
    let Hailstone {
        position: (x, y, _),
        velocity: (dx, dy, _),
    } = stone;
    let slope = Ratio::new(*dy, *dx);
    let intercept = slope * -x + y;
    //println!("TODO ({x},{y}) +({dx},{dy}) ==> y = {slope} * x + {intercept}");
    SlopeIntercept { slope, intercept }
}

#[derive(Debug)]
struct Hailstone {
    position: (N, N, N),
    velocity: (N, N, N),
}

impl Display for Hailstone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {} @ {}, {}, {}",
            self.position.0,
            self.position.1,
            self.position.2,
            self.velocity.0,
            self.velocity.1,
            self.velocity.2
        )
    }
}

fn parse(input: &str) -> Vec<Hailstone> {
    fn parse_tuple(s: &str) -> (N, N, N) {
        let mut parts = s
            .split(',')
            .map(str::trim)
            .map(str::parse)
            .map(Result::unwrap);
        let x = parts.next().unwrap();
        let y = parts.next().unwrap();
        let z = parts.next().unwrap();
        (x, y, z)
    }

    fn parse_hailstone(line: &str) -> Hailstone {
        let (pos, vel) = line.trim().split_once('@').unwrap();
        let position = parse_tuple(pos);
        let velocity = parse_tuple(vel);
        Hailstone { position, velocity }
    }

    input.lines().map(parse_hailstone).collect()
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str = r"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn part1() {
        assert_eq!(2, super::solve1(TEST_INPUT, (7, 7), (27, 27), true));
    }

    //crate::test::aoc_test!(part2, TEST_INPUT, 47);
}
