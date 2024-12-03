use crate::Answer;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    combinator::{map, map_res},
    multi::{many1, many_till},
    sequence::delimited,
    IResult,
};

fn number_pair(input: &str) -> IResult<&str, (i64, i64)> {
    let mut number = map_res(digit1, str::parse::<i64>);
    let (input, o1) = number(input)?;
    let (input, _) = tag(",")(input)?;
    number(input).map(|(i, o2)| (i, (o1, o2)))
}

fn mul_expr(input: &str) -> IResult<&str, (i64, i64)> {
    let mut pattern = delimited(tag("mul("), number_pair, tag(")"));
    pattern(input)
}

fn mul_exprs(input: &str) -> IResult<&str, Vec<(i64, i64)>> {
    let take_one = map(many_till(anychar, mul_expr), |(_, y)| y);
    many1(take_one)(input)
}

pub fn part_a(input: &str) -> Answer {
    let (_, data) = mul_exprs(input).unwrap();
    let res = data.iter().map(|(x, y)| x * y).sum();
    Answer::Number(res)
}

#[derive(Debug)]
enum Token {
    Mul(i64),
    Do,
    Dont,
}

fn token_parser(input: &str) -> IResult<&str, Vec<Token>> {
    let mul_token = map(mul_expr, |(x, y)| Token::Mul(x * y));
    let do_token = map(tag("do()"), |_| Token::Do);
    let dont_token = map(tag("don't()"), |_| Token::Dont);

    let token = alt((mul_token, do_token, dont_token));

    let until_token = map(many_till(anychar, token), |(_, y)| y);
    let mut tokens = many1(until_token);

    tokens(input)
}

pub fn part_b(input: &str) -> Answer {
    let (_, data) = token_parser(input).unwrap();

    let folder = |(skip, cum), token: &Token| match token {
        Token::Mul(m) => (skip, if skip { cum } else { cum + m }),
        Token::Do => (false, cum),
        Token::Dont => (true, cum),
    };
    let (_, res) = data.iter().fold((false, 0), folder);

    Answer::Number(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const TEST_INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT1);
        assert_eq!(result, Answer::Number(161));
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT2);
        assert_eq!(result, Answer::Number(48));
    }
}
