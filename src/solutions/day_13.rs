use crate::math::diophantine;
use crate::{vec2::Vec2i, Answer};

#[derive(Debug, PartialEq)]
struct Problem {
    button_a: Vec2i,
    button_b: Vec2i,
    price: Vec2i,
}

fn parse_vec2i(input: &str, delimiter: &str) -> Vec2i {
    let mut parts = input.trim().split(": ").nth(1).unwrap().split(", ");
    let x = str::parse::<i64>(parts.next().unwrap().split(delimiter).nth(1).unwrap()).unwrap();
    let y = str::parse::<i64>(parts.next().unwrap().split(delimiter).nth(1).unwrap()).unwrap();
    Vec2i { x, y }
}

fn parse_problem(input: &str) -> Problem {
    let mut lines = input.trim().lines();
    let button_a = parse_vec2i(lines.next().unwrap(), "+");
    let button_b = parse_vec2i(lines.next().unwrap(), "+");
    let price = parse_vec2i(lines.next().unwrap(), "=");
    Problem {
        button_a,
        button_b,
        price,
    }
}

pub fn part_a(input: &str) -> Answer {
    let result = input
        .trim()
        .split("\n\n")
        .map(parse_problem)
        .flat_map(|p| solve_problem(&p))
        .sum::<i64>();
    Answer::Number(result)
}

fn solve_problem(problem: &Problem) -> Option<i64> {
    // Problem
    //    na * [ax; ay] + nb * [bx; by] = [cx; cy]
    // can be expressed by a system of Diophantine equations
    //    [ax bx] [na]  =  [cx]
    //    [ay by] [nb]     [cy]

    // find all solutions (na + k u, nb0 + k v) of equation in x
    let (na0, nb0, u, v) = diophantine(problem.button_a.x, problem.button_b.x, problem.price.x)?;

    // substitute solutions into y equation
    // ay na + by nb = cy
    // ay (na0 + k u) + by (nb0 + k v) = cy
    // ay na0 + ay u k + by nb0 + by v k = cy
    // (ay u + by v) k = cy - ay na0 - by nb0
    let ay = problem.button_a.y;
    let by = problem.button_b.y;
    let k_num = problem.price.y - ay * na0 - by * nb0;
    let k_denom = ay * u + by * v;

    match k_denom {
        0 => {
            // infinite solutions, assuming this doesn't happen
            unreachable!("Assuming single solution");
        }
        kd if k_num % k_denom == 0 => {
            // one solution
            let k = k_num / kd;
            let x = na0 + k * u;
            let y = nb0 + k * v;
            Some(3 * x + y)
        }
        _ => {
            // no solutions
            None
        }
    }
}

pub fn part_b(input: &str) -> Answer {
    let result = input
        .trim()
        .split("\n\n")
        .map(parse_problem)
        .map(|p| Problem {
            button_a: p.button_a,
            button_b: p.button_b,
            price: p.price + 10000000000000,
        })
        .flat_map(|p| solve_problem(&p))
        .sum::<i64>();
    Answer::Number(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279
    "};

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::Number(480));
    }
}
