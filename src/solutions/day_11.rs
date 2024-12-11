use crate::hash::{FxHashMap, FxHashMapBuilder};
use crate::math::number_length;
use crate::Answer;
use std::iter::successors;

type NumberCounter = FxHashMap<usize, usize>;

// Keep track of how many we have of each number.
fn transform_number_counter(counter: &NumberCounter) -> NumberCounter {
    let mut ret: NumberCounter = NumberCounter::with_capacity(4_000);
    for (x, c) in counter.iter() {
        let nl = number_length(*x as u64);
        match x {
            0 => {
                *ret.entry(1).or_default() += c;
            }
            x if nl % 2 == 0 => {
                let tmp = 10u64.pow(nl as u32 / 2) as usize;
                *ret.entry(x / tmp).or_default() += c;
                *ret.entry(x % tmp).or_default() += c;
            }
            _ => {
                *ret.entry(x * 2024).or_default() += c;
            }
        }
    }
    ret
}

// Recursive solution with memoization that maps
//   (x, i) -> #numbers in output
fn count_number_with_mem(x: usize, i: usize, mem: &mut FxHashMap<(usize, usize), usize>) -> usize {
    if i == 0 {
        return 1;
    } else if let Some(v) = mem.get(&(x, i)) {
        return *v;
    }

    let nl = number_length(x as u64);
    let res = match x {
        0 => count_number_with_mem(1, i - 1, mem),
        x if nl % 2 == 0 => {
            let tmp = 10u64.pow(nl as u32 / 2) as usize;
            count_number_with_mem(x / tmp, i - 1, mem) + count_number_with_mem(x % tmp, i - 1, mem)
        }
        _ => count_number_with_mem(x * 2024, i - 1, mem),
    };
    mem.insert((x, i), res);
    res
}

// Solve with aggregate transforms.
#[allow(dead_code)]
fn solve1(input: &str, num_iters: usize) -> Answer {
    let mut counter: NumberCounter = NumberCounter::with_capacity(4_000);
    for num in input.trim().split(" ").flat_map(str::parse::<i64>) {
        *counter.entry(num as usize).or_default() += 1;
    }
    let res = successors(Some(counter), |c| Some(transform_number_counter(c)))
        .nth(num_iters)
        .map(|r| r.values().sum::<usize>() as i64)
        .unwrap();
    Answer::Number(res)
}

// Solve with recursive memoization.
#[allow(dead_code)]
fn solve2(input: &str, num_iters: usize) -> Answer {
    let mut mem = FxHashMap::<(usize, usize), usize>::with_capacity(150_000);
    let res = input
        .trim()
        .split(" ")
        .flat_map(str::parse::<usize>)
        .map(|x| count_number_with_mem(x, num_iters, &mut mem))
        .sum::<usize>();
    Answer::Number(res as i64)
}

pub fn part_a(input: &str) -> Answer {
    solve2(input, 25)
}

pub fn part_b(input: &str) -> Answer {
    solve2(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "125 17";

    #[test]
    fn test_solve1_iter6() {
        let result = solve1(TEST_INPUT, 6);
        assert_eq!(result, Answer::Number(22));
    }

    #[test]
    fn test_solve1_iter25() {
        let result = solve1(TEST_INPUT, 25);
        assert_eq!(result, Answer::Number(55312));
    }

    #[test]
    fn test_solve2_iter6() {
        let result = solve2(TEST_INPUT, 6);
        assert_eq!(result, Answer::Number(22));
    }

    #[test]
    fn test_solve2_iter25() {
        let result = solve2(TEST_INPUT, 25);
        assert_eq!(result, Answer::Number(55312));
    }
}
