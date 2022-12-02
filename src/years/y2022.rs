mod d1;
mod d2;
mod d3;

use crate::solutionset::SolutionSet;

pub fn register<S: SolutionSet>(s: &mut S) {
    s.add(2022, 1, 1, None, d1::part1);
    s.add(2022, 1, 2, None, d1::part2);

    s.add(2022, 2, 1, None, d2::part1);
    s.add(2022, 2, 2, None, d2::part2);

    s.add(2022, 3, 1, None, d3::part1);
    s.add(2022, 3, 2, None, d3::part2);
}
