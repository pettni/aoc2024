use crate::Answer;

#[derive(Debug, Copy, Clone)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

fn parse_input(input: &str) -> (Registers, Vec<u64>) {
    let mut lines = input.trim().lines();
    let mut reg_parse = || {
        lines
            .next()
            .and_then(|l| l.split(": ").nth(1))
            .and_then(|s: &str| match str::parse::<u64>(s) {
                Ok(x) => Some(x),
                Err(_) => None,
            })
            .unwrap()
    };
    let a = reg_parse();
    let b = reg_parse();
    let c = reg_parse();
    lines.next(); // skip empty
    let program = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(",")
        .flat_map(str::parse::<u64>)
        .collect::<Vec<_>>();

    (Registers { a, b, c }, program)
}

fn step(ptr: usize, program: &[u64], registers: &mut Registers) -> (usize, Option<u64>) {
    let opcode = program[ptr];
    let operand = program[ptr + 1];

    let combo_op = match operand {
        x if (0..=3).contains(&x) => x,
        4 => registers.a,
        5 => registers.b,
        6 => registers.c,
        _ => unreachable!(),
    };

    let mut stdout: Option<u64> = None;
    match opcode {
        0 => registers.a /= 2u64.pow(combo_op as u32),
        1 => registers.b ^= operand,
        2 => registers.b = combo_op % 8,
        3 => {
            if registers.a != 0 {
                return (0, None);
            }
        }
        4 => registers.b ^= registers.c,
        5 => stdout = Some(combo_op % 8),
        6 => registers.b = registers.a / 2u64.pow(combo_op as u32),
        7 => registers.c = registers.a / 2u64.pow(combo_op as u32),
        _ => unreachable!(),
    }

    (ptr + 2, stdout)
}

fn execute(program: &[u64], registers: &mut Registers) -> Vec<u64> {
    let mut ptr = 0;
    let mut stdout = Vec::new();
    let mut maybe_stdout: Option<u64>;

    while ptr < program.len() {
        (ptr, maybe_stdout) = step(ptr, program, registers);
        if let Some(out) = maybe_stdout {
            stdout.push(out);
        }
    }

    stdout
}

pub fn part_a(input: &str) -> Answer {
    let (mut registers, program) = parse_input(input);
    let stdout = execute(&program, &mut registers);

    Answer::String(
        stdout
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
            .leak::<'static>(),
    )
}

fn step_program(program: &[u64], registers: &mut Registers) -> (bool, u64) {
    let mut ptr = 0;
    let mut maybe_out: Option<u64>;
    let mut stdout: Option<u64> = None;
    for _ in 0..program.len() / 2 {
        (ptr, maybe_out) = step(ptr, program, registers);
        if let Some(out) = maybe_out {
            assert!(stdout.is_none());
            stdout = Some(out);
        }
    }

    (ptr == 0, stdout.unwrap())
}

pub fn part_b(input: &str) -> Answer {
    let (mut registers, program) = parse_input(input);
    // Program facts:
    //  8 instructions
    //    0: 2 4  modify b              [set to a mod 8]
    //    1: 1 1  modify b              [xor with 1 = 0001]
    //    2: 7 5  modify c              [=a/2**b]
    //    3: 1 5  modify b              [xor with 5 = 0101]
    //    4: 4 2  modify b              [^= c]
    //    5: 5 5  out                   [print register b]
    //    6: 0 3  decrease register a   [divide by 8]
    //    7: 3 0  jump                  [restart if register a > 0]

    // a,          b,                      c
    // a,        a%8,                      c
    // a,    (a%8)^1,                      c
    // a,    (a%8)^1,                      a/2^((a%8)^1)
    // a,    (a%8)^4,                      a/2^((a%8)^1)
    // a     ((a%8)^4)^(a/2**((a%8)^1))    a/2**((a%8)^1)
    // a/8

    // Observations:
    // * Each run has one output.
    // * The output only depends on the value of a.
    //   - In particular, it only depends on the first 11 bits of a
    // * After each run the last 3 bits of a are dropped.
    // * After each run the pointer is reset to 0.
    //
    // For a program to output N numbers, the a register must be within [8^(N-1), 8^N)
    //
    // We build up the bits of a from the low to high, filtering out combinations
    // that do not produce the correct series of outputs
    let mut potential_solutions: Vec<(u64, usize)> = Vec::new();

    // First initialize by detecting all combinations of first 11 bits
    // that result in the first output value.
    for bits_to_add in 0..2u64.pow(11) {
        registers.a = bits_to_add;
        let (not_finished, out) = step_program(&program, &mut registers);
        if not_finished && out == program[0] {
            potential_solutions.push((bits_to_add, 0));
        }
    }

    // Now iterate over final options.
    let mut final_candidates: Vec<u64> = vec![];
    while let Some((head, it)) = potential_solutions.pop() {
        if head >= 8u64.pow(program.len() as u32 - 1) {
            // number is already long enough
            final_candidates.push(head);
        } else {
            for bits_to_add in 0..8 {
                let new_value = head | bits_to_add << (11 + 3 * it);
                registers.a = new_value >> (3 * (it + 1)); // drop tail (instead running first)
                let (not_finished, out) = step_program(&program, &mut registers);
                if not_finished && out == program[it + 1] {
                    potential_solutions.push((new_value, it + 1));
                }
            }
        }
    }

    // Select the best result
    let result = final_candidates
        .iter()
        .filter(|c| {
            registers.a = **c;
            program == execute(&program, &mut registers)
        })
        .min()
        .unwrap();

    Answer::Number(*result as i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part_a_1() {
        let mut registers = Registers { a: 0, b: 0, c: 9 };
        let program = vec![2, 6];
        let stdout = execute(&program, &mut registers);
        assert_eq!(registers.b, 1);
        assert_eq!(stdout, vec![]);
    }

    #[test]
    fn test_part_a_2() {
        let mut registers = Registers { a: 10, b: 0, c: 0 };
        let program = vec![5, 0, 5, 1, 5, 4];
        let stdout = execute(&program, &mut registers);
        assert_eq!(stdout, vec![0, 1, 2]);
    }

    const TEST_INPUT: &str = indoc! {"
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
    "};

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::String("4,6,3,5,6,3,5,2,1,0"));
    }

    const TEST_INPUT_B: &str = indoc! {"
        Register A: 2024
        Register B: 0
        Register C: 0

        Program: 0,3,5,4,3,0
    "};

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT_B);
        assert_eq!(result, Answer::Number(117440));
    }
}
