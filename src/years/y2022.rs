mod d1;
mod d2;
mod d3;
mod d4;

use crate::solutionset::SolutionSet;

pub fn register<S: SolutionSet>(s: &mut S) {
    s.add(2022, 1, 1, None, d1::part1);
    s.add(2022, 1, 2, None, d1::part2);

    s.add(2022, 2, 1, None, d2::part1);
    s.add(2022, 2, 2, None, d2::part2);
    s.add(2022, 2, 1, Some("mods"), d2::part1alt);
    s.add(2022, 2, 2, Some("mods"), d2::part2alt);
    s.add(2022, 2, 1, Some("no split"), d2::part1_nosplit);

    s.add(2022, 3, 1, None, d3::part1);
    s.add(2022, 3, 2, None, d3::part2);
    s.add(2022, 3, 1, Some("with set"), d3::part1_set);
    s.add(2022, 3, 2, Some("with set"), d3::part2_set);
    s.add(2022, 3, 2, Some("with fewer sets"), d3::part2_set2);

    s.add(2022, 4, 1, None, d4::part1);
    s.add(2022, 4, 2, None, d4::part2);
}
