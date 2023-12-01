mod d01;

use crate::solutionset::SolutionSet;

pub fn register<S: SolutionSet>(s: &mut S) {
    s.add(2023, 1, 1, None, d01::part1);
    s.add(2023, 1, 2, None, d01::part2);
}
