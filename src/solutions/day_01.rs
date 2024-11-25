use crate::parsing;
use crate::{Answer, Solution};

pub struct Day01;

impl Solution for Day01 {
    fn part_a(&self, input: &str) -> Answer {
        let (_, data) = parsing::parse_matrix(parser, input).unwrap();
        let sum = data
            .iter()
            .flatten()
            .map(|x| (*x == CellType::Vertical) as i64)
            .sum();
        Answer::Number(sum)
    }

    fn part_b(&self, _: &str) -> Answer {
        Answer::Unimplemented
    }
}

#[derive(Debug, PartialEq)]
enum CellType {
    Vertical,
    Horizontal,
    Other,
}

fn parser(c: char) -> Result<CellType, String> {
    match c {
        '|' => Ok(CellType::Vertical),
        '-' => Ok(CellType::Horizontal),
        _ => Ok(CellType::Other),
    }
}
