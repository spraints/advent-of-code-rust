use std::fmt::Display;

pub struct Solver {
    pub year: i32,
    pub day: u32,
    pub part: u8,
    pub label: Option<&'static str>,
    pub f: Box<dyn Fn(String, bool) -> Box<dyn Display>>,
    pub slow: bool,
}

impl Solver {
    fn sort_key(&self) -> (i32, u32, u8, &'static str) {
        (self.year, self.day, self.part, self.label.unwrap_or(""))
    }
}

impl Ord for Solver {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.sort_key().cmp(&other.sort_key())
    }
}

impl Eq for Solver {}

impl PartialEq<Solver> for Solver {
    fn eq(&self, other: &Solver) -> bool {
        self.sort_key() == other.sort_key()
    }
}

impl PartialOrd<Solver> for Solver {
    fn partial_cmp(&self, other: &Solver) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
