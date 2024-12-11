use crate::hash::{FxHashMap, FxHashMapBuilder};
use crate::parsing::parse_rows_of_ints;
use crate::Answer;

pub fn part_a(input: &str) -> Answer {
    let (_, data) = parse_rows_of_ints(input).unwrap();

    let mut col1: Vec<i64> = data.iter().map(|v| v[0]).collect();
    let mut col2: Vec<i64> = data.iter().map(|v| v[1]).collect();
    col1.sort();
    col2.sort();

    let answer = col1.iter().zip(col2).map(|(x, y)| (x - y).abs()).sum();

    Answer::Number(answer)
}

pub fn part_b(input: &str) -> Answer {
    let (_, data) = parse_rows_of_ints(input).unwrap();

    let mut counter: FxHashMap<i64, i64> = FxHashMap::with_capacity(1_000);
    for x in data.iter().map(|v| v[1]) {
        let val = counter.entry(x).or_insert(0);
        *val += 1;
    }

    let mut result = 0;
    for x in data.iter().map(|v| v[0]) {
        let maybe_count = counter.get(&x);
        if maybe_count.is_some() {
            result += x * maybe_count.unwrap();
        }
    }
    Answer::Number(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "};

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::Number(11));
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, Answer::Number(31));
    }
}
