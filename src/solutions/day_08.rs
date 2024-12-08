use crate::{math::gcd, Answer};
use itertools::Itertools;
use std::collections::HashMap;

type Groups = HashMap<char, Vec<(usize, usize)>>;

fn parse_input(input: &str) -> (Groups, usize, usize) {
    let lines: Vec<_> = input.lines().collect();
    let h = lines.len();
    let w = lines[0].len();
    let antennas_iter = lines.iter().enumerate().flat_map(|(i, row)| {
        row.chars()
            .enumerate()
            .flat_map(move |(j, c)| -> Option<(char, (usize, usize))> {
                match c {
                    '.' => None,
                    _ => Some((c, (i, j))),
                }
            })
    });

    let mut groups: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (c, (i, j)) in antennas_iter {
        let c_group = groups.entry(c).or_default();
        c_group.push((i, j));
    }

    (groups, h, w)
}

pub fn part_a(input: &str) -> Answer {
    let (groups, h, w) = parse_input(input);
    let res = groups
        .iter()
        .flat_map(|(_, group)| find_antinodes::<false>(group, h, w))
        .unique()
        .count();
    Answer::Number(res as i64)
}

fn find_antinodes<const PARTB: bool>(
    group: &[(usize, usize)],
    h: usize,
    w: usize,
) -> Vec<(isize, isize)> {
    group
        .iter()
        .combinations(2)
        .flat_map(|v| {
            let (i0, j0) = (v[0].0 as isize, v[0].1 as isize);
            let (i1, j1) = (v[1].0 as isize, v[1].1 as isize);

            let mut di = i1 - i0;
            let mut dj = j1 - j0;

            if PARTB {
                let t = gcd(di.unsigned_abs() as u64, dj.unsigned_abs() as u64) as isize;
                di /= t;
                dj /= t;
                let mut ret: Vec<_> = (0..)
                    .map(|k| -> Option<(isize, isize)> {
                        let i = i0 + k * di;
                        let j = j0 + k * dj;
                        if 0 <= i && i < h as isize && 0 <= j && j < w as isize {
                            return Some((i, j));
                        }
                        None
                    })
                    .take_while(|k| k.is_some())
                    .flatten()
                    .collect();
                let neg = (1..)
                    .map(|k| -> Option<(isize, isize)> {
                        let i = i0 - k * di;
                        let j = j0 - k * dj;
                        if 0 <= i && i < h as isize && 0 <= j && j < w as isize {
                            return Some((i, j));
                        }
                        None
                    })
                    .take_while(|k| k.is_some())
                    .flatten();
                ret.extend(neg);
                ret
            } else {
                vec![(i1 + di, j1 + dj), (i0 - di, j0 - dj)]
            }
        })
        .filter(|(i, j)| *i >= 0 && *i < h as isize && *j >= 0 && *j < w as isize)
        .collect()
}

pub fn part_b(input: &str) -> Answer {
    let (groups, h, w) = parse_input(input);
    let res = groups
        .iter()
        .flat_map(|(_, group)| find_antinodes::<true>(group, h, w))
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
