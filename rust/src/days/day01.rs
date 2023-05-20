use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day01.txt").unwrap();

    let result = input
        .split("\n\n")
        .map(|group| {
            group
                .split("\n")
                .map(|e| e.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();

    (Solution::from(result), Solution::from(0))
}
