use crate::container::BucketQueue;
use crate::map2d::Map;
use crate::vec2::{Dir, Vec2i};
use crate::Answer;
use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq)]
struct State {
    pos: Vec2i,
    dir: Dir,
}

type CostMap = Map<[Option<i64>; 4]>;

pub fn part_a(input: &str) -> Answer {
    let map = Map::<char>::from_lines(input.trim().lines(), &|c| c);
    let start = map.iter().find(|(_, v)| **v == 'S').unwrap().0;
    let end = map.iter().find(|(_, v)| **v == 'E').unwrap().0;
    let cost_map = solve_forward((start, Dir::E), end, &map);
    let optimal_cost = cost_map[&end].iter().flatten().max().unwrap();
    Answer::Number(*optimal_cost)
}

pub fn part_b(input: &str) -> Answer {
    let map = Map::<char>::from_lines(input.trim().lines(), &|c| c);
    let start = map.iter().find(|(_, v)| **v == 'S').unwrap().0;
    let end = map.iter().find(|(_, v)| **v == 'E').unwrap().0;
    let cost_map = solve_forward((start, Dir::E), end, &map);
    let result = solve_reverse(&cost_map, end);
    Answer::Number(result)
}

fn cost_to_go((pos, dir): (Vec2i, Dir), end: Vec2i) -> i64 {
    let linear = pos.manhattan_dist(&end) as i64;
    let dp = end - pos;
    if dp.x == 0 && dir == Dir::N {
        return linear;
    }
    if dp.y == 0 && dir == Dir::E {
        return linear;
    }
    match dir {
        Dir::N => linear + 1000,
        Dir::E => linear + 1000,
        Dir::S => linear + 2000,
        Dir::W => linear + 2000,
    }
}

fn solve_forward((pos, dir): (Vec2i, Dir), end: Vec2i, map: &Map<char>) -> CostMap {
    let mut prio_queue = BucketQueue::new(1001);
    let mut cost_map = map.same_size_with::<[Option<i64>; 4]>([None, None, None, None]);
    let mut best_cost: Option<u64> = None;

    prio_queue.push(0, State { pos, dir });

    while let Some((cost, State { pos, dir })) = prio_queue.pop() {
        let not_competitive = best_cost
            .map(|best_cost| {
                let ctg = cost_to_go((pos, dir), end) as u64;
                best_cost < cost + ctg
            })
            .unwrap_or_default();
        let is_wall = map[&pos] == '#';
        let already_visited = cost_map[&pos][dir as usize].is_some();
        if not_competitive || is_wall || already_visited {
            // do not recurse
            continue;
        }
        cost_map[&pos][dir as usize] = Some(cost as i64);
        if pos == end {
            // reached end
            best_cost = best_cost.or(Some(cost));
            continue;
        }

        // grow tree
        prio_queue.push(
            cost + 1,
            State {
                pos: pos.step(dir, 1),
                dir,
            },
        );
        // only try turning if we don't turn towards a wall
        if map[&pos.step(dir.turn_left(), 1)] != '#' {
            prio_queue.push(
                cost + 1000,
                State {
                    pos,
                    dir: dir.turn_left(),
                },
            );
        }
        if map[&pos.step(dir.turn_right(), 1)] != '#' {
            prio_queue.push(
                cost + 1000,
                State {
                    pos,
                    dir: dir.turn_right(),
                },
            );
        }
    }

    cost_map
}

fn solve_reverse(cost_map: &CostMap, end: Vec2i) -> i64 {
    // now do a reverse search along all paths that are consistent with the cost-map
    let mut rqueue: VecDeque<(State, i64)> = VecDeque::new();
    let opt_cost = cost_map[&end].iter().flatten().min().unwrap();
    for dir in [Dir::N, Dir::E, Dir::S, Dir::W] {
        rqueue.push_back((State { pos: end, dir }, *opt_cost));
    }

    let mut optimal_tiles = cost_map.same_size_with(false);
    while let Some((State { pos, dir }, cost)) = rqueue.pop_front() {
        if !cost_map[&pos][dir as usize]
            .map(|c| c == cost)
            .unwrap_or_default()
        {
            continue; // not on an optimal path
        }

        optimal_tiles[&pos] = true;

        // recurse
        rqueue.push_back((
            State {
                pos: pos.step(-dir, 1),
                dir,
            },
            cost - 1,
        ));
        rqueue.push_back((
            State {
                pos,
                dir: dir.turn_left(),
            },
            cost - 1000,
        ));
        rqueue.push_back((
            State {
                pos,
                dir: dir.turn_right(),
            },
            cost - 1000,
        ));
    }

    optimal_tiles.iter_values().filter(|p| **p).count() as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        ###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############
    "};

    const TEST_INPUT_2: &str = indoc! {"
        #################
        #...#...#...#..E#
        #.#.#.#.#.#.#.#.#
        #.#.#.#...#...#.#
        #.#.#.#.###.#.#.#
        #...#.#.#.....#.#
        #.#.#.#.#.#####.#
        #.#...#.#.#.....#
        #.#.#####.#.###.#
        #.#.#.......#...#
        #.#.###.#####.###
        #.#.#...#.....#.#
        #.#.#.#####.###.#
        #.#.#.........#.#
        #.#.#.#########.#
        #S#.............#
        #################
    "};

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::Number(7036));
    }

    #[test]
    fn test_part_a_2() {
        let result = part_a(TEST_INPUT_2);
        assert_eq!(result, Answer::Number(11048));
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, Answer::Number(45));
    }

    #[test]
    fn test_part_b_2() {
        let result = part_b(TEST_INPUT_2);
        assert_eq!(result, Answer::Number(64));
    }
}
