use crate::trie::Trie;
use crate::Answer;
use rayon::prelude::*;

fn count_valid_patterns_dp(pattern: &str, trie: &Trie) -> u64 {
    let n = pattern.len();
    let mut dp = vec![0; n + 1];
    dp[n] = 1;
    for i in (0..n).rev() {
        let mut node = trie.root();
        for (k, c) in pattern[i..].chars().enumerate() {
            match node.step(c) {
                Some(next) => {
                    if next.is_end() {
                        dp[i] += dp[i + k + 1];
                    }
                    node = next;
                }
                None => {
                    break;
                }
            }
        }
    }
    dp[0]
}

fn parse(input: &str) -> (Trie, Vec<&str>) {
    let mut overall_iter = input.trim().split("\n\n");
    let towels = overall_iter.next().unwrap().split(",").map(|x| x.trim());
    let trie = Trie::from_word_iterator(towels);
    let words = overall_iter.next().unwrap().lines().collect::<Vec<_>>();
    (trie, words)
}

pub fn part_a(input: &str) -> Answer {
    let (trie, words) = parse(input);
    let ret = words
        .par_iter()
        .filter(|x| count_valid_patterns_dp(x, &trie) > 0)
        .count();
    Answer::Number(ret as i64)
}

pub fn part_b(input: &str) -> Answer {
    let (trie, words) = parse(input);
    let ret = words
        .par_iter()
        .map(|x| count_valid_patterns_dp(x, &trie))
        .sum::<u64>();
    Answer::Number(ret as i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb
    "};

    const TEST_INPUT_IMPOSSIBLE: &str = indoc! {"
        r, wr, b, g, bwu, rb, gb, br

        ubwu
        bbrgwb
    "};

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::Number(6));
    }

    #[test]
    fn test_part_a_impossible() {
        let result = part_a(TEST_INPUT_IMPOSSIBLE);
        assert_eq!(result, Answer::Number(0));
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, Answer::Number(16));
    }
}
