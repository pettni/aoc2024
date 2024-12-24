use crate::hash::*;
use crate::Answer;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn from_str(s: &str) -> Self {
        match s {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => unreachable!(),
        }
    }

    fn eval(&self, left: u32, rght: u32) -> u32 {
        match self {
            Op::And => left & rght,
            Op::Or => left | rght,
            Op::Xor => left ^ rght,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Init<'a> {
    name: &'a str,
    val: u32,
}

impl<'a> Init<'a> {
    fn from_line(line: &'a str) -> Self {
        let mut spl = line.split(": ");
        let name = spl.next().unwrap();
        let val = spl.next().unwrap().parse::<u32>().unwrap();
        Self { name, val }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Transition<'a> {
    left: &'a str,
    op: Op,
    rght: &'a str,
    to: &'a str,
}

impl<'a> Transition<'a> {
    fn from_line(line: &'a str) -> Self {
        let mut spl = line.split(" ");

        let left = spl.next().unwrap();
        let op = Op::from_str(spl.next().unwrap());
        let rght = spl.next().unwrap();
        spl.next(); // space
        let to = spl.next().unwrap();

        Transition { left, rght, op, to }
    }
}

fn parse(input: &str) -> (Vec<Init>, Vec<Transition>) {
    let mut line_spl = input.trim().split("\n\n");
    let init = line_spl
        .next()
        .unwrap()
        .lines()
        .map(Init::from_line)
        .collect::<Vec<_>>();
    let transitions = line_spl
        .next()
        .unwrap()
        .lines()
        .map(Transition::from_line)
        .collect::<Vec<_>>();
    (init, transitions)
}

pub fn part_a(input: &str) -> Answer {
    let (init, transitions) = parse(input);

    let mut known_values: FxHashMap<&str, u32> = FxHashMap::new();
    for Init { name, val } in init.iter() {
        known_values.insert(name, *val);
    }

    let mut result: u64 = 0;
    let mut done = false;
    while !done {
        done = true;
        for Transition { left, op, rght, to } in transitions.iter() {
            if known_values.contains_key(to) {
                continue;
            }
            done = false;
            if !known_values.contains_key(left) || !known_values.contains_key(rght) {
                continue;
            }
            let left_val = known_values[left];
            let rght_val = known_values[rght];
            let new_val = op.eval(left_val, rght_val);

            if to.as_bytes()[0] == b'z' && new_val > 0 {
                // add to result
                let idx = to[1..].parse::<u32>().unwrap();
                result |= 1 << idx;
            }
            known_values.insert(to, new_val);
        }
    }

    Answer::Number(result as i64)
}

fn solve_part_b(input: &str, num_swaps: u32) -> Answer {
    Answer::String("Unimpl")
}

pub fn part_b(input: &str) -> Answer {
    solve_part_b(input, 4)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT_S: &str = indoc! {"
        x00: 1
        x01: 1
        x02: 1
        y00: 0
        y01: 1
        y02: 0

        x00 AND y00 -> z00
        x01 XOR y01 -> z01
        x02 OR y02 -> z02
    "};

    const TEST_INPUT_L: &str = indoc! {"
        x00: 1
        x01: 0
        x02: 1
        x03: 1
        x04: 0
        y00: 1
        y01: 1
        y02: 1
        y03: 1
        y04: 1

        ntg XOR fgs -> mjb
        y02 OR x01 -> tnw
        kwq OR kpj -> z05
        x00 OR x03 -> fst
        tgd XOR rvg -> z01
        vdt OR tnw -> bfw
        bfw AND frj -> z10
        ffh OR nrd -> bqk
        y00 AND y03 -> djm
        y03 OR y00 -> psh
        bqk OR frj -> z08
        tnw OR fst -> frj
        gnj AND tgd -> z11
        bfw XOR mjb -> z00
        x03 OR x00 -> vdt
        gnj AND wpb -> z02
        x04 AND y00 -> kjc
        djm OR pbm -> qhw
        nrd AND vdt -> hwm
        kjc AND fst -> rvg
        y04 OR y02 -> fgs
        y01 AND x02 -> pbm
        ntg OR kjc -> kwq
        psh XOR fgs -> tgd
        qhw XOR tgd -> z09
        pbm OR djm -> kpj
        x03 XOR y03 -> ffh
        x00 XOR y04 -> ntg
        bfw OR bqk -> z06
        nrd XOR fgs -> wpb
        frj XOR qhw -> z04
        bqk OR frj -> z07
        y03 OR x01 -> nrd
        hwm AND bqk -> z03
        tgd XOR rvg -> z12
        tnw OR pbm -> gnj
    "};

    #[test]
    fn test_part_a_s() {
        let result = part_a(TEST_INPUT_S);
        assert_eq!(result, Answer::Number(4));
    }

    #[test]
    fn test_part_a_m() {
        let result = part_a(TEST_INPUT_L);
        assert_eq!(result, Answer::Number(2024));
    }

    const TEST_INPUT_B: &str = indoc! {"
        x00: 0
        x01: 1
        x02: 0
        x03: 1
        x04: 0
        x05: 1
        y00: 0
        y01: 0
        y02: 1
        y03: 1
        y04: 0
        y05: 1

        x00 AND y00 -> z05
        x01 AND y01 -> z02
        x02 AND y02 -> z01
        x03 AND y03 -> z03
        x04 AND y04 -> z04
        x05 AND y05 -> z00
    "};

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT_B);
        assert_eq!(result, Answer::String("z00,z01,z02,z05"));
    }
}
