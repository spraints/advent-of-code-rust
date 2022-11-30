use crate::solutionset::SolutionSet;

pub fn register<S: SolutionSet>(s: S) {
    s.add(2021, 1, 1, day1part1);
}

fn day1part1(input: String) -> anyhow::Result<String> {
    todo!()
}
