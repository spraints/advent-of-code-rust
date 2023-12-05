use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;

// Handy references:
// - https://doc.rust-lang.org/std/iter/trait.Iterator.html
// - https://docs.rs/itertools/0.8.2/itertools/trait.Itertools.html
// - https://docs.rs/regex/latest/regex/struct.Regex.html

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let data = parse(&input);
    if vis {
        let keys: Vec<&String> = data.maps.keys().collect();
        println!("maps: {keys:?}");
    }
    let locations = data.seeds.iter().map(|s| map(*s, &data.maps, vis));
    Box::new(locations.min().unwrap())
}

fn map(seed: u128, maps: &HashMap<String, Map>, vis: bool) -> u128 {
    if vis {
        print!("seed {seed} -> ");
    }
    let soil = maps["seed-to-soil"].map(seed);
    if vis {
        print!("soil {soil} -> ");
    }
    let fertilizer = maps["soil-to-fertilizer"].map(soil);
    if vis {
        print!("fertilizer {fertilizer} -> ");
    }
    let water = maps["fertilizer-to-water"].map(fertilizer);
    if vis {
        print!("water {water} -> ");
    }
    let light = maps["water-to-light"].map(water);
    if vis {
        print!("light {light} -> ");
    }
    let temperature = maps["light-to-temperature"].map(light);
    if vis {
        print!("temperature {temperature} -> ");
    }
    let humidity = maps["temperature-to-humidity"].map(temperature);
    if vis {
        print!("humidity {humidity} -> ");
    }
    let location = maps["humidity-to-location"].map(humidity);
    if vis {
        println!("location {location}");
    }
    location
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    let data = parse(&input);
    let locations = data
        .seeds
        .iter()
        .copied()
        .tuples()
        .map(|s| map2(s, &data.maps, vis));
    Box::new(locations.min().unwrap())
}

fn map2(
    seed_range: (u128, u128), /*(start,len)*/
    maps: &HashMap<String, Map>,
    vis: bool,
) -> u128 {
    let stages = [
        "seed",
        "soil",
        "fertilizer",
        "water",
        "light",
        "temperature",
        "humidity",
        "location",
    ];

    let mut ranges = vec![seed_range];
    if vis {
        println!("seed {ranges:?}");
    }

    for fromto in stages.windows(2) {
        let from = fromto[0];
        let to = fromto[1];
        let stage = format!("{}-to-{}", from, to);
        let map = &maps[&stage];
        let mut new_ranges = Vec::new();
        for r in ranges {
            map.map_range(&mut new_ranges, r);
        }
        if vis {
            println!("-> {to}: {new_ranges:?}");
        }
        ranges = new_ranges;
    }

    ranges.into_iter().map(|(a, _)| a).min().unwrap()
}

fn parse(input: &str) -> Data {
    let mut lines = input.lines();

    let (s, seeds) = lines.next().unwrap().split_once(':').unwrap();
    assert_eq!(s, "seeds");
    let seeds = seeds
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut maps = HashMap::new();

    let map_start = regex::Regex::new("(.*) map:").unwrap();
    let mut map_name = "";
    let mut ranges: Vec<Range> = Vec::new();
    for line in lines {
        match map_start.captures(line) {
            Some(c) => {
                if ranges.len() > 0 {
                    ranges.sort_by_key(|r| r.src);
                    maps.insert(map_name.to_owned(), Map { ranges });
                    ranges = Vec::new();
                }
                map_name = c.get(1).unwrap().as_str();
            }
            None => {
                if !line.trim().is_empty() {
                    let range: Vec<u128> =
                        line.trim().split(' ').map(|s| s.parse().unwrap()).collect();
                    assert_eq!(3, range.len());
                    ranges.push(Range {
                        dest: range[0],
                        src: range[1],
                        range: range[2],
                    });
                }
            }
        }
    }
    if ranges.len() > 0 {
        ranges.sort_by_key(|r| r.src);
        maps.insert(map_name.to_owned(), Map { ranges });
    }

    Data { seeds, maps }
}

struct Data {
    seeds: Vec<u128>,
    maps: HashMap<String, Map>,
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn map(&self, source: u128) -> u128 {
        match self.find(source) {
            None => source,
            Some(range) => range.dest + source - range.src,
        }
    }

    fn find(&self, source: u128) -> Option<&Range> {
        for range in &self.ranges {
            if range.src <= source && range.src + range.range > source {
                return Some(range);
            }
        }
        None
    }

    fn map_range(&self, new_ranges: &mut Vec<(u128, u128)>, range: (u128, u128)) {
        let (mut start, mut len) = range;
        for r in &self.ranges {
            //println!("({start}, {len}) vs {r:?}");
            if r.src > start {
                // Map (start..?) to (start..?).
                let diff = r.src - start;
                if diff >= len {
                    //println!("-> ({start},{len}) (done)");
                    new_ranges.push((start, len));
                    return;
                } else {
                    //println!("-> ({start},{diff})");
                    new_ranges.push((start, diff));
                    start = r.src;
                    len -= diff;
                }
            }
            if r.src + r.range > start {
                // Map (start..?) to (r.dest+offset..?).
                let rend = r.src + r.range;
                let inputend = start + len;
                let newstart = r.dest + (start - r.src);
                if inputend <= rend {
                    //println!("-> ({newstart},{len}) (done)");
                    new_ranges.push((newstart, len));
                    return;
                } else {
                    let newlen = rend - start;
                    //println!("-> ({newstart},{newlen})");
                    new_ranges.push((newstart, newlen));
                    start = rend;
                    len -= newlen;
                }
            }
        }
        new_ranges.push((start, len));
    }
}

#[derive(Debug)]
struct Range {
    dest: u128,
    src: u128,
    range: u128,
}

//impl PartialOrd for Range {
//    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//        self.src.partial_cmp(&other.src)
//    }
//}
//
//impl Ord for Range {
//    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//        self.src.cmp(&other.src)
//    }
//}
//
//impl Eq for Range {}
//
//impl PartialEq for Range {
//    fn eq(&self, other: &Self) -> bool {
//        self.src.eq(&other.src)
//    }
//}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    crate::test::aoc_test!(part1, TEST_INPUT, 35);
    crate::test::aoc_test!(part2, TEST_INPUT, 46);
}
