#[cfg(test)]
mod simple_tests {
    use instruction::Expr;
    use pulp::regex::{instruction, parser};
    use parser::parse;

    #[test]
    fn test_char() {
        let regex = "a";
        let parsed = parse(regex).unwrap();
        assert_eq!(parsed, Expr::Char('a'));
    }

    #[test]
    fn test_any() {
        let regex = ".";
        let parsed = parse(regex).unwrap();
        assert_eq!(parsed, Expr::Any);
    }

    #[test]
    fn test_zero_or_more() {
        let regex = "a*";
        let parsed = parse(regex).unwrap();
        assert_eq!(parsed, Expr::ZeroOrMore(Box::new(Expr::Char('a'))));
    }

    #[test]
    fn test_one_or_more() {
        let regex = "a+";
        let parsed = parse(regex).unwrap();
        assert_eq!(parsed, Expr::OneOrMore(Box::new(Expr::Char('a'))));
    }

    #[test]
    fn test_zero_or_one() {
        let regex = "a?";
        let parsed = parse(regex).unwrap();
        assert_eq!(parsed, Expr::ZeroOrOne(Box::new(Expr::Char('a'))));
    }

    #[test]
    fn test_concat() {
        let regex = "ab";
        let parsed = parse(regex).unwrap();
        assert_eq!(parsed, Expr::Text(String::from("ab")));
    }

    #[test]
    fn test_alternate() {
        let regex = "a|b";
        let parsed = parse(regex).unwrap();
        assert_eq!(
            parsed,
            Expr::Alternate(Box::new(Expr::Char('a')), Box::new(Expr::Char('b')))
        );
    }

    #[test]
    fn test_group() {
        let regex = "(a|b)";
        let parsed = parse(regex).unwrap();
        assert_eq!(
            parsed,
            Expr::Alternate(Box::new(Expr::Char('a')), Box::new(Expr::Char('b')))
        );
    }

    #[test]
    fn test_complex() {
        let regex = "(a|b)*c";
        let parsed = parse(regex).unwrap();
        assert_eq!(
            parsed,
            Expr::Concat(
                Box::new(Expr::ZeroOrMore(Box::new(Expr::Alternate(
                    Box::new(Expr::Char('a')),
                    Box::new(Expr::Char('b'))
                )))),
                Box::new(Expr::Char('c'))
            )
        );
    }

    #[test]
    fn test_mismatched_parentheses() {
        let regex = "(a|b";
        let parsed = parse(regex);
        assert!(parsed.is_err());
    }

    #[test]
    fn test_nested_group_with_alternate() {
        let regex = "(a(b|c)d)";
        let parsed = parse(regex).unwrap();
        assert_eq!(
            parsed,
            Expr::Concat(
                Box::new(Expr::Char('a')),
                Box::new(Expr::Concat(
                    Box::new(Expr::Alternate(
                        Box::new(Expr::Char('b')),
                        Box::new(Expr::Char('c'))
                    )),
                    Box::new(Expr::Char('d')),
                )),
            )
        );
    }

    #[test]
    fn test_alternate_concat_group() {
        let regex = "(a|b)c";
        let parsed = parse(regex).unwrap();
        assert_eq!(
            parsed,
            Expr::Concat(
                Box::new(Expr::Alternate(
                    Box::new(Expr::Char('a')),
                    Box::new(Expr::Char('b'))
                )),
                Box::new(Expr::Char('c'))
            )
        );
    }

    #[test]
    fn test_nested_zero_or_more() {
        let regex = "a*(bc)*";
        let parsed = parse(regex).unwrap();
        assert_eq!(
            parsed,
            Expr::Concat(
                Box::new(Expr::ZeroOrMore(Box::new(Expr::Char('a')))),
                Box::new(Expr::ZeroOrMore(Box::new(Expr::Text(String::from("bc")))))
            )
        );
    }

    #[test]
    fn test_multiple_alternate_with_concat() {
        let regex = "a|b|c";
        let parsed = parse(regex).unwrap();
        assert_eq!(
            parsed,
            Expr::Alternate(
                Box::new(Expr::Alternate(
                    Box::new(Expr::Char('a')),
                    Box::new(Expr::Char('b')),
                )),
                Box::new(Expr::Char('c'))
            )
        );
    }

