use std::{cmp::Ordering, collections::BinaryHeap, fmt::Display};

pub fn part1(input: String, vis: bool) -> Box<dyn Display> {
    let map: Vec<Vec<u8>> = input.lines().map(|s| s.as_bytes().to_vec()).collect();
    Box::new(shortest_path(map, vis))
}

pub fn part2(input: String, vis: bool) -> Box<dyn Display> {
    let map: Vec<Vec<u8>> = input.lines().map(|s| s.as_bytes().to_vec()).collect();
    Box::new(shortest_path2(map, vis))
}

fn shortest_path(map: Vec<Vec<u8>>, vis: bool) -> usize {
    let mut dist: Vec<Vec<usize>> = map
        .iter()
        .map(|row| row.iter().map(|_| usize::MAX).collect())
        .collect();
    let mut heap = BinaryHeap::new();
    let (row, col) = find_start(&map);
    dist[row][col] = 0;
    heap.push(State { row, col, cost: 0 });
    while let Some(State { cost, row, col }) = heap.pop() {
        if vis {
            println!("{:?}", dist);
            println!("({},{}) @ {}", row, col, cost);
        }
        if map[row][col] == b'E' {
            return cost;
        }
        if cost > dist[row][col] {
            continue;
        }
        let next_cost = cost + 1;
        let cur = map[row][col];
        if row > 0 && can_move(cur, map[row - 1][col]) && next_cost < dist[row - 1][col] {
            if vis {
                println!("can move to ({},{})", row - 1, col);
            }
            dist[row - 1][col] = next_cost;
            heap.push(State {
                cost: next_cost,
                row: row - 1,
                col,
            });
        }
        if col > 0 && can_move(cur, map[row][col - 1]) && next_cost < dist[row][col - 1] {
            if vis {
                println!("can move to ({},{})", row, col - 1);
            }
            dist[row][col - 1] = next_cost;
            heap.push(State {
                cost: next_cost,
                row,
                col: col - 1,
            });
        }
        if row + 1 < map.len() && can_move(cur, map[row + 1][col]) && next_cost < dist[row + 1][col]
        {
            if vis {
                println!("can move to ({},{})", row + 1, col);
            }
            dist[row + 1][col] = next_cost;
            heap.push(State {
                cost: next_cost,
                row: row + 1,
                col,
            });
        }
        if let Some(c) = map[row].get(col + 1) {
            if can_move(cur, *c) && next_cost < dist[row][col + 1] {
                if vis {
                    println!("can move to ({},{})", row, col + 1);
                }
                dist[row][col + 1] = next_cost;
                heap.push(State {
                    cost: next_cost,
                    row,
                    col: col + 1,
                });
            }
        }
    }

    unreachable!()
}

fn shortest_path2(map: Vec<Vec<u8>>, vis: bool) -> usize {
    let mut dist: Vec<Vec<usize>> = map
        .iter()
        .map(|row| row.iter().map(|_| usize::MAX).collect())
        .collect();
    let mut heap = BinaryHeap::new();

    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == b'S' || *c == b'a' {
                dist[i][j] = 0;
                heap.push(State {
                    row: i,
                    col: j,
                    cost: 0,
                });
            }
        }
    }

    while let Some(State { cost, row, col }) = heap.pop() {
        if vis {
            println!("{:?}", dist);
            println!("({},{}) @ {}", row, col, cost);
        }
        if map[row][col] == b'E' {
            return cost;
        }
        if cost > dist[row][col] {
            continue;
        }
        let next_cost = cost + 1;
        let cur = map[row][col];
        if row > 0 && can_move(cur, map[row - 1][col]) && next_cost < dist[row - 1][col] {
            if vis {
                println!("can move to ({},{})", row - 1, col);
            }
            dist[row - 1][col] = next_cost;
            heap.push(State {
                cost: next_cost,
                row: row - 1,
                col,
            });
        }
        if col > 0 && can_move(cur, map[row][col - 1]) && next_cost < dist[row][col - 1] {
            if vis {
                println!("can move to ({},{})", row, col - 1);
            }
            dist[row][col - 1] = next_cost;
            heap.push(State {
                cost: next_cost,
                row,
                col: col - 1,
            });
        }
        if row + 1 < map.len() && can_move(cur, map[row + 1][col]) && next_cost < dist[row + 1][col]
        {
            if vis {
                println!("can move to ({},{})", row + 1, col);
            }
            dist[row + 1][col] = next_cost;
            heap.push(State {
                cost: next_cost,
                row: row + 1,
                col,
            });
        }
        if let Some(c) = map[row].get(col + 1) {
            if can_move(cur, *c) && next_cost < dist[row][col + 1] {
                if vis {
                    println!("can move to ({},{})", row, col + 1);
                }
                dist[row][col + 1] = next_cost;
                heap.push(State {
                    cost: next_cost,
                    row,
                    col: col + 1,
                });
            }
        }
    }

    unreachable!()
}

fn find_start(map: &[Vec<u8>]) -> (usize, usize) {
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == b'S' {
                return (i, j);
            }
        }
    }
    unreachable!()
}

fn can_move(mut from: u8, mut to: u8) -> bool {
    if from == b'S' {
        from = b'a';
    }
    if to == b'E' {
        to = b'z';
    }
    to <= (from + 1)
}

#[derive(Eq, PartialEq)]
struct State {
    row: usize,
    col: usize,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.row.cmp(&self.row))
            .then_with(|| other.col.cmp(&self.col))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    crate::test::aoc_test!(example, r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
        part1 => 31,
        part2 => 29);
}
