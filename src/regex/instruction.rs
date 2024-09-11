use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Char(char),
    Text(Vec<char>),
    Match,
    Jmp(usize),
    Split(usize, usize),
    Any,
    AnyWhitespace,
    AnyNonWhitespace,
    AnyDigit,
    AnyNonDigit,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Char(char),
    Text(String),
    Concat(Box<Expr>, Box<Expr>),
    Alternate(Box<Expr>, Box<Expr>),
    ZeroOrMore(Box<Expr>),
    OneOrMore(Box<Expr>),
    ZeroOrOne(Box<Expr>),
    Any,
    AnyWhitespace,
    AnyNonWhitespace,
    AnyDigit,
    AnyNonDigit,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Char(c) => write!(f, "char {}", c),
            Instruction::Text(s) => write!(f, "text {}", String::from_iter(s)),
            Instruction::Match => write!(f, "match"),
            Instruction::Jmp(i) => write!(f, "jmp {}", i),
            Instruction::Split(a, b) => write!(f, "split {} {}", a, b),
            Instruction::Any => write!(f, "any"),
            Instruction::AnyWhitespace => write!(f, "any ws"),
            Instruction::AnyNonWhitespace => write!(f, "any non_ws"),
            Instruction::AnyDigit => write!(f, "any digit"),
            Instruction::AnyNonDigit => write!(f, "any non_digit"),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Char(c) => write!(f, "Char({})", c),
            Expr::Text(s) => write!(f, "Text({})", s),
            Expr::Concat(left, right) => write!(f, "Concat({} {})", left, right),
            Expr::Alternate(left, right) => write!(f, "Alternate({} | {})", left, right),
            Expr::ZeroOrMore(expr) => write!(f, "{}*", expr),
            Expr::OneOrMore(expr) => write!(f, "{}+", expr),
            Expr::ZeroOrOne(expr) => write!(f, "{}?", expr),
            Expr::Any => write!(f, "."),
            Expr::AnyWhitespace => write!(f, r"\s"),
            Expr::AnyNonWhitespace => write!(f, r"\S"),
            Expr::AnyDigit => write!(f, r"\d"),
            Expr::AnyNonDigit => write!(f, r"\D"),
        }
    }
}
