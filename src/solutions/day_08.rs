use crate::hash::{FxHashMap, FxHashMapBuilder};
use crate::vec2::Vec2i;
use crate::{math::gcd, Answer};
use itertools::Itertools;

type Groups = FxHashMap<char, Vec<Vec2i>>;

fn parse_input(input: &str) -> (Groups, usize, usize) {
    let lines: Vec<_> = input.lines().collect();
    let h = lines.len();
    let w = lines[0].len();
    let antennas_iter = lines.iter().enumerate().flat_map(|(i, row)| {
        row.chars()
            .enumerate()
            .flat_map(move |(j, c)| -> Option<(char, Vec2i)> {
                match c {
                    '.' => None,
                    _ => Some((c, Vec2i::new(i as i64, j as i64))),
                }
            })
    });

    let mut groups: Groups = Groups::new();
    for (c, p) in antennas_iter {
        groups.entry(c).or_default().push(p);
    }

    (groups, h, w)
}

pub fn part_a(input: &str) -> Answer {
    let (groups, h, w) = parse_input(input);
    let res = groups
        .iter()
        .flat_map(|(_, group)| find_antinodes::<true>(group, h, w))
        .unique()
        .count();
    Answer::Number(res as i64)
}

fn find_antinodes<const PARTA: bool>(group: &[Vec2i], h: usize, w: usize) -> Vec<Vec2i> {
    let pairs_iter =
        (0..group.len()).flat_map(|i| (i + 1..group.len()).map(move |j| (&group[i], &group[j])));

    if PARTA {
        pairs_iter
            .flat_map(|(p, q)| {
                let dp = *p - *q;
                [*p + dp, *q - dp]
            })
            .filter(|p| p.is_in_grid(h, w))
            .collect()
    } else {
        pairs_iter
            .flat_map(|(p, q)| {
                let mut dp = *p - *q;
                let t = gcd(dp.x.unsigned_abs(), dp.y.unsigned_abs()) as i64;
                dp /= t;
                (0..)
                    .map(move |k| -> Vec2i { *p + dp * k })
                    .take_while(|p| p.is_in_grid(h, w))
                    .chain(
                        (1..)
                            .map(move |k| -> Vec2i { *p - dp * k })
                            .take_while(|p| p.is_in_grid(h, w)),
                    )
            })
            .collect()
    }
}

pub fn part_b(input: &str) -> Answer {
    let (groups, h, w) = parse_input(input);
    let res = groups
        .iter()
        .flat_map(|(_, group)| find_antinodes::<false>(group, h, w))
        .unique()
        .count();
    Answer::Number(res as i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
    "};

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::Number(14));
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, Answer::Number(34));
    }

    const TEST_INPUT_PART: &str = indoc! {"
        T.........
        ...T......
        .T........
        ..........
        ..........
        ..........
        ..........
        ..........
        ..........
        ..........
    "};

    #[test]
    fn test_part_b_part() {
        let result = part_b(TEST_INPUT_PART);
        assert_eq!(result, Answer::Number(9));
    }
}
