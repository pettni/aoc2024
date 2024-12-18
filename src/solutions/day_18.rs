use std::cmp::Ordering;

use crate::heap::MinHeap;
use crate::map2d::Map;
use crate::vec2::{Dir, Vec2i, DIRECTIONS};
use crate::Answer;

type State = (Vec2i, u64);

fn parse(input: &str) -> Vec<Vec2i> {
    input
        .trim()
        .lines()
        .map(|l| {
            let mut x = l.split(",").flat_map(str::parse::<i64>);
            Vec2i {
                x: x.next().unwrap(),
                y: x.next().unwrap(),
            }
        })
        .collect::<Vec<_>>()
}

fn solve_part_a(input: &str, h: usize, w: usize, n: usize) -> Answer {
    let bytes = parse(input);
    let mut map = Map::<char>::new_constant(h, w, '.');
    for byte in &bytes[0..n] {
        map[byte] = '#';
    }
    let target = Vec2i {
        x: w as i64 - 1,
        y: h as i64 - 1,
    };
    let mut queue = MinHeap::new(&|s1: &State, s2: &State| s1.1.cmp(&s2.1));
    let mut visited = map.same_size_with(u64::MAX);

    queue.push((Vec2i { x: 0, y: 0 }, 0));
    incremental_djikstra(&map, &target, &mut queue, &mut visited);

    Answer::Number(visited[&target] as i64)
}

pub fn part_a(input: &str) -> Answer {
    solve_part_a(input, 71, 71, 1024)
}

fn incremental_djikstra(
    map: &Map<char>,
    target: &Vec2i,
    queue: &mut MinHeap<(Vec2i, u64), impl Fn(&(Vec2i, u64), &(Vec2i, u64)) -> Ordering>,
    visited: &mut Map<u64>,
) {
    while let Some((cur, cost)) = queue.pop() {
        if !map.get(&cur).map(|x| *x != '#').unwrap_or_default() {
            continue;
        }
        if visited[&cur] < u64::MAX {
            continue;
        }
        visited[&cur] = cost;
        if cur == *target {
            break;
        }

        for dir in [Dir::N, Dir::E, Dir::W, Dir::S] {
            queue.push((cur.step(dir, 1), cost + 1));
        }
    }

    queue.clear();
}

fn solve_part_b(input: &str, h: usize, w: usize, n: usize) -> Answer {
    // Faster options:
    //  - [ ] use bucket queue for Djikstra
    //  - [x] incremental djikstra
    //    - keep track of "parents" and only run updates on nodes that are downstream of new node
    //  - [ ] min-cut?
    let bytes = parse(input);
    let mut map = Map::<char>::new_constant(h, w, '.');
    for byte in &bytes[0..n] {
        map[byte] = '#';
    }

    let mut inc_queue = MinHeap::new(&|s1: &State, s2: &State| s1.1.cmp(&s2.1));
    let mut del_queue = MinHeap::new(&|s1: &State, s2: &State| s1.1.cmp(&s2.1));
    let mut visited: Map<u64> = map.same_size_with(u64::MAX);
    let target = Vec2i {
        x: w as i64 - 1,
        y: h as i64 - 1,
    };

    // compute initial costmap
    inc_queue.push((Vec2i { x: 0, y: 0 }, 0));
    incremental_djikstra(&map, &target, &mut inc_queue, &mut visited);

    // for each new obstacle make an incremental costmap update
    for p in &bytes[n..] {
        if visited[p] == u64::MAX {
            continue;
        }

        // clear out cost for everything downstream of 'visited'
        del_queue.push((*p, visited[p]));
        while let Some((del_node, del_cost)) = del_queue.pop() {
            let del_node_prev_cost = *visited.get(&del_node).unwrap_or(&u64::MAX);
            if del_node_prev_cost == u64::MAX {
                continue;
            }
            visited[&del_node] = u64::MAX;

            // Only recurse if cost for this node is consistent with cost of path through 'p' (i.e.
            // del_cost), and if there is no alternative path to 'del_node' that achieves same
            // cost. If we don't recurse this node becomes an initial node for incremental
            // djikstra.
            let mut recurse = false;
            if del_node_prev_cost == del_cost {
                let has_other_path = del_node != *p
                    && DIRECTIONS.iter().any(|pre_dir| {
                        let pre_node = del_node.step(*pre_dir, 1);
                        let pre_cost = *visited.get(&pre_node).unwrap_or(&u64::MAX);
                        pre_cost == del_node_prev_cost - 1
                    });
                recurse = !has_other_path;
            }

            if recurse {
                for dir in DIRECTIONS {
                    del_queue.push((del_node.step(dir, 1), del_cost + 1));
                }
            } else {
                inc_queue.push((del_node, del_node_prev_cost));
            }
        }

        // update map
        map[p] = '#';

        // run incremental djikstra starting from lower-cost nodes
        incremental_djikstra(&map, &target, &mut inc_queue, &mut visited);

        if visited[&target] == u64::MAX {
            return Answer::String(format!("{},{}", p.x, p.y).leak::<'static>());
        }
    }

    unreachable!();
}

pub fn part_b(input: &str) -> Answer {
    solve_part_b(input, 71, 71, 1024)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0
    "};

    #[test]
    fn test_part_a() {
        let result = solve_part_a(TEST_INPUT, 7, 7, 12);
        assert_eq!(result, Answer::Number(22));
    }

    #[test]
    fn test_part_b() {
        let result = solve_part_b(TEST_INPUT, 7, 7, 12);
        assert_eq!(result, Answer::String("6,1"));
    }
}