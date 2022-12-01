use crate::solutionset::SolutionSet;

pub fn register<S: SolutionSet>(s: S) {
    s.add(2021, 1, 1, day1part1);
}

fn day1part1(input: String) -> anyhow::Result<String> {
    let values: Vec<u32> = input.lines().map(|l| l.trim().parse().unwrap()).collect();
    Ok(format!(
        "{}",
        values.windows(2).filter(|x| x[0] < x[1]).count()
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::dotest;

    #[test]
    fn test_part1_example() {
        dotest(
            7,
            r"199
200
208
210
200
207
240
269
260
263",
            day1part1,
        );
    }
}
