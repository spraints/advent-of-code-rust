mod d01;
mod d02;

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
}
