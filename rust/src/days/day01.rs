use crate::{Solution, SolutionPair};
use itertools::Itertools;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day01.txt").unwrap();

    let part1 = input
        .split("\n\n")
        .map(|group| {
            group
                .split("\n")
                .map(|e| e.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();

    let part2: u32 = input
        .split("\n\n")
        .map(|group| {
            group
                .split("\n")
                .map(|e| e.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .sorted()
        .rev()
        .take(3)
        .sum();

    (Solution::from(part1), Solution::from(part2))
}
