use crate::map2d::Map;
use crate::Answer;

fn count_cols(map: &Map<char>) -> Vec<u32> {
    let mut ret = vec![0; map.w];
    for (c, _) in map.iter().filter(|(_, v)| **v == '#') {
        ret[c.x as usize] += 1;
    }
    ret
}

pub fn part_a(input: &str) -> Answer {
    let all = input
        .trim()
        .split("\n\n")
        .map(|data| Map::<char>::from_lines(data.lines(), &|c| c))
        .collect::<Vec<_>>();
    let lock_cols = all
        .iter()
        .filter(|m| m[(0, 0)] == '#')
        .map(count_cols)
        .collect::<Vec<_>>();
    let keys_cols = all
        .iter()
        .filter(|m| m[(m.h - 1, 0)] == '#')
        .map(count_cols)
        .collect::<Vec<_>>();

    let mut ret = 0;
    for lock in lock_cols.iter() {
        for key in keys_cols.iter() {
            if lock.iter().zip(key.iter()).all(|(x, y)| x + y <= 7) {
                ret += 1;
            }
        }
    }

    Answer::Number(ret)
}

pub fn part_b(input: &str) -> Answer {
    let _ = input;
    Answer::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        #####
        .####
        .####
        .####
        .#.#.
        .#...
        .....

        #####
        ##.##
        .#.##
        ...##
        ...#.
        ...#.
        .....

        .....
        #....
        #....
        #...#
        #.#.#
        #.###
        #####

        .....
        .....
        #.#..
        ###..
        ###.#
        ###.#
        #####

        .....
        .....
        .....
        #....
        #.#..
        #.#.#
        #####
    "};

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::Number(3));
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, Answer::Unimplemented);
    }
}
