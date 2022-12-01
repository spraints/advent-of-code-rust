use std::fmt::Display;

pub fn dotest<F, R>(expected: R, input: &str, f: F)
where
    F: FnOnce(String) -> anyhow::Result<String>,
    R: Display,
{
    assert_eq!(format!("{}", expected), f(input.to_string()).unwrap());
}
