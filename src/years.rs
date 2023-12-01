use crate::solutionset::SolutionSet;

pub mod y2021;
pub mod y2022;
pub mod y2023;

pub fn register<S: SolutionSet>(runner: &mut S) {
    y2021::register(runner);
    y2022::register(runner);
    y2023::register(runner);
}
