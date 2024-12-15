use crate::hash::{FxHashSet, FxHashSetBuilder};
use std::fmt;

use crate::map2d::Map;
use crate::vec2::{Dir, Vec2i};
use crate::Answer;

#[derive(Debug, PartialEq, Clone, Copy)]
enum BoxSide {
    L, // left
    R, // rght
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Wall,
    Box(BoxSide),
    Free,
    Robot,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let o = match &self {
            Self::Wall => '#',
            Self::Box(BoxSide::L) => '[',
            Self::Box(BoxSide::R) => ']',
            Self::Free => '.',
            Self::Robot => '@',
        };
        write!(f, "{}", o)
    }
}

impl Tile {
    pub fn from_char(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            'O' => Self::Box(BoxSide::L),
            '[' => Self::Box(BoxSide::L),
            ']' => Self::Box(BoxSide::R),
            '.' => Self::Free,
            '@' => Self::Robot,
            _ => unreachable!("Invalid tile"),
        }
    }
}

pub fn part_a(input: &str) -> Answer {
    solve::<true>(input)
}

pub fn part_b(input: &str) -> Answer {
    solve::<false>(input)
}

fn parse_input<const PARTA: bool>(input: &str) -> (Map<Tile>, Vec2i, Vec<Dir>) {
    let mut splits = input.trim().split("\n\n");
    let map_str = splits.next().unwrap();
    let move_str = splits.next().unwrap();

    let map = if PARTA {
        Map::from_lines(map_str.lines(), &Tile::from_char)
    } else {
        let iters = map_str.lines().map(|l| {
            l.chars()
                .flat_map(|c| match c {
                    '.' => ['.', '.'],
                    '#' => ['#', '#'],
                    'O' => ['[', ']'],
                    '@' => ['@', '.'],
                    _ => unreachable!(),
                })
                .map(Tile::from_char)
        });
        Map::from_iterators(iters)
    };
    let moves = move_str
        .chars()
        .filter_map(Dir::from_char)
        .collect::<Vec<_>>();

    let (robot_tile, _) = map.iter().find(|(_, v)| **v == Tile::Robot).unwrap();

    (map, robot_tile, moves)
}

fn solve<const PARTA: bool>(input: &str) -> Answer {
    let (mut map, mut state, moves) = parse_input::<PARTA>(input);
    moves.iter().for_each(|m| {
        step::<PARTA>(&mut state, &mut map, *m);
    });
    let result = map
        .iter()
        .filter_map(|(c, v)| match v {
            Tile::Box(BoxSide::L) => Some(100 * c.y + c.x),
            _ => None,
        })
        .sum();
    Answer::Number(result)
}

fn step<const PARTA: bool>(state: &mut Vec2i, map: &mut Map<Tile>, dir: Dir) {
    let to_tile = state.step(dir, 1);

    let move_boxes = if PARTA { move_boxes_a } else { move_boxes_b };

    match map[&to_tile] {
        Tile::Wall => {
            return;
        }
        Tile::Box(side) => {
            let box_to_move = match side {
                BoxSide::L => to_tile,
                BoxSide::R => to_tile.step(Dir::W, 1),
            };
            if !move_boxes(&box_to_move, map, dir) {
                return;
            }
        }
        Tile::Free => {}
        Tile::Robot => {
            unreachable!();
        }
    }

    // execute state update
    assert_eq!(map[&to_tile], Tile::Free);
    map[&to_tile] = Tile::Robot;
    map[&*state] = Tile::Free;
    *state = to_tile;
}

fn move_boxes_a(box_p: &Vec2i, map: &mut Map<Tile>, dir: Dir) -> bool {
    let mut next_free_tile = *box_p;
    while let Tile::Box(BoxSide::L) = map[&next_free_tile] {
        next_free_tile = next_free_tile.step(dir, 1);
    }

    match map[&next_free_tile] {
        Tile::Wall => return false,
        Tile::Free => {
            map[&next_free_tile] = Tile::Box(BoxSide::L);
            map[box_p] = Tile::Free;
        }
        _ => {
            unreachable!();
        }
    }

    true
}

fn move_boxes_b(box_p: &Vec2i, map: &mut Map<Tile>, dir: Dir) -> bool {
    // Double DFS search to recursively move all blocking stones
    let mut visited: FxHashSet<Vec2i> = FxHashSet::new();
    let mut move_stack: Vec<(Vec2i, bool)> = Vec::new();
    move_stack.push((*box_p, false));

    let mut boxes_to_move: Vec<Vec2i> = vec![];

    while let Some((cur_l, free)) = move_stack.pop() {
        assert_eq!(map[&cur_l], Tile::Box(BoxSide::L));

        if free {
            // all downstream boxes are processed and added earlier in 'boxes_to_move'
            boxes_to_move.push(cur_l);
        } else {
            if visited.contains(&cur_l) {
                continue;
            }
            visited.insert(cur_l);
            move_stack.push((cur_l, true));

            // tiles that box must occupy after move
            let next_l = cur_l.step(dir, 1);
            let next_r = next_l.step(Dir::E, 1);

            // tiles occupied after move, but not before move
            let new_tiles: [Option<Vec2i>; 2] = if dir == Dir::N || dir == Dir::S {
                [Some(next_l), Some(next_r)]
            } else if dir == Dir::W {
                [Some(next_l), None]
            } else {
                [Some(next_r), None]
            };

            // make sure each tile in 'new_tiles' can be unblocked by moving other boxes
            for new_tile in new_tiles.iter().flatten() {
                match map[new_tile] {
                    Tile::Wall => {
                        return false;
                    }
                    Tile::Box(BoxSide::L) => {
                        move_stack.push((*new_tile, false));
                    }
                    Tile::Box(BoxSide::R) => {
                        move_stack.push((new_tile.step(Dir::W, 1), false));
                    }
                    Tile::Free => (),
                    Tile::Robot => unreachable!(),
                }
            }
        }
    }

    for cur_l in boxes_to_move.iter() {
        let cur_r = cur_l.step(Dir::E, 1);
        let next_l = cur_l.step(dir, 1);
        let next_r = cur_r.step(dir, 1);
        map[cur_l] = Tile::Free;
        map[&cur_r] = Tile::Free;
        map[&next_l] = Tile::Box(BoxSide::L);
        map[&next_r] = Tile::Box(BoxSide::R);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT_S1: &str = indoc! {"
        ########
        #..O.O.#
        ##@.O..#
        #...O..#
        #.#.O..#
        #...O..#
        #......#
        ########

        <^^>>>vv<v>>v<<
    "};

    const TEST_INPUT_S2: &str = indoc! {"
        #######
        #...#.#
        #.....#
        #..OO@#
        #..O..#
        #.....#
        #######

        <vv<<^^<<^^
    "};

    #[test]
    fn test_part_a_s() {
        let result = part_a(TEST_INPUT_S1);
        assert_eq!(result, Answer::Number(2028));
    }

    #[test]
    fn test_part_b_s() {
        let result = part_b(TEST_INPUT_S2);
        assert_eq!(result, Answer::Number(618));
    }

    const TEST_INPUT: &str = indoc! {"
        ##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########

        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    "};

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::Number(10092));
    }

    // #[test]
    // fn test_part_b() {
    //     let result = part_b(TEST_INPUT);
    //     assert_eq!(result, Answer::Number(9021));
    // }
}
