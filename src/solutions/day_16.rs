use crate::map2d::Map;
use crate::vec2::{Dir, Vec2i};
use crate::Answer;
use std::collections::{BinaryHeap, VecDeque};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct State {
    ncost: i64, // use negative cost since we have a max-heap
    pos: Vec2i,
    dir: Dir,
}

type CostMap = Map<[Option<i64>; 4]>;

pub fn part_a(input: &str) -> Answer {
    let map = Map::<char>::from_lines(input.trim().lines(), &|c| c);
    let start = map.iter().find(|(_, v)| **v == 'S').unwrap().0;
    let end = map.iter().find(|(_, v)| **v == 'E').unwrap().0;
    let cost_map = solve_forward((start, Dir::E), end, &map);
    let optimal_ncost = cost_map[&end].iter().flatten().max().unwrap();
    Answer::Number(-optimal_ncost)
}

pub fn part_b(input: &str) -> Answer {
    let map = Map::<char>::from_lines(input.trim().lines(), &|c| c);
    let start = map.iter().find(|(_, v)| **v == 'S').unwrap().0;
    let end = map.iter().find(|(_, v)| **v == 'E').unwrap().0;
    let cost_map = solve_forward((start, Dir::E), end, &map);
    let result = solve_reverse(&cost_map, end);
    Answer::Number(result)
}

fn solve_forward((pos, dir): (Vec2i, Dir), end: Vec2i, map: &Map<char>) -> CostMap {
    let mut queue: BinaryHeap<State> = BinaryHeap::new(); // max-heap
    let mut cost_map = map.clone_with_value::<[Option<i64>; 4]>([None, None, None, None]);

    queue.push(State { pos, dir, ncost: 0 });

    while let Some(State { pos, dir, ncost }) = queue.pop() {
        if map[&pos] == '#' {
            continue; // ran into a wall
        }
        if cost_map[&pos][dir as usize].is_some() {
            continue; // already visited (with lower or equal cost)
        }
        cost_map[&pos][dir as usize] = Some(ncost);
        if pos == end {
            continue; // reached goal
        }

        queue.push(State {
            pos: pos.step(dir, 1),
            dir,
            ncost: ncost - 1,
        });
        queue.push(State {
            pos,
            dir: dir.turn_left(),
            ncost: ncost - 1000,
        });
        queue.push(State {
            pos,
            dir: dir.turn_right(),
            ncost: ncost - 1000,
        });
    }

    cost_map
}

fn solve_reverse(cost_map: &CostMap, end: Vec2i) -> i64 {
    // now do a reverse search along all paths that are consistent with the cost-map
    let mut rqueue: VecDeque<State> = VecDeque::new();
    let opt_cost = cost_map[&end].iter().flatten().max().unwrap();
    for dir in [Dir::N, Dir::E, Dir::S, Dir::W] {
        rqueue.push_back(State {
            pos: end,
            dir,
            ncost: *opt_cost,
        });
    }

    let mut optimal_tiles = cost_map.clone_with_value(false);
    while let Some(State { pos, dir, ncost }) = rqueue.pop_front() {
        if !cost_map[&pos][dir as usize]
            .map(|c| c == ncost)
            .unwrap_or_default()
        {
            continue; // not on an optimal path
        }

        optimal_tiles[&pos] = true;

        // recurse
        rqueue.push_back(State {
            pos: pos.step(-dir, 1),
            dir,
            ncost: ncost + 1,
        });
        rqueue.push_back(State {
            pos,
            dir: dir.turn_left(),
            ncost: ncost + 1000,
        });
        rqueue.push_back(State {
            pos,
            dir: dir.turn_right(),
            ncost: ncost + 1000,
        });
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
