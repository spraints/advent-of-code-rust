use std::fmt::Display;

use crate::input::get_input;

pub fn dotest<F, R>(expected: R, input: &str, f: F)
where
    F: FnOnce(String) -> anyhow::Result<String>,
    R: Display,
{
    assert_eq!(format!("{}", expected), f(input.to_string()).unwrap());
}

pub fn dotestinput<F, R>(expected: R, year: i32, day: u32, f: F)
where
    F: FnOnce(String) -> anyhow::Result<String>,
    R: Display,
{
    assert_eq!(
        format!("{}", expected),
        f(get_input(year, day, None).unwrap()).unwrap()
    );
}
