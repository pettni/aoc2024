use crate::Answer;

pub fn part_a(input: &str) -> Answer {
    let _ = input;
    Answer::default()
}

pub fn part_b(input: &str) -> Answer {
    let _ = input;
    Answer::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        1 2 3 4 5
        1 2 3 4 5
    "};

    #[test]
    fn test_part_a() {
        let result = part_a(TEST_INPUT);
        assert_eq!(result, Answer::Unimplemented);
    }

    #[test]
    fn test_part_b() {
        let result = part_b(TEST_INPUT);
        assert_eq!(result, Answer::Unimplemented);
    }
}
