mod d01;
mod d02;

use crate::solutionset::SolutionSet;

pub fn register<S: SolutionSet>(s: &mut S) {
    s.add(2023, 1, 1, None, d01::part1);
    s.add(2023, 1, 1, Some("orig"), d01::orig_part1);
    s.add(2023, 1, 2, None, d01::part2);
    s.add(2023, 1, 2, Some("orig"), d01::orig_part2);
    s.add(2023, 2, 1, None, d02::part1);
    s.add(2023, 2, 2, None, d02::part2);
}
