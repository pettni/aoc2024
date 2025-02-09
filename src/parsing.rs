use nom::{
    character::complete::{anychar, digit1, multispace0, newline, not_line_ending, space0, space1},
    combinator::{all_consuming, complete, map_parser, map_res, verify},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded},
    IResult,
};

/// Parse a character and transform it with a function.
pub fn parse_char<'a, T>(
    input: &'a str,
    f: &dyn Fn(char) -> Result<T, String>,
) -> IResult<&'a str, T> {
    map_res(anychar, f)(input)
}

/// Parse non-empty string into a vector.
pub fn parse_vector<'a, T>(
    input: &'a str,
    f: &dyn Fn(char) -> Result<T, String>,
) -> IResult<&'a str, Vec<T>> {
    let line_as_str = verify(not_line_ending, |s: &str| !s.is_empty());
    let vec_of_t = map_parser(line_as_str, all_consuming(many1(|x| parse_char(x, f))));
    let mut parser = preceded(space0, vec_of_t); // ignore leading spaces
    parser(input)
}

/// Parse multi-line string into a 2D matrix.
pub fn parse_matrix_strict<'a, T>(
    input: &'a str,
    f: &dyn Fn(char) -> Result<T, String>,
) -> IResult<&'a str, Vec<Vec<T>>> {
    let mut parser = separated_list1(newline, |x| parse_vector(x, f));
    parser(input)
}

/// Parse a multi-line string into a 2D matrix.
/// Consumes and ignores leading and trailing newlines (if present).
pub fn parse_matrix<'a, T>(
    input: &'a str,
    f: &dyn Fn(char) -> Result<T, String>,
) -> IResult<&'a str, Vec<Vec<T>>> {
    let mut parser = delimited(multispace0, |x| parse_matrix_strict(x, f), multispace0);
    parser(input)
}

/// Parse rows as numbers.
pub fn parse_rows_of_ints(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    let row_parser = map_parser(
        preceded(space0, not_line_ending),
        separated_list1(space1, map_res(digit1, str::parse::<i64>)),
    );
    let mut rows_parser = complete(delimited(
        multispace0,
        separated_list1(newline, row_parser),
        multispace0,
    ));
    rows_parser(input)
}

/// Identity character parser.
pub fn identity(c: char) -> Result<char, String> {
    Ok(c)
}

/// Character parser for boolean.
pub fn char_to_bool(c: char) -> Result<bool, String> {
    match c {
        '1' => Ok(true),
        '0' => Ok(false),
        _ => Err(format!("Invalid bool {c}").to_string()),
    }
}

/// Character parser for int.
pub fn char_to_u32(c: char) -> Result<u32, String> {
    let tmp = c as u32 - '0' as u32;
    if tmp <= 10 {
        Ok(tmp)
    } else {
        Err("Parsing error".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error;

    #[test]
    fn test_parse_char() {
        // succeed on '1'
        assert_eq!(parse_char("123", &char_to_bool), Ok(("23", true)));
        // fail on '2'
        assert_eq!(
            parse_char("23", &char_to_bool),
            Err(nom::Err::Error(error::Error::new(
                "23",
                error::ErrorKind::MapRes
            )))
        );
        // fail on newline
        assert_eq!(
            parse_char("\n01", &char_to_bool),
            Err(nom::Err::Error(error::Error::new(
                "\n01",
                error::ErrorKind::MapRes
            )))
        );
        // fail on space
        assert_eq!(
            parse_char(" 01", &char_to_bool),
            Err(nom::Err::Error(error::Error::new(
                " 01",
                error::ErrorKind::MapRes
            )))
        );
    }

    #[test]
    fn test_parse_vector_char() {
        let vector_str = "01010abc";
        let result = parse_vector(vector_str, &identity);
        assert_eq!(
            result.unwrap().1,
            vec!['0', '1', '0', '1', '0', 'a', 'b', 'c']
        );
    }

    #[test]
    fn test_parse_vector_bool() {
        let vector_str = "01010101";
        let result = parse_vector(vector_str, &char_to_bool);
        assert_eq!(
            result.unwrap().1,
            vec![false, true, false, true, false, true, false, true]
        );
    }

    #[test]
    fn test_parse_vector_bool_bad() {
        let vector_str = "01010201";
        let result = parse_vector(vector_str, &char_to_bool);
        println!("{}", result.as_ref().unwrap_err());
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_matrix() {
        let matrix_str = r#"101010
010101
101010
010101"#;
        let result = parse_matrix(matrix_str, &char_to_bool);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().1,
            vec![
                vec![true, false, true, false, true, false],
                vec![false, true, false, true, false, true],
                vec![true, false, true, false, true, false],
                vec![false, true, false, true, false, true],
            ]
        );
    }

    #[test]
    fn test_parse_matrix_spaces() {
        let matrix_str = r#"
        101010
        010101
        101010
        010101
"#;
        let result = parse_matrix(matrix_str, &char_to_bool);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().1,
            vec![
                vec![true, false, true, false, true, false],
                vec![false, true, false, true, false, true],
                vec![true, false, true, false, true, false],
                vec![false, true, false, true, false, true],
            ]
        );
    }

    #[derive(Debug, PartialEq)]
    enum CellValue {
        Occupied,
        Free,
    }

    fn char_to_cellvalue(c: char) -> Result<CellValue, String> {
        match c {
            'X' => Ok(CellValue::Occupied),
            'O' => Ok(CellValue::Free),
            _ => Err(std::format_args!("Invalid value {}", c).to_string()),
        }
    }

    #[test]
    fn test_parse_to_enum() {
        let matrix_str = r#"
        XOXOXO
        OXOXOX"#;
        let result = parse_matrix(matrix_str, &char_to_cellvalue);
        assert!(result.is_ok());
        assert!(result.as_ref().unwrap().1[0]
            .iter()
            .step_by(2)
            .all(|x| *x == CellValue::Occupied));
        assert!(result.as_ref().unwrap().1[0]
            .iter()
            .skip(1)
            .step_by(2)
            .all(|x| *x == CellValue::Free));
    }
}
