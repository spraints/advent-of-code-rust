use std::fmt::Display;

pub fn dotest<F, R>(expected: R, input: &str, f: F)
where
    F: FnOnce(String, bool) -> Box<dyn Display>,
    R: Display,
{
    assert_eq!(
        format!("{}", expected),
        format!("{}", f(input.to_string(), true))
    );
}

pub fn dotest2<F, R>(expected: R, input: &str, f: F, msg: &str)
where
    F: FnOnce(String, bool) -> Box<dyn Display>,
    R: Display,
{
    assert_eq!(
        format!("{}", expected),
        format!("{}", f(input.to_string(), true)),
        "expected vs actual result from {}",
        msg
    );
}

// Test macro examples:
//   aoc_test!(example, "input", part1 => 7, part2 => 14);
// result:
//   #[test]
//   fn example() {
//     dotest(7, "input", part1);
//     dotest(14, "input", part2);
//   }
macro_rules! aoc_test {
    ($name:ident, $input:expr, $($f:ident => $res:expr),*) => {
        #[test]
        fn $name() {
            $(crate::test::dotest2($res, $input, $f, stringify!($f));)*
        }
    };
}

pub(crate) use aoc_test;
