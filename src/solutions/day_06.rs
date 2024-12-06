use crate::map2d::{Dir, Map, Pos};
use crate::parsing::{identity, parse_matrix};
use crate::Answer;
use bitvec::prelude::*;
use itertools::iproduct;

#[derive(Debug, PartialEq, Clone)]
enum Visited {
    Y,
    N,
}

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Free(Visited),
    Obstacle,
}

type State = (Pos, Dir);

fn parse_board(chars: &[Vec<char>]) -> (State, Map<Tile>) {
    let map = Map::from_vecs(
        chars
            .iter()
            .map(|vec| vec.iter().map(parse_tile).collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );

    let (r, c) = iproduct!(0..chars.len(), 0..chars[0].len())
        .filter(|(r, c)| chars[*r][*c] == '^')
        .next()
        .unwrap();

    ((Pos { r, c }, Dir::N), map)
}

// Make one step.
// Return None if we step outside of map.
fn step(state: &State, map: &Map<Tile>) -> Option<State> {
    let pos_new = map.step_within(&state.0, &state.1, 1)?;
    match map[&pos_new] {
        Tile::Free(_) => Some((pos_new, state.1)),
        Tile::Obstacle => Some((state.0, state.1.turn_right())),
    }
}

// Step until next obstacle and turn.
// Return None if we step outside of map.
fn quick_step(state: &State, map: &Map<Tile>) -> Option<State> {
    let mut pos = state.0;
    while let Some(pos_new) = map.step_within(&pos, &state.1, 1) {
        if map[&pos_new] == Tile::Obstacle {
            return Some((pos, state.1.turn_right()));
        }
        pos = pos_new;
    }
    None
}

fn parse_tile(c: &char) -> Tile {
    match c {
        '.' => Tile::Free(Visited::N),
        '#' => Tile::Obstacle,
        '^' => Tile::Free(Visited::N),
        _ => panic!("Unknown tile '{}'", c),
    }
}

pub fn part_a(input: &str) -> Answer {
    let (_, chars) = parse_matrix(input, &identity).unwrap();
    let (state_inner, map) = parse_board(&chars);
    let mut state: Option<State> = Some(state_inner);

    let mut visited = bitvec![0; 130 * 130];

    while let Some((pos, _)) = state {
        visited.set(pos.c * 130 + pos.r, true);
        state = state.and_then(|s| step(&s, &map));
    }

    let res = visited.count_ones();
    Answer::Number(res as i64)
}

fn has_loop(state0: &State, map: &Map<Tile>, visited_outer: &BitVec) -> bool {
    let mut state: Option<State> = Some(*state0);
    let mut visited = bitvec![0; 130 * 130 * 4];
    while let Some((pos, dir)) = state {
        let state_idx = pos.r * 130 * 4 + pos.c * 4 + dir as usize;
        if visited_outer[state_idx] || visited[state_idx] {
            return true;
        }
        visited.set(state_idx, true);
        state = state.and_then(|s| quick_step(&s, map));
    }
    false
}

pub fn part_b(input: &str) -> Answer {
    // Follow path as in part_a. At each state where there is no obstacle,
    // put an obstacle in front of the guard and unroll to see if we create a loop.

    let (_, chars) = parse_matrix(input, &identity).unwrap();
    let (state0, mut map) = parse_board(&chars);
    let mut state: Option<State> = Some(state0);
    let mut maybe_prev: Option<State> = None;

    // maintain visited states
    let mut visited_outer = bitvec![0; 130 * 130 * 4];
    let mut res = 0;
    while let Some((pos, _)) = state {
        if let Some(prev) = maybe_prev {
            if map[&pos] == Tile::Free(Visited::N) {
                map[&pos] = Tile::Obstacle;
                if has_loop(&prev, &map, &visited_outer) {
                    res += 1;
                }
                map[&pos] = Tile::Free(Visited::N);
            }
            let state_idx = prev.0.r * 130 * 4 + prev.0.c * 4 + prev.1 as usize;
            visited_outer.set(state_idx, true);
        }

        // update state
        map[&pos] = Tile::Free(Visited::Y);
        (maybe_prev, state) = (state, state.as_ref().and_then(|s| step(s, &map)));
    }

    Answer::Number(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
    "#;

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::Number(41));
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, Answer::Number(6));
    }
}
