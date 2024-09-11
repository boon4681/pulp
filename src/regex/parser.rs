use std::{iter::Peekable, str::Chars};

use super::instruction::Expr;

struct Parser<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser {
            input: input.chars().peekable(),
        }
    }

    fn parse(&mut self) -> Result<Expr, String> {
        self.parse_alternate()
    }

    fn parse_alternate(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_concat()?;

        while self.peek_char() == Some('|') {
            self.next_char();
            let right = self.parse_concat()?;
            expr = Expr::Alternate(Box::new(expr), Box::new(right));
        }

        Ok(expr)
    }

    fn parse_concat(&mut self) -> Result<Expr, String> {
        let mut exprs: Vec<Expr> = Vec::new();
        let mut current_text = String::new();
        loop {
            match self.peek_char() {
                Some(')') | Some('|') | None => break,
                _ => {
                    let expr = self.parse_repeat()?;
                    match expr {
                        Expr::Char(ch) => {
                            current_text.push(ch);
                        }
                        _ => {
                            match current_text.len() {
                                0 => {}
                                1 => exprs.push(Expr::Char(current_text.chars().next().unwrap())),
                                _ => exprs.push(Expr::Text(current_text)),
                            }
                            current_text = String::new();
                            exprs.push(expr);
                        }
                    }
                }
            }
        }
        match current_text.len() {
            0 => {}
            1 => exprs.push(Expr::Char(current_text.chars().next().unwrap())),
            _ => exprs.push(Expr::Text(current_text)),
        }

        Ok(match exprs.len() {
            0 => Expr::Text(String::new()),
            1 => exprs.pop().unwrap(),
            _ => exprs
                .into_iter()
                .rev()
                .reduce(|acc, e| Expr::Concat(Box::new(e), Box::new(acc)))
                .unwrap(),
        })
    }

    fn parse_repeat(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_atom()?;

        while let Some(ch) = self.peek_char() {
            match ch {
                '*' => {
                    self.next_char();
                    expr = Expr::ZeroOrMore(Box::new(expr));
                }
                '+' => {
                    self.next_char();
                    expr = Expr::OneOrMore(Box::new(expr));
                }
                '?' => {
                    self.next_char();
                    expr = Expr::ZeroOrOne(Box::new(expr));
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_atom(&mut self) -> Result<Expr, String> {
        match self.next_char() {
            Some('(') => {
                let expr = self.parse_alternate()?;
                if self.next_char() != Some(')') {
                    return Err("Expected closing parenthesis".to_string());
                }
                Ok(expr)
            }
            Some('.') => Ok(Expr::Any),
            Some('\\') => self.parse_escape(),
            Some(ch) => Ok(Expr::Char(ch)),
            None => Err("Unexpected end of input".to_string()),
        }
    }
    fn parse_escape(&mut self) -> Result<Expr, String> {
        match self.next_char() {
            Some(ch) => match ch {
                'n' => Ok(Expr::Char('\n')),
                'r' => Ok(Expr::Char('\r')),
                't' => Ok(Expr::Char('\t')),
                's' => Ok(Expr::AnyWhitespace),
                'S' => Ok(Expr::AnyNonWhitespace),
                'd' => Ok(Expr::AnyDigit),
                'D' => Ok(Expr::AnyNonDigit),
                '\\' | '.' | '(' | ')' | '[' | ']' | '{' | '}' | '*' | '+' | '?' | '^' | '$'
                | '|' => Ok(Expr::Char(ch)),
                _ => Err(format!("Invalid escape sequence: \\{}", ch)),
            },
            None => Err("Unexpected end of input after escape character".to_string()),
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        self.input.peek().copied()
    }

    fn next_char(&mut self) -> Option<char> {
        self.input.next()
    }
}

pub fn parse(input: &str) -> Result<Expr, String> {
    let mut parser = Parser::new(input);
    parser.parse()
}
