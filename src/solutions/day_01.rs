use crate::parsing::parse_rows_of_ints;
use crate::{Answer, Solution};
use std::collections::HashMap;

pub struct Day01;

impl Solution for Day01 {
    fn part_a(&self, input: &str) -> Answer {
        let (_, data) = parse_rows_of_ints(input).unwrap();

        let mut col1: Vec<i64> = data.iter().map(|v| v[0]).collect();
        let mut col2: Vec<i64> = data.iter().map(|v| v[1]).collect();
        col1.sort();
        col2.sort();

        let answer = col1.iter().zip(col2).map(|(x, y)| (x - y).abs()).sum();

        Answer::Number(answer)
    }

    fn part_b(&self, input: &str) -> Answer {
        let (_, data) = parse_rows_of_ints(input).unwrap();

        let mut counter: HashMap<i64, i64> = HashMap::new();
        for x in data.iter().map(|v| v[1]) {
            let val = counter.entry(x).or_insert(0);
            *val += 1;
        }

        let mut result = 0;
        for x in data.iter().map(|v| v[0]) {
            if counter.contains_key(&x) {
                result += x * counter[&x];
            }
        }
        Answer::Number(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
    3   4
    4   3
    2   5
    1   3
    3   9
    3   3
    "#;

    const SOLVER: Day01 = Day01 {};

    #[test]
    fn test_part_a() {
        let result = SOLVER.part_a(TEST_INPUT);
        assert_eq!(result, Answer::Number(11));
    }

    #[test]
    fn test_part_b() {
        let result = SOLVER.part_b(TEST_INPUT);
        assert_eq!(result, Answer::Number(31));
    }
}
