macro_rules! aoc_part {
    ($s:expr, $year:expr, $day:expr, $part:expr, $f:expr) => {
        $s.add($year, $day, $part, Some(stringify!($f)), $f);
    };
}

pub(crate) use aoc_part;

macro_rules! slow_aoc_part {
    ($s:expr, $year:expr, $day:expr, $part:expr, $f:expr) => {
        $s.add_slow($year, $day, $part, Some(stringify!($f)), $f);
    };
}

pub(crate) use slow_aoc_part;
