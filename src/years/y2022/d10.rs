use std::fmt::Display;

pub fn part1(input: String, _vis: bool) -> Box<dyn Display> {
    let mut cycles = 0;
    let mut x = 1;
    let mut signal = 0;
    for inst in input.lines() {
        let (dur, dx): (u8, isize) = match &inst[0..4] {
            "addx" => (2, inst[5..].parse().unwrap()),
            "noop" => (1, 0),
            _ => unreachable!(),
        };
        for _ in 0..dur {
            cycles += 1;
            if cycles % 40 == 20 {
                signal += cycles * x;
            }
        }
        x += dx;
    }
    Box::new(signal)
}

pub fn part2(input: String, _vis: bool) -> Box<dyn Display> {
    let mut cycles = 0;
    let mut sprite_pos = 0;
    let mut res = "\n".to_string();
    for inst in input.lines() {
        let (dur, dx): (u8, isize) = match &inst[0..4] {
            "addx" => (2, inst[5..].parse().unwrap()),
            "noop" => (1, 0),
            _ => unreachable!(),
        };
        for _ in 0..dur {
            let pixel_pos = cycles % 40;
            res.push(if pixel_pos >= sprite_pos && pixel_pos < sprite_pos + 3 {
                '#'
            } else {
                '.'
            });
            cycles += 1;
            if cycles % 40 == 0 {
                res.push('\n');
            }
        }
        sprite_pos += dx;
    }
    Box::new(res)
}

#[cfg(test)]
mod test {
    use super::*;

    crate::test::aoc_tests!(example, EX,
        part1 => 13140,
        part2 => r"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
");

    const EX: &str = r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
}
