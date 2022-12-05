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
