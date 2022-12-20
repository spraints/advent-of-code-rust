mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;

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

use crate::solutionset::SolutionSet;

pub fn register<S: SolutionSet>(s: &mut S) {
    s.add(2022, 20, 1, None, d20::part1);
    s.add(2022, 20, 2, None, d20::part2);

    s.add(2022, 19, 1, None, d19::part1);
    s.add(2022, 19, 2, None, d19::part2);

    s.add(2022, 18, 1, None, d18::part1);
    s.add(2022, 18, 2, None, d18::part2);

    s.add(2022, 17, 1, None, d17::part1);
    s.add(2022, 17, 2, None, d17::part2);

    s.add(2022, 16, 1, None, d16::part1);
    s.add(2022, 16, 2, None, d16::part2);
    s.add(2022, 16, 1, Some("new"), d16::part1_new);
    s.add(2022, 16, 2, Some("new"), d16::part2_new);

    s.add(2022, 15, 1, None, d15::part1);
    s.add(2022, 15, 2, None, d15::part2);

    s.add(2022, 14, 1, None, d14::part1);
    s.add(2022, 14, 2, None, d14::part2);

    s.add(2022, 13, 1, None, d13::part1);
    s.add(2022, 13, 2, None, d13::part2);
    s.add(2022, 13, 2, Some("no sort"), d13::part2_no_sort);

    s.add(2022, 12, 1, None, d12::part1);
    s.add(2022, 12, 2, None, d12::part2);

    s.add(2022, 11, 1, None, d11::part1);
    s.add(2022, 11, 2, None, d11::part2);

    s.add(2022, 10, 1, None, d10::part1);
    s.add(2022, 10, 2, None, d10::part2);

    s.add(2022, 9, 1, None, d9::part1);
    s.add(2022, 9, 2, None, d9::part2);

    s.add(2022, 8, 1, None, d8::part1);
    s.add(2022, 8, 2, None, d8::part2);

    s.add(2022, 7, 1, None, d7::part1);
    s.add(2022, 7, 2, None, d7::part2);

    s.add(2022, 6, 1, None, d6::part1);
    s.add(2022, 6, 2, None, d6::part2);
    s.add(2022, 6, 1, Some("fewer comparisons"), d6::part1_alt);
    s.add(2022, 6, 2, Some("fewer comparisons"), d6::part2_alt);

    //

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
    s.add(2022, 3, 2, Some("with bytes"), d3::part2_bytes);

    s.add(2022, 4, 1, None, d4::part1);
    s.add(2022, 4, 2, None, d4::part2);

    s.add(2022, 5, 1, None, d5::part1);
    s.add(
        2022,
        5,
        1,
        Some("transpose during parse"),
        d5::part1_transpose,
    );
    s.add(2022, 5, 2, None, d5::part2);
    s.add(2022, 5, 2, Some("with fewer allocations"), d5::part2_slices);
}
