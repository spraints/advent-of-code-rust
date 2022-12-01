pub trait SolutionSet {
    fn add<F>(self, year: i32, day: u32, part: u8, f: F)
    where
        F: Fn(String) -> anyhow::Result<String> + 'static;
}