    #[test]
    fn test_complex_nested_groups() {
        let regex = "((a|b)c*)|(d|e)+";
        let parsed = parse(regex).unwrap();
        assert_eq!(
            parsed,
            Expr::Alternate(
                Box::new(Expr::Concat(
                    Box::new(Expr::Alternate(
                        Box::new(Expr::Char('a')),
                        Box::new(Expr::Char('b'))
                    )),
                    Box::new(Expr::ZeroOrMore(Box::new(Expr::Char('c'))))
                )),
                Box::new(Expr::OneOrMore(Box::new(Expr::Alternate(
                    Box::new(Expr::Char('d')),
                    Box::new(Expr::Char('e'))
                ))))
            )
        );
    }

    #[test]
    fn test_multiple_quantifiers_in_sequence() {
        let regex = "a*b+c?";
        let parsed = parse(regex).unwrap();
        assert_eq!(
            parsed,
            Expr::Concat(
                Box::new(Expr::ZeroOrMore(Box::new(Expr::Char('a')))),
                Box::new(Expr::Concat(
                    Box::new(Expr::OneOrMore(Box::new(Expr::Char('b')))),
                    Box::new(Expr::ZeroOrOne(Box::new(Expr::Char('c'))))
                )),
            )
        );
    }

    #[test]
    fn test_complex_combination() {
        let regex = "(a|b)*c+(d(e|f)?)*";
        let parsed = parse(regex).unwrap();
        assert_eq!(
            parsed,
            Expr::Concat(
                Box::new(Expr::ZeroOrMore(Box::new(Expr::Alternate(
                    Box::new(Expr::Char('a')),
                    Box::new(Expr::Char('b'))
                )))),
                Box::new(Expr::Concat(
                    Box::new(Expr::OneOrMore(Box::new(Expr::Char('c')))),
                    Box::new(Expr::ZeroOrMore(Box::new(Expr::Concat(
                        Box::new(Expr::Char('d')),
                        Box::new(Expr::ZeroOrOne(Box::new(Expr::Alternate(
                            Box::new(Expr::Char('e')),
                            Box::new(Expr::Char('f'))
                        ))))
                    ))))
                )),
            )
        );
    }

    #[test]
    fn test_group_with_concatenation_and_quantifier() {
        let regex = "(ab)+";
        let parsed = parse(regex).unwrap();
        assert_eq!(
            parsed,
            Expr::OneOrMore(Box::new(Expr::Text(String::from("ab"))))
        );
    }
    #[test]
    fn test_escape_backslash() {
        let regex = r"\\";
        let parsed = parse(regex).unwrap();
        assert_eq!(parsed, Expr::Char('\\'));
    }

    #[test]
    fn test_escape_digit() {
        let regex = r"\d";
        let parsed = parse(regex).unwrap();
        assert_eq!(parsed, Expr::AnyDigit);
    }
    #[test]
    fn test_escape_non_digit() {
        let regex = r"\D";
        let parsed = parse(regex).unwrap();
        assert_eq!(parsed, Expr::AnyNonDigit);
    }
    #[test]
    fn test_escape_word() {
        let regex = r"\w";
        let parsed = parse(regex).unwrap();
        assert_eq!(parsed, Expr::AnyWord);
    }
    #[test]
    fn test_escape_non_word() {
        let regex = r"\W";
        let parsed = parse(regex).unwrap();
        assert_eq!(parsed, Expr::AnyNonWord);
    }

    #[test]
    fn test_escape_newline() {
        let regex = r"\n";
        let parsed = parse(regex).unwrap();
        assert_eq!(parsed, Expr::Char('\n'));
    }
    #[test]
    fn test_escape_whitespace() {
        let regex = r"\s";
        let parsed = parse(regex).unwrap();
        assert_eq!(parsed, Expr::AnyWhitespace);
    }

    #[test]
    fn test_escape_non_whitespace() {
        let regex = r"\S";
        let parsed = parse(regex).unwrap();
        assert_eq!(parsed, Expr::AnyNonWhitespace);
    }
}
