use crate::map2d::Map;
use crate::math::{dot, nchoosek_iter};
use crate::vec2::{Dir, Vec2i};
use crate::Answer;
use std::collections::HashMap;

trait KeypadButtons {
    fn to_coord(d: Self) -> Vec2i;
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum NumButton {
    Digit(u32),
    A,
}

impl NumButton {
    fn from_char(c: char) -> Option<NumButton> {
        match c {
            'A' => Some(NumButton::A),
            x => Some(NumButton::Digit(x as u32 - '0' as u32)),
        }
    }
}

impl KeypadButtons for NumButton {
    fn to_coord(d: NumButton) -> Vec2i {
        match d {
            NumButton::A => Vec2i { x: 2, y: 3 },
            NumButton::Digit(0) => Vec2i { x: 1, y: 3 },
            NumButton::Digit(d) => Vec2i {
                x: ((d as i64 - 1) % 3),
                y: 2 - (d as i64 - 1) / 3,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DirButton {
    Dir(Dir),
    A,
}

impl KeypadButtons for DirButton {
    fn to_coord(d: DirButton) -> Vec2i {
        match d {
            DirButton::A => Vec2i { x: 2, y: 0 },
            DirButton::Dir(Dir::N) => Vec2i { x: 1, y: 0 },
            DirButton::Dir(Dir::W) => Vec2i { x: 0, y: 1 },
            DirButton::Dir(Dir::S) => Vec2i { x: 1, y: 1 },
            DirButton::Dir(Dir::E) => Vec2i { x: 2, y: 1 },
        }
    }
}

struct Keypads {
    dir_keypad: Map<Option<DirButton>>,
    num_keypad: Map<Option<NumButton>>,
}

impl Keypads {
    fn new() -> Self {
        let mut dir_keypad = Map::<Option<DirButton>>::new_constant(2, 3, None);
        for x in [
            DirButton::A,
            DirButton::Dir(Dir::N),
            DirButton::Dir(Dir::E),
            DirButton::Dir(Dir::S),
            DirButton::Dir(Dir::W),
        ] {
            dir_keypad[&DirButton::to_coord(x)] = Some(x);
        }

        let mut num_keypad = Map::<Option<NumButton>>::new_constant(4, 3, None);
        for x in [NumButton::A] {
            num_keypad[&NumButton::to_coord(x)] = Some(x)
        }
        for d in 0..=9 {
            let x = NumButton::Digit(d);
            num_keypad[&NumButton::to_coord(x)] = Some(x)
        }
        Self {
            dir_keypad,
            num_keypad,
        }
    }

    /// Step one robot.
    /// Input:
    ///  current state
    ///  input
    ///  map representing robot keypad (state space)
    ///
    /// Output:
    ///  updated state
    ///  output (if a button was pressed)
    fn step_robot<T: Copy>(
        state: &mut Vec2i,
        action: DirButton,
        map: &Map<Option<T>>,
    ) -> Option<T> {
        match action {
            DirButton::A => map[&*state],
            DirButton::Dir(d) => {
                *state = state.step(d, 1);
                None
            }
        }
    }

    /// Check if trajectory is valid.
    fn is_traj_valid<T: Copy>(init: &Vec2i, actions: &[DirButton], map: &Map<Option<T>>) -> bool {
        let mut s = *init;
        for a in actions {
            Keypads::step_robot(&mut s, *a, map);
            if !map.contains(&s) || map[&s].is_none() {
                return false;
            }
        }

        map.contains(&s) && map[&s].is_some()
    }
}

fn plan_motion<T: Copy + KeypadButtons>(
    init: T,
    desired_output: &[T],
    keypad: &Map<Option<T>>,
) -> Vec<Vec<DirButton>> {
    let mut cur_pos = T::to_coord(init);
    let mut results: Vec<Vec<DirButton>> = vec![vec![]];
    for output in desired_output.iter() {
        let Vec2i { x: dx, y: dy } = T::to_coord(*output) - cur_pos;
        let dx_unit = DirButton::Dir(if dx < 0 { Dir::W } else { Dir::E });
        let dy_unit = DirButton::Dir(if dy < 0 { Dir::N } else { Dir::S });
        results = results
            .iter()
            .flat_map(|prefix: &Vec<DirButton>| {
                let n = dx.abs() + dy.abs();
                let mut ret = vec![];
                for selection in nchoosek_iter(n as usize, dx.unsigned_abs() as usize) {
                    let mut suffix = vec![dy_unit; n as usize];
                    for dx_sel in selection {
                        suffix[dx_sel] = dx_unit;
                    }
                    if Keypads::is_traj_valid(&cur_pos, &suffix, keypad) {
                        ret.push(
                            prefix
                                .iter()
                                .cloned()
                                .chain(suffix.iter().cloned())
                                .chain([DirButton::A])
                                .collect(),
                        );
                    }
                }
                ret
            })
            .collect();
        cur_pos += &Vec2i { x: dx, y: dy };
    }

    results
}

fn create_sequences(
    r1_presses: &Vec<Vec<Vec<DirButton>>>,
    keypads: &Keypads,
) -> HashMap<Vec<DirButton>, Vec<Vec<DirButton>>> {
    let mut ret = HashMap::new();

    // insert all initial primitives
    for r1_action in r1_presses {
        for action_opt in r1_action {
            for seq in action_opt.split_inclusive(|x| *x == DirButton::A) {
                let motions = plan_motion(DirButton::A, seq, &keypads.dir_keypad);
                ret.insert(seq.to_vec(), motions);
            }
        }
    }

    // fill in missing seq2seq until it maps onto itself
    loop {
        let mut missing_seqs: Vec<Vec<DirButton>> = Vec::new();
        for seqs in ret.values() {
            for motion in seqs {
                for subseq in motion.split_inclusive(|x| *x == DirButton::A) {
                    if !ret.contains_key(subseq) {
                        missing_seqs.push(subseq.to_vec());
                    }
                }
            }
        }
        if missing_seqs.is_empty() {
            break;
        }
        for mseq in missing_seqs.iter() {
            let motions = plan_motion(DirButton::A, mseq, &keypads.dir_keypad);
            ret.insert(mseq.to_vec(), motions);
        }
    }

    ret
}

/// Create vector of motion primitive counts.
fn create_motion_profile(motion: &[DirButton], stoi: &HashMap<Vec<DirButton>, usize>) -> Vec<u64> {
    let mut ret = vec![0; stoi.len()];
    for seq in motion.split_inclusive(|x| *x == DirButton::A) {
        let seq_i = stoi[seq];
        ret[seq_i] += 1;
    }
    ret
}

fn solve(input: &str, num_robots: usize) -> Answer {
    // output
    // R3: <vA  <A   A  >>^A  vA  A <^A  >A <v<A >>^A  vA  ^A  <vA  >^A  <v<A  >^A  >A   A  vA  ^A                       -- len 68
    // R2:   v   <   <     A   >  >   ^   A    <    A   >   A    v    A     <    ^   A   A   >   A   <vA   A   A   >^A   -- len 28
    // R1:                 <              A         ^       A         >              ^   ^       A     v   v   v     A   -- len 12
    // O:                                 0                 2                                    9                   A   -- len 4

    // Observation: When a button gets pressed, all upstream robots are at 'A', and the acting robot is on the button.
    // We therefore solve the problem of 'how many keys to press to execute an A-ending sequence at R1'

    let keypads = Keypads::new();

    let words = input.trim().lines().collect::<Vec<_>>();

    // All O keypresses
    let o_presses = words
        .iter()
        .map(|w| w.chars().flat_map(NumButton::from_char).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Each 0 keypress can be realized by several R1 keypresses.
    let r1_presses: Vec<Vec<Vec<DirButton>>> = o_presses
        .iter()
        .map(|o_action| plan_motion(NumButton::A, o_action, &keypads.num_keypad))
        .collect::<Vec<_>>();

    // Map of Rk sequence -> R{k+1} sequences
    let seq2seq = create_sequences(&r1_presses, &keypads);

    // Indexing
    let mut stoi: HashMap<Vec<DirButton>, usize> = HashMap::new();
    let mut itos: HashMap<usize, Vec<DirButton>> = HashMap::new();
    for (i, k) in seq2seq.keys().enumerate() {
        stoi.insert(k.clone(), i);
        itos.insert(i, k.clone());
    }

    let nseq = seq2seq.len();

    // Profiles
    let profiles = (0..nseq)
        .map(|i| {
            let seq_i = &itos[&i];
            seq2seq[seq_i]
                .iter()
                .map(|motion| create_motion_profile(motion, &stoi))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Build out cost vector.
    // mseq[i]: number of button presses at Rk needed to generate sequence i at robot R0
    // Cost of a motion with profile d at robot 0: d[i]  m[r][i]

    // Start with diagonal matrix with sequence lengths.
    let mut mseq = (0..nseq).map(|i| itos[&i].len() as u64).collect::<Vec<_>>();

    // Update the costs: mseq_new[i] = d_i[k] mseq[k]
    // where d_i minimizes d_i[k] mseq[k]  among all motions that realize primitive i
    for _ in 1..num_robots {
        mseq = profiles
            .iter()
            .map(|ds_i| ds_i.iter().map(|d_i| dot(d_i, &mseq)).min().unwrap())
            .collect::<Vec<_>>();
    }

    // Can now compute the result for each word.
    let result = words
        .iter()
        .zip(r1_presses)
        .map(|(word, r1_action)| {
            let min_len = r1_action
                .iter()
                .map(|r1_input| {
                    let profile = create_motion_profile(r1_input, &stoi);
                    dot(&profile, &mseq)
                })
                .min()
                .unwrap();
            let action_num = word[0..3].parse::<u64>().unwrap();
            action_num * min_len
        })
        .sum::<u64>();

    Answer::Number(result as i64)
}

pub fn part_a(input: &str) -> Answer {
    solve(input, 3)
}

pub fn part_b(input: &str) -> Answer {
    solve(input, 26)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        029A
        980A
        179A
        456A
        379A
    "};

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::Number(126384));
    }

    #[test]
    fn test_iter() {
        let result = solve(TEST_INPUT, 3);
        assert_eq!(result, Answer::Number(126384));
    }
}
