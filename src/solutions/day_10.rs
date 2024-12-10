use crate::map2d::Map;
use crate::vec2::{Dir, Vec2i};
use crate::Answer;
use bitvec::prelude::*;

const DIRECTIONS: [Dir; 4] = [Dir::N, Dir::E, Dir::S, Dir::W];

fn n_peaks_from_trailhead<const PARTB: bool>(trail_head: Vec2i, map: &Map<u32>) -> i64 {
    let mut stack = vec![trail_head];
    let mut peaks = bitvec![0; map.h * map.w];
    let mut n_dist = 0;

    while let Some(cur) = stack.pop() {
        match map[&cur] {
            9 => {
                if PARTB {
                    n_dist += 1;
                } else {
                    peaks.set(cur.linear_idx(map.w), true);
                }
            }
            d => {
                let new_pos = DIRECTIONS
                    .iter()
                    .map(|d| cur.step(*d, 1))
                    .filter(|p| map.get(p).map(|x| *x == d + 1).unwrap_or_default())
                    .collect::<Vec<_>>();
                stack.extend(new_pos);
            }
        }
    }

    if PARTB {
        n_dist
    } else {
        peaks.count_ones() as i64
    }
}

pub fn part_a(input: &str) -> Answer {
    let map = Map::from_iterators(
        input
            .trim()
            .lines()
            .map(|l| l.chars().map(|c| c as u32 - '0' as u32)),
    );

    let ret = map
        .iter()
        .filter(|(_, v)| **v == 0)
        .map(|(t, _)| n_peaks_from_trailhead::<false>(t, &map))
        .sum();

    Answer::Number(ret)
}

pub fn part_b(input: &str) -> Answer {
    let map = Map::from_iterators(
        input
            .trim()
            .lines()
            .map(|l| l.chars().map(|c| c as u32 - '0' as u32)),
    );

    let ret = map
        .iter()
        .filter(|(_, v)| **v == 0)
        .map(|(t, _)| n_peaks_from_trailhead::<true>(t, &map))
        .sum();

    Answer::Number(ret)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT_SMALL: &str = indoc! {"
        0123
        1234
        8765
        9876
    "};

    #[test]
    fn test_part_a_small() {
        let result = part_a(TEST_INPUT_SMALL);
        assert_eq!(result, Answer::Number(1));
    }

    const TEST_INPUT_MED: &str = indoc! {"
        1022922
        2222822
        3222722
        4567654
        2228223
        2229222
        2222201
    "};

    #[test]
    fn test_part_a_med() {
        let result = part_a(TEST_INPUT_MED);
        assert_eq!(result, Answer::Number(3));
    }

    const TEST_INPUT_MED2: &str = indoc! {"
        2290229
        2221298
        2222227
        6543456
        7652987
        8762222
        9872222
    "};

    #[test]
    fn test_part_a_med2() {
        let result = part_a(TEST_INPUT_MED2);
        assert_eq!(result, Answer::Number(4));
    }

    const TEST_INPUT: &str = indoc! {"
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
    "};

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::Number(36));
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, Answer::Number(81));
    }
}
