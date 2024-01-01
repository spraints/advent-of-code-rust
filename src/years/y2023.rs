mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d20;
mod d21;
mod d23;
mod d24;
mod d25;

use crate::registry::aoc_part;
use crate::registry::slow_aoc_part;
use crate::solutionset::SolutionSet;

/* Hints
 * Day 6: abc formula (quadratic equation solving)
 * Day 10: flood fill/BFS. Jordan curve Theorem (basically: crossing a simple curve means swapping between inside/outside)
 * Day 12: dynamic programming (with recursion + memoization as a special top-down implementation of DP)
 * Day 14: cycle detection using solution hashes
 * Day 17: path finding in a graph (Dijkstra)
 * Day 18: Shoelace Theorem (and many other techniques - see above)
 * Day 19: hypercuboid intersections
 * Day 20: least common multiple (lcm)
 * Day 21: quadratic formula extrapolation (see Day 9 for a simple example).
 * Day 22: "shaving" a directed graph (iteratively removing leaves - with the proper data structures this can be done in linear time).
 * Day 23: BFS, recursive pseudo DFS
 * Day 24: linear equation solving (Gaussian elimination), 2D / 3D vector math (dot product, cross product, normal vectors)
 * Day 25: Min-Cut Max-Flow (e.g. Ford-Fulkerson)
 * from https://www.reddit.com/r/adventofcode/comments/18ufl0o/algorithms_for_each_day/
 * https://www.reddit.com/r/adventofcode/comments/18ufl0o/comment/kfkkxjj/?utm_source=reddit&utm_medium=web2x&context=3
 */

pub fn register<S: SolutionSet>(s: &mut S) {
    s.add(2023, 1, 1, None, d01::part1);
    s.add(2023, 1, 1, Some("orig"), d01::orig_part1);
    s.add(2023, 1, 2, None, d01::part2);
    s.add(2023, 1, 2, Some("orig"), d01::orig_part2);
    s.add(2023, 2, 1, None, d02::part1);
    s.add(2023, 2, 1, Some("regexp"), d02::part1_regexp);
    s.add(2023, 2, 1, Some("itertools"), d02::part1_iterate);
    s.add(2023, 2, 2, None, d02::part2);
    s.add(2023, 2, 2, Some("regexp"), d02::part2_regexp);
    s.add(2023, 2, 2, Some("itertools"), d02::part2_iterate);
    aoc_part!(s, 2023, 3, 1, d03::part1);
    aoc_part!(s, 2023, 3, 2, d03::part2);
    aoc_part!(s, 2023, 3, 2, d03::part2_from_oliver);
    aoc_part!(s, 2023, 4, 1, d04::part1);
    aoc_part!(s, 2023, 4, 2, d04::part2);
    aoc_part!(s, 2023, 5, 1, d05::part1);
    aoc_part!(s, 2023, 5, 2, d05::part2);
    aoc_part!(s, 2023, 6, 1, d06::part1);
    aoc_part!(s, 2023, 6, 2, d06::part2);
    aoc_part!(s, 2023, 7, 1, d07::part1);
    aoc_part!(s, 2023, 7, 2, d07::part2);
    aoc_part!(s, 2023, 8, 1, d08::part1);
    aoc_part!(s, 2023, 8, 2, d08::part2);
    aoc_part!(s, 2023, 9, 1, d09::part1);
    aoc_part!(s, 2023, 9, 2, d09::part2);
    aoc_part!(s, 2023, 10, 1, d10::part1);
    aoc_part!(s, 2023, 10, 2, d10::part2);
    aoc_part!(s, 2023, 12, 1, d12::part1);
    aoc_part!(s, 2023, 12, 2, d12::part2);
    aoc_part!(s, 2023, 13, 1, d13::part1);
    aoc_part!(s, 2023, 13, 2, d13::part2);
    aoc_part!(s, 2023, 11, 1, d11::part1);
    aoc_part!(s, 2023, 11, 2, d11::part2);
    aoc_part!(s, 2023, 14, 1, d14::part1);
    aoc_part!(s, 2023, 14, 2, d14::part2);
    aoc_part!(s, 2023, 15, 1, d15::part1);
    aoc_part!(s, 2023, 15, 2, d15::part2);
    aoc_part!(s, 2023, 17, 1, d17::part1);
    aoc_part!(s, 2023, 17, 2, d17::part2);
    aoc_part!(s, 2023, 18, 1, d18::part1);
    aoc_part!(s, 2023, 18, 2, d18::part2);
    aoc_part!(s, 2023, 19, 1, d19::part1);
    aoc_part!(s, 2023, 19, 2, d19::part2);
    aoc_part!(s, 2023, 20, 1, d20::part1);
    aoc_part!(s, 2023, 20, 2, d20::part2);
    aoc_part!(s, 2023, 21, 1, d21::part1);
    slow_aoc_part!(s, 2023, 21, 2, d21::part2);
    // day 22:
    aoc_part!(s, 2023, 23, 1, d23::part1);
    slow_aoc_part!(s, 2023, 23, 2, d23::part2);
    aoc_part!(s, 2023, 24, 1, d24::part1);
    aoc_part!(s, 2023, 24, 2, d24::part2);
    slow_aoc_part!(s, 2023, 25, 1, d25::part1);
    aoc_part!(s, 2023, 25, 2, d25::part2);
    aoc_part!(s, 2023, 16, 1, d16::part1);
    aoc_part!(s, 2023, 16, 2, d16::part2);
}
