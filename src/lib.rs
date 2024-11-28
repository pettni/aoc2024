use std::fmt;

pub mod parsing;
pub mod solutions;

#[derive(Debug, PartialEq)]
pub enum Answer {
    Number(i64),
    Unimplemented,
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Answer::Number(n) => write!(f, "{n}"),
            Answer::Unimplemented => write!(f, "Unimplemented"),
        }
    }
}

pub trait Solution {
    fn part_a(&self, input: &str) -> Answer;
    fn part_b(&self, input: &str) -> Answer;
}

pub struct NoSolution;

impl Solution for NoSolution {
    fn part_a(&self, _: &str) -> Answer {
        Answer::Unimplemented
    }
    fn part_b(&self, _: &str) -> Answer {
        Answer::Unimplemented
    }
}
