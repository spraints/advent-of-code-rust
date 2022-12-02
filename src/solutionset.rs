use std::fmt::Display;

pub trait SolutionSet {
    fn add<F>(&mut self, year: i32, day: u32, part: u8, label: Option<&'static str>, f: F)
    where
        F: Fn(String) -> anyhow::Result<Box<dyn Display>> + 'static;
}
