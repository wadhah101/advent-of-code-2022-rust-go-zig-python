use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum RoundResult {
    Win = 6,
    Loss = 0,
    Draw = 3,
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
    fn against(&self, other: &Self) -> RoundResult {
        if self == other {
            return RoundResult::Draw;
        }
        match (self, other) {
            (Self::Rock, Self::Scissors) => RoundResult::Win,
            (Self::Paper, Self::Rock) => RoundResult::Win,
            (Self::Scissors, Self::Paper) => RoundResult::Win,
            _ => RoundResult::Loss,
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
        .map(|(a, b)| (b.against(&a) as u16 + b as u16) as u32)
        .sum::<u32>();

    // Your solution here...

    (Solution::from(sol1), Solution::from(0))
}
