use crate::Answer;
use rayon::prelude::*;

pub fn part_a(input: &str) -> Answer {
    let m: Vec<_> = input.lines().collect();

    // iterate over starting index
    let nrows = m.len();
    let ncols = m[0].len();

    // seems fastest to parallelize over rows
    let row_counter = |i: usize| {
        let mut count = 0;
        for j in 0..ncols {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let is_xmas = "XMAS".bytes().zip(0isize..).all(|(b, k)| -> bool {
                        let el_at_ii_jj = i
                            .checked_add_signed(dx * k)
                            .and_then(|ii| m.get(ii))
                            .and_then(|row| {
                                j.checked_add_signed(dy * k)
                                    .and_then(|jj| row.as_bytes().get(jj))
                            });
                        el_at_ii_jj == Some(&b)
                    });
                    if is_xmas {
                        count += 1;
                    }
                }
            }
        }
        count
    };

    let count = (0..nrows).into_par_iter().map(row_counter).sum();
    Answer::Number(count)
}

pub fn part_b(input: &str) -> Answer {
    let m: Vec<_> = input.lines().collect();

    let row_counter = |i: usize| {
        let mut row_count = 0;
        for j in 0..m[i].len() - 2 {
            let r0 = m[i].as_bytes();
            let r1 = m[i + 1].as_bytes();
            let r2 = m[i + 2].as_bytes();
            if (r1[j + 1] == b'A')
                && ((r0[j] == b'M' && r2[j + 2] == b'S') || (r0[j] == b'S' && r2[j + 2] == b'M'))
                && ((r2[j] == b'M' && r0[j + 2] == b'S') || (r2[j] == b'S' && r0[j + 2] == b'M'))
            {
                row_count += 1;
            }
        }
        row_count
    };

    let count = (0..m.len() - 2).into_par_iter().map(row_counter).sum();
    Answer::Number(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
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
    "};

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
