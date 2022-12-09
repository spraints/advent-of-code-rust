use std::{collections::HashSet, fmt::Display};

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let (mut hx, mut hy) = (0, 0);
    let (mut tx, mut ty) = (0, 0);
    let mut visited = HashSet::new();
    visited.insert((tx, ty));
    for line in input.lines() {
        let ((dx, dy), steps) = parse(line);
        for _ in 0..steps {
            hx += dx;
            hy += dy;
            if ty - hy > 1 {
                tx = hx;
                ty -= 1;
            } else if hy - ty > 1 {
                tx = hx;
                ty += 1;
            } else if hx - tx > 1 {
                ty = hy;
                tx += 1;
            } else if tx - hx > 1 {
                ty = hy;
                tx -= 1;
            }
            visited.insert((tx, ty));
            if vis {
                println!("H=({},{}), T=({},{})", hx, hy, tx, ty);
            }
        }
    }
    Box::new(visited.len())
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    let mut positions = vec![(0, 0); 10];
    let mut visited = HashSet::new();
    visited.insert((0, 0));
    for line in input.lines() {
        let ((dx, dy), steps) = parse(line);
        for _ in 0..steps {
            let (mut hx, mut hy) = positions[0].clone();
            println!("head: ({},{}) + ({},{})", hx, hy, dx, dy);
            hx += dx;
            hy += dy;
            positions[0] = (hx, hy);
            for i in 1..10 {
                let (mut tx, mut ty) = positions[i].clone();
                if ty - hy > 1 {
                    if vis {
                        println!(" [{}] ({},{}) catch up to ({},{})", i, tx, ty, hx, hy);
                    }
                    if hx > tx {
                        tx += 1
                    } else if tx > hx {
                        tx -= 1
                    }
                    ty -= 1;
                } else if hy - ty > 1 {
                    if vis {
                        println!(" [{}] ({},{}) catch up to ({},{})", i, tx, ty, hx, hy);
                    }
                    if hx > tx {
                        tx += 1
                    } else if tx > hx {
                        tx -= 1
                    }
                    ty += 1;
                } else if hx - tx > 1 {
                    if vis {
                        println!(" [{}] ({},{}) catch up to ({},{})", i, tx, ty, hx, hy);
                    }
                    if ty > hy {
                        ty -= 1;
                    } else if hy > ty {
                        ty += 1;
                    }
                    tx += 1;
                } else if tx - hx > 1 {
                    if vis {
                        println!(" [{}] ({},{}) catch up to ({},{})", i, tx, ty, hx, hy);
                    }
                    if ty > hy {
                        ty -= 1;
                    } else if hy > ty {
                        ty += 1;
                    }
                    tx -= 1;
                }
                if vis {
                    println!("  [{}] ({},{})", i, tx, ty);
                }
                positions[i] = (tx, ty);
                hx = tx;
                hy = ty;
            }
            visited.insert(positions[9].clone());
        }
        if vis {
            println!("{} => {:?}", line, positions);
            /*
            let xs: Vec<isize> = positions.iter().map(|(x, _)| x).copied().collect();
            let ys: Vec<isize> = positions.iter().map(|(_, y)| y).copied().collect();
            let minx = *xs.iter().min().unwrap();
            let maxx = *xs.iter().max().unwrap();
            let miny = *ys.iter().min().unwrap();
            let maxy = *ys.iter().max().unwrap();
            let mut grid = vec![
                vec!['.'; (maxy - miny + 1).try_into().unwrap()];
                (maxx - minx + 1).try_into().unwrap()
            ];
            for (i, (x, y)) in positions.iter().enumerate() {
                let label = match i as u32 {
                    0 => 'H',
                    n => unsafe { char::from_u32_unchecked(30 + n) },
                };
                let line: &mut Vec<char> = &mut grid[(x - minx) as usize];
                line[(y - miny) as usize] = label;
            }
            for y in (miny..maxy).rev() {
                for x in minx..maxx {
                    print!("{}", grid[(x - minx) as usize][(y - miny) as usize]);
                }
                println!();
            }
            */
        }
    }
    // 2528 is too high
    Box::new(visited.len())
}

fn parse(line: &str) -> ((isize, isize), usize) {
    let (dir, steps) = line.split_once(' ').unwrap();
    (
        match dir {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => unreachable!(),
        },
        steps.parse().unwrap(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    crate::test::aoc_test!(example, r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
        part1 => 13,
        part2 => 1);

    crate::test::aoc_test!(longer, r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20", part2 => 35);
}
