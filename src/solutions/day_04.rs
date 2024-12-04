use crate::parsing::{identity, parse_matrix};
use crate::Answer;

pub fn part_a(input: &str) -> Answer {
    let (_, m) = parse_matrix(input, identity).unwrap();

    let check_idx =
        |i0: (usize, usize), i1: (usize, usize), i2: (usize, usize), i3: (usize, usize)| -> bool {
            let a = m[i0.0][i0.1];
            let b = m[i1.0][i1.1];
            let c = m[i2.0][i2.1];
            let d = m[i3.0][i3.1];
            if a == 'X' && b == 'M' && c == 'A' && d == 'S' {
                return true;
            }
            if a == 'S' && b == 'A' && c == 'M' && d == 'X' {
                return true;
            }
            false
        };

    // iterate over starting index
    let nrows = m.len();
    let ncols = m[0].len();
    let mut count = 0;
    for i in 0..nrows {
        for j in 0..ncols {
            // vertical
            if i + 3 < nrows && check_idx((i, j), (i + 1, j), (i + 2, j), (i + 3, j)) {
                count += 1;
            }
            // horizontal
            if j + 3 < ncols && check_idx((i, j), (i, j + 1), (i, j + 2), (i, j + 3)) {
                count += 1;
            }
            // diagonal tl-br
            if i + 3 < nrows
                && j + 3 < ncols
                && check_idx((i, j), (i + 1, j + 1), (i + 2, j + 2), (i + 3, j + 3))
            {
                count += 1;
            }
            // diagonal tr-bl
            if i + 3 < nrows
                && j >= 3
                && check_idx((i, j), (i + 1, j - 1), (i + 2, j - 2), (i + 3, j - 3))
            {
                count += 1;
            }
        }
    }

    Answer::Number(count)
}

pub fn part_b(input: &str) -> Answer {
    let (_, m) = parse_matrix(input, identity).unwrap();

    let nrows = m.len();
    let ncols = m[0].len();

    let mut count = 0;
    for i in 0..nrows - 2 {
        for j in 0..ncols - 2 {
            if m[i + 1][j + 1] != 'A' {
                continue;
            }
            let diag1 = (m[i][j] == 'M' && m[i + 2][j + 2] == 'S')
                || (m[i][j] == 'S' && m[i + 2][j + 2] == 'M');
            let diag2 = (m[i + 2][j] == 'M' && m[i][j + 2] == 'S')
                || (m[i + 2][j] == 'S' && m[i][j + 2] == 'M');
            if diag1 && diag2 {
                count += 1;
            }
        }
    }

    Answer::Number(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
    MMMSXXMASM
    MSAMXMSMSA
    AMXSXMAAMM
    MSAMASMSMX
    XMASAMXAMM
    XXAMMXXAMA
    SMSMSASXSS
    SAXAMASAAA
    MAMMMXMMMM
    MXMXAXMASX
    "#;

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::Number(18));
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, Answer::Number(9));
    }
}
