pub fn part1(input: String) -> anyhow::Result<String> {
    Ok(format!(
        "{}",
        input.split("\n\n").map(sum_group).max().unwrap()
    ))
    //    let values: Vec<u32> = input.lines().map(|l| l.trim().parse().unwrap()).collect();
    //    Ok(format!(
    //        "{}",
    //        values.windows(2).filter(|x| x[0] < x[1]).count()
    //    ))
}

fn sum_group(group: &str) -> u32 {
    group
        .lines()
        .map(|s| s.trim().parse::<u32>().unwrap())
        .sum()
}

pub fn part2(input: String) -> anyhow::Result<String> {
    let mut groups: Vec<u32> = input.split("\n\n").map(sum_group).collect();
    groups.sort();
    groups.reverse();
    Ok(format!("{}", groups.into_iter().take(3).sum::<u32>()))
    //    let values: Vec<u32> = input.lines().map(|l| l.trim().parse().unwrap()).collect();
    //    let sums: Vec<u32> = values.windows(3).map(|x| x[0] + x[1] + x[2]).collect();
    //    Ok(format!(
    //        "{}",
    //        sums.windows(2).filter(|x| x[0] < x[1]).count()
    //    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::dotest;

    #[test]
    fn part1_example() {
        dotest(
            24000,
            r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
            part1,
        );
    }

    #[test]
    fn part2_example() {
        dotest(
            45000,
            r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000",
            part2,
        );
    }
}
