use nom::{character, combinator, multi, sequence, IResult};

/// Parse a character into a type.
pub fn parse_char<T>(f: fn(char) -> Result<T, String>, input: &str) -> IResult<&str, T> {
    combinator::map_res(character::complete::anychar, f)(input)
}

/// Parse a single-line string into a vector.
pub fn parse_vector<T>(f: fn(char) -> Result<T, String>, input: &str) -> IResult<&str, Vec<T>> {
    let line_parser = multi::many1(|x| parse_char(f, x));
    let mut line_parser_spaces = sequence::preceded(character::complete::space0, line_parser);
    line_parser_spaces(input)
}

/// Parse a multi-line string into a 2D matrix.
pub fn parse_matrix<T>(
    f: fn(char) -> Result<T, String>,
    input: &str,
) -> IResult<&str, Vec<Vec<T>>> {
    let mat_parser =
        multi::separated_list1(character::complete::newline, |x| parse_vector::<T>(f, x));
    let mat_parser_spaces = sequence::delimited(
        combinator::opt(character::complete::newline),
        mat_parser,
        combinator::opt(character::complete::newline),
    );
    let mut mat_parser_spaces_eof = sequence::terminated(mat_parser_spaces, combinator::eof);
    mat_parser_spaces_eof(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error;

    pub fn char_to_bool(c: char) -> Result<bool, String> {
        match c {
            '1' => Ok(true),
            '0' => Ok(false),
            _ => Err("Invalid bool".to_string()),
        }
    }

    #[test]
    fn test_parse_char() {
        // succeed on '1'
        assert_eq!(parse_char(char_to_bool, "123"), Ok(("23", true)));
        // fail on '2'
        assert_eq!(
            parse_char(char_to_bool, "23"),
            Err(nom::Err::Error(error::Error::new(
                "23",
                error::ErrorKind::MapRes
            )))
        );
        // fail on newline
        assert_eq!(
            parse_char(char_to_bool, "\n01"),
            Err(nom::Err::Error(error::Error::new(
                "\n01",
                error::ErrorKind::MapRes
            )))
        );
        // fail on space
        assert_eq!(
            parse_char(char_to_bool, " 01"),
            Err(nom::Err::Error(error::Error::new(
                " 01",
                error::ErrorKind::MapRes
            )))
        );
    }

    #[test]
    fn test_parse_vector() {
        let vector_str = "01010101";
        let result = parse_vector(char_to_bool, vector_str);
        assert_eq!(
            result.unwrap().1,
            vec![false, true, false, true, false, true, false, true]
        );
    }

    #[test]
    fn test_parse_matrix() {
        let matrix_str = r#"101010
010101
101010
010101"#;
        let result = parse_matrix(char_to_bool, matrix_str);
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
        let result = parse_matrix(char_to_bool, matrix_str);
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
        let result = parse_matrix(char_to_cellvalue, matrix_str);
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
