use crate::{Answer, Solution};
use nom::{
    character::complete::{digit1, newline, not_line_ending, space1},
    combinator::{map_parser, map_res, verify},
    multi::separated_list1,
    IResult,
};
use std::collections::HashMap;

pub struct Day01;

/// Parse rows of numbers.
fn parse_data(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    let line_as_str = verify(not_line_ending, |s: &str| !s.is_empty());
    let row_parser = map_parser(
        line_as_str,
        separated_list1(space1, map_res(digit1, str::parse::<i64>)),
    );
    let mut rows_parser = separated_list1(newline, row_parser);
    rows_parser(input)
}

impl Solution for Day01 {
    fn part_a(&self, input: &str) -> Answer {
        let (_, data) = parse_data(input).unwrap();

        let mut col1: Vec<i64> = data.iter().map(|v| v[0]).collect();
        let mut col2: Vec<i64> = data.iter().map(|v| v[1]).collect();
        col1.sort();
        col2.sort();

        let answer = col1.iter().zip(col2).map(|(x, y)| (x - y).abs()).sum();

        Answer::Number(answer)
    }

    fn part_b(&self, input: &str) -> Answer {
        let (_, data) = parse_data(input).unwrap();

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

    #[test]
    fn test_part_a() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

        let solver = Day01 {};
        let result = solver.part_a(input);
        assert_eq!(result, Answer::Number(11));
    }

    #[test]
    fn test_part_b() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

        let solver = Day01 {};
        let result = solver.part_b(input);
        assert_eq!(result, Answer::Number(31));
    }
}
