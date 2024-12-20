use crate::map2d::Map;
use crate::vec2::{Vec2i, DIRECTIONS};
use crate::Answer;
use rayon::prelude::*;
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
enum Tile {
    Free,
    Blocked,
    Start,
    End,
}

fn parse(input: &str) -> (Vec2i, Vec2i, Map<Tile>) {
    let map = Map::<Tile>::from_lines(input.trim().lines(), &|c| match c {
        '.' => Tile::Free,
        '#' => Tile::Blocked,
        'S' => Tile::Start,
        'E' => Tile::End,
        _ => unreachable!(),
    });

    let start = map.iter().find(|(_, v)| **v == Tile::Start).unwrap().0;
    let end = map.iter().find(|(_, v)| **v == Tile::End).unwrap().0;

    (start, end, map)
}

fn bfs(start: Vec2i, end_tile: Tile, map: &Map<Tile>) -> Map<u32> {
    let mut queue = VecDeque::<(Vec2i, u32)>::new(); // bfs queue to solve nominal problem
    let mut costmap = map.same_size_with(u32::MAX); // keep track of distance to each visited node
    queue.push_back((start, 0));
    while let Some((pos, cost)) = queue.pop_front() {
        if costmap[&pos] < u32::MAX {
            continue;
        }
        costmap[&pos] = cost;
        match &map[&pos] {
            s if *s == end_tile => break,
            Tile::Blocked => unreachable!(),
            _ => {
                for d in DIRECTIONS {
                    let cand = pos.step(d, 1);
                    if map[&cand] != Tile::Blocked {
                        queue.push_back((cand, cost + 1));
                    }
                }
            }
        }
    }
    costmap
}

fn solve<const PAR: bool>(input: &str, cheat_duration: i32, cheat_count_limit: u32) -> Answer {
    let (start, end, map) = parse(input);
    let costmap_fwd = bfs(start, Tile::End, &map);
    let costmap_rev = bfs(end, Tile::Start, &map);

    let nominal_cost = costmap_fwd[&end];

    let calc_cost_saving = |fwd_cost: u32, p: &Vec2i, (dx, dy): (i32, i32)| -> Option<u32> {
        let p_skip = *p
            + Vec2i {
                x: dx as i64,
                y: dy as i64,
            };
        let alternative_cost = fwd_cost
            .checked_add(dx.unsigned_abs() + dy.unsigned_abs())?
            .checked_add(*costmap_rev.get(&p_skip)?)?;
        let savings = nominal_cost.checked_sub(alternative_cost)?;
        Some(savings)
    };

    let mapper = |(p, fwd_cost): (Vec2i, &u32)| {
        let mut cheat_count = 0;
        // iterate over all skip candidates within manhattan distance 'cheat_duration'
        for dx in -cheat_duration..=cheat_duration {
            let dy_max = cheat_duration - dx.abs();
            for dy in -dy_max..=dy_max {
                if let Some(cost_savings) = calc_cost_saving(*fwd_cost, &p, (dx, dy)) {
                    if cost_savings >= cheat_count_limit {
                        cheat_count += 1;
                    }
                }
            }
        }
        cheat_count
    };

    let iterator = costmap_fwd
        .iter()
        .filter(|(_, rev_cost)| **rev_cost < u32::MAX);
    if PAR {
        Answer::Number(iterator.par_bridge().map(mapper).sum::<i64>())
    } else {
        Answer::Number(iterator.map(mapper).sum::<i64>())
    }
}

pub fn part_a(input: &str) -> Answer {
    solve::<false>(input, 2, 100)
}

pub fn part_b(input: &str) -> Answer {
    solve::<true>(input, 20, 100)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############
    "};

    #[test]
    fn test_part_a_lim0() {
        let result = solve::<false>(TEST_INPUT, 2, 1);
        assert_eq!(result, Answer::Number(44));
    }

    #[test]
    fn test_part_a_lim1() {
        let result = solve::<false>(TEST_INPUT, 2, 10);
        assert_eq!(result, Answer::Number(10));
    }

    #[test]
    fn test_part_a_lim3() {
        let result = solve::<false>(TEST_INPUT, 2, 50);
        assert_eq!(result, Answer::Number(1));
    }

    #[test]
    fn test_part_b_lim0() {
        let result = solve::<false>(TEST_INPUT, 20, 75);
        assert_eq!(result, Answer::Number(3));
    }

    #[test]
    fn test_part_b_lim1() {
        let result = solve::<false>(TEST_INPUT, 20, 70);
        assert_eq!(result, Answer::Number(41));
    }

    const TEST_INPUT_REDDIT: &str = indoc! {"
        #########################################
        #...#.............#.....#.....#.....#...#
        ###.#.###.#########.###.###.#####.###.#.#
        #...#...#.#.#.....#...#...#.#.........#.#
        #..##.###.#.#####.#####.#.#.#.#####.#.#.#
        #.......#.....#.#.....#.#...#...#...#.#.#
        #.###########.#.#.####.####.#.###########
        #.#.#...#...#.....#.................#...#
        #.#.#.#.#.#.###.#.#.###.#########.#####.#
        #.....#...#.....#...#.........#...#.#.#.#
        #####.#####.#####.#.#.#.#.#######.#.#.#.#
        #.....#.........#.#.#...#...#...#.#...#.#
        #.#########.#######.#####.#.##..###.###.#
        #...#.......#.....#.#...#.#...#.....#...#
        #.###.###########.#.###.#.#.###.#######.#
        #.#.#.............#.....#.#...#...#.....#
        ###.#.#####.#####.#.###.#.#####.#####.###
        #...#.#.........#.#...#...#...#.#.....#.#
        ###.###.#.#########.#####.###.#.#.#.#.#.#
        #S#.#...#.#.....#.....#.........#.#.#..E#
        #.#.#.#########.#.#########.#.###.#####.#
        #.....#.........#...#.#...#.#.....#...#.#
        ###.#####..##.#.#####.#.###.#####.###.###
        #.#.#...#.#.#.#.#...#...#...#.........#.#
        #.#.###.###.#.#.#.#####.####.##.#.#####.#
        #.#.#.#.#.#...#.........#.#...#.#.#...#.#
        #.#.#.#.#.#####.###.#.#.#.###.#.###.###.#
        #...#.......#...#...#.#.#.........#.#...#
        #######.#####.#####.###.#.#.#####.#.###.#
        #.............#.....#.#.#.#.....#.......#
        ###############.#####.#.#########.#.#.###
        #.....#...#.#.........#.#...#...#.#.#.#.#
        #.#.#.#.#.#.###.#########.###.###.#####.#
        #.#.#.#.#...........#.#.............#...#
        ###.#.#.###.#######.#.#.#.###.###.#.#.###
        #...#...#...#.#...#.#...#...#.#.#.#.#...#
        ###.#.#######.#.#.#.###.#####.#..##.#.###
        #.#.#...#.....#.#.#.......#.#.#...#.....#
        #.#.#####.###.#.#.#.#.#####.#####.###.#.#
        #.....#.....#.......#.............#...#.#
        #########################################
    "};

    #[test]
    fn test_reddit_case_1() {
        let result = solve::<true>(TEST_INPUT_REDDIT, 20, 30);
        assert_eq!(result, Answer::Number(299));
    }

    #[test]
    fn test_reddit_case_2() {
        let result = solve::<true>(TEST_INPUT_REDDIT, 20, 28);
        assert_eq!(result, Answer::Number(436 + 299));
    }
}
