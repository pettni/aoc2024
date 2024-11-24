use aoc2024::parsing;

fn main() {
    let input = r#"
        101010
        010101
        101010
        010101
        "#;

    let mat = parsing::parse_matrix(|c| c == '1', input);

    println!("Got matrix:\n {:?}", mat.unwrap().1);
}
