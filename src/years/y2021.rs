mod d1;

use crate::solutionset::SolutionSet;

pub fn register<S: SolutionSet>(s: &mut S) {
    s.add(2021, 1, 1, None, d1::part1);
    s.add(2021, 1, 2, None, d1::part2);
}
