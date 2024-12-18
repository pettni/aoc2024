use crate::heap::MinHeap;
use crate::map2d::Map;
use crate::vec2::{Dir, Vec2i};
use crate::Answer;

type State = (Vec2i, u64);

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .trim()
        .lines()
        .map(|l| {
            l.split(",")
                .flat_map(str::parse::<usize>)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn djikstra(map: &Map<char>, h: usize, w: usize) -> Option<u64> {
    let target = Vec2i {
        x: w as i64 - 1,
        y: h as i64 - 1,
    };

    let mut queue = MinHeap::new(&|s1: &State, s2: &State| s1.1.cmp(&s2.1));
    let mut visited = map.clone_with_value(false);

    queue.push((Vec2i { x: 0, y: 0 }, 0));
    while let Some((cur, cost)) = queue.pop() {
        if !map.get(&cur).map(|x| *x != '#').unwrap_or_default() {
            continue;
        }
        if visited[&cur] {
            continue;
        }
        visited[&cur] = true;
        if cur == target {
            return Some(cost);
        }

        for dir in [Dir::N, Dir::E, Dir::W, Dir::S] {
            queue.push((cur.step(dir, 1), cost + 1));
        }
    }

    None
}

fn solve_part_a(input: &str, h: usize, w: usize, n: usize) -> Answer {
    let bytes = parse(input);
    let mut map = Map::<char>::new_constant(h, w, '.');
    for byte in &bytes[0..n] {
        map[(byte[1], byte[0])] = '#';
    }
    let sol = djikstra(&map, h, w).unwrap();
    Answer::Number(sol as i64)
}

pub fn part_a(input: &str) -> Answer {
    solve_part_a(input, 71, 71, 1024)
}

fn solve_part_b(input: &str, h: usize, w: usize, n: usize) -> Answer {
    // Faster options:
    //  - use bucket queue for Djikstra
    //  - only run djikstra on tiles with higher cost than the one that was introduced
    //  - keep track of max-flow / min-cut
    let bytes = parse(input);
    let mut map = Map::<char>::new_constant(h, w, '.');
    for byte in &bytes[0..n] {
        map[(byte[1], byte[0])] = '#';
    }
    for i in n..bytes.len() {
        map[(bytes[i][1], bytes[i][0])] = '#';
        let sol = djikstra(&map, h, w);
        if sol.is_none() {
            return Answer::String(format!("{},{}", bytes[i][0], bytes[i][1]).leak::<'static>());
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
