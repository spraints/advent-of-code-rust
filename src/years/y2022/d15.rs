use std::{collections::BTreeSet, fmt::Display, ops::RangeInclusive};

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    real_part1(input, vis, 2000000)
}

fn real_part1(input: String, vis: bool, y: Coord) -> Box<dyn Display> {
    let sensors = input.lines().map(parse_sensor);
    let mut beacons = BTreeSet::new();
    let mut covered = Vec::new();
    for s in sensors {
        let new = cover(&s, y);
        if vis {
            println!("{:?} => {:?}", s, new);
        }
        covered = merge(covered, new);
        let (bx, by) = s.1;
        if by == y {
            beacons.insert(bx);
        }
    }
    if vis {
        println!("beacons: {:?}", beacons);
        println!("coverage: {:?}", covered);
    }
    let covered: Coord = covered.into_iter().map(|r| 1 + r.end() - r.start()).sum();
    let beacons = beacons.len() as Coord;
    Box::new(covered - beacons)
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    real_part2(input, vis, 4000000)
}

fn real_part2(input: String, vis: bool, max: Coord) -> Box<dyn Display> {
    let (x, y) = real_part2_2(input, vis, max);
    Box::new(x * 4000000 + y)
}

fn real_part2_2(input: String, vis: bool, max: Coord) -> (Coord, Coord) {
    let stepsize = max / 20;
    let sensors: Vec<SensorReading> = input.lines().map(parse_sensor).collect();
    for y in 0..=max {
        if vis && y > 0 && y % stepsize == 0 {
            println!("checking {} ...", y);
        }
        let mut covered = Vec::new();
        for s in &sensors {
            let new = cover(s, y);
            covered = merge(covered, new);
        }
        for c in covered {
            if *c.start() > 0 {
                return (*c.start() - 1, y);
            }
            if *c.end() < max {
                return (*c.end() + 1, y);
            }
        }
    }
    unreachable!()
}

fn merge(
    ranges: Vec<RangeInclusive<Coord>>,
    r: Option<RangeInclusive<Coord>>,
) -> Vec<RangeInclusive<Coord>> {
    fn m(a: RangeInclusive<Coord>, b: RangeInclusive<Coord>) -> RangeInclusive<Coord> {
        let (a1, a2) = a.into_inner();
        let (b1, b2) = b.into_inner();
        (if a1 < b1 { a1 } else { b1 })..=(if a2 > b2 { a2 } else { b2 })
    }
    match (r, ranges.is_empty()) {
        (None, _) => ranges,
        (Some(r), true) => vec![r],
        (Some(mut new), false) => {
            let mut res = Vec::new();
            let mut ranges = ranges.into_iter();
            loop {
                match ranges.next() {
                    // Nothing left, add the new segment and return the result.
                    None => {
                        res.push(new);
                        return res;
                    }
                    // We haven't found where 'new' slots in yet.
                    Some(old) if old.end() < new.start() => res.push(old),
                    // We passed 'new's slot, add new and the rest of the old things and return the
                    // result.
                    Some(old) if old.start() > new.end() => {
                        res.push(new);
                        res.push(old);
                        res.extend(ranges);
                        return res;
                    }
                    Some(old) => new = m(old, new),
                };
            }
        }
    }
}

type Coord = i64;
type SensorReading = ((Coord, Coord), (Coord, Coord));

fn parse_sensor(line: &str) -> SensorReading {
    fn parse_coord(word: &str, prefix: &str, suffix: &str) -> Coord {
        word.strip_prefix(prefix)
            .unwrap()
            .strip_suffix(suffix)
            .unwrap()
            .parse()
            .unwrap()
    }
    let mut words = line.split(' ').skip(2);
    let sensor_x = parse_coord(words.next().unwrap(), "x=", ",");
    let sensor_y = parse_coord(words.next().unwrap(), "y=", ":");
    let mut words = words.skip(4);
    let beacon_x = parse_coord(words.next().unwrap(), "x=", ",");
    let beacon_y = parse_coord(words.next().unwrap(), "y=", "");
    ((sensor_x, sensor_y), (beacon_x, beacon_y))
}

fn cover(s: &SensorReading, y: Coord) -> Option<RangeInclusive<Coord>> {
    let ((sx, sy), (bx, by)) = *s;
    let dist = md((sx, sy), (bx, by));
    if sy + dist < y || sy - dist > y {
        None
    } else {
        let dx = dist - (y - sy).abs();
        Some((sx - dx)..=(sx + dx))
    }
}

fn md(p1: (Coord, Coord), p2: (Coord, Coord)) -> Coord {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

#[cfg(test)]
mod test {
    use super::*;

    fn part1_10(input: String, vis: bool) -> Box<dyn Display> {
        real_part1(input, vis, 10)
    }

    fn part2_20(input: String, vis: bool) -> Box<dyn Display> {
        real_part2(input, vis, 20)
    }

    crate::test::aoc_test!(example, r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3",
        part1_10 => 26,
        part2_20 => 56000011);

    #[test]
    fn merges() {
        fn empty() -> Vec<RangeInclusive<Coord>> {
            Vec::new()
        }
        assert_eq!(empty(), merge(empty(), None));
        assert_eq!(vec![1..=2], merge(empty(), Some(1..=2)));
        assert_eq!(vec![1..=5], merge(vec![1..=4], Some(2..=5)));
    }
}
