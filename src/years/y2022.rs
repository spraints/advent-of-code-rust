mod d1;
mod d2;

use crate::solutionset::SolutionSet;

pub fn register<S: SolutionSet>(s: &mut S) {
    s.add(2022, 1, 1, d1::part1);
    s.add(2022, 1, 2, d1::part2);

    s.add(2022, 2, 1, d2::part1);
    s.add(2022, 2, 2, d2::part2);
}
