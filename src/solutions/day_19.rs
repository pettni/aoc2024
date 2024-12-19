use crate::hash::*;
use crate::trie::Trie;
use crate::Answer;

fn is_pattern_valid<'a>(pattern: &'a str, trie: &Trie, mem: &mut FxHashMap<&'a str, u64>) -> u64 {
    if mem.contains_key(pattern) {
        return mem[pattern];
    }

    let mut node = trie.root();
    let mut this_branch = true; // if this branch is valid (no word splits)
    let mut other_branches = 0; // if other branches are valid
    for (i, c) in pattern.chars().enumerate() {
        match node.step(c) {
            Some(next) => {
                if next.is_end() {
                    other_branches += is_pattern_valid(&pattern[i + 1..], trie, mem);
                }
                node = next;
            }
            None => {
                this_branch = false;
                break;
            }
        }
    }

    let result = if this_branch & node.is_end() {
        other_branches + 1
    } else {
        other_branches
    };
    mem.insert(pattern, result);
    result
}

fn parse(input: &str) -> (Trie, Vec<&str>) {
    let mut overall_iter = input.trim().split("\n\n");
    let towels = overall_iter
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.trim())
        .collect::<Vec<_>>();
    let trie = Trie::from_words(&towels);

    let words = overall_iter.next().unwrap().lines().collect::<Vec<_>>();

    (trie, words)
}

pub fn part_a(input: &str) -> Answer {
    let (trie, words) = parse(input);
    let mut mem: FxHashMap<&str, u64> = FxHashMap::new();
    let ret = words
        .iter()
        .filter(|x| is_pattern_valid(x, &trie, &mut mem) > 0)
        .count();
    Answer::Number(ret as i64)
}

pub fn part_b(input: &str) -> Answer {
    let (trie, words) = parse(input);
    let mut mem: FxHashMap<&str, u64> = FxHashMap::new();
    let ret = words
        .iter()
        .map(|x| is_pattern_valid(x, &trie, &mut mem))
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
