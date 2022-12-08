use std::{collections::HashSet, fmt::Display};

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let heights = parse(&input);
    if vis {
        for row in &heights {
            println!("{:?}", row);
        }
    }
    let mut seen = HashSet::new();
    let rows = heights.len();
    let cols = heights[0].len();
    for row in 0..rows {
        let mut max_height_from_left = -1;
        let mut max_height_from_right = -1;
        for col in 0..cols {
            let h = heights[row][col];
            if h > max_height_from_left {
                if vis {
                    println!("can see > row={} col={} val={}", row, col, h);
                }
                seen.insert((row, col));
                max_height_from_left = h
            }
            let h = heights[row][cols - col - 1];
            if h > max_height_from_right {
                if vis {
                    println!("can see < row={} col={} val={}", row, cols - col - 1, h);
                }
                seen.insert((row, cols - col - 1));
                max_height_from_right = h
            }
        }
    }
    for col in 0..cols {
        let mut max_height_from_top = -1;
        let mut max_height_from_bottom = -1;
        for row in 0..rows {
            let h = heights[row][col];
            if h > max_height_from_top {
                if vis {
                    println!("can see v row={} col={} val={}", row, col, h);
                }
                seen.insert((row, col));
                max_height_from_top = h
            }
            let h = heights[rows - row - 1][col];
            if h > max_height_from_bottom {
                if vis {
                    println!("can see ^ row={} col={} val={}", rows - row - 1, col, h);
                }
                seen.insert((rows - row - 1, col));
                max_height_from_bottom = h
            }
        }
    }
    Box::new(seen.len())
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    let heights = parse(&input);
    let rows = heights.len();
    let cols = heights[0].len();
    let mut max_score = 0;
    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            let score = scenic_score(&heights, row, col);
            if vis {
                println!("({},{})={} => score={}", row, col, heights[row][col], score);
            }
            if score > max_score {
                max_score = score;
            }
        }
    }
    Box::new(max_score)
}

fn scenic_score(heights: &Vec<Vec<i8>>, row: usize, col: usize) -> usize {
    vd(&heights, (row, col), (1, 0))
        * vd(&heights, (row, col), (-1, 0))
        * vd(&heights, (row, col), (0, 1))
        * vd(&heights, (row, col), (0, -1))
}
fn vd(heights: &Vec<Vec<i8>>, pos: (usize, usize), off: (isize, isize)) -> usize {
    let (mut row, mut col) = pos;
    let (roff, coff) = off;
    let h = heights[row][col];
    let mut dist = 0;
    loop {
        row = (row as isize + roff) as usize;
        col = (col as isize + coff) as usize;
        match heights.get(row) {
            None => break,
            Some(row) => match row.get(col) {
                None => break,
                Some(t) => {
                    dist += 1;
                    if *t >= h {
                        break;
                    }
                }
            },
        };
    }
    dist
}

fn parse(input: &str) -> Vec<Vec<i8>> {
    fn parse_line(line: &str) -> Vec<i8> {
        line.as_bytes()
            .into_iter()
            .map(|b| (b - b'0') as i8)
            .collect()
    }
    input.lines().map(parse_line).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    crate::test::aoc_test!(example, r"30373
25512
65332
33549
35390",
        part1 => 21,
        part2 => 8);
}
