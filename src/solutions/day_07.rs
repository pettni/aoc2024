use crate::Answer;

#[derive(Debug, PartialEq)]
struct Problem {
    target: u64,
    numbers: Vec<u64>,
}

fn parse_problems(input: &str) -> Vec<Problem> {
    let mut ret = Vec::new();
    for line in input.lines() {
        let splits = line.split(": ").collect::<Vec<&str>>();
        assert_eq!(splits.len(), 2);
        let target = str::parse::<u64>(splits[0]).unwrap();
        let numbers = splits[1]
            .split(" ")
            .map(str::parse::<u64>)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        ret.push(Problem { target, numbers });
    }
    ret
}

fn int_len(x: u64) -> u32 {
    let mut xc = x;
    let mut res = 0;
    while xc > 0 {
        res += 1;
        xc /= 10;
    }
    res
}

fn iter(problem: &Problem, idx: usize, accum: u64, include_or: bool) -> bool {
    if idx == problem.numbers.len() {
        return accum == problem.target;
    }
    let el = problem.numbers[idx];
    let add_case = iter(problem, idx + 1, accum + el, include_or);
    let mul_case = iter(problem, idx + 1, accum * el, include_or);

    match include_or {
        true => {
            let el_len = int_len(el);
            let or_case = iter(problem, idx + 1, accum * 10u64.pow(el_len) + el, include_or);
            add_case || mul_case || or_case
        }
        false => add_case || mul_case,
    }
}

pub fn part_a(input: &str) -> Answer {
    let problems = parse_problems(input);
    let num_solvable: u64 = problems
        .into_iter()
        .filter(|problem| iter(problem, 0, 0, false))
        .map(|problem| -> u64 { problem.target })
        .sum();
    Answer::Number(num_solvable as i64)
}

pub fn part_b(input: &str) -> Answer {
    let problems = parse_problems(input);
    let num_solvable: u64 = problems
        .into_iter()
        .filter(|problem| iter(problem, 0, 0, true))
        .map(|problem| -> u64 { problem.target })
        .sum();
    Answer::Number(num_solvable as i64)
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
