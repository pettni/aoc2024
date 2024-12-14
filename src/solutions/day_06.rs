use crate::map2d::Map;
use crate::vec2::{Dir, Vec2i};
use crate::Answer;
use bitvec::prelude::*;
use itertools::iproduct;
use rayon::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Visited {
    Y,
    N,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Free(Visited),
    Obstacle,
}

type State = (Vec2i, Dir);

fn parse_board(chars: &[&str]) -> (State, Map<Tile>) {
    let parse_tile = |c: char| -> Tile {
        match c {
            '.' => Tile::Free(Visited::N),
            '#' => Tile::Obstacle,
            '^' => Tile::Free(Visited::N),
            _ => panic!("Unknown tile '{}'", c),
        }
    };
    let map = Map::from_vecs(
        chars
            .iter()
            .map(|vec| vec.chars().map(parse_tile).collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );

    let (r, c) = iproduct!(0..chars.len(), 0..chars[0].len())
        .find(|(r, c)| chars[*r].as_bytes()[*c] == b'^')
        .unwrap();

    ((Vec2i::new(c as i64, r as i64), Dir::N), map)
}

// Make one step.
// Return None if we step outside of map.
fn step(state: &State, map: &Map<Tile>, extra_obs: Option<&Vec2i>) -> Option<State> {
    let pos_new = map.step_within(&state.0, state.1, 1)?;
    let new_tile = extra_obs
        .and_then(|x| {
            if *x == pos_new {
                Some(Tile::Obstacle)
            } else {
                None
            }
        })
        .unwrap_or(map[&pos_new]);
    match new_tile {
        Tile::Free(_) => Some((pos_new, state.1)),
        Tile::Obstacle => Some((state.0, state.1.turn_right())),
    }
}

pub fn part_a(input: &str) -> Answer {
    let chars: Vec<_> = input.lines().collect();
    let (state_inner, map) = parse_board(&chars);
    let mut state: Option<State> = Some(state_inner);

    let mut visited = bitvec![0; 130 * 130];

    while let Some((pos, _)) = state {
        visited.set(pos.linear_idx(130), true);
        state = state.and_then(|s| step(&s, &map, None));
    }

    let res = visited.count_ones();
    Answer::Number(res as i64)
}

// Step until next obstacle and turn.
// Return None if we step outside of map.
fn quick_step(state: &State, map: &Map<Tile>, extra_obs: &Vec2i) -> Option<State> {
    let mut pos = state.0;
    while let Some(pos_new) = map.step_within(&pos, state.1, 1) {
        if pos_new == *extra_obs || map[&pos_new] == Tile::Obstacle {
            return Some((pos, state.1.turn_right()));
        }
        pos = pos_new;
    }
    None
}

fn has_loop(state0: &State, map: &Map<Tile>, extra_obs: &Vec2i) -> bool {
    let mut state: Option<State> = Some(*state0);
    let mut visited = bitvec![0; 130 * 130 * 4];
    while let Some((pos, dir)) = state {
        let state_idx = 4 * pos.linear_idx(130) + dir as usize;
        if visited[state_idx] {
            return true;
        }
        visited.set(state_idx, true);
        state = state.and_then(|s| quick_step(&s, map, extra_obs));
    }
    false
}

pub fn part_b(input: &str) -> Answer {
    // Follow path as in part_a. At each state where there is no obstacle,
    // put an obstacle in front of the guard and unroll to see if we create a loop.

    let chars: Vec<_> = input.lines().collect();
    let (state0, mut map) = parse_board(&chars);
    let mut state: Option<State> = Some(state0);
    let mut maybe_prev: Option<State> = None;

    // collect candidate (init, obstacle_pos) pairs along the nominal path
    let mut candidate_loops: Vec<(State, Vec2i)> = Vec::new();
    while let Some((pos, _)) = state {
        if let Some(prev) = maybe_prev {
            if map[&pos] == Tile::Free(Visited::N) {
                candidate_loops.push((prev, pos));
            }
        }
        // update state
        map[&pos] = Tile::Free(Visited::Y);
        (maybe_prev, state) = (state, state.as_ref().and_then(|s| step(s, &map, None)));
    }

    // check each candidate for a loop in parallel
    let res = candidate_loops
        .par_iter()
        .filter(|(s0, obs_pos)| has_loop(s0, &map, obs_pos))
        .count();

    Answer::Number(res as i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
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
    "};

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
