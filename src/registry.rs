macro_rules! aoc_part {
    ($s:expr, $year:expr, $day:expr, $part:expr, $f:expr) => {
        $s.add($year, $day, $part, Some(stringify!($f)), $f);
    };
}

pub(crate) use aoc_part;
