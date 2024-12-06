use std::ops::RangeBounds;

use crate::parsing::parse_rows_of_ints;
use crate::Answer;

pub fn part_a(input: &str) -> Answer {
    let (_, data) = parse_rows_of_ints(input).unwrap();
    let num_valid_rows = data.iter().filter(is_row_valid).count();
    Answer::Number(num_valid_rows as i64)
}

fn is_row_valid(row: &&Vec<i64>) -> bool {
    let row_folder = |(cinc, cdec): (i64, i64), (x, y): (&i64, &i64)| -> (i64, i64) {
        let diff = y - x;
        if (1..=3).contains(&diff) {
            return (cinc + 1, cdec);
        } else if (-3..=-1).contains(&diff) {
            return (cinc, cdec + 1);
        }
        (cinc, cdec)
    };
    let (ninc, ndec) = row.iter().zip(row.iter().skip(1)).fold((0, 0), row_folder);
    ndec == (row.len() - 1) as i64 || ninc == (row.len() - 1) as i64
}

pub fn part_b(input: &str) -> Answer {
    let (_, data) = parse_rows_of_ints(input).unwrap();
    let r0 = -3..=-1;
    let r1 = 1..=3;
    let num_valid_rows = data
        .iter()
        .filter(|row| {
            is_row_in_range_skips(row, &r0, 1, 0, None)
                || is_row_in_range_skips(row, &r1, 1, 0, None)
        })
        .count();
    Answer::Number(num_valid_rows as i64)
}

fn is_row_in_range_skips<T>(
    row: &&Vec<i64>,
    range: &T,
    nskips: i64,
    idx: usize,
    last_val: Option<i64>,
) -> bool
where
    T: RangeBounds<i64>,
{
    if nskips < 0 {
        return false;
    }
    if idx >= row.len() {
        return true;
    }

    // do not skip element #idx
    let maybe_diff_in_range = last_val.map(|last| range.contains(&(row[idx] - last)));
    let tail_valid = maybe_diff_in_range.unwrap_or(true)
        && is_row_in_range_skips(row, range, nskips, idx + 1, Some(row[idx]));
    // skip element #idx
    let skip_valid = is_row_in_range_skips(row, range, nskips - 1, idx + 1, last_val);

    tail_valid || skip_valid
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "};

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::Number(2));
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, Answer::Number(4));
    }
}
