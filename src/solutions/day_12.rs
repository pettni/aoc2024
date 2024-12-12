use std::iter::successors;

use crate::hash::{FxHashSet, FxHashSetBuilder};
use crate::map2d::Map;
use crate::vec2::{Dir, Vec2i};
use crate::Answer;

type PosDir = (Vec2i, Dir);

const DIRECTIONS: [Dir; 4] = [Dir::N, Dir::E, Dir::S, Dir::W];

/// Take a step along contour of region defined by f_pred.
/// Assumes that s is on the contour pointing in the ccw direction.
fn step_contour_ccw<T, F>(s: &PosDir, map: &Map<T>, f_pred: &F) -> PosDir
where
    F: Fn(&T) -> bool,
{
    let (p, d) = *s;
    // Step along ccw boundary.
    let p_next = p.step(d, 1);
    if map.get(&p_next).map(f_pred).unwrap_or_default() {
        // stepped inside
        let on_rght = p_next.step(d.turn_right(), 1);
        if map.get(&on_rght).map(f_pred).unwrap_or_default() {
            (on_rght, d.turn_right()) // lost track of contour, have to turn right to continue
        } else {
            (p_next, d) // still have wall on the right
        }
    } else {
        (p, d.turn_left()) // stepped outside, need to turn left
    }
}

fn trace_contour<F>(
    s0: &PosDir,
    map: &Map<char>,
    f_pred: &F,
    contour_visited: &mut FxHashSet<PosDir>,
) -> u32
where
    F: Fn(&char) -> bool,
{
    let mut perimiter = 0;
    let mut d_last = s0.1;
    successors(Some((0, *s0)), |(i, s)| {
        if *i > 0 && *s == *s0 {
            None
        } else {
            Some((i + 1, step_contour_ccw(s, map, &f_pred)))
        }
    })
    .for_each(|(_, (p, d))| {
        contour_visited.insert((p, d));
        if d != d_last {
            perimiter += 1;
        }
        d_last = d;
    });

    perimiter
}

pub fn solve<const PARTB: bool>(input: &str) -> Answer {
    let map = Map::from_iterators(input.trim().lines().map(|s| s.chars()));

    let mut cell_visited: FxHashSet<Vec2i> = FxHashSet::with_capacity(141 * 141);
    let mut contour_visited: FxHashSet<PosDir> = FxHashSet::with_capacity(141 * 141 * 4);
    let mut stack: Vec<(Dir, Vec2i)> = Vec::with_capacity(141 * 141 * 4);
    let mut result = 0;

    for (coord, v) in map.iter() {
        let f_region = |c: &char| *c == *v;

        if !cell_visited.contains(&coord) {
            stack.clear();
            let mut area = 0;
            let mut perimiter = 0;
            stack.push((Dir::N, coord));
            while let Some((dir, cur)) = stack.pop() {
                if !map.get(&cur).map(|c| c == v).unwrap_or_default() {
                    // stepped outside region, increase perimiter
                    if PARTB {
                        let s0 = (cur.step(-dir, 1), dir.turn_left());
                        if !contour_visited.contains(&s0) {
                            perimiter += trace_contour(&s0, &map, &f_region, &mut contour_visited);
                        }
                    } else {
                        perimiter += 1;
                    }
                } else if !cell_visited.contains(&cur) {
                    // still inside region, recurse
                    cell_visited.insert(cur);
                    area += 1;
                    stack.extend(DIRECTIONS.map(|dir| (dir, cur.step(dir, 1))));
                }
            }
            result += (area * perimiter) as i64;
        }
    }

    Answer::Number(result)
}

pub fn part_a(input: &str) -> Answer {
    solve::<false>(input)
}

pub fn part_b(input: &str) -> Answer {
    solve::<true>(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT_S: &str = indoc! {"
        AAAA
        BBCD
        BBCC
        EEEC
    "};

    #[test]
    fn test_part_a_s() {
        let result = part_a(TEST_INPUT_S);
        assert_eq!(result, Answer::Number(140));
    }

    #[test]
    fn test_part_b_s() {
        let result = part_b(TEST_INPUT_S);
        assert_eq!(result, Answer::Number(80));
    }

    const TEST_INPUT_M: &str = indoc! {"
        OOOOO
        OXOXO
        OOOOO
        OXOXO
        OOOOO
    "};

    #[test]
    fn test_part_a_m() {
        let result = part_a(TEST_INPUT_M);
        assert_eq!(result, Answer::Number(772));
    }

    #[test]
    fn test_part_b_m() {
        let result = part_b(TEST_INPUT_M);
        assert_eq!(result, Answer::Number(436));
    }

    const TEST_INPUT: &str = indoc! {"
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE
    "};

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::Number(1930));
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, Answer::Number(1206));
    }
}
