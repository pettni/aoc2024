use crate::Answer;
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug)]
struct Problem {
    target: u64,
    numbers: Vec<u64>,
}

fn parse_problem(line: &str) -> Problem {
    let (target_str, numbers_str) = line.split(": ").collect_tuple::<(&str, &str)>().unwrap();
    let target = str::parse::<u64>(target_str).unwrap();
    let numbers = numbers_str
        .split(" ")
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    Problem { target, numbers }
}

fn parse_problems(input: &str) -> Vec<Problem> {
    input.lines().par_bridge().map(parse_problem).collect()
}

fn next_pow10(x: u64) -> u64 {
    let mut xc = x;
    let mut res = 1;
    while xc > 0 {
        res *= 10;
        xc /= 10;
    }
    res
}

fn is_solvable<const PARTB: bool>(problem: &Problem) -> bool {
    let n = problem.numbers.len();

    // stack-allocated 'stack' of fixed capacity
    // using an array + ptr
    let mut stack: [(u64, usize); 64] = [(0, 0); 64];
    let mut stackptr = 0; // next unused element

    stack[stackptr] = (problem.numbers[0], 1);
    stackptr += 1;

    while stackptr > 0 {
        let (total, idx) = stack[stackptr - 1];
        stackptr -= 1;
        if idx == n {
            if total == problem.target {
                return true;
            }
        } else if total <= problem.target {
            let x = problem.numbers[idx];
            stack[stackptr] = (total + x, idx + 1);
            stackptr += 1;
            if PARTB {
                stack[stackptr] = (total * next_pow10(x) + x, idx + 1);
                stackptr += 1;
            }
            stack[stackptr] = (total * x, idx + 1);
            stackptr += 1;
        }
    }
    false
}

pub fn part_a(input: &str) -> Answer {
    let res = parse_problems(input)
        .into_par_iter()
        .filter(is_solvable::<false>)
        .map(|p| p.target as i64)
        .sum();
    Answer::Number(res)
}

pub fn part_b(input: &str) -> Answer {
    let res = parse_problems(input)
        .into_par_iter()
        .filter(is_solvable::<true>)
        .map(|p| p.target as i64)
        .sum();
    Answer::Number(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    "};

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::Number(3749));
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, Answer::Number(11387));
    }
}
