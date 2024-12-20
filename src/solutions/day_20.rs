use crate::map2d::Map;
use crate::vec2::{Vec2i, DIRECTIONS};
use crate::Answer;
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
enum Tile {
    Free,
    Blocked,
    Start,
    End,
}

fn parse(input: &str) -> (Vec2i, Map<Tile>) {
    let map = Map::<Tile>::from_lines(input.trim().lines(), &|c| match c {
        '.' => Tile::Free,
        '#' => Tile::Blocked,
        'S' => Tile::Start,
        'E' => Tile::End,
        _ => unreachable!(),
    });

    let start = map.iter().find(|(_, v)| **v == Tile::Start).unwrap().0;

    (start, map)
}

fn solve(input: &str, cheat_duration: i64, cheat_count_limit: u64) -> Answer {
    let (start, map) = parse(input);

    let mut queue = VecDeque::<(Vec2i, u64)>::new(); // bfs queue to solve nominal problem
    let mut costmap = map.same_size_with(u64::MAX); // keep track of distance to each visited node
    let mut cheat_count = 0; // number of cheats found

    queue.push_back((start, 0));
    while let Some((pos, cost)) = queue.pop_front() {
        // blocked
        if map[&pos] == Tile::Blocked {
            continue;
        }
        // already visisted
        if costmap[&pos] < u64::MAX {
            continue;
        }
        // cheat: see if we could have gotten here by jumping over walls from a tile with lower cost
        // iterate over all skip candidates within manhattan distance 'cheat_duration'
        for dx in -cheat_duration..=cheat_duration {
            let dy_max = cheat_duration - dx.abs();
            for dy in -dy_max..=dy_max {
                let skip_step = pos + Vec2i { x: dx, y: dy };
                if let Some(other_cost) = costmap.get(&skip_step) {
                    if let Some(alternative_cost) =
                        other_cost.checked_add(dx.unsigned_abs() + dy.unsigned_abs())
                    {
                        if let Some(cost_saving) = cost.checked_sub(alternative_cost) {
                            if cost_saving >= cheat_count_limit {
                                cheat_count += 1;
                            }
                        }
                    }
                }
            }
        }
        // update cost for this node
        costmap[&pos] = cost;
        // update queue to proceed towards goal
        match map[&pos] {
            Tile::End => break,
            Tile::Start | Tile::Free => {
                for d in DIRECTIONS {
                    queue.push_back((pos.step(d, 1), cost + 1));
                }
            }
            _ => unreachable!(),
        }
    }

    Answer::Number(cheat_count as i64)
}

pub fn part_a(input: &str) -> Answer {
    solve(input, 2, 100)
}

pub fn part_b(input: &str) -> Answer {
    solve(input, 20, 100)
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
        let result = solve(TEST_INPUT, 2, 1);
        assert_eq!(result, Answer::Number(44));
    }

    #[test]
    fn test_part_a_lim1() {
        let result = solve(TEST_INPUT, 2, 10);
        assert_eq!(result, Answer::Number(10));
    }

    #[test]
    fn test_part_a_lim3() {
        let result = solve(TEST_INPUT, 2, 50);
        assert_eq!(result, Answer::Number(1));
    }

    #[test]
    fn test_part_b_lim0() {
        let result = solve(TEST_INPUT, 20, 75);
        assert_eq!(result, Answer::Number(3));
    }

    #[test]
    fn test_part_b_lim1() {
        let result = solve(TEST_INPUT, 20, 70);
        assert_eq!(result, Answer::Number(41));
    }
}
