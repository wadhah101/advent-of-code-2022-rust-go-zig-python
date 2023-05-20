use crate::{Solution, SolutionPair};
use itertools::Itertools;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_str(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Invalid shape"),
        }
    }
    fn against(&self, other: &Self) -> u16 {
        if self == other {
            return 3;
        }
        match (self, other) {
            (Self::Rock, Self::Scissors) => 6,
            (Self::Paper, Self::Rock) => 6,
            (Self::Scissors, Self::Paper) => 6,
            _ => 0,
        }
    }

    fn score(&self) -> u16 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day02.txt").unwrap();

    let sol1 = input
        .lines()
        .map(|line| line.split_ascii_whitespace().collect::<Vec<&str>>())
        .map(|items| {
            (
                Shape::from_str(items.get(0).unwrap()),
                Shape::from_str(items.get(1).unwrap()),
            )
        })
        .map(|(a, b)| (b.against(&a) + b.score()) as u64)
        .sum::<u64>();

    // Your solution here...

    (Solution::from(sol1), Solution::from(0))
}
