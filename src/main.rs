use pulp::{lexer, Lexer, Regex, Statement};

fn main() {
    // let input = "aaabcbceebc";
    // let regex = Regex::new(r"a*(bc|e)+").unwrap();
    // if let Some(matches) = regex.matches(input) {
    //     println!("Input matches pattern: {}", matches);
    // }
    // let lexer = Lexer::new(Statement::);
    let lexer = Lexer::new(Statement::Save(
        "ident".to_string(),
        Box::new(Statement::Concat(
            vec![
                Statement::Condition {
                    cond: Box::new(Statement::Reader(Regex::new(r"@").unwrap())),
                    inner: Box::new(Statement::Concat(vec![
                        Statement::Reader(Regex::new(r"@").unwrap()),
                        Statement::Reader(Regex::new(r"\w+").unwrap()),
                        Statement::Reader(Regex::new(r"\n").unwrap()),
                    ])),
                },
                Statement::Reader(Regex::new(r"lexer").unwrap()),
                Statement::Save(
                    "block".to_string(),
                    Box::new(Statement::Concat(vec![
                        Statement::Reader(Regex::new(r"\s*{").unwrap()),
                        Statement::Reader(Regex::new(r"\s*}").unwrap()),
                    ])),
                ),
            ], // Box::new(Statement::Reader(Regex::new(r"\w+").unwrap())),
        )),
    ));

    if let Ok(lexer) = lexer {
        lexer.lex("@merge");
    }
    // if let Ok(lexer) = lexer {
    //     lexer.lex("aaabcbcee bc");
    // }
    // let input = "aaabcbceebc";
    // let regex = Regex::new(r"a*(bc|e)+").unwrap();
    // if let Some(matches) = regex.matches(input) {
    //     println!("Input matches pattern: {}", matches);
    // }
}
