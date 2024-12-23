use rayon::prelude::*;
use std::iter::successors;

use crate::Answer;

fn step(mut n: u32) -> u32 {
    // step 1
    n ^= n << 6; // mix
    n <<= 8; // prune [keep 24 bits]
    n >>= 8;

    // step 2
    n ^= n >> 5; // mix
    n <<= 8; // prune [keep 24 bits]
    n >>= 8;

    // step 3
    n ^= n << 11; // mix
    n <<= 8; // prune [keep 24 bits]
    n >>= 8;

    n
}

fn solve_part_a<const N: usize>(input: &str) -> Answer {
    let result = input
        .trim()
        .lines()
        .flat_map(str::parse::<u32>)
        .par_bridge()
        .map(|n| successors(Some(n), |x| Some(step(*x))).nth(N).unwrap() as i64)
        .sum::<i64>();
    Answer::Number(result)
}

pub fn part_a(input: &str) -> Answer {
    solve_part_a::<2000>(input)
}

fn solve_part_b<const N: usize>(input: &str) -> Answer {
    let mut diff_score: Vec<i32> = vec![0; 19 * 19 * 19 * 19];
    for n in input.trim().lines().flat_map(str::parse::<u32>) {
        let mut last_mod_10: i32 = 0;
        let mut diffs: [i32; 4] = [0, 0, 0, 0];
        let mut seen: Vec<bool> = vec![false; 19 * 19 * 19 * 19];
        for (i, k) in successors(Some(n), |x| Some(step(*x))).take(N).enumerate() {
            diffs.rotate_left(1);
            diffs[3] = k as i32 % 10 - last_mod_10;
            let diffs_idx = ((9 + diffs[0]) * 19 * 19 * 19
                + (9 + diffs[1]) * 19 * 19
                + (9 + diffs[2]) * 19
                + (9 + diffs[3])) as usize;
            if i >= 4 && !seen[diffs_idx] {
                diff_score[diffs_idx] += k as i32 % 10;
                seen[diffs_idx] = true;
            }
            last_mod_10 = k as i32 % 10;
        }
    }

    let result = diff_score.iter().max().unwrap();
    Answer::Number(*result as i64)
}

pub fn part_b(input: &str) -> Answer {
    solve_part_b::<2000>(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT_SINGLE: &str = indoc! {"
        123
    "};

    #[test]
    fn test_part_a_single() {
        let result = solve_part_a::<10>(TEST_INPUT_SINGLE);
        assert_eq!(result, Answer::Number(5908254));
    }

    #[test]
    fn test_part_b_single() {
        let result = solve_part_b::<10>(TEST_INPUT_SINGLE);
        assert_eq!(result, Answer::Number(6));
    }

    const TEST_INPUT: &str = indoc! {"
        1
        10
        100
        2024
    "};

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::Number(37327623));
    }

    const TEST_INPUT_B: &str = indoc! {"
        1
        2
        3
        2024
    "};

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT_B);
        assert_eq!(result, Answer::Number(23));
    }
}
