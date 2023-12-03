mod d01;
mod d02;
mod d03;

use crate::registry::aoc_part;
use crate::solutionset::SolutionSet;

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
}
